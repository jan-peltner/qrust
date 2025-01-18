use reqwest::header::HeaderMap;
use reqwest::Url;
use url::ParseError;

pub trait AsStaticStr {
    fn as_static_str(&self) -> &'static str;
}

pub enum UiMode {
    NORMAL,
    INSERT,
}

impl AsStaticStr for UiMode {
    fn as_static_str(&self) -> &'static str {
        match self {
            UiMode::NORMAL => "SELECT",
            UiMode::INSERT => "EDIT",
        }
    }
}

pub struct AppState<'a> {
    pub name: &'a str,
    pub mode: UiMode,
    pub query: String,
    pub response: Option<String>,
    pub should_quit: bool,
}

impl<'a> AppState<'a> {
    /// Initializes the app state
    pub fn init(name: &'a str) -> Result<Self, ParseError> {
        Ok(Self {
            name,
            mode: UiMode::NORMAL,
            query: String::default(),
            response: None,
            should_quit: false,
        })
    }

    /// Toggle UI between NORMAL and INSERT mode
    pub fn toggle_ui_mode(&mut self) {
        match self.mode {
            UiMode::INSERT => self.mode = UiMode::NORMAL,
            UiMode::NORMAL => self.mode = UiMode::INSERT,
        }
    }
}
