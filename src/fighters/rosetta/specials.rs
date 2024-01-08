use super::*;

//specials
unsafe extern "C" fn specialhi(fighter: &mut L2CAgentBase) {

        if macros::is_excute(fighter)
        {
            JostleModule::set_status(fighter.module_accessor, false);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialhi", specialhi,);
}