use super::*;

//special
unsafe extern "C" fn specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_REQUEST_SPECIAL_HI_CANCEL);
        }
}

unsafe extern "C" fn specialairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.);
        if macros::is_excute(agent)
        {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_REQUEST_SPECIAL_HI_CANCEL);
        }
}

unsafe extern "C" fn speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent)
        {
            macros::FT_MOTION_RATE(agent, 1.5);
            if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_GUNMAN) {
                VarModule::on_flag(agent.module_accessor, duckhunt::instance::flag::GUNMAN_REACTIVATE);
            }
        }
    frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent)
        {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_DUCKHUNT_STATUS_SPECIAL_LW_FLAG_CALL_TRIGGER);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialhi", specialhi, Priority::Low);
    agent.game_acmd("game_specialairhi", specialairhi, Priority::Low);
    agent.game_acmd("game_speciallw", speciallw, Priority::Low);
    agent.game_acmd("game_specialairlw", speciallw, Priority::Low);
}