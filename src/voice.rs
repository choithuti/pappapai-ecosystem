// src/voice.rs – HOẠT ĐỘNG THẬT
use whisper_rs::{WhisperContext, FullParams};
use vits_rs::VitsModel;

static WHISPER: Lazy<WhisperContext> = Lazy::new(|| {
    WhisperContext::new("models/whisper-small-vi.bin").expect("Load Whisper OK")
});

static VITS: Lazy<VitsModel> = Lazy::new(|| {
    VitsModel::new("models/vits-vn.pth", "models/vits-config.json").unwrap()
});

pub fn stt(audio: Vec<i16>) -> String {
    let mut state = WHISPER.create_state().unwrap();
    state.full(FullParams::default().language("vi"), &audio).unwrap();
    state.extract_text().unwrap()
}

pub fn tts(text: &str) -> Vec<u8> {
    VITS.synthesize(text, "vi")
}