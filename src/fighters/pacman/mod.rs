use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

//global edits
#[acmd_script( agent = "pacman", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "pacman", script = "game_turndash", category = ACMD_GAME, low_priority)]
unsafe fn turndash(fighter: &mut L2CAgentBase) {
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
#[acmd_script( agent = "pacman", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn attack11(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 1.5, 0.0, 7.5, 5.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 25, 0, 20, 1.7, 0.0, 7.5, 8.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 180, 15, 0, 20, 2.0, 0.0, 7.5, 11.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 361, 15, 0, 20, 2.0, 0.0, 7.5, 11.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 4.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 4.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 4.0, false);
        }
    wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
}

#[acmd_script( agent = "pacman", script = "game_attack12", category = ACMD_GAME, low_priority )]
unsafe fn attack12(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 35, 0, 35, 2.5, 0.0, 7.5, 1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 35, 0, 30, 2.8, 0.0, 7.5, 4.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 30, 0, 30, 3.0, 0.0, 7.5, 8.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
}

#[acmd_script( agent = "pacman", script = "game_attack13", category = ACMD_GAME, low_priority )]
unsafe fn attack13(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 106, 0, 45, 4.5, 0.0, 8.0, 7.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 106, 0, 45, 5.5, 0.0, 8.0, 13.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter)
    {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pacman", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.25);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
            macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 6.5, 87, 50, 0, 43, 5.5, 2.9, -0.5, -0.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 6.5, 87, 50, 0, 43, 3.2, 0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_NORMAL);
            MotionModule::set_rate(fighter.module_accessor, 1.2);
            AttackModule::clear_all(fighter.module_accessor);
        }
}

#[acmd_script( agent = "pacman", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn attacklw3(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 32, 50, 0, 65, 4.5, 0.0, 4.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 0.75);
        }
}

#[acmd_script( agent = "pacman", script = "game_attackhi4", category = ACMD_GAME, low_priority )]
unsafe fn attackhi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
    frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 3.0, 120, 100, 125, 0, 4.0, 0.0, 8.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        }
    wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
    frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("ghost1"), 14.0, 83, 97, 0, 32, 6.2, 0.0, 5.8, 0.0, Some(0.0), Some(5.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        }
    wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("ghost1"), 8.0, 83, 92, 0, 32, 4.5, 0.0, 5.8, 0.0, Some(0.0), Some(5.3), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        }
    wait(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

//others
#[acmd_script( agent = "pacman", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
unsafe fn escapeairslide(fighter: &mut L2CAgentBase) {
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

pub fn install() {
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        attack11,
        attack12,
        attack13,
        attackhi3,
        attacklw3,
        attackhi4,
        escapeairslide
    );
}