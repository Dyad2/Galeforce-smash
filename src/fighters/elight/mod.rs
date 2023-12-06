use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut smashline::Agent::new("elight");
    acmd::install(agent);
    opff::install(agent);
}

// #[acmd_func( 
//     battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, battle_object_kind = FIGHTER_KIND_ELIGHT, animation = "special_hi", animcmd = "game_specialhi")]
// pub fn attacklw3(fighter: &mut L2CFighterCommon) {
//     acmd!({
//         frame(lua_state, 1)
//             macros::FT_MOTION_RATE(fighter, 0.5)
//             IS_EXIST_ARTICLE(17456)
//             if (WorkModule::on_flag(fighter.module_accessor, 0x353770)
//             {
//                 if macros::is_excute(fighter)
//                 {
//                     ArticleModule::add_motion_partial(17456, 60344, 0x07439e926b as u64, 5., 5., false, false, 0., false, true, false)
//                 }
//             }
//             MotionModule::is_changing()
//             if(0x353770(false, true)){
//             if macros::is_excute(fighter){
//             WorkModule::on_flag(fighter.module_accessor, *60348)
//             }
//             }
//         frame(5)
//             macros::FT_MOTION_RATE(fighter, 1)
//         frame(lua_state, 7)
//             if macros::is_excute(fighter){
//             macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 87, 55, 0, 60, 3.0, 0.0, 2.0, 8.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD)
//             macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 93, 60, 0, 67, 2.5, 0.0, 2.0, 13.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD)
//             macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 93, 60, 0, 67, 2.0, 0.0, 2.0, 16.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD)
//             macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 87, 55, 0, 60, 3.0, 0.0, 2.0, 4.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD)
//             }
//         frame(lua_state, 10)
//             if macros::is_excute(fighter){
//             AttackModule::clear_all(fighter.module_accessor);
//             }
//         frame(lua_state, 13)
//             if(WorkModule::on_flag(fighter.module_accessor, 0x353770)
//             {
//                 if macros::is_excute(fighter)
//                 {
//                     ArticleModule::add_motion_partial(17456, 60344, 0x08183db0f4 as u64, 5., 5., false, false, 0., false, true, false)
//                 }
//             }
//             MotionModule::is_changing()
//             if(0x353770(false, true)){
//             if macros::is_excute(fighter){
//             WorkModule::on_flag(fighter.module_accessor, *60348)
//             }
//             macros::FT_MOTION_RATE(fighter, 0.5)
//             frame(lua_state, 24)
//             macros::FT_MOTION_RATE(fighter, 1)
//     });
// }