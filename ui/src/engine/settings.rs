pub struct Settings {
    pub maximized: bool,
    pub fullscreen: bool,
    pub resizable: bool,
    pub initial_window_size: Option<egui::Vec2>,
    pub initial_window_postion: Option<egui::Pos2>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            maximized: false,
            fullscreen: false,
            resizable: true,
            initial_window_size: None,
            initial_window_postion: None,
        }
    }
}
