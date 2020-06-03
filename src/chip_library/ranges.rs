use serde::Deserialize;

#[derive(Deserialize, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
pub enum Ranges {
    Far,
    Near,
    Close,
    #[serde(rename(deserialize = "Self"))]
    Itself,
}

impl std::fmt::Display for Ranges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ranges::Itself => write!(f, "Self"),
            Ranges::Close => write!(f, "Close"),
            Ranges::Near => write!(f, "Near"),
            Ranges::Far => write!(f, "Far"),
        }
    }
}