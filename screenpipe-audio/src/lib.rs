pub mod audio_processing;
mod core;
pub mod encode;
mod multilingual;
pub mod pcm_decode;
pub mod pyannote;
pub mod stt;
mod tokenizer;
pub mod vad_engine;
pub mod whisper;

pub use core::{
    default_input_device, default_output_device, get_device_and_config, list_audio_devices,
    parse_audio_device, record_and_transcribe, trigger_audio_permission, AudioDevice, AudioStream,
    AudioTranscriptionEngine, DeviceControl, DeviceType, LAST_AUDIO_CAPTURE,
};
pub use encode::encode_single_audio;
pub use pcm_decode::pcm_decode;
pub use stt::{create_whisper_channel, resample, stt, AudioInput, TranscriptionResult};
pub use vad_engine::VadEngineEnum;

use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref GLOBAL_WHISPER_MODEL: Arc<Mutex<Option<WhisperModel>>> =
        Arc::new(Mutex::new(None));
}

pub async fn initialize_whisper_model(
    audio_transcription_engine: &AudioTranscriptionEngine,
) -> Result<()> {
    let model = WhisperModel::new(audio_transcription_engine)?;
    let mut global_model = GLOBAL_WHISPER_MODEL.lock().await;
    *global_model = Some(model);
    Ok(())
}
