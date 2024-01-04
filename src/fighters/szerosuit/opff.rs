use super::*;
use crate::fighters::common::opff::common_fighter_frame;

//GA - ?
// type: cancel
//  allows szerosuit to airdodge out of flip kick auto hit. since it no longer burries, this allows her to perform follow ups.
unsafe extern "C" fn  szerosuit_galeforce_attack(fighter: &mut L2CFighterCommon) {
    if !is_operation_cpu(fighter.module_accessor) {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw_flip") {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
            }
            if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                StopModule::cancel_hit_stop(fighter.module_accessor);
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.66);
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
            }
            if MotionModule::frame(fighter.module_accessor) >= 10. {
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
            }
        }
        else {
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
    }
}

unsafe extern "C" fn szerosuit_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    szerosuit_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, szerosuit_frame);
}

