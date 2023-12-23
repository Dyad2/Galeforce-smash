use super::*;

//effect
unsafe extern "C" fn fx_attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_master_sword1"), Hash40::new("tex_master_sword2"), 5, Hash40::new("sword1"), 2.3, 0.0, 0.0, Hash40::new("sword1"), 17.0, 0.0, 0.15, true, Hash40::new("null"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, -90.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("master_swordflare"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            macros::AFTER_IMAGE_OFF(fighter, 5);
        }
    frame(lua_state, 18.);

        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("master_swordflare"), false, true);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("effect_attacks3", fx_attacks3,);
}