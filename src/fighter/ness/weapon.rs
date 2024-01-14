use super::*

//weapon
unsafe extern "C" fn pillar(weapon: &mut L2CAgentBase) {
    
    if macros::is_excute(weapon)
    {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.5, 60, 30, 0, 5, 3.5, 0.0, 3.0, 3.0, Some(0.0), Some(6.0), Some(3.0), 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 0.5, 60, 30, 0, 5, 2.5, 0.0, 10.5, 3.0, None, None, None, 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 0.5, 60, 30, 0, 5, 3.5, 0.0, 3.0, -3.0, Some(0.0), Some(6.0), Some(-3.0), 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::ATTACK(weapon, 3, 0, Hash40::new("top"), 0.5, 60, 30, 0, 5, 2.5, 0.0, 10.5, -3.0, None, None, None, 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::AREA_WIND_2ND_RAD_arg9(weapon, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
    }
}

// #[acmd_func( 
//     battle_object_category = BATTLE_OBJECT_CATEGORY_WEAPON, battle_object_kind = WEAPON_KIND_NESS_PK_FIRE, animation = "pillar_air", animcmd = "game_pillarair")]
// pub fn pillarair(fighter: &mut L2CFighterBase) {
//     acmd!({
//         if macros::is_excute(fighter)
//         {
//             macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 5, 30, 0, 5, 3.5, 0.0, 3.0, 3.0, Some(0.0, Some(6.0, Some(3.0, 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI)
//             macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 5, 30, 0, 5, 2.5, 0.0, 10.5, 3.0, None, None, None, 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI)
//             macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 5, 30, 0, 5, 3.5, 0.0, 3.0, -3.0, Some(0.0, Some(6.0, Some(-3.0, 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI)        
//             macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.5, 5, 30, 0, 5, 2.5, 0.0, 10.5, -3.0, None, None, None, 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI)
//             //AREA_WIND_2ND_RAD_arg9(0, 1, 0.05, 200, 0.6, 0, 10, 20, 60)
//         }
//     });
// }

pub fn install(agent: &mut smashline::Agent) {
    let pk_fire = &mut smashline::Agent::new("ness_pk_fire");

    pk_fire.game_acmd("game_pillar", pillar,);
}