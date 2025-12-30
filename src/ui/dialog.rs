pub struct DialogInfo {
    pub title: &'static str,
    pub message: String,
    pub shown: bool,
}

impl Default for DialogInfo {
    fn default() -> Self {
        Self {
            title: "Title",
            message: "Message".into(),
            shown: true,
        }
    }
}
