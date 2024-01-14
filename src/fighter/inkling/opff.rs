use super::*;
use crate::fighter::common::opff::common_fighter_frame;

// #[fighter_frame( agent = FIGHTER_KIND_INKLING )]
// fn woomy_frame(fighter: &mut L2CFighterCommon) {
//     unsafe {
//         let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
//         let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
//         if curr_motion_kind == hash40("attack_air_b") && (MotionModule::frame(fighter.module_accessor) >= 12. && MotionModule::frame(fighter.module_accessor) <= 13.)
//         || curr_motion_kind == hash40("attack_air_lw") && (MotionModule::frame(fighter.module_accessor) >= 18. && MotionModule::frame(fighter.module_accessor) <= 19.) {
//             if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
//                 FIGHTER_GLOBALS[entry_id as usize].ink_arts = true;
//             }
//             else {
//                 FIGHTER_GLOBALS[entry_id as usize].ink_arts = false;
//             }        
//             if FIGHTER_GLOBALS[entry_id as usize].ink_arts {
//                 FIGHTER_GLOBALS[entry_id as usize].ink_arts_delay += 1;
//                 if FIGHTER_GLOBALS[entry_id as usize].ink_arts_delay % 5 == 0 {
//                     FIGHTER_GLOBALS[entry_id as usize].ink_arts_delay = 0;
//                     ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_INKBULLET, false, 0);
//                 }
//                 MotionModule::set_rate(fighter.module_accessor, 0.0);
//                 if curr_motion_kind == hash40("attack_air_b") {
//                     macros::EFFECT_FOLLOW(fighter, Hash40::new("inkling_splashooter_muzzle"), Hash40::new("muzzle"), -1, 0, 0, 0, 180, 0, 0.8, true);
//                     macros::LAST_PARTICLE_SET_COLOR(fighter, WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R), WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G), WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B));
//                     macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.1, 361, 10, 0, 12, 5.0, 0.0, 3.0, -16.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
//                     AttackModule::set_ink_value(fighter.module_accessor, 0, 10.);
//                 }
//                 if curr_motion_kind == hash40("attack_air_lw") {
//                     macros::EFFECT_FOLLOW(fighter, Hash40::new("inkling_splashooter_muzzle"), Hash40::new("muzzle"), -1, 0, 0, 0, 180, 0, 0.8, true);
//                     macros::LAST_PARTICLE_SET_COLOR(fighter, WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R), WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G), WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B));
//                     macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.1, 361, 10, 0, 12, 5.0, 0.0, -4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
//                     AttackModule::set_ink_value(fighter.module_accessor, 0, 10.);
//                 }
//             }
//             else {
//                 MotionModule::set_rate(fighter.module_accessor, 1.0);
//                 AttackModule::clear_all(fighter.module_accessor);
//                 EffectModule::kill_kind(fighter.module_accessor, Hash40::new("inkling_splashooter_muzzle"), false, false);
//             }
//         }
//     }
// }

unsafe extern "C" fn ink_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, ink_frame);
}