use super::*;

pub unsafe fn run(fighter : &mut L2CFighterCommon, status_kind: i32, situation_kind: i32) {
    //let status_prev = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    let fighter_kind = get_kind(&mut *fighter.module_accessor);

    //required to snap to platforms
    if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        if fighter.global_table[MOTION_FRAME].get_i32() > 2 {
            GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(1));
        }
        else {
            GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    //hold shield to stay on ground while wavedashing
    else if VarModule::is_flag(fighter.battle_object, commons::instance::flag::WAVEDASH) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::set_correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    //shield push, land cancel when not in landing lag, other edge cancel stuff
    else if [*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_WALK].contains(&status_kind) 
      || (!WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL) && status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR)
      || (status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE && KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE).abs() > 0.95) {
        GroundModule::set_correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }

    //character specific cancels
    if fighter_kind == *FIGHTER_KIND_DIDDY {
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            if MotionModule::frame(fighter.module_accessor) >= 5.0 && MotionModule::frame(fighter.module_accessor) <= 20.0 {
                GroundModule::set_correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
            else {
                GroundModule::set_correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            }
        }
    }
    else if fighter_kind == *FIGHTER_KIND_MASTER && [*FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_LANDING, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT_DASH].contains(&status_kind) {
        GroundModule::set_correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    else if fighter_kind == *FIGHTER_KIND_SZEROSUIT && status_kind == *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_START {
        GroundModule::set_correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    //breaks specialhi2. try status instead
    // else if fighter_kind == *FIGHTER_KIND_PIKACHU && *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END == status_kind { // && fighter.global_table[MOTION_FRAME].get_f32() > 42.0 { MotionModule::end_frame(fighter.module_accessor) -10.0
    //     GroundModule::set_correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    // }
}