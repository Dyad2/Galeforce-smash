
//global edits
unsafe extern "C" fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

unsafe extern "C" fn turndash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

//air
unsafe extern "C" fn attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.8);
        }
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.0);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 42, 120, 0, 0, 4.0, 0.0, 4.0, -13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 42, 120, 0, 30, 4.5, 0.0, 4.0, -8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

//grabs
unsafe extern "C" fn catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        frame(lua_state, 8.);
            if macros::is_excute(fighter)
            {
                GrabModule::set_rebound(fighter.module_accessor, true);
            }
        frame(lua_state, 9.);
            if macros::is_excute(fighter)
            {
                macros::CATCH(fighter, 0, Hash40::new("top"), 3.5, 0.0, 5.2, 4.0, Some(0.0), Some(5.2), Some(10.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
                macros::CATCH(fighter, 1, Hash40::new("top"), 1.75, 0.0, 5.2, 2.25, Some(0.0), Some(5.2), Some(11.95), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
            }
        macros::game_CaptureCutCommon(fighter);
        wait(lua_state, 2.);
            if macros::is_excute(fighter)
            {
                smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
                GrabModule::set_rebound(fighter.module_accessor, false);
            }
}

unsafe extern "C" fn catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            GrabModule::set_rebound(fighter.module_accessor, true);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::CATCH(fighter, 0, Hash40::new("top"), 3.5, 0.0, 5.2, -4.0, Some(0.0), Some(5.2), Some(-17.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 1.75, 0.0, 5.2, -2.25, Some(0.0), Some(5.2), Some(-19.25), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }
    macros::game_CaptureCutCommon(fighter);
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            smash_script::grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, false);
        }
}

//other
unsafe extern "C" fn escapeairslide(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_dash", dash,);
    agent.game_acmd("game_turndash", turndash,);
    agent.game_acmd("game_attackairb", attackairb,);
    agent.game_acmd("game_catchdash", catchdash,);
    agent.game_acmd("game_catchturn", catchturn,);
    agent.game_acmd("game_escapeairslide", escapeairslide,);
}