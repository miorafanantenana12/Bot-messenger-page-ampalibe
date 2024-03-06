use crate::query::Query;

pub struct AppState {
    pub query: Query,
}

impl AppState {
    pub fn init() -> Self {
        let query: Query = Query::new();
        Self { query }
    }
}
