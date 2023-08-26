use super::*;

#[acmd_script( agent = "captain", script = "effect_attacks4hi", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacks4hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("captain_smash_line"), Hash40::new("top"), 0, 3, -14, -25, 0, 0, 1.2, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "captain", script = "effect_attacks4", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacks4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("captain_smash_line"), Hash40::new("top"), 0, 10, -16, 0, 0, 0, 1.2, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "captain", script = "effect_attacks4lw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacks4lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("captain_smash_line"), Hash40::new("top"), 0, 16, -12, 25, 0, 0, 1.2, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//sound
#[acmd_script( agent = "captain", scripts = ["sound_specialn", "sound_specialairn"], category = ACMD_SOUND, low_priority)]
unsafe fn sound_specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("vc_captain_001"))
        }
    wait(lua_state, 2.0);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_captain_special_n04"))
        }
    frame(lua_state, 33.0);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_captain_leather_creak01"))
        }
    frame(lua_state, 60.0);
        if macros::is_excute(fighter)
        {
            macros::STOP_SE(fighter, Hash40::new("se_captain_special_n04"));
            macros::PLAY_SE(fighter, Hash40::new("vc_captain_002"));
            macros::PLAY_SE(fighter, Hash40::new("se_captain_special_n03"));
        }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_attacks4hi,
        effect_attacks4,
        effect_attacks4lw,
        sound_specialn
    );
}