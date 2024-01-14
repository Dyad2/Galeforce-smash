use super::*;
use crate::fighter::common::opff::common_fighter_frame;

unsafe extern "C" fn pitb_attairairn_effect_remove(fighter: &mut L2CFighterCommon) {    
    if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_air_n") {
        EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_blackball_attack")}, false, true);
    }
}

unsafe extern "C" fn pitb_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    pitb_attairairn_effect_remove(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, pitb_frame);
}