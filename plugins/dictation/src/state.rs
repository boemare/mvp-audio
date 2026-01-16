use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

use crate::events::DictationPhase;

pub struct DictationState {
    inner: Arc<Mutex<DictationStateInner>>,
}

struct DictationStateInner {
    phase: DictationPhase,
    started_at: Option<Instant>,
    audio_samples: Vec<f32>,
    sample_rate: u32,
}

impl Default for DictationState {
    fn default() -> Self {
        Self::new()
    }
}

impl DictationState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(DictationStateInner {
                phase: DictationPhase::Idle,
                started_at: None,
                audio_samples: Vec::new(),
                sample_rate: 16000,
            })),
        }
    }

    pub async fn get_phase(&self) -> DictationPhase {
        self.inner.lock().await.phase
    }

    pub async fn set_phase(&self, phase: DictationPhase) {
        let mut inner = self.inner.lock().await;
        inner.phase = phase;
        if phase == DictationPhase::Recording {
            inner.started_at = Some(Instant::now());
            inner.audio_samples.clear();
        }
    }

    pub async fn start_recording(&self, sample_rate: u32) -> crate::Result<()> {
        let mut inner = self.inner.lock().await;
        if inner.phase != DictationPhase::Idle {
            return Err(crate::Error::InvalidState(format!(
                "Cannot start recording in {:?} state",
                inner.phase
            )));
        }
        inner.phase = DictationPhase::Recording;
        inner.started_at = Some(Instant::now());
        inner.audio_samples.clear();
        inner.sample_rate = sample_rate;
        Ok(())
    }

    pub async fn add_samples(&self, samples: &[f32]) {
        let mut inner = self.inner.lock().await;
        if inner.phase == DictationPhase::Recording || inner.phase == DictationPhase::Locked {
            inner.audio_samples.extend_from_slice(samples);
        }
    }

    pub async fn lock_recording(&self) -> crate::Result<()> {
        let mut inner = self.inner.lock().await;
        if inner.phase != DictationPhase::Recording {
            return Err(crate::Error::InvalidState(format!(
                "Cannot lock in {:?} state",
                inner.phase
            )));
        }
        inner.phase = DictationPhase::Locked;
        Ok(())
    }

    pub async fn stop_recording(&self) -> crate::Result<(Vec<f32>, u32, u64)> {
        let mut inner = self.inner.lock().await;
        if inner.phase != DictationPhase::Recording && inner.phase != DictationPhase::Locked {
            return Err(crate::Error::InvalidState(format!(
                "Cannot stop in {:?} state",
                inner.phase
            )));
        }

        let duration_ms = inner
            .started_at
            .map(|t| t.elapsed().as_millis() as u64)
            .unwrap_or(0);

        let samples = std::mem::take(&mut inner.audio_samples);
        let sample_rate = inner.sample_rate;

        inner.phase = DictationPhase::Processing;
        inner.started_at = None;

        Ok((samples, sample_rate, duration_ms))
    }

    pub async fn cancel(&self) {
        let mut inner = self.inner.lock().await;
        inner.phase = DictationPhase::Idle;
        inner.started_at = None;
        inner.audio_samples.clear();
    }

    pub async fn complete(&self) {
        let mut inner = self.inner.lock().await;
        inner.phase = DictationPhase::Idle;
    }
}
