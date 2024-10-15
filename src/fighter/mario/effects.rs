use super::*;

unsafe extern "C" fn effect_attackhi3(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mario_atkhi3_arc"), Hash40::new("mario_atkhi3_arc"), Hash40::new("top"), 2, 12, 1, 0, 4, 103, 1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_attackhi3", effect_attackhi3, Priority::Low);
}