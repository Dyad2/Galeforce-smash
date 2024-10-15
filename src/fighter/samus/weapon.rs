use super::*;

//weapon
unsafe extern "C" fn cshotshoot(weapon: &mut L2CAgentBase) {

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0, 361, 42, 0, 14, 1.9, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 28.0, 361, 42, 0, 14, 8.0, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
            smash_script::attack!(weapon, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
        }
}

unsafe extern "C" fn missilehoming(weapon: &mut L2CAgentBase) {

    if macros::is_excute(weapon)
    {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 6.0, 0, 25, 0, 26, 2.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(weapon.module_accessor);
    }
}

pub fn install() {
    let cshot = &mut smashline::Agent::new("samus_cshot");
    let missile = &mut smashline::Agent::new("samus_missile");

    cshot.game_acmd("game_shoot", cshotshoot, Priority::Low);
    missile.game_acmd("game_homing", missilehoming, Priority::Low);
}