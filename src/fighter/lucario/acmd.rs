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

    frame(lua_state, 1.0);
    if macros::is_excute(fighter)
    {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    frame(lua_state, 4.0);
    if macros::is_excute(fighter)
    {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(lua_state, 6.0);
    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 2.5, 361, 15, 0, 35, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 2.5, 361, 15, 0, 25, 2.7, -1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 2.5, 180, 15, 0, 15, 2.7, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("arml"), 2.5, 361, 15, 0, 15, 2.7, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 6.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 6.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 6.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 6.0, false);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter)
    {
        macros::FT_MOTION_RATE(fighter, 0.75);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(lua_state, 11.0);
    if macros::is_excute(fighter)
    {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 29.0);
    if macros::is_excute(fighter)
    {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}

unsafe extern "C" fn attack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.0);
    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 15, 0, 25, 4.0, 0.0, 8.8, 8.0, Some(0.0), Some(8.8), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 6.0, false);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter)
    {
        macros::FT_MOTION_RATE(fighter, 0.75);
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 35.0);
    if macros::is_excute(fighter)
    {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}

unsafe extern "C" fn attack13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));

    frame(lua_state, 9.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 65, 125, 0, 50, 5.5, 0.0, 8.0, 4.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("footr"), 3.5, 65, 125, 0, 50, 3.5, 3.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.0);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.89);
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 30.0);
        if macros::is_excute(fighter)
        {
            FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, 0.0);
        }
    frame(lua_state, 35.0);
        if macros::is_excute(fighter)
        {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                VarModule::on_flag(fighter.module_accessor, commons::instance::flag::HIT_CANCEL);
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
}

unsafe extern "C" fn attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.0);
    if macros::is_excute(fighter)
    {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 19.0);
    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 361, 106, 0, 40, 5.7, 0.0, 8.0, 11.7, Some(0.0), Some(8.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.5, 361, 106, 0, 40, 3.3, 0.0, 8.0, 18.2, Some(0.0), Some(8.0), Some(6.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter)
    {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//air
unsafe extern "C" fn attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(fighter.module_accessor, false);
        }
    frame(lua_state, 5.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 10.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 6.0, 361, 20, 0, 40, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 6.0, 65, 20, 0, 40, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 6.0, 361, 20, 0, 40, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("armr"), 6.0, 65, 20, 0, 40, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 5.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 1.5);
        }
    frame(lua_state, 18.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 8.0, 361, 100, 0, 20, 6.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 361, 100, 0, 20, 6.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 7.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            JostleModule::set_status(fighter.module_accessor, true);
        }
    frame(lua_state, 38.0);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

unsafe extern "C" fn attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.09);
        }
    frame(lua_state, 4.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 12.0);
        if macros::is_excute(fighter)
        {   
            macros::FT_MOTION_RATE(fighter, 1.);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 100, 0, 30, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 361, 100, 0, 30, 3.0, 0.0, 11.0, -9.0, Some(0.0), Some(11.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 40.0);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

unsafe extern "C" fn attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    if !VarModule::is_flag(fighter.module_accessor, lucario::instance::flag::ATTACK_AIR_LW_CHARGED) 
    {
            if macros::is_excute(fighter)
            {
                FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 3.0, 3.0, 8.0, 2.0);
            }
        frame(lua_state, 2.0);
            if macros::is_excute(fighter)
            {
                macros::SET_SPEED_EX(fighter, 0, WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_ATTACK_AIR_LW_SPEED), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_ATTACK_AIR_STATUS_WORK_ID_FLAG_DEC_SPEED);
                KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        frame(lua_state, 4.0);
            if macros::is_excute(fighter)
            {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 100, 30, 0, 20, 3.8, 0.0, -2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 367, 100, 50, 0, 4.6, 0.0, 2.2, 0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 150, 100, 25, 0, 4.6, 0.0, 2.2, 0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
            }
        wait(lua_state, 2.0);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
            }
        frame(lua_state, 11.0);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 100, 0, 60, 5.8, 0.0, -2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 361, 100, 0, 60, 4.6, 0.0, 2.2, 0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
            }
        wait(lua_state, 2.0);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
            }
        frame(lua_state, 25.0);
            if macros::is_excute(fighter)
            {
                KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
    }
    else {
            if macros::is_excute(fighter)
            {
                FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 3.0, 3.0, 8.0, 2.0);
            }
        frame(lua_state, 2.0);
            if macros::is_excute(fighter)
            {
                //macros::SET_SPEED_EX(fighter, WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_ATTACK_AIR_LW_SPEED), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_ATTACK_AIR_STATUS_WORK_ID_FLAG_DEC_SPEED);
                //KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        frame(lua_state, 4.0);
            if macros::is_excute(fighter)
            {
                //WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 367, 30, 0, 20, 3.8, 0.0, -2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 367, 100, 50, 0, 4.6, 0.0, 2.2, 0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 367, 100, 25, 0, 4.6, 0.0, 2.2, 0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 5.0, false);
            }
        wait(lua_state, 2.0);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                MotionModule::set_rate(fighter.module_accessor, 0.4);
            }
        frame(lua_state, 11.0);
            if macros::is_excute(fighter)
            {
                MotionModule::set_rate(fighter.module_accessor, 1.0);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 270, 60, 0, 50, 5.8, 0.0, -2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 270, 60, 0, 50, 4.6, 0.0, 2.2, 0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_KICK);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
            }
        wait(lua_state, 2.0);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
            }
        frame(lua_state, 25.0);
            if macros::is_excute(fighter)
            {
                KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
                VarModule::off_flag(fighter.module_accessor, lucario::instance::flag::ATTACK_AIR_LW_CHARGED);
            }
    }
}

//throw / grabs
unsafe extern "C" fn throwb(agent: &mut L2CAgentBase) {

        if macros::is_excute(agent)
        {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_FORCE_AURAPOWER_ATTACK_POWER_MUL);
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 20, 25, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent)
        {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
            macros::CHECK_FINISH_CAMERA(agent, 5, 0);
            FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
            FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 2.0, y: -4.0, z: 0.0});
        }
    frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent)
        {
            macros::REVERSE_LR(agent);
            let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
            macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 25.0, false); //probably doesnt work lol
        }
}

//other
unsafe extern "C" fn auraburst(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.4);
        }
    frame(lua_state, 83.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 20, 0, 80, 12.5, 0.0, 12.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_AURA);
        }
    wait(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    wait(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
}

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
    agent.game_acmd("game_attack12", attack12,);
    agent.game_acmd("game_attack13", attack13,);
    agent.game_acmd("game_attacks4", attacks4,);
    agent.game_acmd("game_attackairn", attackairn,);
    agent.game_acmd("game_attackairb", attackairb,);
    agent.game_acmd("game_attackairlw", attackairlw,);
    agent.game_acmd("game_throwb", throwb,);
    agent.game_acmd("game_appealhil", auraburst,);
    agent.game_acmd("game_appealhir", auraburst,);
    agent.game_acmd("game_escapeairslide", escapeairslide,);
}