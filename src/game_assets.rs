use ggez::{Context, GameResult};
use ggez::audio::{self, SoundSource};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Image, Scale, DrawMode, DrawParam};
use crate::default_structures::{battle, team_picking, attacks, pokemon};
use mint::{self, Point2, Vector2};
use std::time::Duration;

// **********************************************************************
// Main Game
// **********************************************************************

pub struct Game {
    team: TeamPickingGame,
    battle: battle::Battle,
    picking: bool,
    fight: bool,
    cycle: bool,
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        if self.cycle {
            let theme = self.battle.theme;
            match theme {
                1 => self.stop1(),
                3 => self.stop3(),
                4 => self.stop4(),
                _ => {},
            };
            self.team = TeamPickingGame::new(ctx);
            self.battle = battle::Battle::new(
                [battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),
                battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),], 
        [battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),
                battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),], ctx
            );
            self.picking = true;
            self.fight = false;
            self.cycle = false;
        }

        if self.picking {
            self.picking = !self.team.finished;
            self.team.update(ctx).unwrap()
        } else {
            if !self.fight {
                let o = [self.team.teams.team[0].clone(), self.team.teams.team[1].clone(), self.team.teams.team[2].clone(), self.team.teams.team[3].clone(), 
                    self.team.teams.team[4].clone(), self.team.teams.team[5].clone()];
                let own = self.team.generate_battle_team(ctx, o);
                let e = self.team.teams.generate_ai_team(ctx);
                let enemy = self.team.generate_battle_team(ctx, e);
                
                self.team.assets.music.stop();
    
                self.battle.own_team = own;
                self.battle.enemy_team = enemy;
                self.battle.init_cries(ctx);
                self.fight = true;
            } else {
                self.battle.update(ctx).unwrap();
                if self.battle.finished {
                    self.cycle = true;
                }
            }
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.picking {
            self.team.draw(ctx).unwrap()
        } else {
            self.battle.draw(ctx).unwrap()
        }

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        if self.picking {
            self.team.key_down_event(ctx, keycode, _keymods, _repeat);
        } else {
            self.battle.key_down_event(ctx, keycode, _keymods, _repeat);
        }
    }
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let team = TeamPickingGame::new(ctx);
        let battle = battle::Battle::new(
        [battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),
                battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),], 
        [battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),
                battle::Battlemon::dummy(ctx),battle::Battlemon::dummy(ctx),],
            ctx);

        Ok(Game {
            team, 
            battle, 
            picking: true,
            fight: false,
            cycle: false,
        })
    }

    pub fn stop1(&mut self) {
        self.battle.assets.gen1win.stop();
    }

    pub fn stop3(&mut self) {
        self.battle.assets.gen3win.stop();
    }

    pub fn stop4(&mut self) {
        self.battle.assets.gen4win.stop();
    }
}

// **********************************************************************
// Assets used in every scene
// **********************************************************************

/// Assets used in every scene
/// On Loading all assets are initialized
pub struct GeneralGameAssets {
    title_font: graphics::Font
}

impl GeneralGameAssets {
    pub fn new(ctx: &mut Context) -> GameResult<GeneralGameAssets> {
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
        let mut music = audio::Source::new(ctx, "/sounds/team_picking.mp3")?;
        let background = Image::new(ctx, "/Evening_Sunshine.jpg")?;

        music.set_repeat(true);

        Ok(TeamPickingAssets {
            music,
            background
        })
    }
}

// **********************************************************************
// Structs and Methods for the Main Game
// **********************************************************************

/// Contains the necessary data to run the game
pub struct PokemonGame {
    //TODO implement state - data for the game
    assets: GeneralGameAssets
}

impl PokemonGame {
    pub fn new(_ctx: &mut Context) -> PokemonGame {
        // Load/create resources here: images, fonts, sounds, etc.
        PokemonGame { 
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
    pub finished: bool,
    pub teams: team_picking::Team,
    assets: TeamPickingAssets,
    general: PokemonGame, 
    header: graphics::Text,
    pokemon_selection: bool,
    attack_selection: bool,
    attack_list: bool,
    selected_pokemon_index: i16,
    selected_header_index: i8, 
    selected_attack_header_index: i8,
    selected_attack_index: i16,
    
}

impl TeamPickingGame {
    pub fn new (_ctx: &mut Context) -> TeamPickingGame {
        let mut tpg = TeamPickingGame {
            assets: TeamPickingAssets::new(_ctx).unwrap(),
            teams: team_picking::Team::new(_ctx),
            general: PokemonGame::new(_ctx),
            header: graphics::Text::new("Select your Team"),
            pokemon_selection: false,
            attack_selection: false,
            attack_list: false,
            selected_pokemon_index: 0,
            selected_header_index:  0,
            selected_attack_header_index: 0,
            selected_attack_index: 0, 
            finished: false
        };

        tpg.header.set_font(tpg.general.assets.title_font, Scale{x: 50.0, y: 50.0});
        let _ = tpg.assets.music.play(); //TODO doesn't loop
        tpg
    }
}

impl EventHandler for TeamPickingGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        graphics::clear(_ctx, graphics::WHITE);

        graphics::draw(_ctx, &self.assets.background, graphics::DrawParam::default())?;
        graphics::draw(_ctx, &self.header, graphics::DrawParam::default().dest(Point2{x: 300.0, y:10.0}).color(graphics::BLACK))?;

        Self::draw_pokemon_header(_ctx, self.teams.team[0].clone(), Point2{x:100.0, y:100.0})?;
        Self::draw_pokemon_header(_ctx, self.teams.team[1].clone(), Point2{x:200.0, y:100.0})?;
        Self::draw_pokemon_header(_ctx, self.teams.team[2].clone(), Point2{x:300.0, y:100.0})?;
        Self::draw_pokemon_header(_ctx, self.teams.team[3].clone(), Point2{x:400.0, y:100.0})?;
        Self::draw_pokemon_header(_ctx, self.teams.team[4].clone(), Point2{x:500.0, y:100.0})?;
        Self::draw_pokemon_header(_ctx, self.teams.team[5].clone(), Point2{x:600.0, y:100.0})?;

        let confirm = graphics::Text::new("Confirm");
        graphics::draw(_ctx, &confirm, graphics::DrawParam::default().dest(Point2{x:700.0, y:100.0}))?;

        let rect_p: Point2<f32>;
        match self.selected_header_index {
            0 => rect_p = Point2{x: 95.0, y: 95.0},
            1 => rect_p = Point2{x: 195.0, y: 95.0},
            2 => rect_p = Point2{x: 295.0, y: 95.0},
            3 => rect_p = Point2{x: 395.0, y: 95.0},
            4 => rect_p = Point2{x: 495.0, y: 95.0},
            5 => rect_p = Point2{x: 595.0, y: 95.0},
            6 => rect_p = Point2{x: 695.0, y: 95.0},
            _ => rect_p = Point2{x: 0.0, y: 0.0}
        }

        let rect = graphics::Rect::new(rect_p.x, rect_p.y, 90.0, 30.0);
        let mesh = graphics::Mesh::new_rectangle(_ctx, DrawMode::stroke(1.0), rect, graphics::BLACK)?;
        graphics::draw(_ctx, &mesh, DrawParam::default())?;

        if self.pokemon_selection {
            if self.selected_pokemon_index - 3 >= 0 {
                Self::draw_pokemon_list_entry(_ctx, self.teams.usable_moves_table.get_index((self.selected_pokemon_index - 3) as usize).unwrap().0.clone(), Point2{x:100.0, y:150.0})?;
            }

            if self.selected_pokemon_index - 2 >= 0 {
                Self::draw_pokemon_list_entry(_ctx, self.teams.usable_moves_table.get_index((self.selected_pokemon_index - 2) as usize).unwrap().0.clone(), Point2{x:100.0, y:200.0})?;
            }

            if self.selected_pokemon_index - 1 >= 0 {
                Self::draw_pokemon_list_entry(_ctx, self.teams.usable_moves_table.get_index((self.selected_pokemon_index - 1) as usize).unwrap().0.clone(), Point2{x:100.0, y:250.0})?;
            }

            Self::draw_pokemon_list_entry(_ctx, self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().0.clone(), Point2{x:100.0, y:300.0})?;

            if self.selected_pokemon_index + 1 < (self.teams.usable_moves_table.len()) as i16 {
                Self::draw_pokemon_list_entry(_ctx, self.teams.usable_moves_table.get_index((self.selected_pokemon_index + 1) as usize).unwrap().0.clone(), Point2{x:100.0, y:350.0})?;
            }

            if self.selected_pokemon_index + 2 < (self.teams.usable_moves_table.len()) as i16 {
                Self::draw_pokemon_list_entry(_ctx, self.teams.usable_moves_table.get_index((self.selected_pokemon_index + 2) as usize).unwrap().0.clone(), Point2{x:100.0, y:400.0})?;
            }

            if self.selected_pokemon_index + 3 < (self.teams.usable_moves_table.len()) as i16 {
                Self::draw_pokemon_list_entry(_ctx, self.teams.usable_moves_table.get_index((self.selected_pokemon_index + 3) as usize).unwrap().0.clone(), Point2{x:100.0, y:450.0})?;
            }

            let pok_rect = graphics::Rect::new(40.0, 290.0, 200.0, 50.0);
            let middle = graphics::Mesh::new_rectangle(_ctx, DrawMode::stroke(1.0), pok_rect, graphics::BLACK)?;
            graphics::draw(_ctx, &middle, DrawParam::default())?;

            if !self.attack_selection {
                Self::draw_pokemon_details(_ctx, self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().0.clone())?;
                graphics::draw(
                    _ctx, 
                    &self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().0.assets.front_sprite, 
                    DrawParam::default().dest(Point2 {x: 500.0, y: 150.0}).scale(Vector2 {x: 4.0, y: 4.0}))?;
            } else {
                Self::draw_attack_header(_ctx, self.teams.team[self.selected_header_index as usize].moves[0].clone(), Point2{x: 300.0, y: 150.0})?;
                Self::draw_attack_header(_ctx, self.teams.team[self.selected_header_index as usize].moves[1].clone(), Point2{x: 300.0, y: 200.0})?;
                Self::draw_attack_header(_ctx, self.teams.team[self.selected_header_index as usize].moves[2].clone(), Point2{x: 300.0, y: 250.0})?;
                Self::draw_attack_header(_ctx, self.teams.team[self.selected_header_index as usize].moves[3].clone(), Point2{x: 300.0, y: 300.0})?;

                let rect_a: Point2<f32>;
                match self.selected_attack_header_index {
                    0 => rect_a = Point2 {x: 295.0, y: 145.0},
                    1 => rect_a = Point2 {x: 295.0, y: 195.0},
                    2 => rect_a = Point2 {x: 295.0, y: 245.0},
                    3 => rect_a = Point2 {x: 295.0, y: 295.0},
                    _ => rect_a = Point2 {x: 0.0, y: 0.0},
                }

                let rect_attack = graphics::Rect::new(rect_a.x, rect_a.y, 80.0, 30.0);
                let attack_mesh = graphics::Mesh::new_rectangle(_ctx, DrawMode::stroke(1.0), rect_attack, graphics::BLACK)?;
                graphics::draw(_ctx, &attack_mesh, DrawParam::default())?;

                if self.attack_list {
                    if self.selected_attack_index - 3 >= 0 {
                        Self::draw_attack_header(
                            _ctx, 
                            self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().1[(self.selected_attack_index - 3) as usize].clone(), 
                            Point2{x: 400.0, y: 150.0})?;
                    }

                    if self.selected_attack_index - 2 >= 0 {
                        Self::draw_attack_header(
                            _ctx, 
                            self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().1[(self.selected_attack_index - 2) as usize].clone(), 
                            Point2{x: 400.0, y: 200.0})?;
                    }

                    if self.selected_attack_index - 1 >= 0 {
                        Self::draw_attack_header(
                            _ctx, 
                            self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().1[(self.selected_attack_index - 1) as usize].clone(), 
                            Point2{x: 400.0, y: 250.0})?;
                    }

                    Self::draw_attack_header(
                        _ctx, 
                        self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().1[self.selected_attack_index as usize].clone(), 
                        Point2{x: 400.0, y: 300.0})?;

                    if self.selected_attack_index + 1 < self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().1.len() as i16 {
                        Self::draw_attack_header(
                            _ctx, 
                            self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().1[(self.selected_attack_index + 1) as usize].clone(), 
                            Point2{x: 400.0, y: 350.0})?;
                    }

                    if self.selected_attack_index + 2 < self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().1.len() as i16 {
                        Self::draw_attack_header(
                            _ctx, 
                            self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().1[(self.selected_attack_index + 2) as usize].clone(), 
                            Point2{x: 400.0, y: 400.0})?;
                    }

                    if self.selected_attack_index + 3 < self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().1.len() as i16 {
                        Self::draw_attack_header(
                            _ctx, 
                            self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().1[(self.selected_attack_index + 3) as usize].clone(), 
                            Point2{x: 400.0, y: 450.0})?;
                    }

                    let atk_rect = graphics::Rect::new(390.0, 295.0, 150.0, 40.0);
                    let atk_middle = graphics::Mesh::new_rectangle(_ctx, DrawMode::stroke(1.0), atk_rect, graphics::BLACK)?;
                    graphics::draw(_ctx, &atk_middle, DrawParam::default())?;

                    Self::draw_attack_details(
                        _ctx, 
                        self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().1[self.selected_attack_index as usize].clone(), 
                        Point2 {x: 550.0, y: 250.0})?;
                }
            }
        }

        graphics::present(_ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _keymods: KeyMods, _: bool) {
        if !self.pokemon_selection {
            match key {
                KeyCode::Space => {
                    if self.selected_header_index > 5 {
                        self.finished = true;
                    } else {
                        self.pokemon_selection = true;
                    }
                },
                KeyCode::Return => {
                    if self.selected_header_index > 5 {
                        self.finished = true;
                    } else {
                        self.pokemon_selection = true;
                    }
                },
                KeyCode::Left => {
                    self.selected_header_index = self.selected_header_index - 1;
                    if self.selected_header_index < 0 {
                        self.selected_header_index = 6;
                    }
                },
                KeyCode::Right => {
                    self.selected_header_index = self.selected_header_index + 1;
                    if self.selected_header_index > 6 {
                        self.selected_header_index = 0;
                    }
                },
                KeyCode::Escape => ggez::event::quit(ctx),
                _ => ()
            }
            return;
        }

        if !self.attack_selection {
            match key {
                KeyCode::Space => {
                    self.attack_selection = true;
                    self.teams.team[self.selected_header_index as usize] = self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().0.clone();
                },
                KeyCode::Return => {
                    self.attack_selection = true;
                    self.teams.team[self.selected_header_index as usize] = self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().0.clone();
                },
                KeyCode::Up => {
                    self.selected_pokemon_index = self.selected_pokemon_index - 1;
                    if self.selected_pokemon_index < 0 {
                        self.selected_pokemon_index = (self.teams.usable_moves_table.len() - 1) as i16;
                    }
                },
                KeyCode::Down => {
                    self.selected_pokemon_index = self.selected_pokemon_index + 1;
                    if self.selected_pokemon_index >= self.teams.usable_moves_table.len() as i16 {
                        self.selected_pokemon_index = 0; //TODO buggy for some reason
                    }
                },
                KeyCode::Escape => self.pokemon_selection = false,
                _ => ()
            }
            return;
        }
        
        if !self.attack_list {
            match key {
                KeyCode::Space => self.attack_list = true,
                KeyCode::Return => self.attack_list = true,
                KeyCode::Escape => self.attack_selection = false,
                KeyCode::Up => {
                    self.selected_attack_header_index = self.selected_attack_header_index - 1;
                    if self.selected_attack_header_index < 0 {
                        self.selected_attack_header_index = 3;
                    }
                }, 
                KeyCode::Down => {
                    self.selected_attack_header_index = self.selected_attack_header_index + 1;
                    if self.selected_attack_header_index > 3 {
                        self.selected_attack_header_index = 0;
                    }
                },
                _ => ()
            }
            return;
        }

        match key {
            KeyCode::Space => {
                self.teams.team[self.selected_header_index as usize].moves[self.selected_attack_header_index as usize] = 
                    self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().1[self.selected_attack_index as usize].clone();
                self.selected_attack_index = 0;
                self.attack_list = false;
            },
            KeyCode::Return => {
                self.teams.team[self.selected_header_index as usize].moves[self.selected_attack_header_index as usize] = 
                    self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().1[self.selected_attack_index as usize].clone();
                self.selected_attack_index = 0;
                self.attack_list = false;
            }, 
            KeyCode::Escape => { 
                self.attack_list = false;
                self.selected_attack_index = 0;
            }, 
            KeyCode::Up => {
                self.selected_attack_index = self.selected_attack_index - 1;
                if self.selected_attack_index < 0 {
                    self.selected_attack_index = (self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().1.len() as i16) - 1;
                }
            }, 
            KeyCode::Down => {
                self.selected_attack_index = self.selected_attack_index + 1;
                if self.selected_attack_index >= self.teams.usable_moves_table.get_index(self.selected_pokemon_index as usize).unwrap().1.len() as i16 {
                    self.selected_attack_index = 0;
                }
            }, 
            _ => ()
        }
    }
}

impl TeamPickingGame {
    pub fn draw_pokemon_header(ctx: &mut Context, pok: pokemon::Pokemon, pos: Point2<f32>) -> GameResult<()> {
        let name = graphics::Text::new(pok.name);
        graphics::draw(ctx, &name, graphics::DrawParam::default().dest(pos))?;
        Ok(())
    }

    pub fn draw_pokemon_list_entry(ctx: &mut Context, pok: pokemon::Pokemon, pos: Point2<f32>) -> GameResult<()> {
        let name = graphics::Text::new(pok.name);
        graphics::draw(ctx, &pok.assets.front_sprite, DrawParam::default().dest(Point2 {x: pos.x - 50.0, y: pos.y - 10.0}).scale(Vector2 {x: 0.7, y: 0.7}))?;
        graphics::draw(ctx, &name, graphics::DrawParam::default().dest(pos))?;
        Ok(())
    }

    pub fn draw_pokemon_details(ctx: &mut Context, pok: pokemon::Pokemon) -> GameResult<()> {
        let details = graphics::Text::new(format!("{}", pok));
        graphics::draw(ctx, &details, graphics::DrawParam::default().dest(Point2 {x: 300.0, y: 150.0}).scale(Vector2 {x: 1.5, y:1.5}))?;
        Ok(())
    }

    pub fn draw_attack_header(ctx: &mut Context, atk: attacks::Attack, pos: Point2<f32>) -> GameResult<()> {
        let name = graphics::Text::new(atk.name);
        graphics::draw(ctx, &name, DrawParam::default().dest(pos))?;
        Ok(())
    }

    pub fn draw_attack_details(ctx: &mut Context, atk: attacks::Attack, pos: Point2<f32>) -> GameResult<()> {
        let name = graphics::Text::new(format!("{}", atk));
        graphics::draw(ctx, &name, DrawParam::default().dest(pos).scale(Vector2{x: 1.3, y: 1.3}))?;
        Ok(())
    }

    pub fn generate_battle_team(&mut self, _ctx: &mut Context, team: [pokemon::Pokemon; 6]) -> [battle::Battlemon; 6] {
        [battle::Battlemon::new(team[0].clone()),battle::Battlemon::new(team[1].clone()),battle::Battlemon::new(team[2].clone()),
        battle::Battlemon::new(team[3].clone()),battle::Battlemon::new(team[4].clone()),battle::Battlemon::new(team[5].clone()),]
    }
}

impl EventHandler for battle::Battle {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.timer == 0 {
            match self.state {
                battle::State::Pre => {
                    match self.theme {
                        1 => {self.theme1(); self.timer = 180;},
                        3 => {self.theme3(); self.timer = 150;},
                        4 => {self.theme4(); self.timer = 180;},
                        _ => {},
                    };
                    self.text = "             May thee vibe to the gl'rious battleth theme".to_string();
                    self.textcount = self.text.len();
                    self.state = battle::State::Start; 
                },
        
                battle::State::Start => {
                    if !self.own_sent || self.own_sounds[self.p1].playing() {
                        if !self.own_sent {
                            self.text = format!("You sent out {}", self.own_team[self.p1].name());
                            self.textcount = 0;
                            let _ = self.own_sounds[self.p1].play();
                            self.send();
                            self.own_sent = true;
                            match self.theme {
                                1 => self.dmg = 80,
                                3 => self.dmg = 10,
                                4 => self.dmg = 30,
                                _ => {},
                            };
                        }
                    }
                    else {
                        if !self.enemy_sounds[self.p2].playing() && !self.enemy_sent && self.dmg == 0{
                            self.text = format!("Opponent sent out {}", self.enemy_team[self.p2].name());
                            //self.send();
                            let _ = self.enemy_sounds[self.p2].play();
                            self.send();
                            self.enemy_sent = true;
                            self.textcount = 0;
                        }
                        else {
                            if !self.enemy_sent {
                                self.dmg -= 1;
                            }
                        }

                    }
                    if self.enemy_sent && !self.enemy_sounds[self.p2].playing() {
                        self.state = battle::State::Picking;
                        self.text = "What will you do?".to_string();
                        self.textcount = 0;
                    }
                },
                
                battle::State::Picking => {
                    if self.a1 != battle::Action::Picking && self.a2 != battle::Action::Picking {
                        self.state = self.prio();
                        self.timer = 30;
                    }
                    if self.a2 == battle::Action::Picking {
                        self.enemy_action();
                    }
                },

                battle::State::PickAtk => {if self.a1 != battle::Action::Picking {self.state = battle::State::Picking; self.text = "".to_string();}},
                battle::State::PickSlot => {if self.a1 != battle::Action::Picking {self.state = battle::State::Picking; self.text = "".to_string();}},

                battle::State::Between => {
                    self.state = self.between();
                    self.timer = 5;
                },

                battle::State::A1 => {
                    self.ret = battle::State::E1;
                    match self.a1 {
                        battle::Action::Swap(slot) => self.swap(slot, true),
                        battle::Action::Attack(atk) => self.atk_init(atk, false),
                        _ => {},
                    };
                    self.a1 = battle::Action::Picking;
                    self.state = battle::State::E1;
                },

                battle::State::A2 => {
                    self.ret = battle::State::E3;
                    match self.a2 {
                        battle::Action::Swap(slot) => self.swap(slot, false),
                        battle::Action::Attack(atk) => self.atk_init(atk, true),
                        _ => {},
                    };
                    self.a2 = battle::Action::Picking;
                    self.state = battle::State::E3;
                },

                battle::State::E1 => {
                    self.ret = battle::State::E2;
                    self.apply_effect(self.e1, true);
                    self.e1 = attacks::Effect::None;
                    self.state = battle::State::E2;
                },

                battle::State::E2 => {
                    self.ret = battle::State::Between;
                    self.apply_effect(self.e2, true);
                    self.e2 = attacks::Effect::None;
                    self.state = battle::State::Between;

                },

                battle::State::E3 => {
                    self.ret = battle::State::E4;
                    self.apply_effect(self.e3, false);
                    self.e3 = attacks::Effect::None;
                    self.state = battle::State::E4;

                },

                battle::State::E4 => {
                    self.ret = battle::State::Between;
                    self.apply_effect(self.e4, false);
                    self.e4 = attacks::Effect::None;
                    self.state = battle::State::Between;

                },

                battle::State::After1 => {   // check if any effects to be applied
                    self.own_team[self.p1].flinch = false;
                    self.enemy_team[self.p2].flinch = false;
                    self.stat_eff(true);
                    self.ret = battle::State::After2;
                    self.state = battle::State::After2;
                },

                battle::State::After2 => {
                    self.stat_eff(false);
                    self.state = battle::State::Cycle;
                    self.ret = battle::State::Cycle;
                },

                battle::State::Cycle => {
                    self.state = battle::State::Picking;
                    self.ret = battle::State::Picking;
                    self.text = "What will you do?".to_string();
                    self.textcount = 0;
                }

                battle::State::SelfReplace => {
                    if self.own_team[self.p1].died {
                        self.own_team[self.p1].died = false;
                        self.own_sent = false;
                        self.timer = 120;
                        self.death();
                    }
                    else {
                        let mut done: bool = true;
                        for i in self.own_team.iter() {
                             if i.current_health != 0 && i.name() != "Dummy" {done = false; break;}
                        }
                        if done {self.state = battle::State::Fin; self.won = false;}
                        else {if self.own_team[self.p1].current_health != 0 {self.state = self.ret_state();}}
                    }
                },
                
                battle::State::EnemyReplace => {
                    if self.enemy_team[self.p2].died {
                        self.enemy_team[self.p2].died = false;
                        self.enemy_sent = false;
                        self.timer = 120;
                        self.death();
                    }
                    else {
                        let mut done: bool = true;
                        for i in self.enemy_team.iter() {
                            if i.current_health != 0 {done = false; break;}
                        }
                        //self.state = if done {battle::State::Fin} else {battle::State::EnemyReplace};
                        if done {self.state = battle::State::Fin; self.won = true;}
                        else {self.enemy_swap(); self.state = self.ret_state();}
                    }
                },

                battle::State::Fin => {
                    let theme = match self.theme {
                        1 => self.assets.gen1.stopped(),
                        3 => self.assets.gen3.stopped(),
                        4 => self.assets.gen4.stopped(),
                        _ => true,
                    };
                    if theme {self.finished = true;}
                    else {
                        match self.theme {
                            1 => self.assets.gen1.stop(),
                            3 => self.assets.gen3.stop(),
                            4 => self.assets.gen4.stop(),
                            _ => {},
                        };
                        self.timer = 300;
                        if self.won {
                            self.text = "Congratulations, you've won!".to_string();
                            match self.theme {
                                1 => self.win1(),
                                3 => self.win3(),
                                4 => self.win4(),
                                _ => {},
                            };
                        }
                        else { 
                            self.text = "Bruh, that's kinda cringe".to_string();
                        }
                        self.textcount = 0;
                    }
                },
                _ => {},
            };
            self.textcount += if self.textcount < self.text.len() {1} else {0};
            Ok(())
        }

        else {
            if self.dmg == 0 {} 
            else { 
                if self.user {
                    if self.own_team[self.p1].current_health == 1 && self.dmg > 0 {
                        self.own_team[self.p1].died = true;
                        self.own_team[self.p1].current_health = 0;
                        self.timer = 60; 
                        self.dmg = 1;
                        self.text = format!("{} fainted!", self.own_team[self.p1].name());
                        self.textcount = 0;
                        self.clear_effects(true);
                        self.state = battle::State::SelfReplace;
                        self.a1 = battle::Action::Picking;
                    }
                    else {
                        if self.own_team[self.p1].current_health == self.own_team[self.p1].health() && self.dmg <= 0 {
                            self.dmg = 0;
                        }
                        else {
                            self.own_team[self.p1].current_health = 
                            if self.dmg > 0 {
                                self.own_team[self.p1].current_health - 1
                            }
                            else {
                                self.own_team[self.p1].current_health + 1
                            };
                        }
                    }
                    self.dmg -= if self.dmg > 0 {1} else {if self.dmg < 0 {-1} else {0}};
                }
                else {
                    if self.enemy_team[self.p2].current_health == 1  && self.dmg > 0 {
                        self.enemy_team[self.p2].died = true;
                        self.enemy_team[self.p2].current_health = 0;
                        self.timer = 60; 
                        self.dmg = 1;
                        self.text = format!("Enemy {} fainted!", self.enemy_team[self.p2].name());
                        self.textcount = 0;
                        self.clear_effects(false);
                        self.state = battle::State::EnemyReplace;
                        self.a2 = battle::Action::Picking;
                    }
                    else {
                        if self.enemy_team[self.p2].current_health == self.enemy_team[self.p2].health() && self.dmg <= 0 {
                            self.dmg = 0;
                        }
                        else {
                            self.enemy_team[self.p2].current_health = 
                            if self.dmg > 0 {
                                self.enemy_team[self.p2].current_health - 1
                            }
                            else {
                                self.enemy_team[self.p2].current_health + 1
                            };
                        }
                    }
                    self.dmg -= if self.dmg > 0 {1} else {if self.dmg < 0 {-1} else {0}};
                }
            }
            self.textcount += if self.textcount < self.text.len() {1} else {0};
            self.timer -= 1;
            Ok(())
        }
    }

    
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());
        graphics::draw(ctx, &self.assets.bg1, graphics::DrawParam::default().scale(Point2{x:1.2,y:1.2}))?;
        graphics::draw(ctx, &self.assets.healthbar, graphics::DrawParam::default().dest(Point2{x:0.0,y:0.0}))?;
        graphics::draw(ctx, &self.assets.healthbar2, graphics::DrawParam::default().dest(Point2{x:500.0,y:0.0}))?;
        graphics::draw(ctx, &self.assets.botbox, graphics::DrawParam::default().dest(Point2{x:0.0,y:500.0}))?;

        if self.own_sent {
            if !self.own_team[self.p1].dead() && self.own_team[self.p1].current_health <= self.own_team[self.p1].health()/5 {
                if self.assets.low.playing() {}
                else {self.low();}
            }
            let health1 = graphics::Rect::new(100.0,55.0,150.0 * self.own_team[self.p1].clone().hp_fract(),13.0);
            let c1 = if self.own_team[self.p1].clone().hp_fract() <= 0.2 {graphics::Color::new(1.0,0.0,0.0,1.0)} else {graphics::Color::new(0.0,1.0,0.0,1.0)};
            let h1 = graphics::Mesh::new_rectangle(ctx,graphics::DrawMode::fill(), health1, c1)?;
            graphics::draw(ctx, &h1, graphics::DrawParam::default())?;
            let fract: f32 = (90.0-(self.own_team[self.p1].offset*3) as f32)/90.0;
            let rect = graphics::Rect::new(0.0,0.0,1.0,fract);
            self.own_team[self.p1].offset();
            graphics::draw(ctx, &self.own_team[self.p1].pokemon.assets.back_sprite, graphics::DrawParam::default().src(rect).scale(Point2{x:4.0,y:4.0}).dest(Point2{x:30.0,y:200.0+8.5*(self.own_team[self.p1].offset as f32)}))?;
            match self.own_team[self.p1].status {
                attacks::Status::Burn => graphics::draw(ctx, &self.assets.brn, graphics::DrawParam::default().dest(Point2{x:230.0,y:10.0}).scale(Point2{x:0.5,y:0.5}))?,
                attacks::Status::Freeze(_val) => graphics::draw(ctx, &self.assets.frz, graphics::DrawParam::default().dest(Point2{x:230.0,y:10.0}).scale(Point2{x:0.5,y:0.5}))?,
                attacks::Status::Paralysis => graphics::draw(ctx, &self.assets.par, graphics::DrawParam::default().dest(Point2{x:230.0,y:10.0}).scale(Point2{x:0.5,y:0.5}))?,
                attacks::Status::Sleep(_val) => graphics::draw(ctx, &self.assets.slp, graphics::DrawParam::default().dest(Point2{x:230.0,y:10.0}).scale(Point2{x:0.5,y:0.5}))?,
                attacks::Status::Poison => graphics::draw(ctx, &self.assets.psn, graphics::DrawParam::default().dest(Point2{x:230.0,y:10.0}).scale(Point2{x:0.5,y:0.5}))?,
                _ => {},
            };
        }

        if self.enemy_sent {
            let health2 = graphics::Rect::new(700.0,55.0,-150.0*self.enemy_team[self.p2].clone().hp_fract(),13.0);
            let c2 = if self.enemy_team[self.p2].clone().hp_fract() <= 0.2 {graphics::Color::new(1.0,0.0,0.0,1.0)} else {graphics::Color::new(0.0,1.0,0.0,1.0)};
            let h2 = graphics::Mesh::new_rectangle(ctx,graphics::DrawMode::fill(), health2, c2)?;
            graphics::draw(ctx, &h2, graphics::DrawParam::default())?;
            let fract: f32 = (90.0-(self.enemy_team[self.p2].offset*3) as f32)/90.0;
            let rect = graphics::Rect::new(0.0,0.0,1.0,fract as f32);
            self.enemy_team[self.p2].offset();
            graphics::draw(ctx, &self.enemy_team[self.p2].pokemon.assets.front_sprite, graphics::DrawParam::default().src(rect).scale(Point2{x:4.0,y:4.0}).dest(Point2{x:514.0,y:200.0+8.5*self.enemy_team[self.p2].offset as f32}))?;
            match self.enemy_team[self.p2].status {
                attacks::Status::Burn => graphics::draw(ctx, &self.assets.brn, graphics::DrawParam::default().dest(Point2{x:530.0,y:10.0}).scale(Point2{x:0.5,y:0.5}))?,
                attacks::Status::Freeze(_val) => graphics::draw(ctx, &self.assets.frz, graphics::DrawParam::default().dest(Point2{x:530.0,y:10.0}).scale(Point2{x:0.5,y:0.5}))?,
                attacks::Status::Paralysis => graphics::draw(ctx, &self.assets.par, graphics::DrawParam::default().dest(Point2{x:530.0,y:10.0}).scale(Point2{x:0.5,y:0.5}))?,
                attacks::Status::Sleep(_val) => graphics::draw(ctx, &self.assets.slp, graphics::DrawParam::default().dest(Point2{x:530.0,y:10.0}).scale(Point2{x:0.5,y:0.5}))?,
                attacks::Status::Poison => graphics::draw(ctx, &self.assets.psn, graphics::DrawParam::default().dest(Point2{x:530.0,y:10.0}).scale(Point2{x:0.5,y:0.5}))?,
                _ => {},
            };
        }

        graphics::draw(ctx, &self.assets.ball, graphics::DrawParam::default().dest(Point2{x:345.0,y:0.0}).scale(Point2{x:0.5,y:0.5}))?; 
        
        match self.state {
            battle::State::PickAtk => {
                let mut x: f32 = 25.0;
                let mut y: f32 = 520.0; 
                graphics::draw(ctx, &self.assets.indic, graphics::DrawParam::default().scale(Point2{x:0.5,y:0.5}).dest(Point2{x:5.0+(self.selected%2)as f32*250.0,y:y-3.0+(self.selected/2) as f32*50.0}))?;
                for i in 0..4 {
                    if self.own_team[self.p1].pokemon.moves[i as usize] == attacks::dummy() {
                        continue;
                    }
                    let color: graphics::Color = graphics::BLACK;
                    let atki = graphics::Text::new(format!("{}: {}",i+1,self.own_team[self.p1].pokemon.moves[i as usize].name()));
                    graphics::draw(ctx, &atki, graphics::DrawParam::default().scale(Point2{x:1.25,y:1.25}).dest(Point2{x: x,y: y}).color(color))?;
                    match i {
                        0 => x += 250.0,
                        1 => {x -= 250.0; y += 50.0;},
                        2 => x += 250.0,
                        _ => {},
                    };
                }
            },
            battle::State::SelfReplace | battle::State::PickSlot => {
                if self.state == battle::State::SelfReplace && self.timer != 0 {}
                else {
                let mut x: f32 = 25.0;
                let mut y: f32 = 520.0;
                graphics::draw(ctx, &self.assets.indic, graphics::DrawParam::default().scale(Point2{x:0.5,y:0.5}).dest(Point2{x:5.0+(self.selected%3)as f32*300.0,y:y-3.0+(self.selected/3) as f32*50.0}))?;
                for i in 0..6 {
                    if self.own_team[i as usize].name() == "Dummy" {
                        match i {
                            0 | 1 => x += 300.0,
                            2 => {x -= 600.0; y += 50.0;}
                            _ => x += 300.0,
                        }
                        continue;
                    }
                    let color: graphics::Color = if i != self.p1 {graphics::BLACK} else {graphics::Color::new(1.0,0.0,0.0,1.0)};
                    let poki = graphics::Text::new(format!("{}: {} {}/{}",i+1,self.own_team[i as usize].name(), self.own_team[i as usize].current_health, self.own_team[i as usize].pokemon.health));
                    graphics::draw(ctx, &poki, graphics::DrawParam::default().scale(Point2{x:1.1,y:1.1}).dest(Point2{x: x,y: y}).color(color))?;
                    match i {
                        0 | 1 => x += 300.0,
                        2 => {x -= 600.0; y += 50.0;}
                        _ => x += 300.0,
                    };
                }
                }
            },
            battle::State::Picking => {
                self.textcount += if self.textcount < self.text.len() {1} else {0};
                let mut string = self.text.clone();
                string.truncate(self.textcount+1);
                let info = graphics::Text::new(string);
                if self.textcount == self.text.len() {
                    graphics::draw(ctx, &self.assets.indic, graphics::DrawParam::default().dest(Point2{x:460.0 + (self.selected as f32 *150.0),y:530.0}))?;
                    graphics::draw(ctx, &info, graphics::DrawParam::default().dest(Point2{x:25.0,y:540.0}).scale(Point2{x:1.75,y:1.75}).color(graphics::BLACK))?;
                    graphics::draw(ctx, &graphics::Text::new("1: Attack"), graphics::DrawParam::default().dest(Point2{x:500.0,y:540.0}).scale(Point2{x:1.75,y:1.75}).color(graphics::BLACK))?;
                    graphics::draw(ctx, &graphics::Text::new("2: Switch"), graphics::DrawParam::default().dest(Point2{x:650.0,y:540.0}).scale(Point2{x:1.75,y:1.75}).color(graphics::BLACK))?;
                }
                else {
                    graphics::draw(ctx, &info, graphics::DrawParam::default().dest(Point2{x:25.0,y:540.0}).scale(Point2{x:1.75,y:1.75}).color(graphics::BLACK))?;
                }
            },
            _ => {
                self.textcount += if self.textcount < self.text.len() {1} else {0};
                let mut string = self.text.clone();
                string.truncate(self.textcount+1);
                let info = graphics::Text::new(string);
                graphics::draw(ctx, &info, graphics::DrawParam::default().dest(Point2{x:25.0,y:540.0}).scale(Point2{x:1.75,y:1.75}).color(graphics::BLACK))?;
            }
        }
        if self.own_sent {
            let text1 = graphics::Text::new(self.own_team[self.p1].name());
            graphics::draw(ctx, &text1, graphics::DrawParam::default().dest(Point2{x:17.0,y:12.0}).scale(Point2{x:1.25,y:1.25}).color(graphics::BLACK))?;
            let healthh1 = format!("{}/{}", self.own_team[self.p1].current_health.to_string(), self.own_team[self.p1].health().to_string());
            let hn1 = graphics::Text::new(healthh1);
            graphics::draw(ctx, &hn1, graphics::DrawParam::default().dest(Point2{x:14.0,y:52.0}).scale(Point2{x:1.20,y:1.20}).color(graphics::BLACK))?;
        }
        if self.enemy_sent {
            let text2 = graphics::Text::new(self.enemy_team[self.p2].name());
            graphics::draw(ctx, &text2, graphics::DrawParam::default().dest(Point2{x:540.0,y:12.0}).scale(Point2{x:1.25,y:1.25}).color(graphics::BLACK))?;
            // TODO: calculation for name length for 2nd name text
            let healthh2 = format!("{}/{}", self.enemy_team[self.p2].current_health.to_string(), self.enemy_team[self.p2].health().to_string());
            let hn2 = graphics::Text::new(healthh2);
            graphics::draw(ctx, &hn2, graphics::DrawParam::default().dest(Point2{x:717.0,y:52.0}).scale(Point2{x:1.2,y:1.2}).color(graphics::BLACK))?;
        }
        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _keymods: KeyMods, _: bool) {
        match self.state {
            battle::State::Picking => {match key {
                    KeyCode::Key1 => {if self.selected != 0 {self.select();} self.selected = 0},
                    KeyCode::Key2 => {if self.selected != 1 {self.select();} self.selected = 1;},
                    KeyCode::F => {self.dmg = 500; self.timer = 500; self.user = false; self.ret = battle::State::Picking;}
                    KeyCode::Return => if self.selected == 0 {self.state = battle::State::PickAtk; self.selected = 0; self.pick();} else {self.state = battle::State::PickSlot; self.selected = 0; self.pick();},
                    KeyCode::Key0 => event::quit(ctx),
                    _ => (),
                };
            },
            battle::State::PickAtk => {match key {
                KeyCode::Key0 => event::quit(ctx),
                KeyCode::Escape => {self.state = battle::State::Picking; self.selected = 0; self.pick();},
                KeyCode::Key1 => if self.selected != 0 && self.own_team[self.p1].pokemon.moves[0] != attacks::dummy() {self.select(); self.selected = 0;},
                KeyCode::Key2 => if self.selected != 1 && self.own_team[self.p1].pokemon.moves[1] != attacks::dummy() {self.select(); self.selected = 1;},
                KeyCode::Key3 => if self.selected != 2 && self.own_team[self.p1].pokemon.moves[2] != attacks::dummy() {self.select(); self.selected = 2;},
                KeyCode::Key4 => if self.selected != 3 && self.own_team[self.p1].pokemon.moves[3] != attacks::dummy() {self.select(); self.selected = 3;},
                KeyCode::Return => {self.a1 = battle::Action::Attack(self.own_team[self.p1].pokemon.moves[self.selected as usize]); self.selected = 0; self.pick();},
                _ => (),
                };
            },
            battle::State::PickSlot => {match key {
                KeyCode::Key0 => event::quit(ctx),
                KeyCode::Escape => {self.state = battle::State::Picking; self.selected = 0; self.pick();},
                KeyCode::Key1 => if self.selected != 0 && self.own_team[0].name() != pokemon::dummy_pokemon(ctx).name {self.select(); self.selected = 0;},
                KeyCode::Key2 => if self.selected != 1 && self.own_team[1].name() != pokemon::dummy_pokemon(ctx).name {self.select(); self.selected = 1;},
                KeyCode::Key3 => if self.selected != 2 && self.own_team[2].name() != pokemon::dummy_pokemon(ctx).name {self.select(); self.selected = 2;},
                KeyCode::Key4 => if self.selected != 3 && self.own_team[3].name() != pokemon::dummy_pokemon(ctx).name {self.select(); self.selected = 3;},
                KeyCode::Key5 => if self.selected != 4 && self.own_team[4].name() != pokemon::dummy_pokemon(ctx).name {self.select(); self.selected = 4;},
                KeyCode::Key6 => if self.selected != 5 && self.own_team[5].name() != pokemon::dummy_pokemon(ctx).name {self.select(); self.selected = 5;},
                KeyCode::Return => if !self.own_team[self.selected as usize].dead() && self.selected != self.p1 as u8 {self.own_sent = false; self.a1 = battle::Action::Swap (self.selected as usize); self.selected = 0; self.pick();} else {self.denied();}
                _ => (),
                };
            },
            battle::State::SelfReplace => {match key {
                KeyCode::Key0 => event::quit(ctx),
                KeyCode::Key1 => if self.selected != 0 && self.own_team[0].name() != pokemon::dummy_pokemon(ctx).name {self.select(); self.selected = 0;},
                KeyCode::Key2 => if self.selected != 1 && self.own_team[1].name() != pokemon::dummy_pokemon(ctx).name {self.select(); self.selected = 1;},
                KeyCode::Key3 => if self.selected != 2 && self.own_team[2].name() != pokemon::dummy_pokemon(ctx).name {self.select(); self.selected = 2;},
                KeyCode::Key4 => if self.selected != 3 && self.own_team[3].name() != pokemon::dummy_pokemon(ctx).name {self.select(); self.selected = 3;},
                KeyCode::Key5 => if self.selected != 4 && self.own_team[4].name() != pokemon::dummy_pokemon(ctx).name {self.select(); self.selected = 4;},
                KeyCode::Key6 => if self.selected != 5 && self.own_team[5].name() != pokemon::dummy_pokemon(ctx).name {self.select(); self.selected = 5;},
                /*KeyCode::Key1 => {if self.selected != 0 {self.select();} self.selected = 0;},
                KeyCode::Key2 => {if self.selected != 1 {self.select();} self.selected = 1;},
                KeyCode::Key3 => {if self.selected != 2 {self.select();} self.selected = 2;},
                KeyCode::Key4 => {if self.selected != 3 {self.select();} self.selected = 3;},
                KeyCode::Key5 => {if self.selected != 4 {self.select();} self.selected = 4;},
                KeyCode::Key6 => {if self.selected != 5 {self.select();} self.selected = 5;},
            */    KeyCode::Return => {if !self.own_team[self.selected as usize].dead() {self.swap(self.selected as usize, true);self.state = self.ret_state(); self.selected = 0; self.pick(); ggez::timer::sleep(Duration::new(0,500));} else {self.denied();}},
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
    pub battle_cry: audio::SoundData,
    pub front_sprite: graphics::Image,
    pub back_sprite: graphics::Image,
}

impl PokemonAssets {
    pub fn new(ctx: &mut Context, cry: &'static str, sprite: &'static str, sprite2: &'static str) -> GameResult<PokemonAssets> {
        let battle_cry = audio::SoundData::new(ctx, cry)?;
        let front_sprite = graphics::Image::new(ctx, sprite)?;
        let back_sprite = graphics::Image::new(ctx, sprite2)?;

        Ok(PokemonAssets {
            battle_cry,
            front_sprite,
            back_sprite,
        })
    }
}
