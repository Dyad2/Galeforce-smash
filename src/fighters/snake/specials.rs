
//Specials
unsafe extern "C" fn specialairhihang(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        // if macros::is_excute(fighter)
        // {
        //     smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 9);
        // }
    frame(lua_state, 41.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_CYPHER_HANG_TRANS_ID_CUT_STICK);
            WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
            WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
            WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    frame(lua_state, 89.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_CYPHER_HANG_TRANS_ID_CUT_TIME_OUT);
        }
}

unsafe extern "C" fn specialnstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
        }
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
           macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, false, -1);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, false, smash::app::ArticleOperationTarget(0));
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, true, smash::app::ArticleOperationTarget(0));
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 2.0);
        }
}

unsafe extern "C" fn specialairnstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
        }
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, false, -1);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, false, smash::app::ArticleOperationTarget(0));
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, true, smash::app::ArticleOperationTarget(0));
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 2.0);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialairhihang", specialairhihang,);
    agent.game_acmd("game_specialnstart", specialnstart,);
    agent.game_acmd("game_specialairnstart", specialairnstart,);
}