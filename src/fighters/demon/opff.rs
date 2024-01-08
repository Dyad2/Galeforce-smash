use super::*;
use crate::fighters::common::opff::common_fighter_frame;

//directional inputs
//static mut DIR3 : [i32; 9] = [0; 9];
//static mut DIR2 : [i32; 9] = [0; 9];
static mut STICK_DIR : [i32; 9] = [0; 9];
static mut COMMAND_FRAME : [i32; 9] = [0; 9];
static mut INPUT_IS_COMMAND : [bool; 9] = [false; 9];
static mut IS_COMMAND_FAILED : [bool; 9] = [false; 9];

use galeforce_utils::{vars::*, table_const::*, utils::*};
use custom_var::*;

unsafe extern "C" fn EWGF_simulator_frame(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    //let cat4 = ControlModule::get_command_flag_cat(fighter.module_accessor, 3);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    
    //flash tornado attack cancel
    if curr_motion_kind == hash40("attack_stand_5") && fighter.global_table[MOTION_FRAME].get_i32() <= 3 && (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    || ControlModule::is_enable_flick_jump(fighter.module_accessor) && ControlModule::get_stick_y(fighter.module_accessor) > 0.7 && ControlModule::get_flick_y(fighter.module_accessor) < 3) {
        L2CFighterCommon::change_status_jump_mini_attack(fighter, 0.into());
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    
    //TODO: This is probably not necessary anymore, lol?
    //abolishing fist anim is stacked over oni front kick
    // if curr_motion_kind == hash40("attack_s3_s") && MotionModule::frame(fighter.module_accessor) >= 61.0 {
    //     StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
    // }
    if curr_motion_kind == 68500721045 && MotionModule::frame(fighter.module_accessor) <= 1.0 {
        if !INPUT_IS_COMMAND[entry_id as usize] {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("abolishing_fist"), 1.0, 1.0, 1.0);
        }
    }
    
    //Tsunami kick input - 632A
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND 
      && [*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_WALK, *FIGHTER_STATUS_KIND_WALK_BRAKE, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK].contains(&status_kind) {
        if get_stick_dir(&mut *fighter.module_accessor) == 6 && VarModule::is_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE) {
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE);
            STICK_DIR[entry_id as usize] = 6;
            COMMAND_FRAME[entry_id as usize] = 10;
        }
        if STICK_DIR[entry_id as usize] == 6 {
            if get_stick_dir(&mut *fighter.module_accessor) == 3 {
                STICK_DIR[entry_id as usize] = 3;
                COMMAND_FRAME[entry_id as usize] = 10;
            }
            else if get_stick_dir(&mut *fighter.module_accessor) != 6 {
                IS_COMMAND_FAILED[entry_id as usize] = true;
            }
        }
        if STICK_DIR[entry_id as usize] == 3 {
            if get_stick_dir(&mut *fighter.module_accessor) == 2 {
                STICK_DIR[entry_id as usize] = 2;
                COMMAND_FRAME[entry_id as usize] = 10;
            }
            else if get_stick_dir(&mut *fighter.module_accessor) != 3 {
                IS_COMMAND_FAILED[entry_id as usize] = true;
            }
        }
        if STICK_DIR[entry_id as usize] == 2 {
            if get_stick_dir(&mut *fighter.module_accessor) != 2 && get_stick_dir(&mut *fighter.module_accessor) != 5 {
                IS_COMMAND_FAILED[entry_id as usize] = true;
            }
            else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                INPUT_IS_COMMAND[entry_id as usize] = true;
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_3, false);
            }
        }
        //cleanup
        if COMMAND_FRAME[entry_id as usize] > 0 {
            COMMAND_FRAME[entry_id as usize] -= 1;
        }
        else {
            IS_COMMAND_FAILED[entry_id as usize] = true;
        }
        if IS_COMMAND_FAILED[entry_id as usize] {
            STICK_DIR[entry_id as usize] = 0;
            INPUT_IS_COMMAND[entry_id as usize] = false;
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE);
            IS_COMMAND_FAILED[entry_id as usize] = false;
        }
    }
}

unsafe extern "C" fn demon_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, demon_frame);
}