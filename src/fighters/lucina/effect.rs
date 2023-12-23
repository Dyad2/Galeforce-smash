use super::*;

//effects
unsafe extern "C" fn effectattacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 1, 0, 1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 2, 10.5, 5.5, -20, 0, 22, 0.8, true, 0.5);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), true, true);
        }
}

unsafe extern "C" fn effecthi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            macros::AFTER_IMAGE_OFF(fighter, 1);
        }
}

unsafe extern "C" fn effectspecialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_dolphin_swing"), Hash40::new("top"), 0, 12, -1, 14, -30, 37, 1, true);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        }
}

unsafe extern "C" fn effectspecialhi2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            macros::AFTER_IMAGE_OFF(fighter, 6);
            macros::EFFECT(fighter,Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 0, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    frame(lua_state, 36.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_dolphin_jump"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
            //macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
        }
    frame(lua_state, 37.);
        if macros::is_excute(fighter)
        {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 3, 0, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 2.5);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_dolphin_jump"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, true);
            macros::EFFECT_DETACH_KIND(fighter, Hash40::new("lucina_dolphin_jump"), -1);
            macros::EFFECT_DETACH_KIND(fighter, Hash40::new("lucina_dolphin_swing"), -1);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.35);
        }
    frame(lua_state, 45.);
        if macros::is_excute(fighter)
        {
            macros::AFTER_IMAGE_OFF(fighter, 6);
        }
    frame(lua_state, 46.);
        if macros::is_excute(fighter)
        {
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
            macros::LAST_EFFECT_SET_ALPHA(fighter, 0.85);
        }
    frame(lua_state, 47.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW(fighter,Hash40::new("sys_spin_wind_s"), Hash40::new("top"), 0, 14, -2, 0, 180, -90, 1, true);
            macros::LAST_EFFECT_SET_RATE(fighter,2);
        }
    frame(lua_state, 48.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind_s"), Hash40::new("top"), 0, 14, -1.5, 0, 90, -90, 1.1, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 2);
        }
}

unsafe extern "C" fn effectspecialhi3(fighter: &mut L2CAgentBase) {

        if macros::is_excute(fighter)
        {
            //macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        }
}

unsafe extern "C" fn effectspecialhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::AFTER_IMAGE_OFF(fighter, 4);
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.9);
            macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 10, 0, 0, 0, 180, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
}

//expressions
unsafe extern "C" fn expressionspecialhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
            smash_script::slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
            ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_attacks3", effectattacks3,);
    agent.effect_acmd("effect_attackhi3", effecthi3,);
    agent.effect_acmd("effect_specialhi", effectspecialhi,);
    agent.effect_acmd("effect_specialhi2", effectspecialhi2,);
    agent.effect_acmd("effect_specialhi3", effectspecialhi3,);
    agent.effect_acmd("effect_landingfallaether", effectspecialhi4,);
    agent.expression_acmd("expression_landingfallaether", expressionspecialhi4,);
}