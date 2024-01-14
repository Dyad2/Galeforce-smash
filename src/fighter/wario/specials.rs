use super::*;

//specials
unsafe extern "C" fn specialhijump(agent: &mut L2CAgentBase) {

    frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent)
        {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 127, 100, 110, 0, 3.0, 0.0, 5.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 127, 110, 110, 0, 3.0, 0.0, 5.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 127, 100, 90, 0, 3.0, 0.0, 11.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 127, 100, 90, 0, 3.0, 0.0, 11.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent)
        {
            AttackModule::clear_all(agent.module_accessor);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent)
        {
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 115, 88, 130, 0, 5.0, 0.0, 6.0, 4.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
                macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 115, 88, 130, 0, 5.0, 0.0, 6.0, -3.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
                macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 145, 88, 130, 0, 5.0, 0.0, 13.0, 4.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
                macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.0, 145, 88, 130, 0, 5.0, 0.0, 13.0, -3.0, None, None, None, 0.6, 0.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    frame(agent.lua_state_agent, 23.0);
        if macros::is_excute(agent)
        {
            AttackModule::clear_all(agent.module_accessor);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent)
        {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 45, 130, 0, 70, 5.5, 0.0, 13.0, 7.5, Some(0.0), Some(13.0), Some(-7.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 85, 130, 0, 60, 8.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    wait(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent)
        {
            AttackModule::clear_all(agent.module_accessor);
        }
    frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent)
        {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_HI_FLAG_DISABLE_MOTION_ANGLE);
        }
}

unsafe extern "C" fn speciallwflyr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    {
        macros::FT_MOTION_RATE(agent, 1.5);
    }
    // frame(agent.lua_state_agent, 3.0);
    // if macros::is_excute(agent) {
    //     damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    // }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent)
    {
        macros::FT_MOTION_RATE(agent, 1.0);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent)
    {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 27.0, 35, 75, 0, 50, 11.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_JUMP);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_CRITICAL_HIT);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent)
    {
        //damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_CRITICAL_HIT);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 80, 100, 0, 0, 7.0, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent)
    {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialhijump", specialhijump,);
    agent.game_acmd("game_speciallwflyr", speciallwflyr,);
    agent.game_acmd("game_specialairlwflyr", speciallwflyr,);
}