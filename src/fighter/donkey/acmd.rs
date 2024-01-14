use super::*;

//global edits
unsafe extern "C" fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

unsafe extern "C" fn turndash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

unsafe extern "C" fn attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
        	macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        	macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 6.0, 361, 80, 0, 10, 5.0, 0.0, 0.0, 0.0, Some(7.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.0, 361, 80, 0, 5, 5.5, 8.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL as i32), 0);
        }
}

unsafe extern "C" fn attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.25);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
            macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 8.0, 100, 75, 0, 70, 4.8, 2.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 100, 85, 0, 70, 4.0, 3.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("handr"), 10.0, 100, 85, 0, 70, 5.3, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 3.0, false);
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            AttackModule::clear_all(fighter.module_accessor);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        }
}

//air
unsafe extern "C" fn attackairhi(agent: &mut L2CAgentBase) {

    frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent)
        {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(agent, 0, 0, Hash40::new("head"), 13.0, 90, 90, 0, 32, 7.5, 2.5, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        }
    wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent)
        {
            macros::ATTACK(agent, 0, 0, Hash40::new("head"), 9.0, 85, 90, 0, 45, 7.0, 2.5, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        }
    wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent)
        {
            AttackModule::clear_all(agent.module_accessor);
        }
    frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent)
        {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}
//grabs
unsafe extern "C" fn catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.8);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::CATCH(fighter, 0, Hash40::new("top"), 5.8, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(13.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 2.9, 0.0, 7.0, 1.1, Some(0.0), Some(7.0), Some(16.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }
    macros::game_CaptureCutCommon(fighter);
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.15); //1.2 was faf 37
            smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, false);
        }
}

unsafe extern "C" fn catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.8);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::CATCH(fighter, 0, Hash40::new("top"), 4.6, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(15.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 2.3, 0.0, 7.0, 1.7, Some(0.0), Some(7.0), Some(18.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }
    macros::game_CaptureCutCommon(fighter);
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.15); //1.2 was faf 44
            smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, false);
        }
}

unsafe extern "C" fn catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.8);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::CATCH(fighter, 0, Hash40::new("top"), 5.8, 0.0, 7.0, -4.0, Some(0.0), Some(7.0), Some(-21.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 2.9, 0.0, 7.0, -1.1, Some(0.0), Some(7.0), Some(-24.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }
    macros::game_CaptureCutCommon(fighter);
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.15); //1.2 was faf 40
            smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, false);
        }
}

//others
unsafe extern "C" fn escapeairslide(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_dash", dash,);
    agent.game_acmd("game_turndash", turndash,);
    agent.game_acmd("game_attacklw3", attacklw3,);
    agent.game_acmd("game_attackhi3", attackhi3,);
    agent.game_acmd("game_attackairhi", attackairhi,);
    agent.game_acmd("game_catch", catch,);
    agent.game_acmd("game_catchdash", catchdash,);
    agent.game_acmd("game_catchturn", catchturn,);
    agent.game_acmd("game_escapeairslide", escapeairslide,);
}