use super::*;

unsafe extern "C" fn soundwait3(_fighter: &mut L2CAgentBase) {

    //keep empty, removes a groan
}

pub fn install(agent: &mut smashline::Agent) {
    agent.sound_acmd("sound_wait3", soundwait3, Priority::Low);
}