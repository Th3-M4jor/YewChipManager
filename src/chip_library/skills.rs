use serde::Deserialize;

#[derive(Deserialize, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
pub(crate) enum Skills {
    Sense,
    Info,
    Coding,
    Strength,
    Speed,
    Stamina,
    Charm,
    Bravery,
    Affinity,
    None,
    Varies,
}

impl Skills {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Skills::Sense => "Sense",
            Skills::Info => "Info",
            Skills::Coding => "Coding",
            Skills::Strength => "Strength",
            Skills::Speed => "Speed",
            Skills::Stamina => "Stamina",
            Skills::Charm => "Charm",
            Skills::Bravery => "Bravery",
            Skills::Affinity => "Affinity",
            Skills::None => "--",
            Skills::Varies => "Varies",
        }
    }
}