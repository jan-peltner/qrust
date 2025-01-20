use url::ParseError;

pub trait AsStaticStr {
    fn as_static_str(&self) -> &'static str;
}

/// state machine that handles the ui focus
#[derive(PartialEq)]
pub enum Focus {
    QueryEditor,
    ResponseView,
}

impl AsStaticStr for Focus {
    fn as_static_str(&self) -> &'static str {
        match self {
            Focus::QueryEditor => "QUERY EDITOR",
            Focus::ResponseView => "RESPONSE VIEW",
        }
    }
}

pub struct AppState<'a> {
    pub name: &'a str,
    pub focus: Focus,
    pub query: String,
    pub response: Option<String>,
    pub should_quit: bool,
}

impl<'a> AppState<'a> {
    /// Initializes the app state
    pub fn init(name: &'a str) -> Result<Self, ParseError> {
        Ok(Self {
            name,
            focus: Focus::QueryEditor,
            query: String::default(),
            response: None,
            should_quit: false,
        })
    }

    pub fn handle_focus_transition(&mut self) {
        match self.focus {
            Focus::QueryEditor => self.focus = Focus::ResponseView,
            Focus::ResponseView => self.focus = Focus::QueryEditor,
        }
    }

    pub fn set_query(&mut self, input: String) {
        self.query = input;
    }
}
