use super::*;
use crate::fighters::common::opff::common_fighter_frame;

//GA - Sitrus and Chesto berries
// type: cancel
//  allows puff to wake early after landing rest, and heals 10%
//   cpu is allowed to use it
unsafe extern "C" fn  purin_galeforce_attack(fighter: &mut L2CFighterCommon) {    
    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) && !VarModule::is_flag(fighter.battle_object, commons::instance::flag::DO_ONCE) {
        VarModule::on_flag(fighter.battle_object, commons::instance::flag::DO_ONCE);
        galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
        DamageModule::heal(fighter.module_accessor, -10.0, 0);
    }
    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) && MotionModule::frame(fighter.module_accessor) > 61.0 {
            MotionModule::set_frame(fighter.module_accessor, 170.0, false);
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
            VarModule::on_flag(fighter.battle_object, commons::instance::flag::DO_ONCE);
        }
    }
    else {
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::DO_ONCE);
    }
}

unsafe extern "C" fn purin_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    purin_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, purin_frame);
}