pub struct AppState {
    pub query_url: String,
    pub res: Option<String>,
}

impl AppState {
    pub fn init() -> Self {
        Self {
            query_url: String::new(),
            res: None,
        }
    }

    pub fn append_to_query(&mut self, c: char) {
        self.query_url.push(c);
    }

    pub fn pop_from_query(&mut self) {
        self.query_url.pop();
    }

    pub fn clear_query(&mut self) {
        self.query_url.clear();
    }
}
