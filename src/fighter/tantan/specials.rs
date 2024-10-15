use super::*;

//specials
unsafe extern "C" fn specialhistart(agent: &mut L2CAgentBase) {

    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent)
    {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TANTAN_STATUS_SPECIAL_HI_FLAG_CAN_INPUT_GROUND_ANGLE);
    }
    // frame(agent.lua_state_agent, 8.0);
    // if macros::is_excute(agent) {
    //     macros::HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_XLU);
    //     macros::HIT_NODE(agent, Hash40::new("armr5"), *HIT_STATUS_XLU);
    //     macros::HIT_NODE(agent, Hash40::new("armr4"), *HIT_STATUS_XLU);
    //     macros::HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_XLU);
    //     macros::HIT_NODE(agent, Hash40::new("arml5"), *HIT_STATUS_XLU);
    //     macros::HIT_NODE(agent, Hash40::new("arml4"), *HIT_STATUS_XLU);
    // }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent)
    {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_TANTAN_STATUS_SPECIAL_HI_FLAG_CAN_INPUT_GROUND_ANGLE);
    }
}

unsafe extern "C" fn specialhishort(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        // frame(lua_state, 1.);
        //     if macros::is_excute(fighter)
        //     {
        //        macros::HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
        //        macros::HIT_NODE(fighter, Hash40::new("armr5"), *HIT_STATUS_XLU);
        //        macros::HIT_NODE(fighter, Hash40::new("armr4"), *HIT_STATUS_XLU);
        //        macros::HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
        //        macros::HIT_NODE(fighter, Hash40::new("arml5"), *HIT_STATUS_XLU);
        //        macros::HIT_NODE(fighter, Hash40::new("arml4"), *HIT_STATUS_XLU);
        //     }
        frame(lua_state, 4.);
            if macros::is_excute(fighter)
            {
                macros::HIT_NO(fighter, 2, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 3, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 4, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 5, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 6, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 7, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 8, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 9, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 10, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 12, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 13, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 18, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 19, *HIT_STATUS_OFF);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 70, 20, 0, 85, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            }
        wait(lua_state, 15.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
            }
}

unsafe extern "C" fn specialhilong(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        // frame(lua_state, 1.);
        //     if macros::is_excute(fighter)
        //     {
        //        macros::HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
        //        macros::HIT_NODE(fighter, Hash40::new("armr5"), *HIT_STATUS_XLU);
        //        macros::HIT_NODE(fighter, Hash40::new("armr4"), *HIT_STATUS_XLU);
        //        macros::HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
        //        macros::HIT_NODE(fighter, Hash40::new("arml5"), *HIT_STATUS_XLU);
        //        macros::HIT_NODE(fighter, Hash40::new("arml4"), *HIT_STATUS_XLU);
        //     }
        frame(lua_state, 4.);
            if macros::is_excute(fighter)
            {
                macros::HIT_NO(fighter, 2, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 3, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 4, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 5, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 6, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 7, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 8, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 9, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 10, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 12, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 13, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 18, *HIT_STATUS_OFF);
                macros::HIT_NO(fighter, 19, *HIT_STATUS_OFF);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 70, 20, 0, 100, 5.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            }
        wait(lua_state, 15.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
            }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialhistart", specialhistart, Priority::Low);
    agent.game_acmd("game_specialhishort", specialhishort, Priority::Low);
    agent.game_acmd("game_specialhilong", specialhilong, Priority::Low);
}