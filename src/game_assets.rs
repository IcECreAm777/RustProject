use ggez::{Context, ContextBuilder, GameResult, graphics, filesystem, audio};
use ggez::event::{self, EventHandler};
use crate::default_structures::battle;
use mint;

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

impl EventHandler for battle::Battle {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.timer == 0 {
            match self.state {

                _ => {},
            };
            Ok(())
        }
        else {
            if self.dmg == 0 {} else { 
                if self.user {
                    if self.own_team[self.p1].current_health == 0 {self.timer -= self.dmg; self.dmg = 1;}
                    else {self.own_team[self.p1].current_health -= 1;}
                    self.dmg -= 1;
                }
                else {
                    if self.enemy_team[self.p2].current_health == 0 {self.timer -= self.dmg; self.dmg = 1;}
                    else {self.enemy_team[self.p2].current_health -= 1;}
                    self.dmg -= 1;
                }
                self.timer -= 1;
            }
            Ok(())
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());

        let bar1 = graphics::Rect::new(0.0,0.0,300.0,100.0);
        let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), bar1, graphics::BLACK)?;
        graphics::draw(ctx, &r1, graphics::DrawParam::default())?;
        let bar2 = graphics::Rect::new(500.0,0.0,300.0,100.0);
        let r2 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), bar2, graphics::BLACK)?;
        graphics::draw(ctx, &r2, graphics::DrawParam::default())?;
        let boxx = graphics::Rect::new(0.0,500.0,800.0,100.0);
        let boxxx = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(5.0), boxx, graphics::BLACK)?;
        graphics::draw(ctx, &boxxx, graphics::DrawParam::default())?;

        let health1 = graphics::Rect::new(100.0,50.0,100.0*self.own_team[self.p1].hp_fract(),50.0);
        let c1 = if self.own_team[self.p1].hp_fract() <= 0.2 {graphics::Color::new(1.0,0.0,0.0,1.0)} else {graphics::Color::new(0.0,1.0,0.0,1.0)};
        let h1 = graphics::Mesh::new_rectangle(ctx,graphics::DrawMode::fill(), health1, c1)?;
        let health2 = graphics::Rect::new(600.0,50.0,100.0*self.enemy_team[self.p2].hp_fract(),50.0);
        let c2 = if self.enemy_team[self.p2].hp_fract() <= 0.2 {graphics::Color::new(1.0,0.0,0.0,1.0)} else {graphics::Color::new(0.0,1.0,0.0,1.0)};
        let h2 = graphics::Mesh::new_rectangle(ctx,graphics::DrawMode::fill(), health2, c2)?;
        graphics::draw(ctx, &h1, graphics::DrawParam::default())?;
        graphics::draw(ctx, &h2, graphics::DrawParam::default())?;

        let temp = graphics::Text::new(self.own_team[self.p1].name());
        let temp2 = graphics::Text::new(self.enemy_team[self.p2].name());
        graphics::draw(ctx, &temp, graphics::DrawParam::default().dest(mint::Point2{x:40.0,y:20.0}).color(graphics::WHITE))?;
        graphics::draw(ctx, &temp2, graphics::DrawParam::default().dest(mint::Point2{x:540.0,y:20.0}).color(graphics::WHITE))?;
        let ball = graphics::Text::new("Icon here?");
        graphics::draw(ctx, &ball, graphics::DrawParam::default().dest(mint::Point2{x:365.0,y:50.0}).color(graphics::BLACK))?;
        let info = graphics::Text::new("Info regarding battle, attacks, status changes etc will be shown here");
        graphics::draw(ctx, &info, graphics::DrawParam::default().dest(mint::Point2{x:175.0,y:550.0}).color(graphics::BLACK))?;
        graphics::present(ctx)?;

        Ok(())
    }
}

// **********************************************************************
// Pokemon Assets
// **********************************************************************

#[derive(Clone)]
pub struct PokemonAssets {
    battle_cry: audio::SoundData,
    front_sprite: graphics::Image
}

impl PokemonAssets {
    pub fn new(ctx: &mut Context, cry: &'static str, sprite: &'static str) -> GameResult<PokemonAssets> {
        let battle_cry = audio::SoundData::new(ctx, cry)?;
        let front_sprite = graphics::Image::new(ctx, sprite)?;

        Ok(PokemonAssets {
            battle_cry,
            front_sprite
        })
    }
}
