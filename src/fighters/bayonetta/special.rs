use super::*;

unsafe extern "C" fn specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 20, 0, 15, 0, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 20);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 20);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 20);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.65);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 88, 15, 0, 100, 5.5, 4.5, 1.0, 0.0, Some(0.0), Some(8.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 10.0, false);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 33, 10, 0, 60, 4.5, 10.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.45);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 75, 15, 0, 70, 5.5, 4.5, 0.0, 0.0, Some(0.0), Some(8.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 10.0, false);
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 55, 10, 0, 60, 5.5, 4.5, 0.0, 0.0, Some(0.0), Some(8.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 10.0, false);
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
            AttackModule::clear_all(fighter.module_accessor);
            smash_script::notify_event_msc_cmd!(fighter, 0x2bfb02b69a as u64, true);
        }
    frame(lua_state, 36.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_NO_SHOOTING_ENABLE_CANCEL as i32);
        }
}

unsafe extern "C" fn specialairsu(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(fighter.module_accessor, false);
            macros::FT_MOTION_RATE(fighter, 0.5);
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 5, 0, 20, 0, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 5);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.0);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 40, 45, 0, 70, 5.0, 6.0, 0.0, 3.0, Some(2.0), Some(0.0), Some(3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 38, 45, 0, 70, 5.0, 6.0, 0.0, -3.0, Some(2.0), Some(0.0), Some(-3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 4.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 4.0, false);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 55, 35, 0, 50, 5.0, 6.0, 0.0, 3.0, Some(2.0), Some(0.0), Some(3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 52, 35, 0, 50, 5.0, 6.0, 0.0, -3.0, Some(2.0), Some(0.0), Some(-3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 3.0, false);
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 5.0, 95, 100, 60, 0, 5.0, 6.0, 0.0, 3.0, Some(2.0), Some(0.0), Some(3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 5.0, 91, 100, 50, 0, 5.0, 6.0, 0.0, -3.0, Some(2.0), Some(0.0), Some(-3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 2.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 2.0, false);
        }
    frame(lua_state, 23.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    frame(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(fighter.module_accessor, true);
        }
}

unsafe extern "C" fn specialairsd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(fighter.module_accessor, false);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 75, 20, 0, 110, 4.5, 0.0, 2.0, 4.5, Some(0.0), Some(4.0), Some(3.2), 1.2, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_LANDING_FALL_SPECIAL);
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(fighter.module_accessor, true);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

unsafe extern "C" fn specialsholdend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 3.0);
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 2, 0, 0, 0, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 4.0, 75, 8, 0, 60, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.3, 1.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 2.5);
            macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 4.0, 80, 8, 0, 60, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.3, 1.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
        }
    frame(lua_state, 23.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 2.0);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_END_SPECIAL_S);
        }
    frame(lua_state, 34.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.33);
        }
    frame(lua_state, 37.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 4.0, 98, 65, 0, 55, 5.3, 7.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 4.0, 98, 65, 0, 55, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("waist"), 4.0, 90, 65, 0, 55, 4.5, 0.0, -0.8, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 4.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 4.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 4.0, false);
        }
    frame(lua_state, 46.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

unsafe extern "C" fn specialairsdlanding(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 45, 50, 0, 80, 4.0, 0.0, 4.0, 7.0, Some(0.0), Some(4.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

unsafe extern "C" fn speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, true);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
            macros::SEARCH(fighter, 0, 0,Hash40::new("top"), 11.5, -2.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 1,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_FIEB,*COLLISION_PART_MASK_BODY_HEAD, false);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, false);
        }
    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
            smash_script::search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        }
    frame(lua_state, 44.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
        }
}

unsafe extern "C" fn specialairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, true);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
            macros::SEARCH(fighter, 0, 0,Hash40::new("top"), 11.5, -2.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 1,*COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_FIEB,*COLLISION_PART_MASK_BODY_HEAD, false);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2ea0f68425 as u64, false);
        }
    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
            smash_script::search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        }
    frame(lua_state, 36.);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME_SUCCESS)
        {
            if macros::is_excute(fighter)
            {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    frame(lua_state, 44.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialhi", specialhi,);
    agent.game_acmd("game_specialairhi", specialhi);
    agent.game_acmd("game_specialairsu", specialairsu);
    agent.game_acmd("game_specialairsd", specialairsd);
    agent.game_acmd("game_specialsholdend", specialsholdend);
    agent.game_acmd("game_specialsdlanding", specialairsdlanding);
    agent.game_acmd("game_speciallw", speciallw);
    agent.game_acmd("game_specialairlw", specialairlw);
}