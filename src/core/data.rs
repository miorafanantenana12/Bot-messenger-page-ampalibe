use serde::{Deserialize, Serialize};

const MAX_VALUE_AUTORIZED: usize = 500;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page(pub usize, pub usize);

impl Default for Page {
    fn default() -> Self {
        Self(0, 10)
    }
}

trait Verify: ToString {
    fn verify(&self) -> String;
}

impl Verify for String {
    fn verify(&self) -> String {
        if self.len() >= MAX_VALUE_AUTORIZED {
            self[..MAX_VALUE_AUTORIZED].to_string()
        } else {
            self.clone()
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Data {
    value: String,
    page: Option<Page>,
}

impl Data {
    pub fn new<T: Serialize>(value: T, page: Option<Page>) -> Self {
        let value = serde_json::to_string(&value).unwrap_or_default().verify();
        Self { value, page }
    }

    pub fn from_string<T: ToString>(s: T) -> Self {
        serde_json::from_str(&s.to_string()).unwrap_or_default()
    }

    pub fn get_value<T: for<'a> Deserialize<'a> + Default>(&self) -> T {
        serde_json::from_str::<T>(&self.value).unwrap_or_default()
    }

    pub fn get_page(&self) -> Option<Page> {
        self.page.clone()
    }
}
