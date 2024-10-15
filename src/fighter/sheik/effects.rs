use super::*;

//effect
unsafe extern "C" fn effectattackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
                VisibilityModule::set_whole(fighter.module_accessor, false);
                macros::EFFECT(fighter, Hash40::new("sheik_fushin_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
            }
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
                VisibilityModule::set_whole(fighter.module_accessor, true);
            }
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sheik_attack_cut"), Hash40::new("top"), 1, 7, 6, 0, 20, 45, 0.7, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sheik_attack_cut"), Hash40::new("top"), -1, 7, 6, 0, 20, -130, 0.7, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
}

unsafe extern "C" fn effectattackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 13, -2.5, 78, 0, 0, 1.1, true);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            if !VarModule::is_flag(fighter.module_accessor, sheik::instance::flag::ATTACK_AIR_LW_W) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sheik_atk_lw"), Hash40::new("top"), 0, -10, 2, -105, 0, 0, 0.8, true);
            }
        }
}

unsafe extern "C" fn effectescapeair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
                wait(lua_state, 3.);
                    if macros::is_excute(fighter)
                    {
                        VisibilityModule::set_whole(fighter.module_accessor, false);
                        macros::EFFECT(fighter, Hash40::new("sheik_fushin_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
                    }
                wait(lua_state, 19.);
                    if macros::is_excute(fighter)
                    {
                        macros::EFFECT(fighter, Hash40::new("sheik_fushin_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
                        VisibilityModule::set_whole(fighter.module_accessor, true);
                    }
            }
        }
}

unsafe extern "C" fn effectescapeairslide(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
                wait(lua_state, 3.);
                    if macros::is_excute(fighter)
                    {
                        VisibilityModule::set_whole(fighter.module_accessor, false);
                        macros::EFFECT(fighter, Hash40::new("sheik_fushin_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
                    }
                wait(lua_state, 19.);
                    if macros::is_excute(fighter)
                    {
                        macros::EFFECT(fighter, Hash40::new("sheik_fushin_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
                        VisibilityModule::set_whole(fighter.module_accessor, true);
                    }
            }
        }
}

unsafe extern "C" fn effectcliffcatch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            VisibilityModule::set_whole(fighter.module_accessor, true);
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("sys_cliff_catch"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
}

unsafe extern "C" fn effectlandingheavy(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
}

unsafe extern "C" fn effectescapef(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
                macros::EFFECT(fighter, Hash40::new("sheik_fushin_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
                VisibilityModule::set_whole(fighter.module_accessor, false);
            }
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
                macros::EFFECT(fighter, Hash40::new("sheik_fushin_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
                VisibilityModule::set_whole(fighter.module_accessor, true);
            }
        }
}

unsafe extern "C" fn effectescapeb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
                macros::EFFECT(fighter, Hash40::new("sheik_fushin_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
                VisibilityModule::set_whole(fighter.module_accessor, false);
            }
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
                macros::EFFECT(fighter, Hash40::new("sheik_fushin_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
                VisibilityModule::set_whole(fighter.module_accessor, true);
            }
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_attackdash", effectattackdash, Priority::Low);
    agent.effect_acmd("effect_attackairlw", effectattackairlw, Priority::Low);
    agent.effect_acmd("effect_escapeair", effectescapeair, Priority::Low);
    agent.effect_acmd("effect_escapeairslide", effectescapeairslide, Priority::Low);
    agent.effect_acmd("effect_cliffcatch", effectcliffcatch, Priority::Low);
    agent.effect_acmd("effect_landingheavy", effectlandingheavy, Priority::Low);
    agent.effect_acmd("effect_escapef", effectescapef, Priority::Low);
    agent.effect_acmd("effect_escapeb", effectescapeb, Priority::Low);
}