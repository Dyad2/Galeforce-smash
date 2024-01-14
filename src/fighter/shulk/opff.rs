use super::*;
use crate::fighter::common::opff::common_fighter_frame;

//GA - ?
// type: cancel
//  allows shulk to cancel the first part of up special if they never input the second part.
unsafe extern "C" fn shulk_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    
    if !is_operation_cpu(fighter.module_accessor) {
        if curr_motion_kind == hash40("special_hi") || curr_motion_kind == hash40("special_air_hi") {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                VarModule::on_flag(fighter.module_accessor,commons::instance::flag::GALEFORCE_ATTACK_ON);
            }
            if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) && MotionModule::frame(fighter.module_accessor) >= 30.0 {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SHULK_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_ADD_SHIFT_INPUT); //what the fuck is this?
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
                VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                VarModule::on_flag(fighter.module_accessor, commons::instance::flag::DISABLE_SPECIAL_HI)
            }
        }
        else {
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
        if is_special_reset(&mut *fighter.module_accessor) {
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::DISABLE_SPECIAL_HI)
        }
    }
}

unsafe extern "C" fn shulk_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    shulk_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, shulk_frame);
}