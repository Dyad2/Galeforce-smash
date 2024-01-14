use super::*;
use crate::fighter::common::opff::common_fighter_frame;

unsafe extern "C" fn  wiifit_attack11_reverse(fighter: &mut L2CFighterCommon) {
    let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
    
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_11") && fighter.global_table[MOTION_FRAME].get_i32() > 6 && (ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor)) < -0.1 {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, -1.0);
    }
}

unsafe extern "C" fn wiifit_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    wiifit_attack11_reverse(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, wiifit_frame);
}