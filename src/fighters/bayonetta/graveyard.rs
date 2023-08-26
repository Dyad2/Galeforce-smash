
//abk multihit
#[acmd_script( agent = "bayonetta", script = "game_specialairsu", category = ACMD_GAME, low_priority)]
unsafe fn specialairsu(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 { //purple
        frame(lua_state, 1.);
            if macros::is_excute(fighter)
            {
                JostleModule::set_status(fighter.module_accessor, false);
                macros::FT_MOTION_RATE(fighter, 0.37);
                smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 5, 0, 20, 0, false);
                smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 5);
            }
        frame(lua_state, 12.);
            if macros::is_excute(fighter)
            {
                macros::FT_MOTION_RATE(fighter, 1.25);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
                macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 1.2, 27, 100, 110, 0, 5.0, 3.5, 0.0, 3.0, Some(-1.0), Some(0.0), Some(3.0), 0.85, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 1.2, 27, 100, 110, 0, 5.0, 3.5, 0.0, -3.0, Some(-1.0), Some(0.0), Some(-3.0), 0.85, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 1.2, 368, 100, 110, 0, 4.5, 7.75, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 3, 0, Hash40::new("kneer"), 1.2, 33, 100, 120, 0, 5.0, 3.5, 0.0, 3.0, Some(-1.0), Some(0.0), Some(3.0), 0.85, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 4, 0, Hash40::new("kneer"), 1.2, 33, 100, 120, 0, 5.0, 3.5, 0.0, -3.0, Some(-1.0), Some(0.0), Some(-3.0), 0.85, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                let attack_pos = smash::phx::Vector2f {x: 15., y: 7.5};
                AttackModule::set_vec_target_pos(fighter.module_accessor, 2, Hash40{hash: hash40("top")}, &attack_pos, 7, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 4.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 4.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 4.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 4.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 4.0, false);
                AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            }
        frame(lua_state, 15.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 1.2, 27, 100, 100, 0, 5.0, 3.5, 0.0, 3.0, Some(-1.0), Some(0.0), Some(3.0), 0.85, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 1.2, 27, 100, 100, 0, 5.0, 3.5, 0.0, -3.0, Some(-1.0), Some(0.0), Some(-3.0), 0.85, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 1.2, 368, 100, 110, 0, 4.5, 7.75, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 3, 0, Hash40::new("kneer"), 1.2, 33, 100, 115, 0, 5.5, 3.5, 0.0, 3.0, Some(-1.0), Some(0.0), Some(3.0), 0.85, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 4, 0, Hash40::new("kneer"), 1.2, 33, 100, 115, 0, 5.0, 3.5, 0.0, -3.0, Some(-1.0), Some(0.0), Some(-3.0), 0.85, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                let attack_pos = smash::phx::Vector2f {x: 15., y: 7.5};
                AttackModule::set_vec_target_pos(fighter.module_accessor, 2, Hash40{hash: hash40("top")}, &attack_pos, 8, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 4.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 4.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 4.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 4.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 4.0, false);
                AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            }
        frame(lua_state, 18.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 1.2, 27, 100, 90, 0, 5.0, 3.5, 0.0, 3.0, Some(-1.0), Some(0.0), Some(3.0), 0.85, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 1.2, 27, 100, 90, 0, 5.0, 3.5, 0.0, -3.0, Some(-1.0), Some(0.0), Some(-3.0), 0.85, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 1.2, 368, 100, 110, 0, 4.5, 7.75, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 3, 0, Hash40::new("kneer"), 1.2, 33, 100, 105, 0, 5.0, 3.5, 0.0, 3.0, Some(-1.0), Some(0.0), Some(3.0), 0.85, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 4, 0, Hash40::new("kneer"), 1.2, 33, 100, 105, 0, 5.0, 3.5, 0.0, -3.0, Some(-1.0), Some(0.0), Some(-3.0), 0.85, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                let attack_pos = smash::phx::Vector2f {x: 15., y: 7.5};
                AttackModule::set_vec_target_pos(fighter.module_accessor, 2, Hash40{hash: hash40("top")}, &attack_pos, 8, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 4.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 4.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 4.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 4.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, 4.0, false);
                AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            }
        frame(lua_state, 21.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                macros::FT_MOTION_RATE(fighter, 1.0);
            }
        frame(lua_state, 22.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 3.5, 45, 62, 0, 50, 5.7, 6.0, 0.0, 3.0, Some(1.0), Some(0.0), Some(3.0), 1.2, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 3.5, 45, 62, 0, 50, 5.0, 6.0, 0.0, -4.0, Some(1.0), Some(0.0), Some(-4.0), 1.2, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 8.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 8.0, false);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
            }
        frame(lua_state, 25.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
                smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
            }
        frame(lua_state, 30.);
            if macros::is_excute(fighter)
            {
                JostleModule::set_status(fighter.module_accessor, true);
            }
        frame(lua_state, 33.);
            if macros::is_excute(fighter)
            {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
            }
    }
    else {
        frame(lua_state, 1.);
            if macros::is_excute(fighter)
            {
                JostleModule::set_status(fighter.module_accessor, false);
                macros::FT_MOTION_RATE(fighter, 0.5);
                smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 5, 0, 20, 0, false);
                smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 5);
            }
        frame(lua_state, 12.);
            if macros::is_excute(fighter)
            {
                macros::FT_MOTION_RATE(fighter, 1.0);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
                macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 40, 50, 0, 60, 5.0, 6.0, 0.0, 3.0, Some(2.0), Some(0.0), Some(3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 38, 50, 0, 60, 5.0, 6.0, 0.0, -3.0, Some(2.0), Some(0.0), Some(-3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 1.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 1.0, false);
            }
        frame(lua_state, 15.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 65, 35, 0, 60, 5.0, 6.0, 0.0, 3.0, Some(2.0), Some(0.0), Some(3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 60, 35, 0, 60, 5.0, 6.0, 0.0, -3.0, Some(2.0), Some(0.0), Some(-3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 1.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 1.0, false);
            }
        frame(lua_state, 18.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 83, 33, 0, 55, 5.0, 6.0, 0.0, 3.0, Some(2.0), Some(0.0), Some(3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 78, 33, 0, 55, 5.0, 6.0, 0.0, -3.0, Some(2.0), Some(0.0), Some(-3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 1.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 1.0, false);
            }
        frame(lua_state, 20.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 100, 112, 60, 0, 5.0, 6.0, 0.0, 3.0, Some(2.0), Some(0.0), Some(3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 95, 112, 50, 0, 5.0, 6.0, 0.0, -3.0, Some(2.0), Some(0.0), Some(-3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 1.0, false);
                AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 1.0, false);
            }
        frame(lua_state, 23.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
            }
        frame(lua_state, 25.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(fighter.module_accessor);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
                smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
            }
        frame(lua_state, 30.);
            if macros::is_excute(fighter)
            {
                JostleModule::set_status(fighter.module_accessor, true);
            }
    }
}

//new anim makes this script bad :(
#[acmd_script( agent = "bayonetta", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn airb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 10, 3, 10, 5, true);
            MotionModule::set_rate(fighter.module_accessor, 2.0);
        }
    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 19.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 120, 0, 16, 4.6, 0.0, 13.0, -14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.5, 361, 120, 0, 12, 4.2, 0.0, 13.0, -11.7, Some(0.0), Some(11.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    frame(lua_state, 38.);
        if macros::is_excute(fighter)
        {                
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}


//abk backup
#[acmd_script( agent = "bayonetta", script = "game_specialairsu", category = ACMD_GAME, low_priority)]
unsafe fn specialairsu(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(fighter.module_accessor, false);
            macros::FT_MOTION_RATE(fighter, 0.5);
            smash_script::notify_event_msc_cmd!(fighter, 0x2d51fcdb09 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 5, 0, 20, 0, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2b7cb92b79 as u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 5);
        }
    frame(lua_state, 12.);
        if macros::is_excute(fighter)
        {
            macros::FT_MOTION_RATE(fighter, 1.0);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 40, 50, 0, 60, 5.0, 6.0, 0.0, 3.0, Some(2.0), Some(0.0), Some(3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 38, 50, 0, 60, 5.0, 6.0, 0.0, -3.0, Some(2.0), Some(0.0), Some(-3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 1.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 1.0, false);
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 65, 35, 0, 60, 5.0, 6.0, 0.0, 3.0, Some(2.0), Some(0.0), Some(3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 60, 35, 0, 60, 5.0, 6.0, 0.0, -3.0, Some(2.0), Some(0.0), Some(-3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 1.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 1.0, false);
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 83, 33, 0, 55, 5.0, 6.0, 0.0, 3.0, Some(2.0), Some(0.0), Some(3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 78, 33, 0, 55, 5.0, 6.0, 0.0, -3.0, Some(2.0), Some(0.0), Some(-3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 1.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 1.0, false);
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 100, 112, 60, 0, 5.0, 6.0, 0.0, 3.0, Some(2.0), Some(0.0), Some(3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.0, 95, 112, 50, 0, 5.0, 6.0, 0.0, -3.0, Some(2.0), Some(0.0), Some(-3.0), 1.2, 1.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 1.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 1.0, false);
        }
    frame(lua_state, 23.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        }
    frame(lua_state, 25.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            JostleModule::set_status(fighter.module_accessor, true);
        }
}