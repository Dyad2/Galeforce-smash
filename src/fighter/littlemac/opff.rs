use super::*;
use crate::fighter::common::opff::common_fighter_frame;

//little mac Galeforce Attack
// type: cancel
//  cancels ftilt 1 with spotdodge
unsafe extern "C" fn littlemac_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    
    if !is_operation_cpu(fighter.module_accessor) {
        if curr_motion_kind == hash40("attack_s3_s") {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                }
            }
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == false {
                if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
                    if MotionModule::frame(fighter.module_accessor) <= 7.0 {
                        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE, false);
                        galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                    }
                }
            }
        }
        else {
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
    }
    if is_status_damage(&mut *fighter.module_accessor) && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S) {
        WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
    }
}

unsafe extern "C" fn littlemac_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    littlemac_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, littlemac_frame);
}