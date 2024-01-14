use super::*;
use crate::fighter::common::opff::common_fighter_frame;

unsafe extern "C" fn ike_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    
    //GA - Counter
    // type: cancel
    // Allows cancelling the counter attack when counter detects a hit. Grants intangibility
    // TODO: rewrite in status
    if curr_motion_kind == hash40("special_lw_hit") || curr_motion_kind == hash40("special_air_lw_hit") {
        if fighter.global_table[MOTION_FRAME].get_i32() <= 3 && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
            galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
            VarModule::set_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER, 20);
            CancelModule::enable_cancel(fighter.module_accessor);
            if situation_kind == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
            }
            else {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
            }
        }
    }
    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
        if VarModule::get_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER) > 0 {
            VarModule::sub_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER, 1);
            HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
        }
        else {
            HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
    }
}

unsafe extern "C" fn ike_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    ike_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, ike_frame);
}