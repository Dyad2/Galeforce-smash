use super::*;

//specials
unsafe extern "C" fn buddy_specialsstart(_fighter: &mut L2CAgentBase) {

    //keep empty, slows down startup
}

unsafe extern "C" fn buddy_specialairsend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.33);
        }
    frame(lua_state, 27.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_LANDING_HEAVY);
        }
}

unsafe extern "C" fn buddy_specialsend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.33);
        }
    frame(lua_state, 27.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_LANDING_HEAVY);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialhi", buddy_specialsstart,);
    agent.game_acmd("game_specialhi", buddy_specialairsend,);
    agent.game_acmd("game_specialairhi", buddy_specialsend);
}