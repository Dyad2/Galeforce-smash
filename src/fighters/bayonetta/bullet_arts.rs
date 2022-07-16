use std::arch::asm;
use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::sv_battle_object;
use smash::lua2cpp::L2CAgentBase;
use smash::{phx::Vector3f, lua2cpp::L2CFighterCommon};
use smash::app::sv_animcmd::*;
use smash::app::sv_animcmd;
use smashline::*;
use smash_script::*;

use galeforce_utils::vars::*;
use custom_var::*;

//attacks
#[acmd_script( agent = "bayonetta", script = "game_shootingarml_atkon_attack11", category = ACMD_GAME, low_priority)]
unsafe fn shootingarml_atkon_attack11(fighter: &mut L2CAgentBase) {
    let _boma = fighter.module_accessor; //the smash script line needs this or else it doesn't compile. immutable borrow stuff

    if macros::is_excute(fighter)
    {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
            macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 0, Hash40::new("top"), 1.0, 361, 80, 0, 10, 2.5, 0.0, 10.5, 9.0, Some(0.0), Some(10.5), Some(69.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        }
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 10.5, 9.0, Some(0.0), Some(10.5), Some(69.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        smash_script::notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, WorkModule::get_int64(_boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 8, 4);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_shootingarml_atkoff_attack11", category = ACMD_GAME, low_priority)]
unsafe fn shootingarml_atkoff_attack11(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
            AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00) as i32, false);
        }
        AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01) as i32, false);
    }
}

//specials
#[acmd_script( agent = "bayonetta", scripts = ["game_shootingarml_atkon_specialhi", "game_shootingarml_atkon_specialairhi"], category = ACMD_GAME, low_priority)]
unsafe fn shooting_on_specialhi(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        VarModule::on_flag(fighter.battle_object,bayonetta::status::flag::SPECIAL_HI_SHOOT);
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 1, Hash40::new("top"), 0.5, 361, 0, 0, 0, 11.0, 0.0, 11.0, 0.0, Some(0.0), Some(11.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
    }
}

#[acmd_script( agent = "bayonetta", scripts = ["game_shootingarml_atkoff_specialhi", "game_shootingarml_atkoff_specialairhi"], category = ACMD_GAME, low_priority)]
unsafe fn shooting_off_specialhi(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01) as i32, false);
    }
}

//other
#[acmd_script( agent = "bayonetta", script = "game_shootinglegl_atkon_escapeb", category = ACMD_GAME, low_priority)]
unsafe fn shootonescapeblegl(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor;

    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 0, Hash40::new("footl"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(68.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        smash_script::notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 8, 4);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_shootinglegr_atkon_escapeb", category = ACMD_GAME, low_priority)]
unsafe fn shootonescapeblegr(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor; //the smash script line needs this or else it doesn't compile. immutable borrow stuff

    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 1, Hash40::new("footr"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(68.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        smash_script::notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 8, 4);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_shootingarml_atkon_escapef", category = ACMD_GAME, low_priority)]
unsafe fn shootonescapefarmrl(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor; //the smash script line needs this or else it doesn't compile. immutable borrow stuff

    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 1, Hash40::new("footl"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(68.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        smash_script::notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 16, 4);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_shootingarmr_atkon_escapef", category = ACMD_GAME, low_priority)]
unsafe fn shootonescapefarmr(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor; //the smash script line needs this or else it doesn't compile. immutable borrow stuff

    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 1, Hash40::new("footr"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(68.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        smash_script::notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 8, 4);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        shootingarml_atkon_attack11,shootingarml_atkoff_attack11,

        shooting_on_specialhi,shooting_off_specialhi,

        shootonescapeblegl,
        shootonescapeblegr,

        shootonescapefarmrl,
        shootonescapefarmr,
    );
}