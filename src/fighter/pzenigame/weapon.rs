use super::*;

//weapon
unsafe extern "C" fn waterregular(weapon: &mut L2CAgentBase) {

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 1.6, 45, 100, 10, 0, 2.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            AttackModule::enable_safe_pos(weapon.module_accessor);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    let bottlewater = &mut smashline::Agent::new("pzenigame_water");

    bottlewater.game_acmd("game_regular", waterregular,);
}