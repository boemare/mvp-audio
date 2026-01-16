use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

use crate::events::HotkeyAction;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct HotkeyConfig {
    pub id: String,
    pub keys: Vec<String>,
    pub enable_lock: bool,
    pub lock_tap_count: u8,
    pub unlock_tap_count: u8,
    pub tap_timeout_ms: u64,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            keys: Vec::new(),
            enable_lock: true,
            lock_tap_count: 2,
            unlock_tap_count: 3,
            tap_timeout_ms: 400,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyState {
    Idle,
    Pressed,
    Held,
    Locked,
}

pub struct HotkeyTracker {
    state: HotkeyState,
    last_tap: Option<Instant>,
    tap_count: u8,
    config: HotkeyConfig,
}

impl HotkeyTracker {
    pub fn new(config: HotkeyConfig) -> Self {
        Self {
            state: HotkeyState::Idle,
            last_tap: None,
            tap_count: 0,
            config,
        }
    }

    pub fn on_key_down(&mut self) -> Option<HotkeyAction> {
        let now = Instant::now();

        if let Some(last) = self.last_tap {
            if now.duration_since(last) < Duration::from_millis(self.config.tap_timeout_ms) {
                self.tap_count += 1;

                if self.config.enable_lock {
                    if self.tap_count >= self.config.lock_tap_count
                        && self.state == HotkeyState::Held
                    {
                        self.state = HotkeyState::Locked;
                        self.tap_count = 0;
                        return Some(HotkeyAction::Lock);
                    }
                    if self.tap_count >= self.config.unlock_tap_count
                        && self.state == HotkeyState::Locked
                    {
                        self.state = HotkeyState::Idle;
                        self.tap_count = 0;
                        return Some(HotkeyAction::Unlock);
                    }
                }
            } else {
                self.tap_count = 1;
            }
        } else {
            self.tap_count = 1;
        }

        self.last_tap = Some(now);

        if self.state == HotkeyState::Idle {
            self.state = HotkeyState::Pressed;
            return Some(HotkeyAction::Activate);
        }

        None
    }

    pub fn on_key_held(&mut self) {
        if self.state == HotkeyState::Pressed {
            self.state = HotkeyState::Held;
        }
    }

    pub fn on_key_up(&mut self) -> Option<HotkeyAction> {
        match self.state {
            HotkeyState::Locked => None,
            HotkeyState::Pressed | HotkeyState::Held => {
                self.state = HotkeyState::Idle;
                Some(HotkeyAction::Deactivate)
            }
            HotkeyState::Idle => None,
        }
    }

    pub fn get_state(&self) -> HotkeyState {
        self.state
    }

    pub fn get_config(&self) -> &HotkeyConfig {
        &self.config
    }
}

pub struct HotkeysState {
    inner: Arc<Mutex<HotkeysStateInner>>,
}

struct HotkeysStateInner {
    hotkeys: HashMap<String, HotkeyTracker>,
    pressed_keys: HashSet<String>,
    is_listening: bool,
}

impl Default for HotkeysState {
    fn default() -> Self {
        Self::new()
    }
}

impl HotkeysState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HotkeysStateInner {
                hotkeys: HashMap::new(),
                pressed_keys: HashSet::new(),
                is_listening: false,
            })),
        }
    }

    pub async fn register_hotkey(&self, config: HotkeyConfig) {
        let mut inner = self.inner.lock().await;
        let id = config.id.clone();
        inner.hotkeys.insert(id, HotkeyTracker::new(config));
    }

    pub async fn unregister_hotkey(&self, id: &str) -> bool {
        let mut inner = self.inner.lock().await;
        inner.hotkeys.remove(id).is_some()
    }

    pub async fn get_registered_hotkeys(&self) -> Vec<HotkeyConfig> {
        let inner = self.inner.lock().await;
        inner
            .hotkeys
            .values()
            .map(|t| t.get_config().clone())
            .collect()
    }

    pub async fn set_listening(&self, listening: bool) {
        let mut inner = self.inner.lock().await;
        inner.is_listening = listening;
    }

    pub async fn is_listening(&self) -> bool {
        let inner = self.inner.lock().await;
        inner.is_listening
    }

    pub async fn on_key_event(
        &self,
        key: String,
        pressed: bool,
    ) -> Vec<(String, HotkeyAction)> {
        let mut inner = self.inner.lock().await;
        let mut actions = Vec::new();

        if pressed {
            inner.pressed_keys.insert(key.clone());
        } else {
            inner.pressed_keys.remove(&key);
        }

        let pressed_keys_snapshot = inner.pressed_keys.clone();

        for (id, tracker) in inner.hotkeys.iter_mut() {
            let hotkey_keys: HashSet<_> = tracker.get_config().keys.iter().cloned().collect();

            let all_pressed = hotkey_keys.iter().all(|k| pressed_keys_snapshot.contains(k));

            if pressed && all_pressed {
                if let Some(action) = tracker.on_key_down() {
                    actions.push((id.clone(), action));
                }
            } else if !pressed && hotkey_keys.contains(&key) {
                if let Some(action) = tracker.on_key_up() {
                    actions.push((id.clone(), action));
                }
            }
        }

        actions
    }

    pub async fn get_pressed_keys(&self) -> Vec<String> {
        let inner = self.inner.lock().await;
        inner.pressed_keys.iter().cloned().collect()
    }
}
