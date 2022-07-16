
use super::*;

#[acmd_script( agent = "lucina", script = "sound_attacks3", category = ACMD_SOUND, low_priority)]
unsafe fn soundattacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SEQUENCE(fighter,Hash40::new("seq_lucina_rnd_attack"));
            macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_m"));
        }
}

#[acmd_script( agent = "lucina", script = "sound_specialhi", category = ACMD_SOUND, low_priority)]
unsafe fn soundspecialhi(_fighter: &mut L2CAgentBase) {

    //removes vanilla sound, moved to specialhi2
}

#[acmd_script( agent = "lucina", script = "sound_specialhi2", category = ACMD_SOUND, low_priority)]
unsafe fn soundspecialhi2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter,Hash40::new("se_lucina_special_h01"));
            macros::PLAY_SEQUENCE(fighter,Hash40::new("seq_lucina_rnd_special_h"));
        }
    frame(lua_state, 39.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter,Hash40::new("se_lucina_jump01"));
            macros::PLAY_SEQUENCE(fighter,Hash40::new("seq_lucina_rnd_jump"));
        }
    frame(lua_state, 45.);
        if macros::is_excute(fighter)
        {
            //macros::PLAY_SE(fighter,Hash40::new("se_lucina_special_s04l"));
        }
}

// #[acmd_script( agent = "lucina", script = "sound_specialhi3", category = ACMD_SOUND, low_priority)]
// unsafe fn soundspecialhi3(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;

// }

#[acmd_script( agent = "lucina", script = "sound_landingfallaether", category = ACMD_SOUND, low_priority)]
unsafe fn soundspecialhi4(fighter: &mut L2CAgentBase) {
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
pub fn install() {
    smashline::install_acmd_scripts!(
        soundattacks3,
        soundspecialhi,
        soundspecialhi2,
        //soundspecialhi3,
        soundspecialhi4
    );
}