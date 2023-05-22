use super::*;

//effects
#[acmd_script( agent = "roy", script = "effect_attackairlw", category = ACMD_EFFECT, low_priority)]
unsafe fn effectairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true)
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("roy_fire"), false, false);
        }
}

#[acmd_script( agent = "roy", script = "effect_attackairn", category = ACMD_EFFECT, low_priority)]
unsafe fn effectairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 8.0);
        if macros::is_excute(fighter)
        {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_roy_sword1"), Hash40::new("tex_roy_sword2"), 10, Hash40::new("sword1"), 0.0, 0.0, -0.8, Hash40::new("sword1"), -0.0, -0.0, 14.5, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("roy_sword_light"), Hash40::new("sword1"), 0, 0, 10.5, 0, 0, 0, 1.0, true);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.6);
        }
    wait(lua_state, 3.0);
        if macros::is_excute(fighter)
        {
            macros::AFTER_IMAGE_OFF(fighter, 4);
        }
    wait(lua_state, 4.0);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("roy_sword_light"), false, true);
        }
}

//sound
#[acmd_script( agent = "roy", script = "sound_attackairlw", category = ACMD_SOUND, low_priority)]
unsafe fn soundairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_roy_rnd_attack_air"));
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_roy_attackair_f01"));
            macros::PLAY_SE(fighter, Hash40::new("se_roy_special_h01"));
        }
}

#[acmd_script( agent = "roy", script = "sound_attackairn", category = ACMD_SOUND, low_priority)]
unsafe fn soundairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.0);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_roy_rnd_attack_air"));
        }
    wait(lua_state, 3.0);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_roy_attackair_n01"));
        }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effectairlw,
        effectairn,
        soundairlw,
        soundairn
    );
}