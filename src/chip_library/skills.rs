use serde::Deserialize;

#[derive(Deserialize, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
pub enum Skills {
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

impl std::fmt::Display for Skills {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Skills::Sense => write!(f, "Sense"),
            Skills::Info => write!(f, "Info"),
            Skills::Coding => write!(f, "Coding"),
            Skills::Strength => write!(f, "Strength"),
            Skills::Speed => write!(f, "Speed"),
            Skills::Stamina => write!(f, "Stamina"),
            Skills::Charm => write!(f, "Charm"),
            Skills::Bravery => write!(f, "Bravery"),
            Skills::Affinity => write!(f, "Affinity"),
            Skills::None => write!(f, "--"),
            Skills::Varies => write!(f, "Varies"),
        }
    }
}