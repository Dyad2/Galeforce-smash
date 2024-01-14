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

//ground
unsafe extern "C" fn attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.35);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
           macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
           macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 37, 40, 0, 53, 3.5, 0.0, 11.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 37, 40, 0, 53, 3.5, 0.0, 10.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 37, 36, 0, 53, 4.0, 0.0, 10.0, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 37, 40, 0, 53, 3.5, 0.0, 11.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 37, 40, 0, 53, 3.5, 0.0, 11.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 37, 36, 0, 53, 4.0, 0.0, 11.0, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
           macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
           macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 2.0);
        }
    wait(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.65);
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        }
}

unsafe extern "C" fn attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 30, 0, 25, 3.5, 0.0, 5.0, 2.0, None, None, None, 1.4, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 80, 30, 0, 25, 3.5, 0.0, 4.0, 7.0, None, None, None, 1.4, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 86, 30, 0, 25, 3.5, 0.0, 3.0, 12.0, None, None, None, 1.4, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter) 
        {
            MotionModule::set_rate(fighter.module_accessor, 1.2); //was 1.428
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        }
}

//other
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
    agent.game_acmd("game_attacks3", attacks3,);
    agent.game_acmd("game_attacklw3", attacklw3,);
    agent.game_acmd("game_escapeairslide", escapeairslide,);
}