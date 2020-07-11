mod default_structures;
mod game_assets;

use crate::game_assets::Game;

use ggez::{ ContextBuilder, graphics};
use ggez::event::{self};
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
    let mut game = Game::new(&mut ctx).unwrap();
        
    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}
