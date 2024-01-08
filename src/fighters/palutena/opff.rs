use super::*;
use crate::fighters::common::opff::common_fighter_frame;

//GA - Divine lightstep
// type: cancel
//  allows palu to cancel dash attack with a jump
unsafe extern "C" fn palutena_galeforce_attack_frame(fighter: &mut L2CFighterCommon) {
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            if (ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
                fighter.sub_transition_group_check_ground_jump_mini_attack();
                fighter.sub_transition_group_check_ground_jump();
                galeforce_apply_effect(&mut *fighter.module_accessor, 1.0);
                VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
            }
        }
    }
    else {
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
}

//moving up tilt
unsafe extern "C" fn palutena_attackhi3_move_frame(fighter: &mut L2CFighterCommon) {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_hi3") && fighter.global_table[MOTION_FRAME].get_i32() < 45 && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        let palu_attackhi3_speed = smash::phx::Vector3f { x: 0.12 * ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor), y: 0., z: 0.0 };
        let x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    
        if x_vel.abs() < 0.5 && !fighter.global_table[IS_STOP].get_bool() {
            KineticModule::add_speed(fighter.module_accessor, &palu_attackhi3_speed);
        }
    }
}

//summon board test!
unsafe extern "C" fn palutena_summon_board_frame(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if [hash40("special_lw"), hash40("special_air_lw")].contains(&curr_motion_kind) && fighter.global_table[MOTION_FRAME].get_i32() <= 5 && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CHANGE_STATUS_REFLECT);
        fighter.change_status(FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_REFLECT.into(), false.into());
    }
}

unsafe extern "C" fn palutena_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    palutena_galeforce_attack_frame(fighter);
    palutena_attackhi3_move_frame(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, palutena_frame);
}
