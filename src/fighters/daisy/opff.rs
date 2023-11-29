use super::*;
use crate::fighters::common::opff::common_fighter_frame;

pub unsafe extern "C" fn daisy_galeforce_attack(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    
    //jump cancel her up b?
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
    else if status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END && VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
        if fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
            galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
        }
    }
    else {
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
}

unsafe extern "C" fn daisy_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    daisy_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, daisy_frame);
}