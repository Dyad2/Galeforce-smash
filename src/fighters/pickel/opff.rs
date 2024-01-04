use super::*;
use crate::fighters::common::opff::common_fighter_frame;

unsafe extern "C" fn steve_fixes(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
    
    //N1 -> mining
    //N2 -> craft?
    //N3 -> place block
    if [
       *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_FALL, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_FALL_AERIAL, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP, 
       *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_AERIAL, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WAIT].contains(&status_kind) 
       && fighter.global_table[MOTION_FRAME].get_i32() < 5 && (ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor)) < -0.2 {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, -1.0);
    }
    //removing pmlg. might not affect all setups? 
    if (fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR && !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR))
     || (fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND && !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK)) {
        VarModule::off_flag(fighter.battle_object, pickel::instance::flag::ALLOW_SPECIAL_N);
    }
    else {
        VarModule::on_flag(fighter.battle_object, pickel::instance::flag::ALLOW_SPECIAL_N);
    }
}

unsafe extern "C" fn steve_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    steve_fixes(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, steve_frame);
}