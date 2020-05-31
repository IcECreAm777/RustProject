pub mod pokemon;
pub mod attacks;
pub mod battle;
pub mod team_picking;

use std::fmt::{Display, Result, Formatter};

#[derive(Copy, Clone)]
pub enum Type {
    Bug,
    Dragon,
    Electric,
    Fighting,
    Fire,
    Flying,
    Ghost,
    Grass,
    Ground,
    Ice,
    Normal,
    Poison,
    Psychic,
    Rock,
    Water,
    None
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Type::Bug => write!(f, "Bug"),
            Type::Dragon => write!(f, "Dragon"),
            Type::Electric => write!(f, "Electric"),
            Type::Fighting => write!(f, "Fighting"),
            Type::Fire => write!(f, "Fire"),
            Type::Flying => write!(f, "Flying"),
            Type::Ghost => write!(f, "Ghost"),
            Type::Grass => write!(f, "Grass"),
            Type::Ground => write!(f, "Ground"),
            Type::Ice => write!(f, "Ice"),
            Type::Normal => write!(f, "Normal"),
            Type::Poison => write!(f, "Poison"),
            Type::Psychic => write!(f, "Psychic"),
            Type::Rock => write!(f, "Rock"),
            Type::Water => write!(f, "Water"),
            Type::None => write!(f, "None")
        }
    }
}