use super::*;

//specials
unsafe extern "C" fn speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.0);
        if macros::is_excute(fighter)
        {
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, *FIGHTER_LUCARIO_SHIELD_KIND_SPLIT, *FIGHTER_LUCARIO_SHIELD_GROUP_KIND_SPLIT);
            shield(fighter.lua_state_agent);
            MotionModule::set_rate(fighter.module_accessor, 1.2);
        }
    frame(lua_state, 25.0);
        if macros::is_excute(fighter)
        {
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, *FIGHTER_LUCARIO_SHIELD_KIND_SPLIT, *FIGHTER_LUCARIO_SHIELD_GROUP_KIND_SPLIT);
            shield(fighter.lua_state_agent);
            MotionModule::set_rate(fighter.module_accessor, 1.33);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_speciallw", speciallw,);
    agent.game_acmd("game_specialairlw", speciallw,);
}