use super::*;
use crate::fighter::common::opff::common_fighter_frame;

// #[fighter_frame( agent = FIGHTER_KIND_JACK )]
// fn thief_frame(fighter: &mut L2CFighterCommon) {
//     unsafe {
//         let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
//       if curr_motion_kind == hash40("special_air_n_shoot") {
//           if (MotionModule::frame(fighter.module_accessor) > 9. && MotionModule::frame(fighter.module_accessor) < 12.)
//               || (MotionModule::frame(fighter.module_accessor) > 21. && MotionModule::frame(fighter.module_accessor) < 24.)
//               || (MotionModule::frame(fighter.module_accessor) > 33. && MotionModule::frame(fighter.module_accessor) < 36.)
//               || (MotionModule::frame(fighter.module_accessor) > 21. && MotionModule::frame(fighter.module_accessor) < 24.) {
//                   macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
//                   macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
//                   macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
//                   if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
//                       macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
//                       macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
//                       macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
//                   }
//           }
//           if (MotionModule::frame(fighter.module_accessor) > 10. && MotionModule::frame(fighter.module_accessor) < 13.)
//               || (MotionModule::frame(fighter.module_accessor) > 22. && MotionModule::frame(fighter.module_accessor) < 25.)
//               || (MotionModule::frame(fighter.module_accessor) > 34. && MotionModule::frame(fighter.module_accessor) < 37.)
//               || (MotionModule::frame(fighter.module_accessor) > 22. && MotionModule::frame(fighter.module_accessor) < 25.) {
//                   AttackModule::clear_all(fighter.module_accessor);
//           }
//       }
//      WorkModule::off_flag(fighter.module_accessor, FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ALLOW_SUMMON);
//    }
//}

unsafe extern "C" fn jack_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, jack_frame);
}