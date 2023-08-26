use super::*;

#[acmd_script( agent = "lucina", script = "game_specials1", category = ACMD_GAME, low_priority)]
unsafe fn specials1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
            MotionModule::set_rate(fighter.module_accessor, 0.9);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.85, 361, 30, 0, 25, 6.5, 0.0, 9.0, 13.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.85, 90, 30, 0, 25, 5.5, 0.0, 9.0, 16.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.85, 361, 30, 0, 25, 5.5, 0.0, 9.0, 8.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 5.0, false);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
            MotionModule::set_rate(fighter.module_accessor, 1.1);
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        }
}

//special case, because of the momentum
#[acmd_script( agent = "lucina", script = "game_specialairs1", category = ACMD_GAME, low_priority)]
unsafe fn specialairs1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
            MotionModule::set_rate(fighter.module_accessor, 0.9);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.85, 361, 30, 0, 25, 6.5, 0.0, 9.0, 13.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.85, 90, 30, 0, 25, 5.5, 0.0, 9.0, 16.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.85, 361, 30, 0, 25, 5.5, 0.0, 9.0, 8.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 5.0, false);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) && !(ControlModule::get_stick_y(fighter.module_accessor) < -0.23) {
                macros::SET_SPEED_EX(fighter, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(fighter.module_accessor), 1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
            MotionModule::set_rate(fighter.module_accessor, 1.1);
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        }
}

#[acmd_script( agent = "lucina", scripts = ["game_specials2hi", "game_specialairs2hi"], category = ACMD_GAME, low_priority)]
unsafe fn specials2hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
            MotionModule::set_rate(fighter.module_accessor, 0.9);
        }
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.85, 90, 30, 0, 30, 4.5, 0.0, 6.0, 12.5, Some(0.0), Some(16.0), Some(12.5), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.85, 361, 30, 0, 30, 3.0, 0.0, 6.0, 16.0, Some(0.0), Some(18.0), Some(16.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.85, 90, 30, 0, 30, 4.5, 0.0, 9.0, 8.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
            MotionModule::set_rate(fighter.module_accessor, 1.1);
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        }
}

//s2lw is s2s, there is no s2lw actually. don't know why the script is lw
#[acmd_script( agent = "lucina", scripts = ["game_specials2lw", "game_specialairs2lw"], category = ACMD_GAME, low_priority)]
unsafe fn specials2lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
            MotionModule::set_rate(fighter.module_accessor, 0.9);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.85, 361, 25, 0, 30, 6.5, 0.0, 9.0, 13.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.85, 361, 25, 0, 30, 6.6, 0.0, 9.0, 18.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.85, 361, 25, 0, 30, 4.5, 0.0, 9.0, 8.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 5.0, false);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
            MotionModule::set_rate(fighter.module_accessor, 1.1);
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        }
}

#[acmd_script( agent = "lucina", scripts = ["game_specials3s", "game_specialairs3s"], category = ACMD_GAME, low_priority)]
unsafe fn specials3s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
            MotionModule::set_rate(fighter.module_accessor, 0.9);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.325, 50, 30, 0, 50, 4.5, 0.0, 6.0, 9.0, Some(0.0), Some(15.0), Some(9.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.325, 65, 30, 0, 50, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(16.0), Some(12.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.325, 35, 30, 0, 50, 4.5, 0.0, 9.0, 3.5, Some(0.0), Some(11.0), Some(3.5), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 5.0, false);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
            MotionModule::set_rate(fighter.module_accessor, 1.1);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        }
}

#[acmd_script( agent = "lucina", scripts = ["game_specials3hi", "game_specialairs3hi"], category = ACMD_GAME, low_priority)]
unsafe fn specials3hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
            MotionModule::set_rate(fighter.module_accessor, 0.9);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.325, 50, 30, 0, 55, 6.5, 0.0, 9.0, 12.5, Some(0.0), Some(11.0), Some(12.5), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.325, 55, 30, 0, 55, 5.5, 0.0, 9.0, 16.5, Some(0.0), Some(11.5), Some(16.5), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.325, 40, 30, 0, 55, 5.5, 0.0, 9.0, 8.9, Some(0.0), Some(8.0), Some(8.9), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 5.0, false);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
            MotionModule::set_rate(fighter.module_accessor, 1.1);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        }
}

#[acmd_script( agent = "lucina", scripts = ["game_specials3lw", "game_specialairs3lw"], category = ACMD_GAME, low_priority)]
unsafe fn specials3lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
            MotionModule::set_rate(fighter.module_accessor, 0.9);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("sword1"), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Z), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Z), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Z));
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.325, 52, 30, 0, 40, 4.8, 0.0, 5.6, 14.0, Some(0.0), Some(7.0), Some(10.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.325, 52, 30, 0, 40, 3.0, 0.0, 4.0, 19.0, Some(0.0), Some(7.0), Some(10.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 5.0, false);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
            MotionModule::set_rate(fighter.module_accessor, 1.1);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        }
}

#[acmd_script( agent = "lucina", script = "game_specials4hi", category = ACMD_GAME, low_priority)]
unsafe fn specials4hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.7, 79, 40, 0, 85, 4.5, 0.0, 6.0, 10.5, Some(0.0), Some(21.0), Some(10.5), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.7, 87, 40, 0, 80, 3.0, 0.0, 6.0, 15.0, Some(0.0), Some(21.0), Some(15.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.7, 79, 40, 0, 85, 4.0, 0.0, 21.0, 11.0, Some(0.0), Some(24.0), Some(4.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 5.7, 87, 40, 0, 80, 3.0, 0.0, 23.0, 14.0, Some(0.0), Some(27.0), Some(4.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.45);
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script( agent = "lucina", script = "game_specialairs4hi", category = ACMD_GAME, low_priority)]
unsafe fn specialairs4hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.7, 79, 40, 0, 85, 4.5, 0.0, 6.0, 10.5, Some(0.0), Some(21.0), Some(10.5), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.7, 87, 40, 0, 80, 3.0, 0.0, 6.0, 15.0, Some(0.0), Some(21.0), Some(15.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.7, 79, 40, 0, 85, 4.0, 0.0, 21.0, 11.0, Some(0.0), Some(24.0), Some(4.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 5.7, 87, 40, 0, 80, 3.0, 0.0, 23.0, 14.0, Some(0.0), Some(27.0), Some(4.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.45);
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script( agent = "lucina", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {   
            MotionModule::set_rate(fighter.module_accessor, 3.0);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {  
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
}

#[acmd_script( agent = "lucina", script = "game_specialhi2", category = ACMD_GAME, low_priority)]
unsafe fn specialhi2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.25);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 91, 100, 130, 0, 4.0, 0.0, 5.0, 14.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 89, 100, 130, 0, 5.0, 0.0, 5.0, 8.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 90, 100, 115, 0, 5.0, 0.0, 13.0, 8.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 6.0, 91, 100, 115, 0, 4.0, 0.0, 13.0, 14.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -5.0, 0);
            }
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 38.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.25);
            let speed = smash::phx::Vector3f { x: 0.0, y: 4.75, z: 0.0 };
            KineticModule::add_speed(fighter.module_accessor, &speed);
            KineticModule::resume_energy_all(fighter.module_accessor);
        }
    frame(lua_state, 39.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.5);
        }
    frame(lua_state, 44.);
        if macros::is_excute(fighter)
        {
            let speed = smash::phx::Vector3f { x: 1.0, y: 0.66, z: 1.0 };
            KineticModule::mul_speed(fighter.module_accessor, &speed,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    frame(lua_state, 46.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            let speed = smash::phx::Vector3f { x: 1.0, y: 0.0, z: 1.0 };
            KineticModule::mul_speed(fighter.module_accessor, &speed,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            JostleModule::set_status(fighter.module_accessor, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 366, 100, 60, 0, 8.5, 0.0, 17.0, 14.0, Some(0.0), Some(8.0), Some(14.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 335, 100, 120, 0, 8.0, 0.0, 20.0, 6.0, Some(0.0), Some(12.0), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
        }
    frame(lua_state, 61.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(fighter.module_accessor, true);
            AttackModule::clear_all(fighter.module_accessor);
        }
    // frame(lua_state, 40)
    //     if macros::is_excute(fighter)
    //     {
    //         //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR)
    //         //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE)
    //         //KineticModule::change_kinetic(*FIGHTER_KINETIC_TYPE_FLY);
    //     }
}

#[acmd_script( agent = "lucina", script = "game_specialhi3", category = ACMD_GAME, low_priority)]
unsafe fn specialhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 63.);
        if macros::is_excute(fighter)
        {
            smash_script::camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, -20);
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, 0, -5.33);
            ADD_SPEED_NO_LIMIT(fighter.lua_state_agent);
            macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 5.0, 361, 100, 80, 0, 8.5, 0.0, 14.0, 12.0, Some(0.0), Some(7.0), Some(12.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 5.0, 270, 100, 125, 0, 8.5, 0.0, 14.0, 12.0, Some(0.0), Some(7.0), Some(12.0), 0.7, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, -12.0, false);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
}

#[acmd_script( agent = "lucina", script = "game_landingaether", category = ACMD_GAME, low_priority)]
unsafe fn landspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 60, 115, 0, 70, 9.0, 0.0, 6.0, 11.8, Some(0.0), Some(11.0), Some(11.8), 1.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
}

#[acmd_script( agent = "lucina", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority)]
unsafe fn speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SHIELD);
            MotionModule::set_rate(fighter.module_accessor, 1.33);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SHIELD);
            MotionModule::set_rate(fighter.module_accessor, 1.15);
        }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        specials1,
        specialairs1,
        specials2hi,
        specials2lw,
        specials3hi,
        specials3s,
        specials3lw,
        specialairs4hi,
        specials4hi,
        specialhi,
        specialhi2,
        specialhi3,
        landspecial,
        speciallw,
    );
}