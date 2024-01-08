use super::*;
use crate::fighters::common::opff::common_fighter_frame;

//GA - Warlock's Darkest Flight
// type: new move / cancel
//  allows cancelling aerial side special with up special. the opponent is grabbed automatically
unsafe extern "C" fn ganon_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    
    if curr_motion_kind == hash40("special_air_s_catch") {
        if MotionModule::frame(fighter.module_accessor) >= 10.0 && MotionModule::frame(fighter.module_accessor) <= 15.0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                galeforce_apply_effect(&mut *fighter.module_accessor, 1.0);
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
        }
    }
    if ![*FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL,
      *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) {
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
}

unsafe extern "C" fn ganon_speciallw_breverse(fighter: &mut L2CFighterCommon) {
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

unsafe extern "C" fn ganon_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    ganon_galeforce_attack(fighter);
    ganon_speciallw_breverse(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, ganon_frame);
}