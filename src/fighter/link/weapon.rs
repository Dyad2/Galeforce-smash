use super::*;

unsafe extern "C" fn fly(weapon: &mut L2CAgentBase) {
    //forward smash
    if macros::is_excute(weapon)
    {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 3.0, 361, 1, 1, 0, 1.8, 0.0, 0.5, -7.0, Some(0.0), Some(0.5), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
}

pub fn install() {
    let swordbeam = &mut smashline::Agent::new("link_swordbeam");

    swordbeam.game_acmd("game_fly", fly,);
}