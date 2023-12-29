use super::*;
use crate::fighters::common::opff::common_fighter_frame;

unsafe extern "C" fn pikachu_reenable_side_special(fighter: &mut L2CFighterCommon) {
    if is_special_reset(&mut *fighter.module_accessor) {
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::DISABLE_SPECIAL_S);
    }
}

//GA: Quick Attack Cancel
// if pikachu hits with either up b 1 or 2, it can cancel the landing with a jump.
//  only works on the ground.
unsafe extern "C" fn pikachu_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let status_kind_prev = StatusModule::prev_status_kind(fighter.module_accessor, 0);    
    
    if status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL && status_kind_prev == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END && VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
        //WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS);
        if check_jump_input(fighter.module_accessor) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
            if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() || fighter.sub_transition_group_check_ground_jump().get_bool() {
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
            }
        }
    }
    if ![*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&status_kind) {
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
}

unsafe extern "C" fn miisword_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    pikachu_reenable_side_special(fighter);
    pikachu_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, miisword_frame);
}