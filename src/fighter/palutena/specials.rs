use super::*;

//specials
unsafe extern "C" fn speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 6.8, 6.8);
            macros::FT_MOTION_RATE(fighter, 1.25);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_SHIELD);
            fighter.clear_lua_stack();
            // smash_script::lua_args!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
            // shield(fighter.lua_state_agent);
            //macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 12.3, 0.0, 10.0, 1.5, None, None, None, *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
            macros::FT_MOTION_RATE(fighter, 0.75);
        }
    frame(lua_state, 35.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_SHIELD);
            // fighter.clear_lua_stack();
            // smash_script::lua_args!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
            // shield(fighter.lua_state_agent);

            FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.0, 3.2);

            // fighter.clear_lua_stack();
            // smash_script::lua_args!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
            // search(fighter.lua_state_agent);
        }
}

unsafe extern "C" fn speciallwattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 2.5);
        }
    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 65, 0, 50, 7.0, 0.0, 10.5, 13.0, Some(0.0), Some(10.5),Some(14.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 65, 0, 30, 9.0, 0.0, 10.5, 9.0, Some(0.0), Some(10.5),Some(20.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
            AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        }
    wait(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.5);
            AttackModule::clear_all(fighter.module_accessor);
        }
}

unsafe extern "C" fn speciallwreflect(fighter: &mut L2CAgentBase) {

        if macros::is_excute(fighter)
        {
            if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR) {
                shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD, false, -1);
            }
        }
    frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter)
        {
            shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        }
    frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 0.6);
        }
    frame(fighter.lua_state_agent, 35.0);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.0);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_speciallw", speciallw, Priority::Low);
    agent.game_acmd("game_specialairlw", speciallw, Priority::Low);
    agent.game_acmd("game_speciallwattack", speciallwattack, Priority::Low);
    agent.game_acmd("game_specialairlwattack", speciallwattack, Priority::Low);
    agent.game_acmd("game_speciallwreflect", speciallwreflect, Priority::Low);
    agent.game_acmd("game_specialairlwreflect", speciallwreflect, Priority::Low);
}