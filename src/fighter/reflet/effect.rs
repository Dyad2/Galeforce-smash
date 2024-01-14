use super::*;

//effects
unsafe extern "C" fn fx_attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                frame(lua_state, 7.);
                    if macros::is_excute(fighter)
                    {
                        macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_swoed_flare2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
                        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -2, 5, 3, 0, 0, 0, 0.5, true);
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.7, 1, 0.9)
                    }
                frame(lua_state, 8.);
                    if macros::is_excute(fighter)
                    {
                        macros::EFFECT(fighter, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 10, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
                    }
                frame(lua_state, 11.);
                    if macros::is_excute(fighter)
                    {
                        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
                    }
                frame(lua_state, 17.);
                    if macros::is_excute(fighter)
                    {
                        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                    }
                frame(lua_state, 22.);
                    if macros::is_excute(fighter)
                    {
                        macros::AFTER_IMAGE_OFF(fighter, 2)
                    }
            }
            else {
                frame(lua_state, 7.);
                    if macros::is_excute(fighter)
                    {
                        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 7, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 11.5, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
                        macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
                        macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
                        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -2, 5, 3, 0, 0, 0, 0.5, true);
                    }
                frame(lua_state, 8.);
                    if macros::is_excute(fighter)
                    {
                        macros::EFFECT(fighter, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 10, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
                    }
                frame(lua_state, 11.);
                    if macros::is_excute(fighter)
                    {
                        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
                    }
                frame(lua_state, 17.);
                    if macros::is_excute(fighter)
                    {
                        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                    }
                frame(lua_state, 22.);
                    if macros::is_excute(fighter)
                    {
                        macros::EFFECT_OFF_KIND(fighter, Hash40::new("reflet_thunderswoed"), false, false);
                        macros::EFFECT_OFF_KIND(fighter, Hash40::new("reflet_thunderswoed_flare"), false, false);
                        macros::AFTER_IMAGE_OFF(fighter, 2);
                    }
            }
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_attackdash", fx_attackdash,);
}