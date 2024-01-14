use super::*;

//specials
unsafe extern "C" fn speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_SHIELD);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_APPLY_POWERUP_MOTION_RATE);
            MotionModule::set_rate(fighter.module_accessor, 1.33);
        }
    frame(lua_state, 26.0);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_SHIELD);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_APPLY_POWERUP_MOTION_RATE);
            MotionModule::set_rate(fighter.module_accessor, 1.15);
        }
}
pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_speciallw", speciallw,);
    agent.game_acmd("game_specialairlw", speciallw,);
}