use super::*;
use crate::fighters::common::opff::common_fighter_frame;

unsafe extern "C" fn toonlink_specialhi_move(fighter: &mut L2CFighterCommon) {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi") {
        if MotionModule::frame(fighter.module_accessor) < 45. {
            let toonlink_upb_speed = smash::phx::Vector3f { x: 0.14 * ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor), y: 0., z: 0.0 };
            let x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            // if (x_vel <= 0.5 && x_vel >= 0.0) || toonlink_upb_speed.x < 0.0 {
            //     KineticModule::add_speed(fighter.module_accessor, &toonlink_upb_speed);
            // }
            // if (x_vel >= -0.5 && x_vel <= 0.0) || toonlink_upb_speed.x > 0.0 {
            //     KineticModule::add_speed(fighter.module_accessor, &toonlink_upb_speed);
            // }
            if x_vel.abs() < 0.5 && !fighter.global_table[IS_STOP].get_bool() {
                KineticModule::add_speed(fighter.module_accessor, &toonlink_upb_speed);
            }
        }
    }
    // if curr_motion_kind == hash40("special_air_hi") {
    //     if MotionModule::frame(fighter.module_accessor) <= 3.0 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
    //         MotionModule::set_rate(fighter.module_accessor, 0.1);
    //         FIGHTER_GLOBALS[entry_id_int as usize].tink_specialhi_charge += 0.02;
    //         FIGHTER_GLOBALS[entry_id_int as usize].tink_specialhi_charging = true;
    //     }
    //     else {
    //         MotionModule::set_rate(fighter.module_accessor, 1.0);
    //         FIGHTER_GLOBALS[entry_id_int as usize].tink_specialhi_charging = false;
    //     }
    // }
}
unsafe extern "C" fn toonlink_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    toonlink_specialhi_move(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, toonlink_frame);
}