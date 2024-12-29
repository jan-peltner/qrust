pub trait AsStaticStr {
    fn as_static_str(&self) -> &'static str;
}

pub enum UiMode {
    NORMAL,
    EDITING,
}

impl AsStaticStr for UiMode {
    fn as_static_str(&self) -> &'static str {
        match self {
            UiMode::NORMAL => "SELECT",
            UiMode::EDITING => "EDIT",
        }
    }
}

pub enum HttpVerb {
    POST,
    GET,
    PUT,
    DELETE,
    PATCH,
}

impl AsStaticStr for HttpVerb {
    fn as_static_str(&self) -> &'static str {
        match self {
            HttpVerb::POST => "POST",
            HttpVerb::GET => "GET",
            HttpVerb::PUT => "PUT",
            HttpVerb::DELETE => "DELETE",
            HttpVerb::PATCH => "PATCH",
        }
    }
}

pub struct Param {
    key: String,
    value: String,
}

pub struct Query {
    pub url: String,
    pub verb: HttpVerb,
    pub params: Option<Vec<Param>>,
}

impl Query {
    pub fn append(&mut self, c: char) {
        self.url.push(c);
    }

    pub fn pop(&mut self) {
        self.url.pop();
    }

    pub fn clear(&mut self) {
        self.url.clear();
    }
}

pub struct AppState {
    pub mode: UiMode,
    pub query: Query,
    pub response: Option<String>,
}

impl AppState {
    /// Initializes the app state
    pub fn init(http_verb: Option<HttpVerb>) -> Self {
        if let Some(verb) = http_verb {
            AppState {
                mode: UiMode::NORMAL,
                query: Query {
                    url: String::new(),
                    verb,
                    params: None,
                },
                response: None,
            }
        } else {
            AppState {
                mode: UiMode::NORMAL,
                query: Query {
                    url: String::new(),
                    verb: HttpVerb::GET,
                    params: None,
                },
                response: None,
            }
        }
    }

    pub fn toggle_ui_mode(&mut self) {
        match self.mode {
            UiMode::EDITING => self.mode = UiMode::NORMAL,
            UiMode::NORMAL => self.mode = UiMode::EDITING,
        }
    }
}
