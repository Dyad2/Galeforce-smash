use super::*;
use crate::fighters::common::opff::common_fighter_frame;

unsafe extern "C" fn kamui_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    
    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
        if [hash40("special_s_wall_attack_b"), hash40("special_s_wall_attack_f")].contains(&curr_motion_kind) {
            MotionModule::set_rate(fighter.module_accessor, 1.4);
        }
    }
}

unsafe extern "C" fn kamui_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    kamui_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, kamui_frame);
}