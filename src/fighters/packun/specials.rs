use super::*;

//specials
unsafe extern "C" fn speciallwstart(fighter: &mut L2CAgentBase) {

        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.6);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_speciallwstart", speciallwstart,);
    agent.game_acmd("game_specialairlwstart", speciallwstart,);
}