use super::*;
use crate::fighter::common::opff::common_fighter_frame;

//GA - ?
// type: cancel
//  allows minmin to cancel up tilt with grounded up special for vertical combos
unsafe extern "C" fn tantan_galeforce_attack(fighter: &mut L2CFighterCommon) {
    if !is_operation_cpu(fighter.module_accessor) {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_hi3") {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                }
            }
            if !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) && MotionModule::frame(fighter.module_accessor) >= 15.0 {
                    StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
                    galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
                    VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                }
            }
        }
        else {
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
    }
}

unsafe extern "C" fn tantan_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    tantan_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, tantan_frame);
}