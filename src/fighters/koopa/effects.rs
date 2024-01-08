use super::*;

//effect
unsafe extern "C" fn fx_attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 5, 14, -2, 10, -20, 0, 1.4, true, *EF_FLIP_YZ);
            macros::LAST_PARTICLE_SET_COLOR(fighter, 1, 0.544, 0.54);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 5, 16, -6, 10, -20, 0, 1.8, true, *EF_FLIP_YZ);
            macros::LAST_PARTICLE_SET_COLOR(fighter, 1, 0.544, 0.54);
            macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 9.0, 15.5, /*10, 22,*/ 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 360, true, 0.7);
        }
}

unsafe extern "C" fn fx_attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopa_scratch"), Hash40::new("koopa_scratch"), Hash40::new("top"), 0, 17, 2, -1, 16, -107, 2, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
            let handle = EffectModule::get_last_handle(fighter.module_accessor) as u32;
            let scratch_offset = smash::phx::Vector3f{ x: 0.0, y: 13.5, z: 4.5};
            let scratch_scale = smash::phx::Vector3f{ x: 1.66, y: 1.66, z: 1.66};
            EffectModule::set_pos(fighter.module_accessor, handle, &scratch_offset);
            EffectModule::set_scale(fighter.module_accessor, handle, &scratch_scale);
        }
}

//expression
unsafe extern "C" fn ex_landingheavy(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            smash_script::slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
            if !VarModule::is_flag(fighter.module_accessor, commons::instance::flag::WAVEDASH) {
                macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            }
        }
    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            smash_script::slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_attack11", fx_attack11,);
    agent.effect_acmd("effect_attackairf", fx_attackairf,);
    agent.expression_acmd("expression_landingheavy", ex_landingheavy,);
}