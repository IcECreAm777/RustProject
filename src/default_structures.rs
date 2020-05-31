pub mod pokemon;
pub mod attacks;
pub mod battle;
pub mod team_picking;

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