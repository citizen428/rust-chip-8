use crate::chip8;

pub struct WebSpeaker {}

impl WebSpeaker {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        WebSpeaker {}
    }
}

impl chip8::Speaker for WebSpeaker {
    fn beep(&mut self, _status: bool) {
        // TODO: Implement Web Audio API
    }
}
