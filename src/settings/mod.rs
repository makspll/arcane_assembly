pub struct WindowSettings {
    // TODO: persisted user settings + UI
    pub width: u32,
    pub height: u32,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
        }
    }
}
