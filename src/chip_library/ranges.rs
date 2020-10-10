use serde::Deserialize;

#[derive(Deserialize, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
pub(crate) enum Ranges {
    Varies,
    Far,
    Near,
    Close,
    #[serde(rename(deserialize = "Self"))]
    Itself,
}

impl Ranges {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Ranges::Itself => "Self",
            Ranges::Close => "Close",
            Ranges::Near => "Near",
            Ranges::Far => "Far",
            Ranges::Varies => "Var",
        }
    }
}