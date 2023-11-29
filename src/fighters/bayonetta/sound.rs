use super::*;

//air
unsafe extern "C" fn attackairb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_bayonetta_rnd_attack_air_fb"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_bayonetta_attackair_b01"));
    }
}

//others
unsafe extern "C" fn throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_bayonetta_rnd_attack_throw_f"));
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_bayonetta_smash_s02"));
            macros::PLAY_SE(fighter, Hash40::new("se_bayonetta_wecked_chargefull"));
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_bayonetta_throw_f01"));
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_bayonetta_smash_h03"));
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.sound_acmd("sound_attackairb", attackairb,);
    agent.sound_acmd("sound_throwf", throwf);
}