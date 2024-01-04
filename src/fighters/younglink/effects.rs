use super::*;

//effect
unsafe extern "C" fn fxattack100(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("younglink_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    wait(lua_state, 22.);
}

unsafe extern "C" fn fx_attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("younglink_sword_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
    frame(lua_state, 17.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("younglink_sword_flare"), false, false);
        }
}

//sound
unsafe extern "C" fn sfx_specialairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_STATUS(fighter, Hash40::new("vc_younglink_special_h01"));
            macros::PLAY_SE(fighter, Hash40::new("se_younglink_005"));
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_younglink_006"));
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_younglink_007"));
        }
    frame(lua_state, 31.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_younglink_008"));
        }
    frame(lua_state, 44.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_younglink_009"));
        }
    // frame(lua_state, 51.);
    //     if macros::is_excute(fighter)
    //     {
    //         macros::PLAY_SE(fighter, Hash40::new("se_younglink_special_h04"));
    //     }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("effect_attack100", fxattack100,);
    agent.effect_acmd("effect_attackairhi", fx_attackairhi,);
    agent.sound_acmd("sound_specialairhi", sfx_specialairhi,);
}