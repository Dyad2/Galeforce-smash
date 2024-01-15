use super::*;
use crate::fighter::common::opff::common_fighter_frame;

unsafe extern "C" fn eflame_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, eflame_frame);
}