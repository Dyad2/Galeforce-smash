use super::*;
use smash::app::utility;

fn on_start(fighter: &mut L2CFighterCommon) {
    unsafe {
        if utility::get_kind(&mut *fighter.module_accessor) != *FIGHTER_KIND_REFLET {
            return;
        }
        WorkModule::set_int(fighter.module_accessor, 8, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_start(on_start);
}