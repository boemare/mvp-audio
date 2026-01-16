use tauri::Manager;

use crate::events::DictationPhase;
use crate::state::DictationState;

pub struct Dictation<'a, R: tauri::Runtime, M: Manager<R>> {
    manager: &'a M,
    _runtime: std::marker::PhantomData<fn() -> R>,
}

impl<'a, R: tauri::Runtime, M: Manager<R>> Dictation<'a, R, M> {
    pub async fn get_phase(&self) -> DictationPhase {
        let state = self.manager.state::<DictationState>();
        state.get_phase().await
    }

    pub async fn is_recording(&self) -> bool {
        let phase = self.get_phase().await;
        matches!(phase, DictationPhase::Recording | DictationPhase::Locked)
    }
}

pub trait DictationPluginExt<R: tauri::Runtime> {
    fn dictation(&self) -> Dictation<'_, R, Self>
    where
        Self: Manager<R> + Sized;
}

impl<R: tauri::Runtime, T: Manager<R>> DictationPluginExt<R> for T {
    fn dictation(&self) -> Dictation<'_, R, Self>
    where
        Self: Sized,
    {
        Dictation {
            manager: self,
            _runtime: std::marker::PhantomData,
        }
    }
}
