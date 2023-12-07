use super::*

//specials
unsafe extern "C" fn specialairnshoot(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    //OH LORD THIS ONE LOL
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    //new one, up
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(19.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(10.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 19.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 31.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 34.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 37.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 40.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 43.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 3.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 3.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 4.5, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 4.5, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

unsafe extern "C" fn speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 0, *FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW);
            smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        }
    frame(lua_state, 29.);
        if macros::is_excute(fighter)
        {
            smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, 0, *FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW);
            smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        }
}

unsafe extern "C" fn speciallwcounter(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 361, 51, 0, 80, 17.5, 0.0, 8.75, 8.75, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
            AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        }
    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialairnshoot", specialairnshoot,);
    agent.game_acmd("game_speciallw", speciallw,);
    agent.game_acmd("game_speciallwcounter", speciallwcounter,);
}