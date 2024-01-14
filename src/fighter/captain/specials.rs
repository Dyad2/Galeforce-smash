use super::*;

//specials
unsafe extern "C" fn captain_specialsend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.5);
            JostleModule::set_status(fighter.module_accessor, false);
            smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
        }
    frame(lua_state, 6.0);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.8);
            macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 10.0, 85, 30, 0, 82, 6.0, 0.0, -2.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 85, 30, 0, 82, 5.5, 0.0, 9.0, 5.5, Some(0.0), Some(9.0), Some(12.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
        }
    wait(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear(fighter.module_accessor, 1, false);
            smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 10.0, 85, 30, 0, 82, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        }
    wait(lua_state, 2.0);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.75);
            AttackModule::clear_all(fighter.module_accessor);
            JostleModule::set_status(fighter.module_accessor, true);
        }
}

unsafe extern "C" fn captain_specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.75);
        }
    frame(lua_state, 6.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
        }
    frame(lua_state, 53.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.66);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.0, 367, 100, 80, 0, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 5.0, 367, 100, 80, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 5.0, 367, 100, 80, 0, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == FIGHTER_KIND_KIRBY {
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 4.0, 367, 100, 80, 0, 4.5, -3.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 4.0, 367, 100, 80, 0, 2.5, 1.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 4.0, 367, 80, 80,  0, 4.2, 5.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            }
        }
    wait(lua_state, 7.0);
        if macros::is_excute(fighter)
        {
            //VarModule::on_flag(fighter.module_accessor, captain::status::flag::ALLOW_SPECIAL_N_CRITICAL);
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 1, Hash40::new("armr"), 8.0, 361, 125, 0, 50, 4.5, -1.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 1, Hash40::new("shoulderr"), 8.0, 361, 125, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 1, Hash40::new("armr"), 8.0, 361, 125, 0, 50, 4.0, 4.2, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 3.0);
        if macros::is_excute(fighter)
        {
            //MotionModule::set_rate(fighter.module_accessor, 1.1);
            AttackModule::clear_all(fighter.module_accessor);
        }
}

unsafe extern "C" fn captain_specialairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            macros::SET_SPEED_EX(fighter, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(fighter.module_accessor), 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            MotionModule::set_rate(fighter.module_accessor, 1.75);
        }
    frame(lua_state, 6.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
            KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    frame(lua_state, 53.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.66);
            macros::SET_SPEED_EX(fighter, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(fighter.module_accessor), 1.66, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.0, 367, 100, 80, 0, 4.5, -1.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 5.0, 367, 100, 80, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 5.0, 367, 100, 80, 0, 4.0, 4.2, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == FIGHTER_KIND_KIRBY {
                macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 4.0, 367, 100, 80, 0, 2.5, -3.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 4.0, 367, 100, 80, 0, 5.0, 1.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 4.0, 367, 100, 80, 0, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.33, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            }
        }
    wait(lua_state, 7.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 1, Hash40::new("armr"), 8.0, 361, 125, 0, 50, 4.5, -1.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 1, Hash40::new("shoulderr"), 8.0, 361, 125, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 1, Hash40::new("armr"), 8.0, 361, 125, 0, 50, 4.0, 4.2, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 3.0);
        if macros::is_excute(fighter)
        {
            //MotionModule::set_rate(fighter.module_accessor, 1.1);
            AttackModule::clear_all(fighter.module_accessor);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialsend", captain_specialsend,);
    agent.game_acmd("game_specialn", captain_specialn);
    agent.game_acmd("game_specialairn", captain_specialairn);
}