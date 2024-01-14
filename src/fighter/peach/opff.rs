use super::*;
use crate::fighter::common::opff::common_fighter_frame;

unsafe extern "C" fn peach_delayed_float_from_ground(fighter: &mut L2CFighterCommon) {
    //delayed float
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) >= 4 || StatusModule::prev_status_kind(fighter.module_accessor, 0) != *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            VarModule::on_flag(fighter.module_accessor, peach::instance::flag::ALLOW_FLOAT);
            //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT);
            //WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_ENABLE_UNIQ);
        }
        else {
            VarModule::off_flag(fighter.module_accessor, peach::instance::flag::ALLOW_FLOAT);
            //WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_ENABLE_UNIQ);
        }
    }
}

//GA - 
// type: cancel
//  allows to pull a turnip after landing forward throw
unsafe extern "C" fn peach_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);    
    
    if curr_motion_kind == hash40("throw_f") && fighter.global_table[MOTION_FRAME].get_i32() >= 20 {
        //VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
        }
    }
    else {
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
}

unsafe extern "C" fn peach_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    peach_delayed_float_from_ground(fighter);
    peach_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, peach_frame);
}