use super::*;

//effects
unsafe extern "C" fn effectattackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 1.3, true, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
            //macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 10, 3, -32, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
    frame(lua_state, 40.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_flash_01"), false, true);
        }
}

unsafe extern "C" fn effect_attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), 2, 0, 0.5, 0, 0, 0, 2. * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2. * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), 2, 0, -0.5, 0, 0, 0, 2. * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, 0.5, 0, 0, 0, 1.7 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9 * 1.66, true);
            macros::BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_dash_attack"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
            macros::BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
            macros::BURN_COLOR_NORMAL(fighter);
        }
}

unsafe extern "C" fn effectattackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), 2, 0, 0.5, 0, 0, 0, 2. * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2. * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), 2, 0, -0.5, 0, 0, 0, 2. * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, 0.5, 0, 0, 0, 1.7 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9 * 1.66, true);
            macros::BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
        }
    frame(lua_state, 51.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
        }
    frame(lua_state, 64.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), 2, 0, 0.5, 0, 0, 0, 2. * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2. * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), 2, 0, -0.5, 0, 0, 0, 2. * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, 0.5, 0, 0, 0, 1.7 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9 * 1.66, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9 * 1.66, true);
            macros::BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
        }
    frame(lua_state, 71.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
            macros::BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
            macros::BURN_COLOR_NORMAL(fighter);
        }
}

//sound
unsafe extern "C" fn soundattackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_samusd_smash_s01"));
        }
    wait(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_samusd_smash_s02"))
        }
}

unsafe extern "C" fn soundattackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_STATUS(fighter, Hash40::new("se_samusd_attackair_h01"));
        }
    frame(lua_state, 65.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_STATUS(fighter, Hash40::new("se_samusd_smash_s02"));
        }
}

//expression
unsafe extern "C" fn expressionattackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
        if macros::is_excute(fighter)
        {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, smash::app::ArticleOperationTarget(0));
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, true, -1);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, Hash40::new("s4s"), false, 0.0);
            smash_script::slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, true, -1);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, Hash40::new("s4s"), false, 0.0);
                ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, 1.0);
                smash_script::slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            }
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackll"), 0);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
        }
    frame(lua_state, 35.);
        if macros::is_excute(fighter)
        {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, smash::app::ArticleOperationTarget(0));
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("effect_attackhi3", effectattackhi3,);
    agent.game_acmd("effect_attacks3hi", effectattackhi3,);
    agent.game_acmd("effect_attackdash", effect_attackdash,);
    agent.effect_acmd("effect_attackairn", effectattackairn,);
    agent.sound_acmd("sound_attackhi3", soundattackhi3,);
    agent.sound_acmd("sound_attacks3hi", soundattackhi3,);
    agent.sound_acmd("sound_attackairn", soundattackairn,);
    agent.expression_acmd("expression_attackhi3", expressionattackhi3,);
    agent.expression_acmd("expression_attacks3hi", expressionattackhi3,);
}
