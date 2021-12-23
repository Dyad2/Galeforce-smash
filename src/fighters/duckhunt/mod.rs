use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd::*;
use smash::app::sv_animcmd;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;

//global edits
#[acmd_script( agent = "duckhunt", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "duckhunt", script = "game_turndash", category = ACMD_GAME, low_priority)]
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

//ground
#[acmd_script( agent = "duckhunt", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 30, 0, 20, 2.3, 0.0, 4.0, 6.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 361, 30, 0, 20, 2.3, 0.0, 4.0, 8.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 180, 20, 0, 20, 2.5, 0.0, 4.0, 11.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 361, 20, 0, 20, 2.5, 0.0, 4.0, 11.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
}

#[acmd_script( agent = "duckhunt", script = "game_attack12", category = ACMD_GAME, low_priority)]
unsafe fn attack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 5.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 30, 0, 30, 3.5, 0.0, 3.5, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 361, 25, 0, 25, 5.5, 0.0, 7.5, 5.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        }
}

#[acmd_script( agent = "duckhunt", script = "game_attack100end", category = ACMD_GAME, low_priority)]
unsafe fn attack100end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 5.);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 140, 0, 70, 6.0, 0.0, 5.5, 6.0, Some(0.0), Some(7.0), Some(17.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        }
    wait(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
}


// #[acmd_func(
//     battle_object_category =  BATTLE_OBJECT_CATEGORY_FIGHTER, battle_object_kind = FIGHTER_KIND_DUCKHUNT, animation = "attack_100", animcmd = "game_attack100")]
// pub fn attack100(fighter: &mut L2CFighterCommon) {
//     acmd!({ 
//             if macros::is_excute(fighter)
//             {
//                 FighterAreaModuleImpl::enable_fix_jostle_area(4.5, 0.5)
//                 macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 361, 15, 0, 16, 4.5, 0.0, 7.5, 4.5, None, None, None, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//                 macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 6.5, 0.0, 7.5, 14.0, Some(0.0, Some(7.5, Some(6.5, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//                 AttackModule::set_add_reaction_frame(ID=0, lua_state, 2, Unk=false)
//                 AttackModule::set_add_reaction_frame(ID=1, lua_state, 2, Unk=false)
//                 ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1,stunMul=5)
//             }
//             wait(lua_state, 1)
//             if macros::is_excute(fighter){
//             AttackModule::clear_all(boma);
//             WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
//             }
//             wait_loop_clear(0)
//             FT_MOTION_RATE(FSM=1)
//             frame(lua_state, 2)
//             if macros::is_excute(fighter){
//             FighterAreaModuleImpl::enable_fix_jostle_area(4.5, 0.5)
//             macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 361, 15, 0, 16, 4.5, 0.0, 7.5, 4.5, None, None, None, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//             macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 6.5, 0.0, 7.5, 14.0, Some(0.0, Some(7.5, Some(6.5, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//             AttackModule::set_add_reaction_frame(ID=0, lua_state, 2, Unk=false)
//             AttackModule::set_add_reaction_frame(ID=1, lua_state, 2, Unk=false)
//             ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1,stunMul=5)
//             }
//             wait(lua_state, 1)
//             if macros::is_excute(fighter){
//             AttackModule::clear_all(boma);
//             WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
//             }
//             frame(0, 4)
//             if macros::is_excute(fighter){
//             FighterAreaModuleImpl::enable_fix_jostle_area(4.5, 0.5)
//             macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 361, 15, 0, 16, 4.5, 0.0, 7.5, 4.5, None, None, None, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//             macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 6.5, 0.0, 7.5, 14.0, Some(0.0, Some(7.5, Some(6.5, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//             AttackModule::set_add_reaction_frame(ID=0, lua_state, 2, Unk=false)
//             AttackModule::set_add_reaction_frame(ID=1, lua_state, 2, Unk=false)
//             ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1,stunMul=5)
//             }
//             wait(lua_state, 1)
//             if macros::is_excute(fighter){
//             AttackModule::clear_all(boma);
//             WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
//             }
//             frame(0, 6)
//             if macros::is_excute(fighter){
//             FighterAreaModuleImpl::enable_fix_jostle_area(4.5, 0.5)
//             macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 361, 15, 0, 16, 4.5, 0.0, 7.5, 4.5, None, None, None, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//             macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 6.5, 0.0, 7.5, 14.0, Some(0.0, Some(7.5, Some(6.5, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//             AttackModule::set_add_reaction_frame(ID=0, lua_state, 2, Unk=false)
//             AttackModule::set_add_reaction_frame(ID=1, lua_state, 2, Unk=false)
//             ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1,stunMul=5)
//             }
//             wait(lua_state, 1)
//             if macros::is_excute(fighter){
//             AttackModule::clear_all(boma);
//             WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
//             }
//             frame(0, 8)
//             if macros::is_excute(fighter){
//             FighterAreaModuleImpl::enable_fix_jostle_area(4.5, 0.5)
//             macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 361, 15, 0, 16, 4.5, 0.0, 7.5, 4.5, None, None, None, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//             macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 6.5, 0.0, 7.5, 14.0, Some(0.0, Some(7.5, Some(6.5, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//             AttackModule::set_add_reaction_frame(ID=0, lua_state, 2, Unk=false)
//             AttackModule::set_add_reaction_frame(ID=1, lua_state, 2, Unk=false)
//             ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1,stunMul=5)
//             }
//             wait(lua_state, 1)
//             if macros::is_excute(fighter){
//             AttackModule::clear_all(boma);
//             WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
//             }
//             frame(0, 10)
//             if macros::is_excute(fighter){
//             FighterAreaModuleImpl::enable_fix_jostle_area(4.5, 0.5)
//             macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 361, 15, 0, 16, 4.5, 0.0, 7.5, 4.5, None, None, None, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//             macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 6.5, 0.0, 7.5, 14.0, Some(0.0, Some(7.5, Some(6.5, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//             AttackModule::set_add_reaction_frame(ID=0, lua_state, 2, Unk=false)
//             AttackModule::set_add_reaction_frame(ID=1, lua_state, 2, Unk=false)
//             ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1,stunMul=5)
//             }
//             wait(lua_state, 1)
//             if macros::is_excute(fighter){
//             AttackModule::clear_all(boma);
//             WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
//             }
//             frame(0, 12)
//             if macros::is_excute(fighter){
//             FighterAreaModuleImpl::enable_fix_jostle_area(4.5, 0.5)
//             macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 361, 15, 0, 16, 4.5, 0.0, 7.5, 4.5, None, None, None, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//             macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 6.5, 0.0, 7.5, 14.0, Some(0.0, Some(7.5, Some(6.5, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//             AttackModule::set_add_reaction_frame(ID=0, lua_state, 2, Unk=false)
//             AttackModule::set_add_reaction_frame(ID=1, lua_state, 2, Unk=false)
//             ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1,stunMul=5)
//             }
//             wait(lua_state, 1)
//             if macros::is_excute(fighter){
//             AttackModule::clear_all(boma);
//             WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
//             }
//             frame(0, 14)
//             if macros::is_excute(fighter){
//             FighterAreaModuleImpl::enable_fix_jostle_area(4.5, 0.5)
//             macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 361, 15, 0, 16, 4.5, 0.0, 7.5, 4.5, None, None, None, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//             macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 12, 6.5, 0.0, 7.5, 14.0, Some(0.0, Some(7.5, Some(6.5, 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD)
//             AttackModule::set_add_reaction_frame(ID=0, lua_state, 2, Unk=false)
//             AttackModule::set_add_reaction_frame(ID=1, lua_state, 2, Unk=false)
//             ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1,stunMul=5)
//             }
//             wait(lua_state, 1)
//             if macros::is_excute(fighter){
//             AttackModule::clear_all(boma);
//             WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK)
//             }
//             frame(0, 16)
//     });
// }

#[acmd_script( agent = "duckhunt", script = "game_attackhi3", category = ACMD_GAME, low_priority)]
unsafe fn attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        frame(lua_state, 1.);
            if macros::is_excute(fighter)
            {
                macros::FT_MOTION_RATE(fighter, 0.8);
            }
        frame(lua_state, 8.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 85, 30, 0, 95, 6.0, 0.0, 15.0, 0.0, Some(0.0), Some(11.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
                macros::FT_MOTION_RATE(fighter, 1.0);
            }
        wait(lua_state, 5.);
            if macros::is_excute(fighter)
            {
                AttackModule::clear_all(boma);
            }
        frame(lua_state, 24.);
            if macros::is_excute(fighter)
            {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    CancelModule::enable_cancel(boma);
                }
            }
}

//air
#[acmd_script( agent = "duckhunt", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 1.5, 367, 90, 0, 0, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false,0, 0.0, 6, false, false, false,false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 1.5, 100, 90, 60, 0, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false,0, 0.0, 6, false, false, false,false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 1.5, 367, 90, 0, 0, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false,0, 0.0, 6, false, false, false,false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 3, 0, Hash40::new("kneel"), 1.5, 100, 90, 60, 0, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false,0, 0.0, 6, false, false, false,false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 4, 0, Hash40::new("armr"), 1.5, 367, 90, 0, 0, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false,0, 0.0, 6, false, false, false,false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 5, 0, Hash40::new("armr"), 1.5, 100, 90, 60, 0, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false,0, 0.0, 6, false, false, false,false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 6, 0, Hash40::new("arml"), 1.5, 367, 90, 0, 0, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false,0, 0.0, 6, false, false, false,false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 7, 0, Hash40::new("arml"), 1.5, 100, 90, 60, 0, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false,0, 0.0, 6, false, false, false,false, true, *COLLISION_SITUATION_MASK_G , *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    frame(lua_state, 35.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 36.);
        if macros::is_excute(fighter)
        {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 86, 0, 50, 7.0, 0.0, 10.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false,0, 0.0, 0, false, false, false,false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    frame(lua_state, 38.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(boma);
        }
    frame(lua_state, 44.);
        if macros::is_excute(fighter)
        {
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}

//special
#[acmd_script( agent = "duckhunt", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_REQUEST_SPECIAL_HI_CANCEL);
        }
}


#[acmd_script( agent = "duckhunt", script = "game_specialairhi", category = ACMD_GAME, low_priority)]
unsafe fn specialairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_REQUEST_SPECIAL_HI_CANCEL);
        }
}


//other
#[acmd_script( agent = "duckhunt", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
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
        attack11,
        attack12,
        attack100end,
        attackhi3,
        attackairn,
        specialhi,
        specialairhi,
        escapeairslide
    );
}