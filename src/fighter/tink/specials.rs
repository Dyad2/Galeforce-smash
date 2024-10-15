use super::*;

//specials
unsafe extern "C" fn specials1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOOMERANG, false, -1);
            MotionModule::set_rate(fighter.module_accessor, 1.5);
        }
    frame(lua_state, 27.);
        if macros::is_excute(fighter)
        {
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOOMERANG, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specials1", specials1, Priority::Low);
    agent.game_acmd("game_specialairs1", specials1, Priority::Low);
    agent.game_acmd("game_specials", specials1, Priority::Low);
    agent.game_acmd("game_specialairs", specials1, Priority::Low);
}