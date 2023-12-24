use super::*

//special
#[acmd_script( agent = "ness", scripts = ["game_specials", "game_specialairs"], category = ACMD_GAME, low_priority)]
unsafe extern "C" fn specials(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            if ArticleModule::get_active_num(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE) < 1 {
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE, false, -1);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT);
            }
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specials", specials,);
    agent.game_acmd("game_specialairs", specials,);
}