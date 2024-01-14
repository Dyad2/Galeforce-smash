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
unsafe extern "C" fn attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.5);
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.8, 361, 35, 0, 20, 2.5, 0.0, 9.3, 6.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.8, 361, 33, 0, 20, 2.7, 0.0, 9.3, 8.5, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.8, 180, 20, 0, 15, 2.7, 0.0, 9.3, 12.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.8, 361, 20, 0, 15, 2.7, 0.0, 9.3, 12.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
        }
}

unsafe extern "C" fn attackhi3(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter)
    {
        MotionModule::set_rate(fighter.module_accessor, 0.5);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter)
    {
        MotionModule::set_rate(fighter.module_accessor, 1.33);
        macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 6.0, 100, 125, 0, 20, 3.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 100, 125, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("kneer"), 8.0, 100, 110, 0, 20, 6.0, 3.9, -0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter)
    {
        MotionModule::set_rate(fighter.module_accessor, 1.1);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 361, 80, 0, 40, 3.4, 4.6, -0.7, 0.0, Some(-3.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

unsafe extern "C" fn attacks3lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 5.0, 361, 20, 0, 50, 3.4, 4.6, -0.7, 0.0, Some(-3.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

//air
unsafe extern "C" fn attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 1.8, 367, 100, 120, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 1.8, 367, 100, 120, 0, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 1.8, 75, 110, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("kneer"), 1.8, 75, 110, 0, 50, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 11.);
        if macros::is_excute(fighter)
        {
            let vertspeed = smash::phx::Vector3f {x: 0.0, y: 0.5, z: 0.0}; // y 0.3
            KineticModule::add_speed(fighter.module_accessor, &vertspeed);
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 1.3, 367, 100, 120, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 1.3, 367, 100, 120, 0, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 1.3, 75, 130, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("kneel"), 1.3, 75, 130, 0, 50, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            let vertspeed = smash::phx::Vector3f {x: 0.0, y: 1.4, z: 0.0}; // y 1.3
            KineticModule::add_speed(fighter.module_accessor, &vertspeed);
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 1.8, 367, 100, 120, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 1.8, 367, 100, 120, 0, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 1.8, 75, 130, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("kneer"), 1.8, 75, 130, 0, 50, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            let vertspeed = smash::phx::Vector3f {x: 0.0, y: 1.15, z: 0.0}; // y 0.9
            KineticModule::add_speed(fighter.module_accessor, &vertspeed);
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 2.8, 367, 100, 120, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 2.8, 367, 100, 120, 0, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 2.8, 75, 130, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 3, 0, Hash40::new("kneel"), 2.8, 75, 130, 0, 50, 3.5, 5.1, -0.8, 1.2, None, None, None, 0.66,  1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 26.);
        if macros::is_excute(fighter)
        {
            let vertspeed = smash::phx::Vector3f {x: 0.0, y: 0.75, z: 0.0}; // y 0.5
            KineticModule::add_speed(fighter.module_accessor, &vertspeed);
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 4.8, 70, 103, 0, 60, 7.0, 0.0, 0.0, 0.0, None, None, None, 2.0,  1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 4.8, 70, 103, 0, 60, 6.5, 5.1, -0.8, 1.2, None, None, None, 2.0,  1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 4.8, 70, 103, 0, 60, 5.5, 0.0, 0.0, 0.0, None, None, None, 2.0,  1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F,  false, 0,  0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_STATUS_ATTACK_AIR_FLAG_LANDING_DISABLE_ATTACK);
        }
    wait(lua_state, 1.);
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

unsafe extern "C" fn attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 9.0, 361, 100, 0, 10, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 9.0, 361, 100, 0, 10, 4.0, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 6.0, 361, 100, 0, 10, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 6.0, 361, 100, 0, 10, 3.5, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 2.5)
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.8);
        }
    wait(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

unsafe extern "C" fn attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 10.0, 30, 109, 0, 20, 4.6, 5.9, -0.9, 1.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.0, 30, 109, 0, 20, 4.6, 0.9, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 361, 88, 0, 10, 4.6, 5.9, -0.9, 1.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 361, 88, 0, 10, 4.6, 0.9, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

unsafe extern "C" fn attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    wait(lua_state, 1.);
        for _ in 0..5 {
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.4, 325, 100, 20, 0, 4.5, 0.0, 8.2, -0.5, Some(0.0), Some(9.0), Some(2.0), 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                    macros::ATTACK(fighter, 1, 0, Hash40::new("toel"), 1.4, 110, 30, 0, 30, 4.5, -0.5, -0.5, 0.0, Some(-0.5), Some(-0.5), Some(0.0), 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                    macros::ATTACK(fighter, 2, 0, Hash40::new("toel"), 1.4, 367, 100, 60, 0, 6.0, 0.5, -4.0, 0.0, None, None, None, 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                }
            wait(lua_state, 2.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(fighter.module_accessor);
                }
            wait(lua_state, 1.);
        }
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 140, 0, 50, 6.0, 0.0, 9.0, -1.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 60, 140, 0, 50, 7.0, 0.0, 2.8, 3.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
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
    agent.game_acmd("game_attack11", attack11,);
    agent.game_acmd("game_attackhi3", attackhi3,);
    agent.game_acmd("game_attacks3", attacks3,);
    agent.game_acmd("game_attacks3lw", attacks3lw,);
    agent.game_acmd("game_attackairf", attackairf,);
    agent.game_acmd("game_attackairn", attackairn,);
    agent.game_acmd("game_attackairb", attackairb,);
    agent.game_acmd("game_attackairlw", attackairlw,);
    agent.game_acmd("game_escapeairslide", escapeairslide,);
}