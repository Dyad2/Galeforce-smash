use super::*;

//weapon
unsafe extern "C" fn laserfly(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 3.0, 361, 0, 0, 0, 1.44, 0.0, 0.0, 0.8, Some(0.0), Some(0.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_blaster_throw_down"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FOX_BLASTER, *ATTACK_REGION_ENERGY);
        }
    wait(lua_state, 4.);
        if macros::is_excute(weapon)
        {
            macros::ATK_POWER(weapon, 0, 2);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    let blasterbullet = &mut smashline::Agent::new("fox_blasterbullet");

    blasterbullet.game_acmd("game_fly", laserfly,);
}