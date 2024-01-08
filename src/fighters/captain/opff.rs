use super::*;
use crate::fighters::common::opff::common_fighter_frame;

pub unsafe extern "C" fn captain_b_reverse_speciallw(fighter : &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    
    if [hash40("special_air_lw"), hash40("special_lw")].contains(&curr_motion_kind) && fighter.global_table[MOTION_FRAME].get_i32() <= 4 {
        if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) < 0.0 && !VarModule::is_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE) {
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE);
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
            FighterKineticEnergyMotion::reverse_chara_dir(fighter_kinetic_energy_motion);
        }
    }
    else {
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE);
    }
}

unsafe extern "C" fn captain_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    captain_b_reverse_speciallw(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, captain_frame);
}