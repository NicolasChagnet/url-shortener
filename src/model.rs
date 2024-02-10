use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Url {
    from: String,
    to: String
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UrlTo {
    pub to: String
}

impl Url {
    pub fn new(from: &str, to: &str) -> Url {
        Url {
            from: from.to_string(),
            to: to.to_string()
        }
    }
    pub fn get_from(&self) -> &str { &self.from }
    pub fn get_to(&self) -> &str { &self.to }
}