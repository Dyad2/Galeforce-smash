use super::*;

unsafe extern "C" fn specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
            macros::FT_MOTION_RATE(fighter, 0.7);
        }
    frame(lua_state, 9.0);
        if macros::is_excute(fighter)
        {
            smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, *FIGHTER_PEACH_SHIELD_KIND_KINOPIO_GUARD, *FIGHTER_PEACH_SHIELD_GROUP_KIND_KINOPIO_GUARD);
            macros::FT_MOTION_RATE(fighter, 0.6);
        }
    frame(lua_state, 35.0);
        if macros::is_excute(fighter)
        {
            smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, *FIGHTER_PEACH_SHIELD_KIND_KINOPIO_GUARD, *FIGHTER_PEACH_SHIELD_GROUP_KIND_KINOPIO_GUARD);
            macros::FT_MOTION_RATE(fighter, 0.9);
        }
    frame(lua_state, 44.0);
        if macros::is_excute(fighter)
        {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO, smash::app::ArticleOperationTarget(0));
            macros::FT_MOTION_RATE(fighter, 1.0);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_special_n", specialn, Priority::Low);
    agent.game_acmd("game_special_air_n", specialn, Priority::Low);
}