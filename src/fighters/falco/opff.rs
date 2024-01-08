use super::*;
use crate::fighters::common::opff::common_fighter_frame;

//GA - Bird of prey
// type: buff / cancel
//  Hitting an opponent with reflector without reflecting a projectile gives falco an airdash
unsafe extern "C" fn falco_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    
    if !is_operation_cpu(fighter.module_accessor) {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
            }
            //remove GA is falco reflects (FIXME: doesn't do anything)
            if ReflectModule::is_reflect(fighter.module_accessor) {
                VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                VarModule::off_flag(fighter.module_accessor, falco::instance::flag::AIRDASH);
            }
        }
        //interrupts an aerial with airdash
        if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            zelda_buff_effect(fighter);
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            && (ControlModule::get_stick_x(fighter.module_accessor).abs() >= 0.1 || ControlModule::get_stick_y(fighter.module_accessor).abs() >= 0.1)
            && !is_hitlag(fighter.module_accessor) && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
            && situation_kind == *SITUATION_KIND_AIR 
            && !CancelModule::is_enable_cancel(fighter.module_accessor) 
            && status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                MotionModule::change_motion_kind(fighter.module_accessor, smash::phx::Hash40{hash: hash40("escape_air_slide")});
                VarModule::on_flag(fighter.module_accessor, falco::instance::flag::AIRDASH);
                VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.66);
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
            //interrupts an airdodge with an aerial
            if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) && (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.66);
                VarModule::on_flag(fighter.module_accessor, falco::instance::flag::AIRDASH);
                VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            }
        }
        if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&situation_kind) {
            VarModule::off_flag(fighter.module_accessor, falco::instance::flag::AIRDASH);
        }
    }
}

//Fast fall at any point during down special
unsafe extern "C" fn falco_speciallw_fastfall(fighter: &mut L2CFighterCommon) {
    if !is_operation_cpu(fighter.module_accessor) {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW && fighter.global_table[MOTION_FRAME].get_i32() >= 4 && (ControlModule::get_command_flag_cat(fighter.module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }
}

unsafe extern "C" fn elight_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    falco_speciallw_fastfall(fighter);
    falco_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, elight_frame);
}