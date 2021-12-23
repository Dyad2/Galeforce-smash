use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd::*;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;

//global edits
#[acmd_script( agent = "edge", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "edge", script = "game_turndash", category = ACMD_GAME, low_priority)]
unsafe fn turndash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "edge", script = "game_attackhi4", category = ACMD_GAME, low_priority)]
unsafe fn attackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        frame(lua_state, 13.);
            if macros::is_excute(fighter){
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            }
        frame(lua_state, 15.);
            if macros::is_excute(fighter)
            {
               macros::FT_MOTION_RATE(fighter, 0.6);
            }
        frame(lua_state, 25.);
            if macros::is_excute(fighter)
            {
               macros::FT_MOTION_RATE(fighter, 1.);
            }
        frame(lua_state, 27.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 2, 0, Hash40::new_raw(0x075bd7e757), 10.5, 92, 79, 0, 62, 4.5, 5.2, 0.0, 3.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 0, 0, Hash40::new_raw(0x075bd7e757), 15.5, 92, 79, 0, 67, 5.7, 13.8, 0.0, 3.5, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 1, 0, Hash40::new_raw(0x075bd7e757), 11.5, 92, 79, 0, 67, 4.3, 21.0, 0.0, 3.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 3, 0, Hash40::new_raw(0x075bd7e757), 10.5, 92, 79, 0, 62, 4.5, 5.2, 0.0, 3.0, Some(4.5), Some(-2.0), Some(9.5), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 4, 0, Hash40::new_raw(0x075bd7e757), 15.5, 92, 79, 0, 67, 5.7, 13.8, 0.0, 3.5, Some(12.5), Some(-2.5), Some(11.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 5, 0, Hash40::new_raw(0x075bd7e757), 11.5, 92, 79, 0, 67, 4.3, 21.0, 0.0, 3.0, Some(19.0), Some(-3.0), Some(13.5), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        frame(lua_state, 28.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear(boma, 3, false);
                AttackModule::clear(boma, 4, false);
                AttackModule::clear(boma, 5, false);
                macros::ATTACK(fighter, 2, 0, Hash40::new_raw(0x075bd7e757), 12.0, 97, 79, 0, 62, 4.5, 5.2, 0.0, 3.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 0, 0, Hash40::new_raw(0x075bd7e757), 17.0, 97, 79, 0, 67, 5.7, 13.8, 0.0, 3.5, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 1, 0, Hash40::new_raw(0x075bd7e757), 13.0, 97, 79, 0, 67, 4.3, 21.0, 0.0, 3.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        frame(lua_state, 29.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 2, 0, Hash40::new_raw(0x075bd7e757), 12.0, 83, 79, 0, 62, 4.5, 5.2, 0.0, 3.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 0, 0, Hash40::new_raw(0x075bd7e757), 17.0, 83, 79, 0, 67, 5.7, 13.8, 0.0, 3.5, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 1, 0, Hash40::new_raw(0x075bd7e757), 13.0, 83, 79, 0, 67, 4.3, 21.0, 0.0, 3.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        frame(lua_state, 30.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 17.0, 97, 79, 0, 67, 5.7, 0.0, 34.0, 5.0, Some(0.0), Some(34.0), Some(-5.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 13.0, 97, 79, 0, 67, 4.3, 0.0, 40.5, 8.5, Some(0.0), Some(40.5), Some(-8.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        frame(lua_state, 31.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear(boma, 3, false);
                AttackModule::clear(boma, 4, false);
            }
        frame(lua_state, 33.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(boma);
            }
}

#[acmd_script( agent = "edge", script = "game_attacks4", category = ACMD_GAME, low_priority)]
unsafe fn attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
        frame(lua_state, 1.);
            if macros::is_excute(fighter)
            {
               macros::FT_MOTION_RATE(fighter, 0.8);
            }
        frame(lua_state, 10.);
            if macros::is_excute(fighter)
            {
               macros::FT_MOTION_RATE(fighter, 1.0);
            }
        frame(lua_state, 11.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            }
        frame(lua_state, 22.);
            if macros::is_excute(fighter)
            {
                FighterAreaModuleImpl::enable_fix_jostle_area(boma, 8., 3.);
            }
        frame(lua_state, 26.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new_raw(0x075bd7e757), 13.5, 361, 81, 0, 54, 4.0, 0.0, -1.0, 4.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 2, 0, Hash40::new_raw(0x075bd7e757), 20.0, 361, 78, 0, 57, 4.0, 7.0, -1.0, 4.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 3, 0, Hash40::new_raw(0x075bd7e757), 20.0, 361, 78, 0, 57, 4.0, 15.0, -1.0, 4.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 4, 0, Hash40::new_raw(0x075bd7e757), 15.0, 361, 85, 0, 57, 4.0, 21.5, -1.0, 4.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 13.5, 361, 81, 0, 54, 5.8, 0.0, 8.5, 6.5, Some(0.0), Some(8.5), Some(6.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
                macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 20.0, 361, 78, 0, 57, 5.5, 0.0, 9.0, 16.5, Some(0.0), Some(9.0), Some(21.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL,*COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        wait(lua_state, 2.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(boma);
            }
        frame(lua_state, 46.);
            if macros::is_excute(fighter)
            {
                FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4., 3.);
            }
}

#[acmd_script( agent = "edge", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
unsafe fn escapeairslide(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        attackhi4,
        attacks4,
        escapeairslide
    );
}