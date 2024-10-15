use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use smash::app::FighterSpecializer_Kirby;
use std::mem;

use smash_script::*;
use smashline::*;

use galeforce_utils::{
    vars::*,
    table_const::*,
};

mod acmd;
mod effects;
mod opff;
mod specials;

//unused?
//weapon
// #[acmd_script( agent = "kirby_finalcuttershot", script = "game_finalcutterregular", category = ACMD_GAME, low_priority)]
// unsafe fn finalcuttershot(weapon: &mut L2CAgentBase) {
//     let lua_state = weapon.lua_state_agent;

//     frame(lua_state, 1.);
//         if macros::is_excute(weapon)
//         {
//             weapon.clear_lua_stack();
//             smash_script::lua_args!(weapon, 0.5);
//             macros::LAST_EFFECT_SET_RATE(weapon, 0.5);
//             sv_animcmd::LAST_EFFECT_SET_RATE_WORK(weapon.lua_state_agent);
//             macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0, 90, 95, 0, 85, 5.0, 0.0, 5.6, -5.1, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
//         }
//     frame(lua_state, 2.);
//         if macros::is_excute(weapon)
//         {
//             macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 6.0, 90, 30, 0, 60, 3.8, 0.0, 3.0, -6.1, Some(0.0), Some(9.0), Some(-9.6), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
//         }
// }

//kirby cannot use varmodule for this yet because it gets cleared as the game ends.
static mut LAST_HAT : [i32; 9] = [1; 9];

pub fn install() {
    let agent = &mut smashline::Agent::new("kirby");
    acmd::install(agent);
    effects::install(agent);
    opff::install(agent);
    specials::install(agent);
    agent.install();
}