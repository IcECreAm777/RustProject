mod default_structures;
mod game_assets;

use crate::default_structures::{pokemon, attacks, team_picking, battle};
use crate::game_assets::{PokemonGame, TeamPickingGame};

use ggez::{Context, ContextBuilder, GameResult, graphics, filesystem};
use ggez::event::{self, EventHandler};
use ggez::conf::{WindowSetup, WindowMode, FullscreenType};

fn main() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) =
    ContextBuilder::new("Pokemon Rust edition", "Niklas Rosseck _ Kilian Woick _ Henning Gütschow")
        .window_setup(WindowSetup::default().title("Pokemon Stadium - Rust Edition").icon("/ball.png"))
        .window_mode(WindowMode::default().fullscreen_type(FullscreenType::Windowed))
        .build()
        .unwrap();
    ggez::graphics::set_default_filter(&mut ctx, graphics::FilterMode::Nearest);
    //init game
    //let mut my_game = TeamPickingGame::new(&mut ctx);
    let mut my_game = battle::Battle::new(
        [battle::Battlemon::dummy(&mut ctx),battle::Battlemon::dummy(&mut ctx),battle::Battlemon::dummy(&mut ctx),battle::Battlemon::dummy(&mut ctx),battle::Battlemon::dummy(&mut ctx),battle::Battlemon::dummy(&mut ctx),], 
        [battle::Battlemon::dummy(&mut ctx),battle::Battlemon::dummy(&mut ctx),battle::Battlemon::dummy(&mut ctx),battle::Battlemon::dummy(&mut ctx),battle::Battlemon::dummy(&mut ctx),battle::Battlemon::dummy(&mut ctx),],
        &mut ctx);
        // for testing Battle stuff seperated
        
    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}
