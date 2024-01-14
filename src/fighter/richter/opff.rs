use super::*;
use crate::fighter::common::opff::common_fighter_frame;

unsafe extern "C" fn richter_specialhi_nohelplessness(fighter: &mut L2CFighterCommon) {
    unsafe {
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);

        if (curr_motion_kind == hash40("special_air_hi") || curr_motion_kind == hash40("special_hi")) && MotionModule::frame(fighter.module_accessor) >= 45.0 {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::DISABLE_SPECIAL_HI)
        }
        if is_special_reset(&mut *fighter.module_accessor) {
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::DISABLE_SPECIAL_HI)
        }
    }
}

unsafe extern "C" fn richter_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    richter_specialhi_nohelplessness(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, richter_frame);
}