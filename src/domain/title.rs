#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Title {
    pub value: String,
}

impl Title {
    pub fn new(value: &str) -> Self {
        Title {
            value: value.to_string(),
        }
    }
}
