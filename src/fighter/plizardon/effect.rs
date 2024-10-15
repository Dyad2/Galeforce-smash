use super::*;

//effects
unsafe extern "C" fn effectairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_SCALE_W(fighter, 0.66, 0.66, 0.66);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_attackairhi", effectairhi, Priority::Low);
}