use ggez::{Context, ContextBuilder, GameResult, filesystem};
use ggez::audio::{self, Source, SoundSource};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Image, Color};
use crate::default_structures::{battle, team_picking, attacks};
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
        let title_font = graphics::Font::new(ctx, "/Pokemon_Solid.ttf")?;

        Ok(GeneralGameAssets {
            title_font
        })
    }
}

struct TeamPickingAssets {
    music: audio::Source,
    background: Image
}

impl TeamPickingAssets {
    fn new(ctx: &mut Context) -> GameResult<TeamPickingAssets> {
        let music = audio::Source::new(ctx, "/sounds/team_picking.mp3")?;
        let background = Image::new(ctx, "/team_picking_background.png")?;

        Ok(TeamPickingAssets {
            music,
            background
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
    inputs: InputState, 
    assets: GeneralGameAssets
}

impl PokemonGame {
    pub fn new(_ctx: &mut Context) -> PokemonGame {
        // Load/create resources here: images, fonts, sounds, etc.
        PokemonGame { 
            inputs: InputState::default(),
            assets: GeneralGameAssets::new(_ctx).unwrap()
        }
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

pub struct TeamPickingGame {
    assets: TeamPickingAssets,
    teams: team_picking::Team,
    general: PokemonGame
}

impl TeamPickingGame {
    pub fn new (_ctx: &mut Context) -> TeamPickingGame {
        let mut tpg = TeamPickingGame {
            assets: TeamPickingAssets::new(_ctx).unwrap(),
            teams: team_picking::Team::new(_ctx),
            general: PokemonGame::new(_ctx)
        };
        let _ = tpg.assets.music.play_detached(); //TODO doesn't loop
        tpg
    }
}

impl EventHandler for TeamPickingGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        //TODO update code here

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        graphics::clear(_ctx, graphics::WHITE);

        graphics::draw(_ctx, &self.assets.background, graphics::DrawParam::default())?;

        //TODO draw code here

        graphics::present(_ctx)
    }
}

impl EventHandler for battle::Battle {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.timer == 0 {
            match self.state {
                battle::State::Picking => {
                    if self.text != "What will you do?" {self.text = "What will you do?".to_string();}
                    if self.a1 != battle::Action::Picking && self.a2 != battle::Action::Picking {
                        self.state = self.prio();
                        self.timer = 5;
                    }
                    self.a2 = battle::Action::Attack(self.enemy_team[self.p2].pokemon.moves[1]);
                },
                battle::State::PickAtk => {if self.a1 != battle::Action::Picking {self.state = battle::State::Picking; self.text = "".to_string();}},
                battle::State::PickSlot => {if self.a1 != battle::Action::Picking {self.state = battle::State::Picking; self.text = "".to_string();}},

                battle::State::Between => {
                    self.state = self.between();
                    self.timer = 5;
                },
                battle::State::A1 => {
                    self.ret = battle::State::Between;
                    match self.a1 {
                        battle::Action::Swap(slot) => self.check_swap(slot),
                        battle::Action::Attack(atk) => self.atk_init(atk, false),
                        _ => {},
                    };
                    self.a1 = battle::Action::Picking;
                    self.state = battle::State::Between;
                },
                battle::State::A2 => {
                    self.ret = battle::State::Between;
                    match self.a2 {
                        battle::Action::Swap(slot) => self.swap(slot, false),
                        battle::Action::Attack(atk) => self.atk_init(atk, true),
                        _ => {},
                    };
                    self.a2 = battle::Action::Picking;
                    self.state = battle::State::Between;
                },
                battle::State::After1 => {   // check if any effects to be applied
                    self.own_team[self.p1].flinch = false;
                    self.enemy_team[self.p2].flinch = false;
                    self.stat_eff(true);
                    self.ret = battle::State::After2;
                    self.state = battle::State::After2;
                }
                battle::State::After2 => {
                    self.stat_eff(false);
                    self.state = battle::State::Picking;
                }


                battle::State::SelfReplace => {
                    let mut done: bool = true;
                    for i in self.own_team.iter() {
                         if i.current_health != 0 {done = false; break;}
                    }
                    self.state = if done {battle::State::Fin} else {battle::State::SelfReplace};
                    if done {event::quit(ctx);}
                    if self.own_team[self.p1].current_health != 0 {self.state = self.ret_state();}
        
                }
                battle::State::EnemyReplace => {
                    let mut done: bool = true;
                    for i in self.enemy_team.iter() {
                        if i.current_health != 0 {done = false; break;}
                    }
                    self.state = if done {battle::State::Fin} else {battle::State::EnemyReplace};
                    if !done {self.enemy_swap();} else {event::quit(ctx);}
                    self.state = self.ret_state();
                }
                _ => {},
            };
            Ok(())
        }
        else {
            if self.dmg == 0 {} 
            else { 
                if self.user {
                    if self.own_team[self.p1].current_health == 1 {
                        self.own_team[self.p1].current_health = 0;
                        self.timer = 90; self.dmg = 1;
                        self.text = String::new();
                        self.text.push_str(self.own_team[self.p1].name());
                        self.text.push_str(" fainted!");
                        self.state = battle::State::SelfReplace;
                        self.a1 = battle::Action::Picking;
                    }
                    else {self.own_team[self.p1].current_health -= 1;}
                    self.dmg -= 1;
                }
                else {
                    if self.enemy_team[self.p2].current_health == 1 {
                        self.enemy_team[self.p2].current_health = 0;
                        self.timer = 150; self.dmg = 1;
                        self.text = String::new();
                        self.text.push_str(self.enemy_team[self.p2].name());
                        self.text.push_str(" fainted!");
                        self.state = battle::State::EnemyReplace;
                        self.a2 = battle::Action::Picking;
                    }
                    else {self.enemy_team[self.p2].current_health -= 1;}
                    self.dmg -= 1;
                }
            }
            self.timer -= 1;
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

        let health1 = graphics::Rect::new(100.0,50.0,100.0 * self.own_team[self.p1].clone().hp_fract(),50.0);
        let c1 = if self.own_team[self.p1].clone().hp_fract() <= 0.2 {graphics::Color::new(1.0,0.0,0.0,1.0)} else {graphics::Color::new(0.0,1.0,0.0,1.0)};
        let h1 = graphics::Mesh::new_rectangle(ctx,graphics::DrawMode::fill(), health1, c1)?;
        let health2 = graphics::Rect::new(600.0,50.0,100.0*self.enemy_team[self.p2].clone().hp_fract(),50.0);
        let c2 = if self.enemy_team[self.p2].clone().hp_fract() <= 0.2 {graphics::Color::new(1.0,0.0,0.0,1.0)} else {graphics::Color::new(0.0,1.0,0.0,1.0)};
        let h2 = graphics::Mesh::new_rectangle(ctx,graphics::DrawMode::fill(), health2, c2)?;
        graphics::draw(ctx, &h1, graphics::DrawParam::default())?;
        graphics::draw(ctx, &h2, graphics::DrawParam::default())?;

        let temp = graphics::Text::new(self.own_team[self.p1].clone().name());
        let temp2 = graphics::Text::new(self.enemy_team[self.p2].clone().name());
        graphics::draw(ctx, &temp, graphics::DrawParam::default().dest(mint::Point2{x:40.0,y:20.0}).color(graphics::WHITE))?;
        graphics::draw(ctx, &temp2, graphics::DrawParam::default().dest(mint::Point2{x:540.0,y:20.0}).color(graphics::WHITE))?;
        let ball = graphics::Text::new("Icon here?");
        graphics::draw(ctx, &ball, graphics::DrawParam::default().dest(mint::Point2{x:365.0,y:50.0}).color(graphics::BLACK))?;
        let info = graphics::Text::new(self.text.as_str());
        graphics::draw(ctx, &info, graphics::DrawParam::default().dest(mint::Point2{x:175.0,y:550.0}).color(graphics::BLACK))?;
        let mut healthh1 = self.own_team[self.p1].current_health.to_string();
        healthh1.push_str("/");
        healthh1.push_str(self.own_team[self.p1].pokemon.health.to_string().as_str());
        let hn1 = graphics::Text::new(healthh1);
        graphics::draw(ctx, &hn1, graphics::DrawParam::default().dest(mint::Point2{x:100.0,y:25.0}).color(graphics::WHITE))?;
        let mut healthh2 = self.enemy_team[self.p2].current_health.to_string();
        healthh2.push_str("/");
        healthh2.push_str(self.enemy_team[self.p2].pokemon.health.to_string().as_str());
        let hn2 = graphics::Text::new(healthh2);
        graphics::draw(ctx, &hn2, graphics::DrawParam::default().dest(mint::Point2{x:600.0,y:25.0}).color(graphics::WHITE))?;
        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _keymods: KeyMods, _: bool) {
        match self.state {
            battle::State::Picking => {match key {
                    KeyCode::Key1 => {self.state = battle::State::PickAtk; self.text = "Choose an attack".to_string();},
                    KeyCode::Key2 => {self.state = battle::State::PickSlot; self.text = "Choose a Pokemon".to_string();},
                    KeyCode::Key0 => event::quit(ctx),
                    _ => (),
                };
            },
            battle::State::PickAtk => {match key {
                KeyCode::Key0 => event::quit(ctx),
                KeyCode::Escape => self.state = {self.text = "What will you do".to_string(); battle::State::Picking},
                KeyCode::Key1 => self.a1 = battle::Action::Attack(self.own_team[self.p1].pokemon.moves[0]),
                KeyCode::Key2 => self.a1 = battle::Action::Attack(self.own_team[self.p1].pokemon.moves[1]),
                KeyCode::Key3 => self.a1 = battle::Action::Attack(self.own_team[self.p1].pokemon.moves[2]),
                KeyCode::Key4 => self.a1 = battle::Action::Attack(self.own_team[self.p1].pokemon.moves[3]),
                _ => (),
                };
            },
            battle::State::PickSlot => {match key {
                KeyCode::Key0 => event::quit(ctx),
                KeyCode::Escape => self.state = {self.text = "What will you do".to_string(); battle::State::Picking},
                KeyCode::Key1 => {self.a1 = if 0 != self.p1 {battle::Action::Swap(0)} else {battle::Action::Picking}; if 0 == self.p1 {self.text = "Cannot switch to Pokemon that is sent out already".to_string();}},
                KeyCode::Key2 => {self.a1 = if 1 != self.p1 {battle::Action::Swap(1)} else {battle::Action::Picking}; if 1 == self.p1 {self.text = "Cannot switch to Pokemon that is sent out already".to_string();}},
                KeyCode::Key3 => {self.a1 = if 2 != self.p1 {battle::Action::Swap(2)} else {battle::Action::Picking}; if 2 == self.p1 {self.text = "Cannot switch to Pokemon that is sent out already".to_string();}},
                KeyCode::Key4 => {self.a1 = if 3 != self.p1 {battle::Action::Swap(3)} else {battle::Action::Picking}; if 3 == self.p1 {self.text = "Cannot switch to Pokemon that is sent out already".to_string();}},
                KeyCode::Key5 => {self.a1 = if 4 != self.p1 {battle::Action::Swap(4)} else {battle::Action::Picking}; if 4 == self.p1 {self.text = "Cannot switch to Pokemon that is sent out already".to_string();}},
                KeyCode::Key6 => {self.a1 = if 5 != self.p1 {battle::Action::Swap(5)} else {battle::Action::Picking}; if 5 == self.p1 {self.text = "Cannot switch to Pokemon that is sent out already".to_string();}},
                _ => (),
                };
            },
            battle::State::SelfReplace => {match key {
                KeyCode::Key0 => event::quit(ctx),
                KeyCode::Key1 => self.check_swap(0),
                KeyCode::Key2 => self.check_swap(1),
                KeyCode::Key3 => self.check_swap(2),
                KeyCode::Key4 => self.check_swap(3),
                KeyCode::Key5 => self.check_swap(4),
                KeyCode::Key6 => self.check_swap(5), 
                _ => (),
                };
            }
            _ => if key == KeyCode::Key0 {event::quit(ctx)},
        };
        ()
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
