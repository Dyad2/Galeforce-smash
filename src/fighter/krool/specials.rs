use super::*;

unsafe extern "C" fn specialhi(agent: &mut L2CAgentBase) {
        if macros::is_excute(agent)
        {
            MotionModule::set_rate(agent.module_accessor, 1.2);
            FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 6.5, 6.5);
        }
    frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent)
        {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 160, 50, 80, 0, 2.5, 0.0, 7.0, 0.0, Some(0.0), Some(5.0), Some(0.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 165, 50, 120, 0, 2.5, 0.0, 10.5, -6.0, Some(0.0), Some(3.5), Some(-7.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 165, 50, 120, 0, 2.5, 0.0, 10.5, 6.0, Some(0.0), Some(3.5), Some(7.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.0, 240, 50, 50, 0, 1.5, 0.0, 13.5, 3.0, Some(0.0), Some(13.5), Some(-3.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
    frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent)
        {
            KineticModule::set_consider_ground_friction(agent.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent)
        {
            KineticModule::set_consider_ground_friction(agent.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    frame(agent.lua_state_agent, 39.0);
        if macros::is_excute(agent)
        {
            AttackModule::clear_all(agent.module_accessor);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 55, 140, 0, 50, 8.0, 0.0, 7.6, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 55, 140, 0, 50, 8.0, 0.0, 7.6, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
    frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent)
        {
            AttackModule::clear_all(agent.module_accessor);
            KineticModule::set_consider_ground_friction(agent.module_accessor, true, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1);
        }
    frame(agent.lua_state_agent, 51.0);
        if macros::is_excute(agent)
        {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
            FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 8.0, 8.0);
        }
}

unsafe extern "C" fn specialairhi(agent: &mut L2CAgentBase) {
        if macros::is_excute(agent)
        {
            MotionModule::set_rate(agent.module_accessor, 1.2);
        }
    frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent)
        {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 367, 0, 0, 50, 7.0, 0.0, 11.0, 3.5, Some(0.0), Some(11.0), Some(-3.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent)
        {
            AttackModule::clear_all(agent.module_accessor);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    for _ in 0..10
    {
            if macros::is_excute(agent)
            {
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 20, 0, 50, 4.3, 0.0, 11.0, 3.5, Some(0.0), Some(11.0), Some(-3.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            }
        wait(agent.lua_state_agent, 4.0);
            if macros::is_excute(agent)
            {
                AttackModule::clear_all(agent.module_accessor);
            }
    }
        if macros::is_excute(agent)
        {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 60, 180, 0, 50, 5.0, 0.0, 11.0, 3.5, Some(0.0), Some(11.0), Some(-3.5), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
    wait(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent)
        {
            AttackModule::clear_all(agent.module_accessor);
        }
    frame(agent.lua_state_agent, 51.0);
        if macros::is_excute(agent)
        {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG4);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1);
        }
}

unsafe extern "C" fn speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_LW_FLAG_SHIELD);
            macros::FT_MOTION_RATE(fighter, 0.85);
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_LW_FLAG_SHIELD);
            macros::FT_MOTION_RATE(fighter, 1.0);
        }
    frame(lua_state, 43.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.7);
        }
    frame(lua_state, 68.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.5);
        }
    frame(lua_state, 82.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialhi", specialhi,);
    agent.game_acmd("game_specialairhi", specialairhi,);
    agent.game_acmd("game_specialairlw", speciallw,);
    agent.game_acmd("game_speciallw", speciallw,);
}