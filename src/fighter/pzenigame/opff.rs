use super::*;
use crate::fighter::common::opff::common_fighter_frame;

unsafe extern "C" fn pzenigame_specialn_reverse(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    
    if [*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_SHOOT].contains(&status_kind) && fighter.global_table[MOTION_FRAME].get_i32() <= 5 {
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

unsafe extern "C" fn pzenigame_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    pzenigame_specialn_reverse(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, pzenigame_frame);
}