use super::*;

//specials
unsafe extern "C" fn specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_ACC_X);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 60, 62, 0, 80, 10.0, 0.0, 6.5, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 57, 60, 0, 80, 9.5, 0.0, 6.5, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 53, 58, 0, 80, 9.0, 0.0, 6.5, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 50, 56, 0, 80, 8.5, 0.0, 6.5, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
    wait(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
}

unsafe extern "C" fn specialairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_ACC_X);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 62, 0, 80, 10.0, 0.0, 6.5, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        }
    wait(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 57, 60, 0, 80, 9.5, 0.0, 6.5, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 53, 58, 0, 80, 9.0, 0.0, 6.5, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 50, 56, 0, 80, 8.5, 0.0, 6.5, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        }
    wait(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialhi", specialhi,);
    agent.game_acmd("game_specialairhi", specialairhi,);
}