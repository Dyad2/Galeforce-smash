use super::*;

unsafe extern "C" fn appealhisound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.0);
    if macros::is_excute(fighter)
    {
        macros::PLAY_SE(fighter, Hash40::new("se_lucario_appeal_h01"));
        macros::PLAY_SE(fighter, Hash40::new("vc_lucario_appeal01"));
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.sound_acmd("sound_appealhir", appealhisound, Priority::Low);
    agent.sound_acmd("sound_appealhil", appealhisound, Priority::Low);
}