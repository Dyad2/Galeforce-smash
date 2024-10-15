use super::*;

//effects
unsafe extern "C" fn fxattackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rockman_slashcraw"), Hash40::new("rockman_slashcraw"), Hash40::new("top"), 0, 10, 9, 0, 0, 0, 1, true, *EF_FLIP_YZ);
            EffectModule::set_rate_last(fighter.module_accessor, 0.7);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_attackairb", fxattackairb, Priority::Low);
}