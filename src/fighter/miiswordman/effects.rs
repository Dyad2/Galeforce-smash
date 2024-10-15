use super::*;

//effects
unsafe extern "C" fn effectattackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, 2, 0.5, -90, 0, 0, 0.5, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.75, 0.9, 1);
            macros::AFTER_IMAGE4_ON_WORK_arg29(fighter, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD, 5, Hash40::new("haver"), 0.0, 0.2, 0.0, Hash40::new("haver"), -0.0, 0.2, 0.0, true, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE, Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.1, 0.1);
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0.65 * PostureModule::lr(fighter.module_accessor), 8.65, 0, 0, 0, 0, 0.88, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.5)
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_DETACH_KIND(fighter, Hash40::new("sys_attack_speedline"), -1);
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            macros::AFTER_IMAGE_OFF(fighter, 2);
        }
}

unsafe extern "C" fn effectlandingairlw(fighter: &mut L2CAgentBase) {

        if macros::is_excute(fighter)
        {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
}

unsafe extern "C" fn soundlandingairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_LANDING_SE(fighter, Hash40::new("se_miiswordsman_landing02"));
        }
}

unsafe extern "C" fn exlandingairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            notify_event_msc_cmd!(fighter, 0x26769bd1de as u64, 0, 30, 8);
        }
    frame(lua_state, 51.);
        if macros::is_excute(fighter)
        {
            ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_attackairlw", effectattackairlw, Priority::Low);
    agent.effect_acmd("effect_landingairlw", effectlandingairlw, Priority::Low);

    agent.sound_acmd("sound_landingairlw", soundlandingairlw, Priority::Low);

    agent.expression_acmd("expression_landingairlw", exlandingairlw, Priority::Low);
}