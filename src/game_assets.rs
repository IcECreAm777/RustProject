use ggez::{Context, ContextBuilder, GameResult, graphics, filesystem, audio};
use ggez::event::{self, EventHandler};

// **********************************************************************
// Assets used in every scene
// **********************************************************************

/// Assets used in every scene
/// On Loading all assets are initialized
struct GeneralGameAssets {
    title_font: graphics::Font
}

impl GeneralGameAssets {
    fn new(ctx: &mut Context) -> GameResult<GeneralGameAssets> {
        let title_font = graphics::Font::new(ctx, "/PokemonSolid.ttf")?;

        Ok(GeneralGameAssets {
            title_font
        })
    }
}

struct TeamPickingAssets {
    music: audio::SoundData
}

impl TeamPickingAssets {
    fn new(ctx: &mut Context) -> GameResult<TeamPickingAssets> {
        let music = audio::SoundData::new(ctx, "/sounds/team_picking.mp3")?;

        Ok(TeamPickingAssets {
            music
        })
    }
}

/// handles incoming input
struct InputState {
    x_axis: f32,
    y_axis: f32,
    a: bool,
    b: bool
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            x_axis: 0.0,
            y_axis: 0.0,
            a: false,
            b: false
        }
    }
}

// **********************************************************************
// Structs and Methods for the Main Game
// **********************************************************************

/// Contains the necessary data to run the game
pub struct PokemonGame {
    //TODO implement state - data for the game
}

impl PokemonGame {
    pub fn new(_ctx: &mut Context) -> PokemonGame {
        // Load/create resources here: images, fonts, sounds, etc.
        PokemonGame { }
    }
}

impl EventHandler for PokemonGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        // Draw code here...

        graphics::present(ctx)
    }
}