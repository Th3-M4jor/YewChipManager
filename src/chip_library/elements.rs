use serde::Deserialize;

#[derive(Deserialize, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
pub enum Elements {
    Fire,
    Aqua,
    Elec,
    Wood,
    Wind,
    Sword,
    Break,
    Cursor,
    Recovery,
    Invis,
    Object,
    Null,
}