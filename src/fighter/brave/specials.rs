use super::*;

//Specials
unsafe extern "C" fn brave_speciallw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.4);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_FLASH, false, -1);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_ENABLE_CONTROL_ENERGY);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
}

unsafe extern "C" fn brave_specialairlw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.4);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_FLASH, false, -1);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_ENABLE_CONTROL_ENERGY);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
}

unsafe extern "C" fn brave_speciallw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        frame(lua_state, 6.);
            if macros::is_excute(fighter)
            {
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_EXPLOSION, false, -1);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_ENABLE_CONTROL_ENERGY);
            }
}

unsafe extern "C" fn brave_specialairlw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_EXPLOSION, false, -1);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_ENABLE_CONTROL_ENERGY);
        }
}

unsafe extern "C" fn brave_speciallw5(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.4);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_STRENGTH);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_EXPLOSION, false, -1);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_ENABLE_CONTROL_ENERGY);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
}

unsafe extern "C" fn brave_specialairlw5(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 0.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.4);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_STRENGTH);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_EXPLOSION, false, -1);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_ENABLE_CONTROL_ENERGY);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_speciallw3", brave_speciallw3); //sizzle
    agent.game_acmd("game_specialairlw3", brave_specialairlw3);
    agent.game_acmd("game_speciallw4", brave_speciallw4); //boom
    agent.game_acmd("game_specialairlw4", brave_specialairlw4);
    agent.game_acmd("game_speciallw5", brave_speciallw5); //kaboom
    agent.game_acmd("game_specialairlw5", brave_specialairlw5);
}