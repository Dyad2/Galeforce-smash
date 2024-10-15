use super::*;

unsafe extern "C" fn fx_specialn(agent: &mut L2CAgentBase) {
        if macros::is_excute(agent)
        {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent)
        {
            macros::EFFECT_FOLLOW(agent, Hash40::new("luigi_fb_shoot"), Hash40::new("havel"), 0, 0, 0, -30, 0, 0, 1, true);
            macros::FLASH(agent, 0, 1, 0, 0.353);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec_s"), Hash40::new("havel"), 0, 0, 0, -30, 0, 0, 1, true);
        }
    frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent)
        {
            macros::COL_NORMAL(agent);
        }
    frame(agent.lua_state_agent, 37.0);
        if macros::is_excute(agent)
        {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("luigi_fb_shoot"), false, false);
        }
}

unsafe extern "C" fn fx_specialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent)
        {
            macros::EFFECT_FOLLOW(agent, Hash40::new("luigi_fb_shoot"), Hash40::new("havel"), 0, 0, 0, -30, 0, 0, 1, true);
            macros::FLASH(agent, 0, 1, 0, 0.353);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec_s"), Hash40::new("havel"), 0, 0, 0, -30, 0, 0, 1, true);
        }
    frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent)
        {
            macros::COL_NORMAL(agent);
        }
    frame(agent.lua_state_agent, 37.0);
        if macros::is_excute(agent)
        {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("luigi_fb_shoot"), false, false);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_specialn", fx_specialn, Priority::Low);
    agent.effect_acmd("effect_specialairn", fx_specialairn, Priority::Low);
}