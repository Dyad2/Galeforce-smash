use super::*;

//effect
unsafe extern "C" fn effect_attackdash(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) 
        {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        }
    frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) 
        {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("toonlink_sword1"), Hash40::new("toonlink_sword2"), 7, Hash40::new("sword1"), 0.5, 0.0, -0.5, Hash40::new("sword1"), 10.57, -0.13, -0.12, true, Hash40::new("null"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("toonlink_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) 
        {
            macros::AFTER_IMAGE_OFF(fighter, 5);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("toonlink_sword"), false, false);
        }
}

unsafe extern "C" fn fx_appeallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("toonlink_appeal_sweat"), Hash40::new("head"), 0, -1, 0, 0, 0, 0, 1, true);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("toonlink_appeal_sword"), Hash40::new("top"), 0, 7.5, 2, 0, 0, 20, 1, 1, 2, 0, 10, 30, 10, true);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("toonlink_sword1"), Hash40::new("toonlink_sword2"), 4, Hash40::new("sword1"), 0.5, 0.0, -0.5, Hash40::new("sword1"), 10.57, -0.13, -0.12, true, Hash40::new("toonlink_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
        }
    wait(lua_state, 5.);
        if macros::is_excute(fighter)
        {
           macros::AFTER_IMAGE_OFF(fighter, 2);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("toonlink_appeal_sword"), Hash40::new("top"), 0, 6.5, 2, 0, 0, 200, 1, 1, 2, 0, 10, 30, 10, true);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("toonlink_sword1"), Hash40::new("toonlink_sword2"), 4, Hash40::new("sword1"), 0.5, 0.0, -0.5, Hash40::new("sword1"), 10.57, -0.13, -0.12, true, Hash40::new("toonlink_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
        }
    wait(lua_state, 5.);
        if macros::is_excute(fighter)
        {
           macros::AFTER_IMAGE_OFF(fighter, 2);
        }
    frame(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("toonlink_appeal_sword"), Hash40::new("top"), 0, 6.5, 2, 0, -20, -25, 1, 1, 2, 0, 10, 30, 10, true);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("toonlink_sword1"), Hash40::new("toonlink_sword2"), 4, Hash40::new("sword1"), 0.5, 0.0, -0.5, Hash40::new("sword1"), 10.57, -0.13, -0.12, true, Hash40::new("toonlink_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
        }
    wait(lua_state, 5.);
        if macros::is_excute(fighter)
        {
           macros::AFTER_IMAGE_OFF(fighter, 2);
        }
    frame(lua_state, 33.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("toonlink_appeal_sword"), Hash40::new("top"), 0, 7.5, 2, 0, -20, 190, 1, 1, 2, 0, 10, 30, 10, true);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("toonlink_sword1"), Hash40::new("toonlink_sword2"), 4, Hash40::new("sword1"), 0.5, 0.0, -0.5, Hash40::new("sword1"), 10.57, -0.13, -0.12, true, Hash40::new("toonlink_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
        }
    wait(lua_state, 5.);
        if macros::is_excute(fighter)
        {
           macros::AFTER_IMAGE_OFF(fighter, 2);
        }
    frame(lua_state, 41.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("toonlink_appeal_sword"), Hash40::new("top"), 0, 7.5, 2, 0, -20, 20, 1, 1, 2, 0, 10, 30, 10, true);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("toonlink_sword1"), Hash40::new("toonlink_sword2"), 4, Hash40::new("sword1"), 0.5, 0.0, -0.5, Hash40::new("sword1"), 10.57, -0.13, -0.12, true, Hash40::new("toonlink_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
        }
    wait(lua_state, 5.);
        if macros::is_excute(fighter)
        {
           macros::AFTER_IMAGE_OFF(fighter, 2);
        }
    frame(lua_state, 69.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("toonlink_appeal_sweat_b"), Hash40::new("head"), 2.5, 1, 0, 0, 0, 0, 1, true)
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_attackdash", effect_attackdash, Priority::Low);
    agent.effect_acmd("effect_appeallwr", fx_appeallw, Priority::Low);
    agent.effect_acmd("effect_appeallwl", fx_appeallw, Priority::Low);
}