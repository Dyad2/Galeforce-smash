use super::*;

//weapons
unsafe extern "C" fn deinmove(weapon: &mut L2CAgentBase) {
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

unsafe extern "C" fn phantommax(weapon: &mut L2CAgentBase) {
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

pub fn install() {
    let deins = &mut smashline::Agent::new("zelda_dein_s");
    let phantom = &mut smashline::Agent::new("zelda_phantom");

    deins.game_acmd("game_move", deinmove, Priority::Low);
    phantom.game_acmd("game_attackmax", phantommax, Priority::Low);
}