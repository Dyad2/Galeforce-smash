use super::*;
use crate::fighter::common::opff::common_fighter_frame;

//Sonic GA
// jump cancel down air on hit when jump is held / pressed in hitlag
unsafe extern "C" fn sonic_galeforce_attack(fighter: &mut L2CFighterCommon) {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") 
      && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
      && check_jump_input(fighter.module_accessor) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
        if fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
            galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
        }
    }
}

unsafe extern "C" fn miisword_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    sonic_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, miisword_frame);
}