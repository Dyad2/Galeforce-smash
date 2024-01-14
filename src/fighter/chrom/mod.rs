// // use smash::phx::Hash40;
// use smash::hash40;
// use smash::lib::lua_const::*;
// use smash::app::lua_bind::*;
// use smash::lua2cpp::L2CAgentBase;
// use smash::lua2cpp::L2CFighterCommon;
// use smash::app::sv_animcmd::*;
// use smashline::*;
// use smash_script::{
//macros,
//lua_args,
//};

// use crate::fighter::common::galeforce::*;
// use galeforce_utils::{vars::*, utils::*};
// use custom_var::*;

// #[fighter_frame( agent = FIGHTER_KIND_CHROM )]
// fn chrom_frame(fighter: &mut L2CFighterCommon) {
//     unsafe {
//         let status_kind = StatusModule::status_kind(fighter.module_accessor);

//         //GA - Exalt's Exertion
//         // type: cancel
//         //  Hits side b 3 (any angle) and cancel it with a wavedash. Disables side b for s short time.
//         if !is_operation_cpu(fighter.module_accessor) {
//             if status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3 {
//                 if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
//                     VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
//                 }
//             }
//             else {
//                 VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
//             }
//             if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
//                 StopModule::cancel_hit_stop(fighter.module_accessor);
//                 if ControlModule::get_stick_y(fighter.module_accessor) > 0.0 {
//                     ControlModule::set_main_stick_y(fighter.module_accessor, 0.0);
//                 }
//                 galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
//                 StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
//                 StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
//                 //MotionModule::change_motion_kind(fighter.module_accessor, smash::phx::Hash40{hash: hash40("escape_air_slide")});
//                 VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
//                 VarModule::set_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER, 420);
//             }
//             if VarModule::get_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER) >= 0 {
//                 chrom_disable_dance_effect(fighter);
//                 VarModule::sub_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER, 1);
//             }
//         }
//     }
// }

// //global edits
// #[acmd_script( agent = "chrom", script = "game_dash", category = ACMD_GAME, low_priority)]
// unsafe fn dash(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;

//     frame(lua_state, 15.);
//         if macros::is_excute(fighter)
//         {
//             WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
//         }
// }

// #[acmd_script( agent = "chrom", script = "game_turndash", category = ACMD_GAME, low_priority)]
// unsafe fn turndash(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;

//     frame(lua_state, 1.);
//         if macros::is_excute(fighter)
//         {
//             WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
//         }
//     frame(lua_state, 16.);
//         if macros::is_excute(fighter)
//         {
//             WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
//         }
// }

// //ground
// #[acmd_script( agent = "chrom", script = "game_attack11", category = ACMD_GAME, low_priority)]
// unsafe fn attack11(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;

//     frame(lua_state, 5.0);
//         if macros::is_excute(fighter)
//         {
//             FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 5., 5.);
//             macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 62, 60, 0, 40, 3.7, 0.0, 10.0, 7.0, Some(0.0), Some(10.0), Some(5.5),  1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
//             macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 6.5, 62, 60, 0, 40, 3.5, 0.0, 0.0, 1.4, Some(0.0), Some(0.0), Some(0.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
//             macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 6.5, 62, 60, 0, 40, 3.5, 0.0, 0.0, 8.1, Some(0.0), Some(0.0), Some(0.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
//         }
//     frame(lua_state, 7.0);
//         if macros::is_excute(fighter)
//         {
//             AttackModule::clear_all(fighter.module_accessor);
//         }
//     frame(lua_state, 28.0);
//         if macros::is_excute(fighter)
//         {
//             if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
//                 CancelModule::enable_cancel(fighter.module_accessor);
//             }
//         }
// }

// //air
// #[acmd_script( agent = "chrom", script = "game_attackairn", category = ACMD_GAME, low_priority)]
// unsafe fn attackairn(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
    
//     frame(lua_state, 7.0);
//         if macros::is_excute(fighter)
//         {
//             WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
//         }
//     frame(lua_state, 8.0);
//         if macros::is_excute(fighter)
//         {
//             macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 11.0, 50, 72, 0, 50, 5.6, 0.0, 0.0, -1.5, Some(0.0), Some(0.0), Some(0.0),  1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
//             macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 11.0, 50, 2, 0, 50, 4.2, 2.5, 0.0, 0.7, Some(0.0), Some(0.0), Some(0.0),  1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
//             macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 11.0, 50, 72, 0, 50, 4.2, 2.5, 0.0, 7.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
//         }
//     wait(lua_state, 7.0);
//         if macros::is_excute(fighter)
//         {
//             AttackModule::clear_all(fighter.module_accessor);
//         }
//     frame(lua_state, 41.0);
//         if macros::is_excute(fighter)
//         {
//             WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
//         }
// }

// //specials
// #[acmd_script( agent = "chrom", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority)]
// unsafe fn speciallw(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;

//     frame(lua_state, 1.0);
//         if macros::is_excute(fighter)
//         {
//             MotionModule::set_rate(fighter.module_accessor, 1.15);
//         }
//     frame(lua_state, 8.0);
//         if macros::is_excute(fighter)
//         {
//             WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SHIELD);
//             MotionModule::set_rate(fighter.module_accessor, 1.33);
//         }
//     frame(lua_state, 30.0);
//         if macros::is_excute(fighter)
//         {
//             WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SHIELD);
//             MotionModule::set_rate(fighter.module_accessor, 1.15);
//         }
// }

// //others
// #[acmd_script( agent = "chrom", script = "game_appealhil", category = ACMD_GAME, low_priority)]
// unsafe fn falchionusel(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;

//     frame(lua_state, 40.0);
//         if macros::is_excute(fighter)
//         {
//             DamageModule::heal(fighter.module_accessor, -2.0, 0);
//         }
// }

// #[acmd_script( agent = "chrom", script = "game_appealhir", category = ACMD_GAME, low_priority)]
// unsafe fn falchionuser(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;

//     frame(lua_state, 40.0);
//         if macros::is_excute(fighter)
//         {
//             DamageModule::heal(fighter.module_accessor, -2.0, 0);
//         }
// }

// #[acmd_script( agent = "chrom", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
// unsafe fn escapeairslide(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;

//     frame(lua_state, 14.);
//         if macros::is_excute(fighter)
//         {
//             WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
//             notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
//         }
//     frame(lua_state, 24.);
//         if macros::is_excute(fighter)
//         {
//             WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
//         }
// }

// //effects
// #[acmd_script( agent = "chrom", script = "effect_attackairn", category = ACMD_EFFECT, low_priority)]
// unsafe fn effectairn(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;

//     frame(lua_state, 8.0);
//         if macros::is_excute(fighter)
//         {
//             macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_chrom_sword1"), Hash40::new("tex_chrom_sword2"), 9, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
//             macros::EFFECT_FOLLOW(fighter, Hash40::new("chrom_sword_light"), Hash40::new("sword1"), 0, 0, 11, 0, 0, 0, 0.4, true);
//         }
//     wait(lua_state, 3.0);
//         if macros::is_excute(fighter)
//         {
//             macros::AFTER_IMAGE_OFF(fighter, 4);
//         }
//     wait(lua_state, 4.0);
//         if macros::is_excute(fighter)
//         {
//             macros::EFFECT_OFF_KIND(fighter, Hash40::new("chrom_sword_light"), false, true);
//         }
// }

// // sounds
// #[acmd_script( agent = "chrom", script = "sound_attackairn", category = ACMD_SOUND, low_priority)]
// unsafe fn soundairn(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;

//     frame(lua_state, 5.0);
//         if macros::is_excute(fighter)
//         {
//             macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_chrom_rnd_attack"));
//         }
//     //wait(Frames=1)
//     //if(is_excute){
//     //	macros::PLAY_SE(hash40("se_chrom_swing_s"))
//     //}
//     wait(lua_state, 3.0);
//         if macros::is_excute(fighter)
//         {
//             macros::PLAY_SE(fighter, Hash40::new("se_chrom_attackair_n01"));
//         }
// }

// pub fn install() {
//     smashline::install_agent_frames!(
//         chrom_frame
//     );
//     smashline::install_acmd_scripts!(
//         dash,
//         turndash,
//         attack11,
//         attackairn,
//         speciallw,
//         falchionusel,
//         falchionuser,
//         effectairn,
//         soundairn,
//         escapeairslide
//     );
// }