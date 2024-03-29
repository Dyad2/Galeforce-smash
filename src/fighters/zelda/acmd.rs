use super::*;


//weapons
#[acmd_script( agent = "zelda_deins", script = "game_move", category = ACMD_GAME, low_priority)]
unsafe fn deinmove(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;

        if macros::is_excute( weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 2.0, 52, 97, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 1.5, 65, 60, 0, 50, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::AREA_WIND_2ND_RAD_arg9(weapon, 0, 2, 0.05, 200, 1, 0, 0, 12, 60);
        }
    frame(lua_state, 6.);
        if macros::is_excute( weapon)
        {
            AttackModule::clear_all(weapon.module_accessor);
        }
    frame(lua_state, 20.);
        if macros::is_excute( weapon)
        {
            AreaModule::erase_wind(weapon.module_accessor, 0);
        }
}

#[acmd_script( agent = "zelda_phantom", script = "game_attackmax", category = ACMD_GAME, low_priority)]
unsafe fn phantommax(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;

    frame(lua_state, 0.);
        if macros::is_excute( weapon)
        {
            MotionModule::set_rate(weapon.module_accessor, 0.8);
            macros::ATTACK( weapon, 0, 0, Hash40::new("top"), 0.0, 361, 100, 140, 0, 6.0, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            macros::ATTACK( weapon, 1, 0, Hash40::new("top"), 0.0, 6, 100, 85, 0, 8.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
    frame(lua_state, 5.);
        if macros::is_excute( weapon)
        {
            AttackModule::clear_all(weapon.module_accessor);
        }
    frame(lua_state, 6.);
        if macros::is_excute( weapon)
        {
            macros::ATTACK( weapon, 0, 0, Hash40::new("handr"), 6.0, 46, 68, 0, 60, 6.0, 2.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK( weapon, 1, 0, Hash40::new("handr"), 6.0, 46, 68, 0, 60, 6.0, 2.0, 1.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK( weapon, 2, 0, Hash40::new("handr"), 6.0, 46, 68, 0, 60, 6.0, 2.0, 2.0, 16.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
    wait(lua_state, 11.);
        if macros::is_excute( weapon)
        {
            AttackModule::clear_all(weapon.module_accessor);
            MotionModule::set_rate(weapon.module_accessor, 1.1);
        }
}

//global edits
#[acmd_script( agent = "zelda", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "zelda", script = "game_turndash", category = ACMD_GAME, low_priority)]
unsafe fn turndash(fighter: &mut L2CAgentBase) {
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

//Ground
#[acmd_script( agent = "zelda", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 35, 0, 25, 2.4, 0.0, 12.0, 4.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 361, 30, 0, 25, 2.4, 0.0, 12.0, 7.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 30, 0, 25, 2.8, 0.0, 5.5, 4.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 180, 20, 0, 25, 3.2, 0.0, 11.8, 10.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 2.5, 361, 20, 0, 25, 3.2, 0.0, 11.8, 10.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 25, 0, 40, 3.0, 0.0, 11.8, 5.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 361, 25, 0, 40, 3.0, 0.0, 11.8, 8.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 25, 0, 40, 3.2, 0.0, 5.5, 5.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 180, 20, 0, 25, 3.5, 0.0, 11.5, 11.5, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 2.5, 361, 25, 0, 40, 3.5, 0.0, 11.5, 11.5, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 2.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 2.0, false);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 65, 80, 0, 50, 5.0, 0.0, 10.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 0.7);
        }
}

#[acmd_script( agent = "zelda", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.2, 100, 110, 0, 45, 5.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 7.2, 100, 110, 0, 45, 2.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        }
    wait(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script( agent = "zelda", script = "game_attacks3hi", category = ACMD_GAME, low_priority)]
unsafe fn attacks3hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 11.5, 361, 70, 0, 72,2.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 11.5, 361, 70, 0, 72,2.5, 0.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 11.5, 361, 70, 0, 72,2.5, 3.2, 0.0, -0.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 3, 0, Hash40::new("handl"), 11.5, 361, 70, 0, 72,2.5, 3.5, 0.0, 0.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 4, 0, Hash40::new("handl"), 11.5, 361, 70, 0, 72,2.5, 6.2, 0.0, 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
            MotionModule::set_rate(fighter.module_accessor, 1.136) //motion_rate 0.88
        }
}

#[acmd_script( agent = "zelda", script = "game_attacks3", category = ACMD_GAME, low_priority)]
unsafe fn attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 11.5, 361, 70, 0, 72,2.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 11.5, 361, 70, 0, 72,2.5, 0.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 11.5, 361, 70, 0, 72,2.5, 3.2, 0.0, -0.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 3, 0, Hash40::new("handl"), 15.0, 361, 70, 0, 72,2.5, 3.5, 0.0, 0.7, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 4, 0, Hash40::new("handl"), 15.0, 361, 70, 0, 72,2.5, 6.2, 0.0, 1.4, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 5, 0, Hash40::new("kneel"), 11.5, 361, 70, 0, 72,3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 1.136); //motion_rate 0.88
        }
}

#[acmd_script( agent = "zelda", script = "game_attacks3lw", category = ACMD_GAME, low_priority)]
unsafe fn attacks3lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 11.5, 361, 70, 0, 72,2.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 11.5, 361, 70, 0, 72,2.5, 0.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 11.5, 361, 70, 0, 72,2.5, 3.2, 0.0, -0.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 3, 0, Hash40::new("handl"), 15.0, 361, 70, 0, 72,2.5, 3.5, 0.0, 0.7, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 4, 0, Hash40::new("handl"), 15.0, 361, 70, 0, 72,2.5, 6.2, 0.0, 1.4, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false,false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 1.136) //motion_rate 0.88
        }
}

#[acmd_script( agent = "zelda", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 62, 125, 0, 15, 2.9, 0.0, 2.0, 2.33, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 62, 125, 0, 15, 2.9, 0.0, 1.7, 5.66, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.5, 62, 125, 0, 15, 2.9, 0.0, 1.5, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    wait(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 1.25);
        }
}

//Air
#[acmd_script( agent = "zelda", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        frame(lua_state, 3.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        frame(lua_state, 5.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 80, 0, 5.0, 0.0, 10.0, 5.0, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 367, 100, 0, 0, 1.5, 0.0, 6.5, 5.0, Some(0.0), Some(6.5), Some(10.0), 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 0, 0, 1.5, 0.0, 13.0, 5.0, Some(0.0), Some(13.0), Some(10.0), 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 367, 100, 80, 0, 5.0, 0.0, 13.0, -5.0, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 1.5, 367, 100, 0, 0, 1.5, 0.0, 9.5, -3.0, Some(0.0), Some(9.5), Some(-8.0), 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 1.5, 367, 100, 0, 0, 1.5, 0.0, 16.0, -3.0, Some(0.0), Some(16.0), Some(-8.0), 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 4.0, false);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 4.0, false);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 4, 4.0, false);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 5, 4.0, false);
            }
        wait(lua_state, 19.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
            }
        wait(lua_state, 2.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 40, 110, 0, 35, 5.0, 0.0, 9.0, 8.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 40, 110, 0, 35, 5.0, 0.0, 7.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 2, 0, Hash40::new("head"), 5.0, 40, 110, 0, 35, 5.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 40, 110, 0, 35, 5.0, 0.0, 13.5, -7.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            }
        wait(lua_state, 2.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
            }
        frame(lua_state, 38.);
            if macros::is_excute(fighter)
            {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "zelda", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        frame(lua_state, 0.);
            if macros::is_excute(fighter)
            {
                MotionModule::set_rate(fighter.module_accessor, 1.66);
            }
        frame(lua_state, 3.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        frame(lua_state, 8.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 361, 95, 0, 30, 2.2, -1.0, 4.4, 11.6, Some(1.0), Some(4.4), Some(11.6), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 10.0, 361, 90, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 10.0, 361, 90, 0, 30, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 3, 0, Hash40::new("toel"), 10.0, 361, 90, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
            }
        wait(lua_state, 2.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear(fighter.module_accessor, 0, false);
                MotionModule::set_rate(fighter.module_accessor, 1.1)
            }
        wait(lua_state, 3.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
            }
        frame(lua_state, 45.);
            if macros::is_excute(fighter)
            {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
}

#[acmd_script( agent = "zelda", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 96, 0, 30, 2.2, -1.0, 4.4, -11.1, Some(1.0), Some(4.4), Some(-11.1), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 361, 96, 0, 20, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 7.0, 361, 96, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("toer"), 7.0, 361, 96, 0, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear(fighter.module_accessor, 0, false);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 1.01);
        }
    frame(lua_state, 48.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

//specials

#[acmd_script( agent = "zelda", scripts = ["game_specialn", "game_specialairn"], category = ACMD_GAME, low_priority)]
unsafe fn specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_START);
            HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
            //MotionModule::set_rate(fighter.module_accessor, 1.33);
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            //MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 160, 100, 50, 0, 8.5, 0.0, 7.0, -0.5, Some(0.0), Some(7.0), Some(0.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 160, 100, 50, 0, 4.0, 0.0, 8.0, -10.0, Some(0.0), Some(8.0), Some(10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        }
    frame(lua_state, 17.);
        if macros::is_excute(fighter)
        {
            HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    frame(lua_state, 27.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 100, 0, 50, 8.5, 0.0, 8.0, -3.5, Some(0.0), Some(8.0), Some(3.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 100, 0, 40, 5.0, 0.0, 8.0, -11.0, Some(0.0), Some(8.0), Some(11.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 40.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END);
        }
}

#[acmd_script( agent = "zelda", script = "game_specialairhistart", category = ACMD_GAME, low_priority)]
unsafe fn specialairhistart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_NONE);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 91, 100, 160, 0, 8.0, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 17.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
}

#[acmd_script( agent = "zelda", script = "game_specialhistart", category = ACMD_GAME, low_priority)]
unsafe fn specialhistart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 91, 100, 160, 0, 8.0, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script( agent = "zelda", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            if !VarModule::is_flag(fighter.battle_object, zelda::instance::flag::SPECIAL_HI_CANCEL) {
                JostleModule::set_status(fighter.module_accessor, true);
                macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 10.0, 361, 90, 0, 80, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 1, 0, Hash40::new("rot"), 7.0, 361, 90, 0, 60, 15.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            }
            else {
                JostleModule::set_status(fighter.module_accessor, true);
                macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 6.0, 361, 70, 0, 70, 15.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            }
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.25);
        }
}

#[acmd_script( agent = "zelda", script = "game_specialairhi", category = ACMD_GAME, low_priority)]
unsafe fn specialairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            if !VarModule::is_flag(fighter.battle_object, zelda::instance::flag::SPECIAL_HI_CANCEL) {
                JostleModule::set_status(fighter.module_accessor, true);
                smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
                macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 10.0, 361, 90, 0, 80, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
                macros::ATTACK(fighter, 1, 0, Hash40::new("rot"), 7.0, 361, 90, 0, 60, 15.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            }
            else {
                JostleModule::set_status(fighter.module_accessor, true);
                smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
                macros::ATTACK(fighter, 1, 0, Hash40::new("rot"), 6.0, 361, 70, 0, 70, 15.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            }
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_1);
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_DIVE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CONTROL);
        }
}

//Grabs / throws
#[acmd_script( agent = "zelda", script = "game_throwb", category = ACMD_GAME, low_priority)]
unsafe fn throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 45, 82, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F,  0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    frame(lua_state, 26.);
        if macros::is_excute(fighter)
        {
            macros::CHECK_FINISH_CAMERA(fighter, 11, 7);
            FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(),1.3);
            FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x:-5., y:0., z:0.});
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        }
    frame(lua_state, 27.);
        if macros::is_excute(fighter)
        {
            macros::REVERSE_LR(fighter);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        }
}

//other
#[acmd_script( agent = "zelda", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
unsafe fn escapeairslide(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        deinmove,
        phantommax,
        dash,
        turndash,
        attack11,
        attacks3,
        attacks3hi,
        attacks3lw,
        attackhi3,
        attacklw3,
        attackairn,
        attackairb,
        attackairf,
        specialn,
        specialhistart,
        specialairhistart,
        specialhi,
        specialairhi,
        throwb,
        escapeairslide
    );
}