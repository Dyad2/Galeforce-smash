use smash::phx::{Hash40, Vector3f};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;

use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::fighters::common::FIGHTER_GLOBALS;
use crate::fighters::common::galeforce::*;
use crate::utils::*;

#[fighter_frame( agent = FIGHTER_KIND_NESS )]
fn ness_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let curr_motion_kind = MotionModule::motion_kind(boma);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let status_kind = StatusModule::status_kind(boma);

        //GA - ?
        // type: restriction lift
        //  hitting an opponent with either the projectile or self hit of up special allows ness to not enter an helpless state.
        if !is_operation_cpu(boma) {
            if curr_motion_kind == hash40("special_air_hi") && AttackModule::is_attack_occur(boma) {
                FIGHTER_GLOBALS[entry_id as usize].ga_on = true;
            }
            else if curr_motion_kind == hash40("special_air_hi_end") {
                if FIGHTER_GLOBALS[entry_id as usize].ga_on {
                    StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
                    galeforce_apply_effect(boma, 0.75);
                    FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
                }
            }
            else if ![*FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_AGAIN, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_REFLECT ].contains(&status_kind) {
                FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
            }
        }
    }
}

//weapon
#[acmd_script( agent = "ness_pk_fire", script = "game_pillar", category = ACMD_GAME, low_priority)]
unsafe fn pillar(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
    if macros::is_excute(weapon)
    {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.5, 60, 30, 0, 5, 3.5, 0.0, 3.0, 3.0, Some(0.0), Some(6.0), Some(3.0), 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 0.5, 60, 30, 0, 5, 2.5, 0.0, 10.5, 3.0, None, None, None, 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 0.5, 60, 30, 0, 5, 3.5, 0.0, 3.0, -3.0, Some(0.0), Some(6.0), Some(-3.0), 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::ATTACK(weapon, 3, 0, Hash40::new("top"), 0.5, 60, 30, 0, 5, 2.5, 0.0, 10.5, -3.0, None, None, None, 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::AREA_WIND_2ND_RAD_arg9(weapon, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
    }
}

// #[acmd_func( 
//     battle_object_category = BATTLE_OBJECT_CATEGORY_WEAPON, battle_object_kind = WEAPON_KIND_NESS_PK_FIRE, animation = "pillar_air", animcmd = "game_pillarair")]
// pub fn pillarair(fighter: &mut L2CFighterBase) {
//     acmd!({
//         if macros::is_excute(fighter)
//         {
//             macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 5, 30, 0, 5, 3.5, 0.0, 3.0, 3.0, Some(0.0, Some(6.0, Some(3.0, 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI)
//             macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 5, 30, 0, 5, 2.5, 0.0, 10.5, 3.0, None, None, None, 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI)
//             macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 5, 30, 0, 5, 3.5, 0.0, 3.0, -3.0, Some(0.0, Some(6.0, Some(-3.0, 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI)        
//             macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.5, 5, 30, 0, 5, 2.5, 0.0, 10.5, -3.0, None, None, None, 0.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI)
//             //AREA_WIND_2ND_RAD_arg9(0, 1, 0.05, 200, 0.6, 0, 10, 20, 60)
//         }
//     });
// }

//global edits
#[acmd_script( agent = "ness", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "ness", script = "game_turndash", category = ACMD_GAME, low_priority)]
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

//air
#[acmd_script( agent = "ness", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 8.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 2.5, 367, 100, 35, 0, 5.5, 4.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 2.5, 367, 100, 35, 0, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 5.0, 73, 180, 0, 40, 7.5, 3.0, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 5.0, 73, 180, 0, 40, 4.5, -3.0, 0.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
    frame(lua_state, 17.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(boma, 1.05);
        }
    frame(lua_state, 37.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

//special
#[acmd_script( agent = "ness", script = "game_specials", category = ACMD_GAME, low_priority)]
unsafe fn specials(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            if ArticleModule::get_active_num(boma, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE) < 1 {
                ArticleModule::generate_article(boma, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE, false, 0);
                WorkModule::on_flag(boma, *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT);
            }
        }
}

#[acmd_script( agent = "ness", script = "game_specialairs", category = ACMD_GAME, low_priority)]
unsafe fn specialairs(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            if ArticleModule::get_active_num(boma, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE) < 1 {
                ArticleModule::generate_article(boma, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE, false, 0);
                WorkModule::on_flag(boma, *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT);
            }
        }
}

//other
#[acmd_script( agent = "ness", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
    smashline::install_agent_frames!(
        ness_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        pillar,
        attackairhi,
        specials,
        specialairs,
        escapeairslide
    );
}