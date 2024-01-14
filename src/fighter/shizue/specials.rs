use super::*;

//specials
unsafe extern "C" fn speciallwset(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_LW_FLAG_SET);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_speciallwset", speciallwset,);
}