use super::*;

//effects
unsafe extern "C" fn effectattack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::AFTER_IMAGE4_ON_arg29(fighter,Hash40::new("tex_marth_sword1"), Hash40::new("tex_marth_sword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), 0.0, 0.0, 12.4, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::AFTER_IMAGE_OFF(fighter, 1);
        }
}

unsafe extern "C" fn effectattack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::AFTER_IMAGE4_ON_arg29(fighter,Hash40::new("tex_marth_sword1"), Hash40::new("tex_marth_sword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), 0.0, 0.0, 12.4, true, Hash40::new("marth_sword_blue"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
        	macros::AFTER_IMAGE_OFF(fighter,2);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_attack11", effectattack11,);
    agent.effect_acmd("effect_attack12", effectattack12,);
}