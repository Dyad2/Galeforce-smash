use super::*;

//effect
// #[acmd_script( agent = "kirby_finalcuttershot", script = "game_finalcutterregular", category = ACMD_GAME, low_priority)]
// unsafe fn effectfinalcutterregular(weapon: &mut L2CAgentBase) {
//     let lua_state = weapon.lua_state_agent;
//     let fighter.module_accessor = sv_system::battle_object_module_accessor(lua_state);
    
//         if macros::is_excute(weapon)
//         {
//             //macros::EFFECT_FOLLOW(weapon, Hash40::new("kirby_fcut"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
//             MotionModule::set_rate(fighter.module_accessor, 0.1);
//         }
// }

unsafe extern "C" fn effect_attacklw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -1, 2.5, 6, 0, -10, 5, 0.9, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_b"), true, true);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_attacklw3", effect_attacklw3,);
}