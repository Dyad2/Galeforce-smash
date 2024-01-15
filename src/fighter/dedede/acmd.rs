use super::*;

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

//ground
unsafe extern "C" fn attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 15, 0, 35, 3.0, 0.0, 6.5, 4.0, Some(0.0), Some(6.5), Some(13.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 180, 15, 0, 25, 4.0, 0.0, 6.5, 18.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 15, 0, 25, 4.0, 0.0, 6.5, 18.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
        AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
        }
}

unsafe extern "C" fn attack100(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    for _ in 0..100000000 {
        for _ in 0..6 {
                if macros::is_excute(fighter)
                {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.75, 361, 10, 0, 10, 3.5, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(15.5), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
                    macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.75, 361, 10, 0, 8, 7.0, 0.0, 7.5, 22.0, None, None, None, 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
                    AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
                    macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2.0);
                    AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
                    macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2.0);
                }
            wait(lua_state, 1.);
                if macros::is_excute(fighter)
                {
                    AttackModule::clear_all(fighter.module_accessor);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
                }
            wait(lua_state, 1.);
        }
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.75, 361, 10, 0, 10, 3.5, 0.0, 7.5, 10.0, Some(0.0), Some(7.5), Some(15.5), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.75, 361, 10, 0, 8, 7.0, 0.0, 7.5, 22.0, None, None, None, 0.5, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
                macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 5.0);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
                macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 5.0);
            }
        wait(lua_state, 1.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
            }
        //wait_loop_clear(lua_state, 0);
        wait(lua_state, 1.);
    }
}

unsafe extern "C" fn attack100end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(lua_state, 4.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 75, 162, 0, 80, 7.0, 0.0, 13.0, 17.0, Some(0.0), Some(7.5), Some(17.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 75, 162, 0, 80, 7.0, 0.0, 13.0, 24.0, Some(0.0), Some(7.5), Some(24.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            }
        wait(lua_state, 2.);
            if macros::is_excute(fighter)
            {
            AttackModule::clear_all(fighter.module_accessor);
            }
}

unsafe extern "C" fn attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
            MotionModule::set_rate(fighter.module_accessor, 3.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 11.0, 33, 92, 0, 35, 3.5, -7.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 11.0, 33, 92, 0, 35, 3.5, 1.0, 0.0, 0.0, Some(8.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 2, 0, Hash40::new("hammer2"), 11.0, 33, 92, 0, 35, 8.3, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 8.0, 37, 92, 0, 30, 3.5, -7.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 8.0, 37, 92, 0, 30, 3.5, 1.0, 0.0, 0.0, Some(8.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 2, 0, Hash40::new("hammer2"), 8.0, 37, 92, 0, 30, 8.3, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        }
    frame(lua_state, 27.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

unsafe extern "C" fn attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 35, 75, 0, 60, 7.0, 0.0, 6.5, 12.0, Some(0.0), Some(6.5), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 35, 75, 0, 60, 5.0, 0.0, 5.0, 12.0, Some(0.0), Some(5.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    frame(lua_state, 17.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

unsafe extern "C" fn attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 32.);
        if macros::is_excute(fighter)
        {
            FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 6.0, 7.0);
        }
    frame(lua_state, 34.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
    frame(lua_state, 40.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 0.0, 0, 0, 0, 0, 3.5, -6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 1, 0, Hash40::new("hammer2"), 0.0, 0, 0, 0, 0, 3.5, -12.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 2, 0, Hash40::new("hammer2"), 16.0, 45, 55, 0, 85, 4.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            let vec2f = smash::phx::Vector2f {x: 16.0, y: 2.0};
            AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), &vec2f, 2, false);
            AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &vec2f, 2, false);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear(fighter.module_accessor, 0, false);
            macros::ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 18.5, 45, 85, 0, 50, 3.5, 6.0, -3.0, 0.0, Some(3.0), Some(-5.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 2, 0, Hash40::new("hammer2"), 25.0, 361, 74, 0, 70, 6.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        }
    frame(lua_state, 44.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 60, 30, 0, 100, 8.0, 0.0, 5.0, 5.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.0, 361, 1, 0, 55, 10.0, 0.0, 10.0, -70.0, Some(0.0), Some(10.0), Some(70.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false,f32::NAN, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

unsafe extern "C" fn attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(fighter.module_accessor, false);
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 6);
        }
    frame(lua_state, 26.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(fighter.module_accessor, true);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 361, 85, 0, 65, 7.0, 0.0, 5.0, 11.8, Some(0.0), Some(5.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 85, 0, 65, 6.0, 0.0, 4.0, 11.8, Some(0.0), Some(4.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    frame(lua_state, 36.);
        if macros::is_excute(fighter)
        {
            smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 85, 0, 55, 4.5, 0.0, 4.0, 11.8, Some(0.0), Some(4.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    frame(lua_state, 39.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 85, 0, 55, 4.0, 0.0, 3.5, 11.8, Some(0.0), Some(3.5), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    frame(lua_state, 42.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

//air
unsafe extern "C" fn attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.33);
            macros::ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 1.5, 367, 100, 40, 0, 6.0, 17.0, 0.0, -3.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 1.5, 367, 100, 40, 0, 6.0, 17.0, 0.0, 6.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 2, 0, Hash40::new("hammer1"), 1.5, 95, 100, 40, 0, 4.0, 9.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 3, 0, Hash40::new("hammer1"), 1.5, 95, 100, 40, 0, 4.0, 3.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        }
    wait(lua_state, 23.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
    wait(lua_state, 1.);
    
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 5.0, 65, 165, 0, 55, 6.5, 17.0, 0.0, -3.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 5.0, 65, 165, 0, 55, 6.5, 17.0, 0.0, 6.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 2, 0, Hash40::new("hammer1"), 5.0, 65, 165, 0, 55, 5.0, 9.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 3, 0, Hash40::new("hammer1"), 5.0, 65, 165, 0, 55, 5.0, 3.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 42.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

unsafe extern "C" fn attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.5);
        }
    frame(lua_state, 5.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 7.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 60, 95, 0, 40, 12.0, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    frame(lua_state, 10.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 80, 95, 0, 40, 9.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    frame(lua_state, 30.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 35.0);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

unsafe extern "C" fn attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            MotionModule::set_rate(fighter.module_accessor, 0.9);
        }
    frame(lua_state, 11.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.8);
        }
    frame(lua_state, 12.0);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 10.0, 361, 90, 0, 30, 5.0, -12.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
            macros::ATTACK(fighter, 1, 0, Hash40::new("hammer2"), 12.0, 361, 98, 0, 30, 8.0, -2.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        }
    frame(lua_state, 16.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 40.0);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

//other
unsafe extern "C" fn escapeairslide(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        }
}

//effect
unsafe extern "C" fn expressionlandingheavy(fighter: &mut L2CAgentBase) {

        if macros::is_excute(fighter)
        {
            smash_script::slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            if !VarModule::is_flag(fighter.module_accessor, commons::instance::flag::WAVEDASH) {
                macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            }
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_dash", dash,);
    agent.game_acmd("game_turndash", turndash,);
    agent.game_acmd("game_attack11", attack11,);
    agent.game_acmd("game_attack100", attack100,);
    agent.game_acmd("game_attack100end", attack100end,);
    agent.game_acmd("game_attacks3", attacks3,);
    agent.game_acmd("game_attacklw3", attacklw3,);
    agent.game_acmd("game_attacks4", attacks4,);
    agent.game_acmd("game_attackdash", attackdash,);
    agent.game_acmd("game_attackairhi", attackairhi,);
    agent.game_acmd("game_attackairn", attackairn,);
    agent.game_acmd("game_attackairf", attackairf,);
    agent.game_acmd("game_escapeairslide", escapeairslide,);
    agent.game_acmd("expression_landingheavy", expressionlandingheavy,);
}