use super::*;
use crate::fighter::common::opff::common_fighter_frame;

unsafe extern "C" fn snake_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, snake_frame);
}