use super::*;

//weapons
unsafe extern "C" fn boom_fly(weapon: &mut L2CAgentBase) {

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 6.0, 50, 50, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 37, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::enable_safe_pos(weapon.module_accessor);
        }
}

unsafe extern "C" fn boom_turn(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;

    wait(lua_state, 1.0);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 3.0, 50, 100, 70, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1.5, 0.0, 37, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
}

pub fn install() {
    let boomerang = &mut smashline::Agent::new("toonlink_boomerang");

    boomerang.game_acmd("game_fly", boom_fly, Priority::Low);
    boomerang.game_acmd("game_turn", boom_turn, Priority::Low);
}