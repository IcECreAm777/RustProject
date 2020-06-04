pub mod pokemon;
pub mod attacks;
pub mod battle;
pub mod team_picking;

use std::fmt::{Display, Result, Formatter};
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, std::hash::Hash)]
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
// initialize HashMaps for type effectivenesses
static BUG: Type = Type::Bug;
static DRAGON: Type = Type::Dragon;
static ELECTRIC: Type = Type::Electric;
static FIGHTING: Type = Type::Fighting;
static FIRE: Type = Type::Fire;
static FLYING: Type = Type::Flying;
static GHOST: Type = Type::Ghost;
static GRASS: Type = Type::Grass;
static GROUND: Type = Type::Ground;
static ICE: Type = Type::Ice;
static NORMAL: Type = Type::Normal;
static POISON: Type = Type::Poison;
static PSYCHIC: Type = Type::Psychic;
static ROCK: Type = Type::Rock;
static WATER: Type = Type::Water;

pub fn normap() -> HashMap<Type,f32> {let map = [(ROCK,0.5),(GHOST,0.0)].iter().cloned().collect(); map}
pub fn fimap() -> HashMap<Type,f32> {let map = [(FIRE,0.5),(WATER,0.5),(GRASS,2.0),(ICE,2.0),(BUG,2.0),(ROCK,0.5),(DRAGON,0.5)].iter().cloned().collect(); map}
pub fn wamap() -> HashMap<Type,f32> {let map = [(FIRE,2.0),(WATER,0.5),(GRASS,0.5),(GROUND,2.0),(ROCK,2.0),(DRAGON,0.5)].iter().cloned().collect(); map}
pub fn elmap() -> HashMap<Type,f32> {let map = [(WATER,2.0),(ELECTRIC,0.5),(GRASS,0.5),(GROUND,0.0),(FLYING,2.0),(DRAGON,0.5)].iter().cloned().collect(); map}
pub fn gramap() -> HashMap<Type,f32> {let map = [(FIRE,0.5),(WATER,2.0),(GRASS,0.5),(POISON,0.5),(GROUND,2.0),(FLYING,0.5),(BUG,0.5),(ROCK,2.0),(DRAGON,0.5)].iter().cloned().collect(); map}
pub fn icemap() -> HashMap<Type,f32> {let map = [(WATER,0.5),(GRASS,2.0),(ICE,0.5),(GROUND,2.0),(FLYING,2.0),(DRAGON,2.0)].iter().cloned().collect(); map}
pub fn figmap() -> HashMap<Type,f32> {let map = [(NORMAL,2.0),(ICE,2.0),(POISON,0.5),(FLYING,0.5),(PSYCHIC,0.5),(BUG,0.5),(ROCK,2.0),(GHOST,0.0)].iter().cloned().collect(); map}
pub fn poimap() -> HashMap<Type,f32> {let map = [(GRASS,2.0),(POISON,0.5),(GROUND,0.5),(BUG,0.5),(ROCK,0.5),(GHOST,0.5)].iter().cloned().collect(); map}
pub fn gromap() -> HashMap<Type,f32> {let map = [(FIRE,2.0),(ELECTRIC,2.0),(GRASS,0.5),(POISON,2.0),(FLYING,0.0),(BUG,0.5)].iter().cloned().collect(); map}
pub fn flymap() -> HashMap<Type,f32> {let map = [(ELECTRIC,0.5),(GRASS,2.0),(FIGHTING,2.0),(BUG,2.0),(ROCK,0.5)].iter().cloned().collect(); map}
pub fn psymap() -> HashMap<Type,f32> {let map = [(FIGHTING,2.0),(POISON,2.0),(PSYCHIC,0.5),(GHOST,2.0)].iter().cloned().collect(); map}
pub fn bugmap() -> HashMap<Type,f32> {let map = [(FIRE,0.5),(GRASS,2.0),(FIGHTING,0.5),(POISON,2.0),(FLYING,0.5),(PSYCHIC,2.0),(GHOST,0.5)].iter().cloned().collect(); map}
pub fn rockmap() -> HashMap<Type,f32> {let map = [(FIRE,2.0),(ICE,2.0),(FIGHTING,0.5),(GROUND,0.5),(FLYING,2.0),(BUG,2.0)].iter().cloned().collect(); map}
pub fn ghomap() -> HashMap<Type,f32> {let map = [(NORMAL,0.0),(PSYCHIC,2.0),(GHOST,2.0)].iter().cloned().collect(); map}
pub fn dramap() -> HashMap<Type,f32> {let map = [(DRAGON,2.0)].iter().cloned().collect(); map}

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