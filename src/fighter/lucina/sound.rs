use super::*;

unsafe extern "C" fn soundattacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SEQUENCE(fighter,Hash40::new("seq_lucina_rnd_attack"));
            macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_m"));
        }
}

unsafe extern "C" fn soundspecialhi(_fighter: &mut L2CAgentBase) {

    //removes vanilla sound, moved to specialhi2
}

unsafe extern "C" fn soundspecialhi2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter,Hash40::new("se_lucina_special_h01"));
            macros::PLAY_SE(fighter, Hash40::new("vc_lucina_attack07")); //down smash sound
            macros::PLAY_SEQUENCE(fighter,Hash40::new("seq_lucina_rnd_special_h"));
        }
    // frame(lua_state, 39.);
    //     if macros::is_excute(fighter)
    //     {
    //         macros::PLAY_SE(fighter,Hash40::new("se_lucina_jump01"));
    //         macros::PLAY_SEQUENCE(fighter,Hash40::new("seq_lucina_rnd_jump"));
    //     }
    frame(lua_state, 46.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter,Hash40::new("se_lucina_special_s04l"));
            //macros::PLAY_SE(agent, Hash40::new("vc_lucina_attack06"));
        }
}

unsafe extern "C" fn soundspecialhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_LANDING_SE(fighter, Hash40::new("se_lucina_landing02"));
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_lucina_step_right_m"));
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_lucina_step_left_m"));
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.sound_acmd("sound_attacks3", soundattacks3,);
    agent.sound_acmd("sound_specialhi", soundspecialhi,);
    agent.sound_acmd("sound_specialhi2", soundspecialhi2,);
    agent.sound_acmd("sound_landingfallaether", soundspecialhi4,);
}