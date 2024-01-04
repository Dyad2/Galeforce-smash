use super::*;
use crate::fighters::common::opff::common_fighter_frame;

//GA - Winning Road
// type: cancel
//  allows roy to cancel some grounded attacks with up taunt, gaining armor to bait counter attacks and gaining HP
unsafe extern "C" fn roy_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    
    if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3].contains(&status_kind) {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            VarModule::set_float(fighter.battle_object, roy::instance::float::GALEFORCE_ATTACK_TIMER, MotionModule::frame(fighter.module_accessor));
            VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) && MotionModule::frame(fighter.module_accessor) >= VarModule::get_float(fighter.battle_object, roy::instance::float::GALEFORCE_ATTACK_TIMER) + 10.0 {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U);
                //WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_APPEAL, false);
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                VarModule::set_float(fighter.battle_object, roy::instance::float::GALEFORCE_ATTACK_TIMER, 999.0);
            }
        }
    }
    else if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
        let damage_info = DamageModule::start_damage_info_log(fighter.module_accessor) as *mut smash::app::DamageInfo;
        if DamageModule::check_no_reaction(fighter.module_accessor, damage_info) == 1 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    else {
        VarModule::set_float(fighter.battle_object, roy::instance::float::GALEFORCE_ATTACK_TIMER, 999.0);
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
}

unsafe extern "C" fn roy_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    roy_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, roy_frame);
}