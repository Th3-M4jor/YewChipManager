use serde::Deserialize;

#[derive(Deserialize, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
pub(crate) enum Skills {
    Perception,
    Info,
    Tech,
    Strength,
    Agility,
    Endurance,
    Charm,
    Valor,
    Affinity,
    None,
    Varies,
}

impl Skills {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Skills::Perception => "PER",
            Skills::Info => "INF",
            Skills::Tech => "TCH",
            Skills::Strength => "STR",
            Skills::Agility => "AGI",
            Skills::Endurance => "END",
            Skills::Charm => "CHM",
            Skills::Valor => "VLR",
            Skills::Affinity => "AFF",
            Skills::None => "--",
            Skills::Varies => "VAR",
        }
    }
}