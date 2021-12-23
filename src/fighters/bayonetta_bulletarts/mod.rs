use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::sv_battle_object;
use smash::lua2cpp::L2CAgentBase;
use smash::{phx::Vector3f, lua2cpp::L2CFighterCommon};
use smash::app::sv_animcmd::*;
use smash::app::sv_animcmd;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;

    //escape B
    //on
#[acmd_script( agent = "bayonetta", script = "game_shootinglegl_atkon_escapeb", category = ACMD_GAME, low_priority)]
unsafe fn shootonescapeblegl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            if !WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
                macros::ATTACK(fighter, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00 as u64, 0, Hash40::new("footl"), 0.5, 361, 100, 0, 10, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(12.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
            }
            macros::ATTACK(fighter, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01 as u64, 0, Hash40::new("footl"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(68.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
            smash_script::notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, 16, 4);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_shootinglegr_atkon_escapeb", category = ACMD_GAME, low_priority)]
unsafe fn shootonescapeblegr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        if macros::is_excute(fighter)
        {
            if !WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
                macros::ATTACK(fighter, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_02 as u64, 1, Hash40::new("footr"), 0.5, 361, 100, 0, 10, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(12.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
            }
            macros::ATTACK(fighter, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_03 as u64, 1, Hash40::new("footr"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(68.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
            smash_script::notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_03, 8, 4);
        }
}

    //off
#[acmd_script( agent = "bayonetta", script = "game_shootinglegl_atkoff_escapeb", category = ACMD_GAME, low_priority)]
unsafe fn shootoffescapeblegl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
        if macros::is_excute(fighter)
        {
            if !WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
                AttackModule::clear(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00, false);
            }
            AttackModule::clear(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, false);
        }
}

#[acmd_script( agent = "bayonetta", script = "game_shootinglegr_atkoff_escapeb", category = ACMD_GAME, low_priority)]
unsafe fn shootoffescapeblegr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
        if macros::is_excute(fighter)
        {
            if !WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
                AttackModule::clear(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_02, false);
            }
            AttackModule::clear(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_03, false);
        }
}


pub fn install() {
    smashline::install_acmd_scripts!(
        shootonescapeblegl,
        shootonescapeblegr,
        shootoffescapeblegl,
        shootoffescapeblegr,
    );
}