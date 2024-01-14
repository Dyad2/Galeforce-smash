use super::*;
use crate::fighter::common::opff::common_fighter_frame;

//dash attack to neutral air when diddy falls offstage
pub unsafe extern "C" fn diddy_dash_attack_to_nair(fighter : &mut L2CFighterCommon) {    
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR && !is_status_damage(&mut *fighter.module_accessor) {
        if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
            ControlModule::set_attack_air_kind(fighter.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_N);
        }
    }
}

//jab 3 replaced with ftilt
pub unsafe extern "C" fn diddy_ftilt_replace_jab3(fighter : &mut L2CFighterCommon) {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_12") && MotionModule::frame(fighter.module_accessor) >= 7.0 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        ControlModule::set_main_stick_x(fighter.module_accessor, ControlModule::get_stick_x(fighter.module_accessor).abs() * PostureModule::lr(fighter.module_accessor)); //prevents pivoting the ftilt/jab 3
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
        MotionModule::change_motion_kind(fighter.module_accessor, Hash40{hash: hash40("attack_s3_s")});
    }
}

unsafe extern "C" fn diddy_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    diddy_dash_attack_to_nair(fighter);
    diddy_ftilt_replace_jab3(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, diddy_frame);
}