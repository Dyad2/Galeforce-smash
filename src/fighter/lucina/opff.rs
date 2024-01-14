use super::*;
use crate::fighter::common::opff::common_fighter_frame;

unsafe extern "C" fn lucina_aether(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    
    //please make status for those holy
    if curr_motion_kind == hash40("special_hi") {
        VarModule::on_flag(fighter.module_accessor, marcina::instance::flag::LUCINA_SPECIAL_HI_LANDING);
        AttackModule::clear_all(fighter.module_accessor);
        if MotionModule::frame(fighter.module_accessor) >= 12. {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_2"), 13.0, 1.0, 0.0, false, false);
        }
    }
    else if curr_motion_kind == hash40("special_hi_2") {
        if MotionModule::frame(fighter.module_accessor) >= 40. {
            let speedhi2 = smash::phx::Vector3f { x: 0.085 * ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor), y: 0., z: 0.0 };
            KineticModule::add_speed(fighter.module_accessor, &speedhi2);
            KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
        if MotionModule::frame(fighter.module_accessor) >= 62. && MotionModule::frame(fighter.module_accessor) < 63. {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_3"), 63.0, 1.0, 0.0, false, false);
        }
    }
    else if curr_motion_kind == hash40("special_hi_3") {
        let speedhi3 = smash::phx::Vector3f { x: 0.085 * ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor), y: 0., z: 0.0 };
        KineticModule::add_speed(fighter.module_accessor, &speedhi3);
        if MotionModule::frame(fighter.module_accessor) >= 65. {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_3"), 63.0, 1.0, 0.0, true, false);
        }
    }
    if curr_motion_kind == hash40("landing_fall_special") && VarModule::is_flag(fighter.module_accessor, marcina::instance::flag::LUCINA_SPECIAL_HI_LANDING) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_aether"), 1.0, 1.0, false, 0.0, false, false);
    }
    if ![hash40("special_hi"), hash40("special_hi_2"), hash40("special_hi_3"), hash40("landing_fall_special"), hash40("landing_aether"), 51563581894].contains(&curr_motion_kind) {
        VarModule::off_flag(fighter.module_accessor, marcina::instance::flag::LUCINA_SPECIAL_HI_LANDING);
    }
    if is_status_damage(&mut *fighter.module_accessor) {
        macros::AFTER_IMAGE_OFF(fighter, 1);
    }
}

//GA - Exalt's Exertion
// type: cancel
//  Hits side b 3 (any angle) and cancel it with a wavedash. Disables side b for s short time.
unsafe extern "C" fn lucina_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);

    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3].contains(&status_kind) { //hoping the common status doesnt work for s4
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
    }
    else {
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) && !StopModule::is_stop(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                VarModule::set_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER, 360);
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
            }
        }
        else if fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor) >= 0.1 && ControlModule::get_flick_x(fighter.module_accessor) < 5 {
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
            VarModule::set_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER, 360);
            galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
        }
    }
    if VarModule::get_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER) >= 0 {
        chrom_disable_dance_effect(fighter);
        VarModule::sub_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER, 1);
    }
}

unsafe extern "C" fn lucina_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    lucina_galeforce_attack(fighter);
    lucina_aether(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, lucina_frame);
}