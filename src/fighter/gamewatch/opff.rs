use super::*;
use crate::fighter::common::opff::common_fighter_frame;

//GA: Judge storage. doesnt work
//unsafe extern "C" fn gameWatch_galeforce_attack(fighter: &mut L2CFighterCommon) {
//    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    
//    //fixme: doesnt work? the exact same code works for the belmonts
//    //if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
//    //    VarModule::on_flag(fighter.module_accessor,commons::instance::flag::DISABLE_SPECIAL_HI)
//    //}
//    //if is_special_reset(&mut *fighter.module_accessor) {
//    //    VarModule::off_flag(fighter.module_accessor, commons::instance::flag::DISABLE_SPECIAL_HI);
//    //}
    
//    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
//        if !VarModule::is_flag(fighter.module_accessor,commons::instance::flag::GALEFORCE_ATTACK_ON) {
//            //if ControlModule::get_flick_y(fighter.module_accessor) < 1 {
//                VarModule::on_flag(fighter.module_accessor,commons::instance::flag::GALEFORCE_ATTACK_ON);
//                galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
//                println!("gnw stored judge! ");
//                if fighter.global_table[STICK_Y].get_f32() >= 0.15  {
//                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND) != 9 {
//                        VarModule::set_int(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND) + 1, gamewatch::instance::int::JUDGE_STORED_KIND);
//                    }
//                }
//                else if fighter.global_table[STICK_Y].get_f32() >= -0.15 {
//                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND) != 1 {
//                        VarModule::set_int(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND) - 1, gamewatch::instance::int::JUDGE_STORED_KIND);
//                    }
//                }
//            //}
//        }
//    }
//}

unsafe extern "C" fn gameWatch_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    //gameWatch_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, gameWatch_frame);
}