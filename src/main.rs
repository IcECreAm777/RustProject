mod default_structures;

use crate::default_structures::{pokemon, attacks, team_picking, battle};

use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::conf::WindowSetup;

fn main() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) =
    ContextBuilder::new("Pokemon Rust edition", "Niklas Rosseck | Kilian Woick | Henning Gütschow")
        .window_setup(WindowSetup::default().title("Pokemon Stadium - Rust Edition").icon(""))
        .build()
        .unwrap();

    //init game
    let mut my_game = PokemonGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct PokemonGame {
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
