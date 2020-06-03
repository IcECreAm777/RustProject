use crate::default_structures::{pokemon, attacks};
use std::collections::HashMap;
use std::vec::Vec;
use indexmap::map::*;
use rand::prelude::*;

pub struct Team {
    pub usable_moves_table: IndexMap<pokemon::Pokemon, Vec<attacks::Attack>>,
    pub team: [pokemon::Pokemon; 6]
}

impl Team {
    pub fn new() -> Team {
        let mut team = Team {
            usable_moves_table: IndexMap::with_capacity(151),
            team: [pokemon::dummy_pokemon(); 6]
        };
        team.init_usable_moves();
        team
    }

    pub fn pick_pokemon(&mut self, index: usize, pokemon: pokemon::Pokemon) {
        self.team[index] = pokemon;
    }

    pub fn pick_attack(&mut self, pindex: usize, aindex: usize, attack: attacks::Attack) {
        self.team[pindex].moves[aindex] = attack;
    } 

    pub fn generate_ai_team(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.team.len() {
            let pokedex = rng.gen_range(0, self.usable_moves_table.len());
            self.team[i] = *self.usable_moves_table.get_index(pokedex).unwrap().0;
            //prevent duplicates
            while self.has_dup(&self.team[i].moves) {
                for j in 0..self.team[i].moves.len() {
                    let moves = self.usable_moves_table.get_index(pokedex).unwrap().1;
                    self.team[i].moves[j] = moves[rng.gen_range(0, moves.len())];
                }
            }
        }
    } 

    pub fn has_dup<T: PartialEq>(&self, slice: &[T]) -> bool {
        for i in 1..slice.len() {
            if slice[i..].contains(&slice[i - 1]) {
                return true;
            }
        }
        false
    }

    fn init_usable_moves(&mut self) {
        self.usable_moves_table = IndexMap::with_capacity(151);

        //abra 
        self.usable_moves_table.insert(pokemon::abra(), vec![
            attacks::teleport(),
            attacks::mega_punch(),
            attacks::mega_kick(),
            attacks::toxic(),
            attacks::body_slam(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::submission(),
            attacks::counter(),
            attacks::seismic_toss(),
            attacks::rage(),
            attacks::psychic(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::metronome(),
            attacks::skull_bash(),
            attacks::rest(),
            attacks::thunder_wave(),
            attacks::psywave(),
            attacks::tri_attack(),
            attacks::substitue(),
            attacks::flash()
        ]);

        //aerodacty
        self.usable_moves_table.insert(pokemon::aerodactyl(), vec![
            attacks::wing_attack(),
            attacks::agility(),
            attacks::supersonic(),
            attacks::bite(),
            attacks::take_down(),
            attacks::hyper_beam(),
            attacks::razor_wind(),
            attacks::whirlwind(),
            attacks::toxic(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::hyper_beam(),
            attacks::rage(),
            attacks::dragon_rage(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::fire_blast(),
            attacks::swift(),
            attacks::sky_attack(),
            attacks::rest(),
            attacks::substitue(),
            attacks::fly()
        ]);

        //alakazam
        self.usable_moves_table.insert(pokemon::alakazam(), vec![
            attacks::teleport(),
            attacks::kinesis(),
            attacks::confusion(),
            attacks::disable(),
            attacks::psybeam(),
            attacks::recover(),
            attacks::psychic(),
            attacks::reflect(),
            attacks::mega_punch(),
            attacks::mega_kick(),
            attacks::toxic(),
            attacks::body_slam(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::hyper_beam(),
            attacks::submission(),
            attacks::counter(),
            attacks::seismic_toss(),
            attacks::rage(),
            attacks::dig(),
            attacks::psychic(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::bide(),
            attacks::metronome(),
            attacks::skull_bash(),
            attacks::rest(),
            attacks::thunder_wave(),
            attacks::psywave(),
            attacks::tri_attack(),
            attacks::substitue(),
            attacks::flash()
        ]);

        //arbok
        self.usable_moves_table.insert(pokemon::arbok(), vec![
            attacks::wrap(),
            attacks::leer(),
            attacks::poison_sting(),
            attacks::bite(),
            attacks::glare(),
            attacks::screech(),
            attacks::acid(),
            attacks::toxic(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::hyper_beam(),
            attacks::mega_drain(),
            attacks::earthquake(),
            attacks::fissure(),
            attacks::dig(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::bide(),
            attacks::skull_bash(),
            attacks::rest(),
            attacks::rock_slide(),
            attacks::substitue(),
            attacks::strength()
        ]);

        //arcanine
        self.usable_moves_table.insert(pokemon::arcanine(), vec![
            attacks::roar(),
            attacks::ember(),
            attacks::leer(),
            attacks::take_down(),
            attacks::toxic(),
            attacks::body_slam(),
            attacks::double_edge(),
            attacks::hyper_beam(),
            attacks::rage(),
            attacks::dragon_rage(),
            attacks::dig(),
            attacks::teleport(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::fire_blast(),
            attacks::swift(),
            attacks::skull_bash(),
            attacks::rest(),
            attacks::substitue(),
            attacks::agility(),
            attacks::bite(),
            attacks::flamethrower()
        ]);

        //articuno
        self.usable_moves_table.insert(pokemon::articuno(), vec![
            attacks::peck(),
            attacks::ice_beam(),
            attacks::blizzard(),
            attacks::agility(),
            attacks::mist(),
            attacks::razor_wind(),
            attacks::whirlwind(),
            attacks::toxic(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::bubble_beam(),
            attacks::water_gun(),
            attacks::hyper_beam(),
            attacks::rage(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::swift(),
            attacks::sky_attack(),
            attacks::rest(),
            attacks::substitue(),
            attacks::fly()
        ]);

        //beedrill
        self.usable_moves_table.insert(pokemon::beedrill(), vec![
            attacks::fury_attack(),
            attacks::focus_energy(),
            attacks::twineedle(),
            attacks::rage(),
            attacks::pin_missile(),
            attacks::agility(),
            attacks::sword_dance(),
            attacks::toxic(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::hyper_beam(),
            attacks::rage(),
            attacks::mega_drain(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::swift(),
            attacks::skull_bash(),
            attacks::rest(),
            attacks::substitue(),
            attacks::cut(),
            attacks::harden(),
            attacks::poison_sting(),
            attacks::string_shot()
        ]);

        //bellsprout
        self.usable_moves_table.insert(pokemon::bellsprout(), vec![
            attacks::vine_whip(),
            attacks::growth(),
            attacks::wrap(),
            attacks::poison_powder(),
            attacks::sleep_powder(),
            attacks::stun_spore(),
            attacks::acid(),
            attacks::razor_leaf(),
            attacks::slam(),
            attacks::sword_dance(),
            attacks::toxic(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::rage(),
            attacks::mega_drain(),
            attacks::solar_beam(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::rest(),
            attacks::substitue(),
            attacks::cut()
        ]);

        //blastoise
        self.usable_moves_table.insert(pokemon::blastoise(), vec![
            attacks::tackle(),
            attacks::tail_whip(),
            attacks::bubble(),
            attacks::water_gun(),
            attacks::bite(),
            attacks::withdraw(),
            attacks::skull_bash(),
            attacks::hydro_pump(),
            attacks::mega_punch(),
            attacks::mega_kick(),
            attacks::toxic(),
            attacks::body_slam(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::bubble_beam(),
            attacks::ice_beam(),
            attacks::blizzard(),
            attacks::hyper_beam(),
            attacks::submission(),
            attacks::counter(),
            attacks::seismic_toss(),
            attacks::rage(),
            attacks::earthquake(),
            attacks::fissure(),
            attacks::dig(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::skull_bash(),
            attacks::rest(),
            attacks::substitue(),
            attacks::surf(),
            attacks::strength()
        ]);

        //bulbasaur
        self.usable_moves_table.insert(pokemon::bulbasur(), vec![
            attacks::tackle(), 
            attacks::growl(),
            attacks::leech_seed(),
            attacks::vine_whip(),
            attacks::poison_powder(),
            attacks::razor_leaf(),
            attacks::growth(),
            attacks::sleep_powder(),
            attacks::solar_beam(),
            attacks::sword_dance(),
            attacks::toxic(),
            attacks::body_slam(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::rage(),
            attacks::mega_drain(),
            attacks::solar_beam(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::rest(),
            attacks::substitue(),
            attacks::cut()
        ]);

        //butterfree
        self.usable_moves_table.insert(pokemon::butterfree(), vec![
            attacks::confusion(),
            attacks::poison_powder(),
            attacks::stun_spore(),
            attacks::sleep_powder(),
            attacks::supersonic(),
            attacks::whirlwind(),
            attacks::gust(),
            attacks::psybeam(),
            attacks::razor_wind(),
            attacks::toxic(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::hyper_beam(),
            attacks::rage(),
            attacks::mega_drain(),
            attacks::solar_beam(),
            attacks::psychic(),
            attacks::teleport(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::swift(),
            attacks::rest(),
            attacks::psywave(),
            attacks::substitue(),
            attacks::flash()
        ]);

        //caterpie
        self.usable_moves_table.insert(pokemon::caterpie(), vec![
            attacks::tackle(),
            attacks::string_shot()
        ]);

        //chansey
        self.usable_moves_table.insert(pokemon::chansey(), vec![
            attacks::pound(),
            attacks::tail_whip(),
            attacks::double_slap(),
            attacks::sing(),
            attacks::growl(),
            attacks::minimize(),
            attacks::defense_curl(),
            attacks::light_screen(),
            attacks::double_edge(),
            attacks::mega_punch(),
            attacks::mega_kick(),
            attacks::toxic(),
            attacks::body_slam(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::bubble_beam(),
            attacks::water_gun(),
            attacks::ice_beam(),
            attacks::blizzard(),
            attacks::hyper_beam(),
            attacks::submission(),
            attacks::counter(),
            attacks::seismic_toss(),
            attacks::rage(),
            attacks::solar_beam(),
            attacks::thunderbolt(),
            attacks::thunder(),
            attacks::psychic(),
            attacks::teleport(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::metronome(),
            attacks::egg_bomb(),
            attacks::fire_blast(),
            attacks::skull_bash(),
            attacks::soft_boiled(),
            attacks::rest(),
            attacks::thunder_wave(),
            attacks::psywave(),
            attacks::tri_attack(),
            attacks::substitue(),
            attacks::strength(),
            attacks::flash()
        ]);

        //charizard
        self.usable_moves_table.insert(pokemon::charizard(), vec![
            attacks::scratch(),
            attacks::growl(),
            attacks::ember(),
            attacks::leer(),
            attacks::rage(),
            attacks::slash(),
            attacks::flamethrower(),
            attacks::fire_spin(),
            attacks::mega_punch(),
            attacks::sword_dance(),
            attacks::mega_kick(),
            attacks::toxic(),
            attacks::body_slam(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::hyper_beam(),
            attacks::submission(),
            attacks::counter(),
            attacks::seismic_toss(),
            attacks::rage(),
            attacks::dragon_rage(),
            attacks::earthquake(),
            attacks::fissure(),
            attacks::dig(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::fire_blast(),
            attacks::swift(),
            attacks::skull_bash(),
            attacks::rest(),
            attacks::substitue(),
            attacks::cut(),
            attacks::fly(),
            attacks::strength()
        ]);

        //charmaleon
        self.usable_moves_table.insert(pokemon::charmaleon(), vec![
            attacks::scratch(),
            attacks::growl(),
            attacks::ember(),
            attacks::leer(),
            attacks::rage(),
            attacks::slash(),
            attacks::flamethrower(),
            attacks::fire_spin(),
            attacks::mega_punch(),
            attacks::sword_dance(),
            attacks::mega_kick(),
            attacks::toxic(),
            attacks::body_slam(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::submission(),
            attacks::counter(),
            attacks::seismic_toss(),
            attacks::rage(),
            attacks::dragon_rage(),
            attacks::dig(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::fire_blast(),
            attacks::swift(),
            attacks::skull_bash(),
            attacks::rest(),
            attacks::substitue(),
            attacks::cut(),
            attacks::strength()
        ]);

        //charmander
        self.usable_moves_table.insert(pokemon::charmander(), vec![
            attacks::scratch(),
            attacks::growl(),
            attacks::ember(),
            attacks::leer(),
            attacks::rage(),
            attacks::slash(),
            attacks::flamethrower(),
            attacks::fire_spin(),
            attacks::mega_punch(),
            attacks::mega_kick(),
            attacks::sword_dance(),
            attacks::toxic(),
            attacks::body_slam(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::submission(),
            attacks::counter(),
            attacks::seismic_toss(),
            attacks::rage(),
            attacks::dragon_rage(),
            attacks::dig(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::fire_blast(),
            attacks::swift(),
            attacks::skull_bash(),
            attacks::rest(),
            attacks::substitue(),
            attacks::cut(),
            attacks::strength()
        ]);

        //clefable
        self.usable_moves_table.insert(pokemon::clefable(), vec![
            attacks::sing(),
            attacks::double_slap(),
            attacks::minimize(),
            attacks::metronome(),
            attacks::mega_punch(),
            attacks::mega_kick(),
            attacks::toxic(),
            attacks::body_slam(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::bubble_beam(),
            attacks::water_gun(),
            attacks::ice_beam(),
            attacks::blizzard(),
            attacks::hyper_beam(),
            attacks::submission(),
            attacks::counter(),
            attacks::seismic_toss(),
            attacks::rage(),
            attacks::solar_beam(),
            attacks::thunderbolt(),
            attacks::thunder(),
            attacks::psychic(),
            attacks::teleport(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::metronome(),
            attacks::fire_blast(),
            attacks::skull_bash(),
            attacks::rest(),
            attacks::thunder_wave(),
            attacks::psywave(),
            attacks::tri_attack(),
            attacks::substitue(),
            attacks::strength(),
            attacks::flash(),
            attacks::defense_curl(),
            attacks::growl(),
            attacks::light_screen(),
            attacks::pound()
        ]);

        //clefairy
        self.usable_moves_table.insert(pokemon::clefairy(), vec![
            attacks::pound(),
            attacks::growl(),
            attacks::sing(),
            attacks::double_slap(),
            attacks::minimize(),
            attacks::metronome(),
            attacks::defense_curl(),
            attacks::light_screen(),
            attacks::mega_punch(),
            attacks::mega_kick(),
            attacks::toxic(),
            attacks::body_slam(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::bubble_beam(),
            attacks::water_gun(),
            attacks::ice_beam(),
            attacks::blizzard(),
            attacks::submission(),
            attacks::counter(),
            attacks::seismic_toss(),
            attacks::rage(),
            attacks::solar_beam(),
            attacks::thunderbolt(),
            attacks::thunder(),
            attacks::psychic(),
            attacks::teleport(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::metronome(),
            attacks::fire_blast(),
            attacks::skull_bash(),
            attacks::rest(),
            attacks::thunder_wave(),
            attacks::psywave(),
            attacks::tri_attack(),
            attacks::substitue(),
            attacks::strength(),
            attacks::flash()
        ]);

        //cloyster
        self.usable_moves_table.insert(pokemon::cloyster(), vec![
            attacks::withdraw(),
            attacks::supersonic(),
            attacks::clamp(),
            attacks::aurora_beam(),
            attacks::spike_cannon(),
            attacks::toxic(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::bubble_beam(),
            attacks::water_gun(),
            attacks::ice_beam(),
            attacks::blizzard(),
            attacks::hyper_beam(),
            attacks::rage(),
            attacks::teleport(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::self_destruct(),
            attacks::swift(),
            attacks::rest(),
            attacks::explosion(),
            attacks::tri_attack(),
            attacks::substitue(),
            attacks::surf(),
            attacks::leer(),
            attacks::tackle()
        ]);

        //cubone
        self.usable_moves_table.insert(pokemon::cubone(), vec![
            attacks::growl(),
            attacks::bone_club(),
            attacks::tail_whip(),
            attacks::headbutt(),
            attacks::leer(),
            attacks::focus_energy(),
            attacks::thrash(),
            attacks::bonemerang(),
            attacks::rage(),
            attacks::mega_punch(),
            attacks::mega_kick(),
            attacks::toxic(),
            attacks::body_slam(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::bubble_beam(),
            attacks::water_gun(),
            attacks::ice_beam(),
            attacks::blizzard(),
            attacks::submission(),
            attacks::counter(),
            attacks::seismic_toss(),
            attacks::rage(),
            attacks::earthquake(),
            attacks::fissure(),
            attacks::dig(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::bide(),
            attacks::fire_blast(),
            attacks::skull_bash(),
            attacks::rest(),
            attacks::substitue(),
            attacks::strength()
        ]);

        self.usable_moves_table.insert(pokemon::dewgong(), vec![
            attacks::headbutt(),
            attacks::growl(),
            attacks::aurora_beam(),
            attacks::rest(),
            attacks::take_down(),
            attacks::ice_beam(),
            attacks::toxic(),
            attacks::horn_drill(),
            attacks::body_slam(),
            attacks::double_edge(),
            attacks::bubble_beam(),
            attacks::water_gun(),
            attacks::blizzard(),
            attacks::hyper_beam(),
            attacks::pay_day(),
            attacks::rage(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::bide(),
            attacks::skull_bash(),
            attacks::substitue(),
            attacks::surf(),
            attacks::strength()
            ]);
        
        self.usable_moves_table.insert(pokemon::diglett(), vec![
            attacks::scratch(),
            attacks::growl(),
            attacks::dig(),
            attacks::sand_attack(),
            attacks::slash(),
            attacks::earthquake(),
            attacks::toxic(),
            attacks::body_slam(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::rage(),
            attacks::fissure(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::bide(),
            attacks::rest(),
            attacks::rock_slide(),
            attacks::substitue(),
            attacks::cut()
        ]);

        self.usable_moves_table.insert(pokemon::ditto(), vec![
            attacks::transform()
        ]);

        self.usable_moves_table.insert(pokemon::dodrio(), vec![
            attacks::peck(),
            attacks::growl(),
            attacks::fury_attack(),
            attacks::drill_peck(),
            attacks::rage(),
            attacks::tri_attack(),
            attacks::agility(),
            attacks::whirlwind(),
            attacks::toxic(),
            attacks::body_slam(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::hyper_beam(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::skull_bash(),
            attacks::sky_attack(),
            attacks::rest(),
            attacks::substitue(),
            attacks::fly()
        ]);
        self.usable_moves_table.insert(pokemon::doduo(), vec![
            attacks::peck(),
            attacks::growl(),
            attacks::fury_attack(),
            attacks::drill_peck(),
            attacks::rage(),
            attacks::tri_attack(),
            attacks::agility(),
            attacks::whirlwind(),
            attacks::toxic(),
            attacks::body_slam(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::skull_bash(),
            attacks::sky_attack(),
            attacks::rest(),
            attacks::substitue(),
            attacks::fly()
        ]);
        self.usable_moves_table.insert(pokemon::dragonair(), vec![
            attacks::wrap(),
            attacks::leer(),
            attacks::thunder_wave(),
            attacks::agility(),
            attacks::slam(),
            attacks::dragon_rage(),
            attacks::hyper_beam(),
            attacks::toxic(),
            attacks::horn_drill(),
            attacks::body_slam(),
            attacks::take_down(),
            attacks::double_edge(),
            attacks::bubble_beam(),
            attacks::water_gun(),
            attacks::ice_beam(),
            attacks::blizzard(),
            attacks::rage(),
            attacks::thunderbolt(),
            attacks::thunder(),
            attacks::mimic(),
            attacks::double_team(),
            attacks::reflect(),
            attacks::bide(),
            attacks::fire_blast(),
            attacks::swift(),
            attacks::skull_bash(),
            attacks::rest(),
            attacks::substitue(),
            attacks::surf(),
        ]);
    }
}