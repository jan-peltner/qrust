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

pub struct Query {
    pub url: Url,
    pub headers: HeaderMap,
}

pub struct AppState {
    pub name: String,
    pub mode: UiMode,
    pub query: Query,
    pub response: Option<String>,
}

impl AppState {
    /// Initializes the app state
    pub fn init(headers: HeaderMap, endpoint: String, name: String) -> Result<Self, ParseError> {
        Ok(Self {
            name,
            mode: UiMode::NORMAL,
            query: Query {
                url: Url::parse(&endpoint)?,
                headers,
            },
            response: None,
        })
    }

    pub fn toggle_ui_mode(&mut self) {
        match self.mode {
            UiMode::INSERT => self.mode = UiMode::NORMAL,
            UiMode::NORMAL => self.mode = UiMode::INSERT,
        }
    }
}
