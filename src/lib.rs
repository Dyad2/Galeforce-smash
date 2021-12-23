#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

mod fighters;
mod ecb_shifts;
mod edge_cancels;
mod utils;
mod var;

use skyline::nro::{self, NroInfo};
use skyline::hooks::{getRegionAddress, Region};
use skyline::nn::ro::LookupSymbol;
use smash::resource::find_subsequence;
use smash::hash40;
use smash::app::{BattleObjectModuleAccessor, sv_battle_object, utility::get_kind, lua_bind::*};
use smash::lib::lua_const::*;

use crate::var::*;
use crate::utils::*; //is_cpu
use crate::fighters::common::FIGHTER_GLOBALS; //galeforce stuff
use crate::fighters::common::TOTAL_FIGHTER; //jostling

pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;
pub static mut FIGHTER_MANAGER_ADDR: usize = 0;

static FLOAT_SEARCH_CODE: &[u8] = &[0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9];
static mut FLOAT_OFFSET : usize = 8;

// static INT_SEARCH_CODE: &[u8] = &[0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9];
// static mut INT_OFFSET : usize = 8;

// static INT64_SEARCH_CODE: &[u8] = &[0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x15, 0x40, 0xf9];
// static mut INT64_OFFSET : usize = 8;


//author: ayerbe
#[skyline::hook(replace = smash::app::lua_bind::WorkModule::on_flag)]
pub unsafe fn on_flag_replace(
boma: &mut smash::app::BattleObjectModuleAccessor,
flag: i32) -> u64 {
    let curr_motion_kind = MotionModule::motion_kind(boma);

    if flag == *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI {
        if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) 
        && (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::get_stick_y(boma) >= 0.7)) {
            return 0;
        }
        else {
            original!()(boma, flag)
        }
    }
    else {
        original!()(boma, flag)
    }
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_replace(
boma: &mut smash::app::BattleObjectModuleAccessor,
term: i32
) -> bool {
    let fighter_kind = smash::app::utility::get_kind(boma);
    let curr_motion_kind = MotionModule::motion_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
    let prev_status_kind_1 = StatusModule::prev_status_kind(boma, 1);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let situation_kind = StatusModule::situation_kind(boma);

    //falling from platforms
    //when hit, check if character is falling off
    if ![*FIGHTER_STATUS_KIND_CATCHED_CUT_GANON, *FIGHTER_STATUS_KIND_CLUNG_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *FIGHTER_STATUS_KIND_CATCHED_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON].contains(&prev_status_kind_1) {
        if StopModule::is_stop(boma) && situation_kind == *SITUATION_KIND_GROUND {
            FIGHTER_GLOBALS[entry_id as usize].push_off_hit = true;
        }
        if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_DAMAGE].contains(&prev_status_kind) && situation_kind == *SITUATION_KIND_AIR && FIGHTER_GLOBALS[entry_id as usize].push_off_hit {
            FIGHTER_GLOBALS[entry_id as usize].push_off_disabled = true;
            FIGHTER_GLOBALS[entry_id as usize].push_off_hit = false;
        }
        if curr_motion_kind == 50017544460 {
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 30 && FIGHTER_GLOBALS[entry_id as usize].push_off_disabled && situation_kind != *SITUATION_KIND_GROUND {
                return false;
            }
            //allow characters to land :)
            else {
                FIGHTER_GLOBALS[entry_id as usize].push_off_disabled = false;
                return true;
            }
        }
    }

    //TODO: can be done as a new motion in motionlists
    if fighter_kind == *FIGHTER_KIND_LUCINA {
        if curr_motion_kind == hash40("landing_fall_special") {
            if FIGHTER_GLOBALS[entry_id as usize].luci_up_b_landing {
                if MotionModule::frame(boma) < 70.0 {
                    return false;
                }
            }
        }
    }
    if FIGHTER_GLOBALS[entry_id as usize].ganon_ga_is_victim && status_kind == *FIGHTER_STATUS_KIND_FALL {
        return false;
    }
    if fighter_kind == *FIGHTER_KIND_BUDDY && FIGHTER_GLOBALS[entry_id as usize].banjump {
        if ![*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON].contains(&term) {
            return false;
        }
    }
    if fighter_kind == *FIGHTER_KIND_ROY && FIGHTER_GLOBALS[entry_id as usize].ga_on && status_kind == *FIGHTER_STATUS_KIND_WAIT {
        if term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U {
            return false;
        }
    }
    if fighter_kind == *FIGHTER_KIND_CHROM {
        if FIGHTER_GLOBALS[entry_id as usize].ga_on && status_kind == *FIGHTER_STATUS_KIND_WAIT {
            if term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR {
                return false;
            }
        }
        if FIGHTER_GLOBALS[entry_id as usize].chrom_dance_disable_time >= 0 {
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S {
                return false;
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_GEKKOUGA {
        if FIGHTER_GLOBALS[entry_id as usize].greninja_ga_xlu > 0 
        && [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR,
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4,
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4,
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4,
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH,
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S].contains(&term) {
            return false;
        }
    }

    //aerial turn
    if !is_operation_cpu(boma) {
        if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_APPEAL_METHOD_INITIATE) && WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_APPEAL_METHOD_DELAY_FRAME) <= 3 {
            if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, 
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW, 
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N, 
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND].contains(&term) {
                return false;
            }
        }
    }

    //special moves disabling
    if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_ALL) && [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW].contains(&term) {
        return false;
    }
    if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI) && term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
        return false;
    }
    if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S) && term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S {
        return false;
    }
    if fighter_kind == *FIGHTER_KIND_MASTER {
        if FIGHTER_GLOBALS[entry_id as usize].ga_on && status_kind == *FIGHTER_STATUS_KIND_FALL {
            if ![*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON].contains(&term) {
                return false;
            }
        }
    }

    //prevents doing some actions before wavedash lag is over
    if curr_motion_kind == hash40("landing_heavy") && FIGHTER_GLOBALS[entry_id as usize].wavedash {
        let pre_correction = MotionModule::frame(boma) * MotionModule::rate(boma);
        let correction_rate = (MotionModule::frame(boma) + 1.0) / pre_correction;
        let corrected_frame = (MotionModule::frame(boma) + 1.0) * correction_rate;
        //println!("adjusted frame: {}", corrected_frame);
        if corrected_frame >= 10. && corrected_frame < 15. {
            if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON].contains(&term) {
                return true;
            }
            else {
                return false;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_FALCO {
        if curr_motion_kind == hash40("escape_air_slide") {
            if MotionModule::frame(boma) <= 1.0 {
                FIGHTER_GLOBALS[entry_id as usize].falco_stick_x = ControlModule::get_stick_x(boma);
                FIGHTER_GLOBALS[entry_id as usize].falco_stick_y = ControlModule::get_stick_y(boma);
            }
            //up
            if FIGHTER_GLOBALS[entry_id as usize].falco_stick_y >= 0.46 && FIGHTER_GLOBALS[entry_id as usize].falco_stick_x.abs() < 0.46 {
                if 93. <= MotionModule::frame(boma) {
                    FIGHTER_GLOBALS[entry_id as usize].falco_dairdodge_faf = true;
                }
            }
            //side
            if FIGHTER_GLOBALS[entry_id as usize].falco_stick_x.abs() >= 0.46 && FIGHTER_GLOBALS[entry_id as usize].falco_stick_y.abs() < 0.46 {
                if 73. <= MotionModule::frame(boma) {
                    FIGHTER_GLOBALS[entry_id as usize].falco_dairdodge_faf = true;
                }
            }
            //down
            if FIGHTER_GLOBALS[entry_id as usize].falco_stick_y <= -0.46 && FIGHTER_GLOBALS[entry_id as usize].falco_stick_x.abs() < 0.46 {
                if 61. <= MotionModule::frame(boma) {
                    FIGHTER_GLOBALS[entry_id as usize].falco_dairdodge_faf = true;
                }
            }
            if !FIGHTER_GLOBALS[entry_id as usize].falco_airdash {
                if FIGHTER_GLOBALS[entry_id as usize].falco_dairdodge_faf {
                    return true;
                }
                else {
                    if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL,
                         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
                         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
                         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND].contains(&term) {
                        return false;
                    }
                }
            }
        }
        else {
            FIGHTER_GLOBALS[entry_id as usize].falco_stick_x = 0.0;
            FIGHTER_GLOBALS[entry_id as usize].falco_stick_y = 0.0;
            FIGHTER_GLOBALS[entry_id as usize].falco_dairdodge_faf = false;
        }
    }

    original!()(boma, term)
}

// samus missile disabling
#[skyline::hook(replace = smash::app::lua_bind::ArticleModule::get_active_num)]
pub unsafe fn get_active_num_replace(
    boma: &mut smash::app::BattleObjectModuleAccessor,
    article_kind: i32,
) -> u64 {
    let fighter_kind = smash::app::utility::get_kind(boma);
    let ret_missile = original!()(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_MISSILE);
    let ret_super = original!()(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_SUPERMISSILE);

    if fighter_kind == *FIGHTER_KIND_SAMUS {
        if ret_missile == 1 || ret_super == 1 {
            return 9; //random high number so another missile won't be generated
        }
    }

    original!()(boma, article_kind)
}

// disabling rolls at the beginning of dash / turn dash. it's probably a param?
#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request_from_script)]
pub unsafe fn change_status_request_from_script_replace(
    boma: &mut smash::app::BattleObjectModuleAccessor,
    status: i32,
    unk: bool
) -> u64 {
    let status_kind = StatusModule::status_kind(boma);

    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_RYU_STATUS_KIND_DASH_BACK, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK].contains(&status_kind) {
        if [*FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&status) {
            ControlModule::clear_command(boma, true);
            return 0;
        }
        else {
            original!()(boma, status, unk)
        }
    }
    else {
        original!()(boma, status, unk)
    }
}

//C stick fixes!
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_attack_air_kind)]
pub unsafe fn get_attack_air_kind_replace(
    boma: &mut smash::app::BattleObjectModuleAccessor,
) -> i32 {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
        //FIGHTER_GLOBALS[entry_id as usize].stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
        //FIGHTER_GLOBALS[entry_id as usize].stick_y = ControlModule::get_stick_y(boma);
        WorkModule::set_float(boma, ControlModule::get_stick_x(boma) * PostureModule::lr(boma), FIGHTER_INSTANCE_WORK_ID_FLOAT_SUBSTICK_X);
        WorkModule::set_float(boma, ControlModule::get_stick_y(boma), FIGHTER_INSTANCE_WORK_ID_FLOAT_SUBSTICK_Y);
    }
    else {
        //FIGHTER_GLOBALS[entry_id as usize].stick_x = 0.0;
        //FIGHTER_GLOBALS[entry_id as usize].stick_y = 0.0;
        WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SUBSTICK_X);
        WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SUBSTICK_Y);
    }

    //if !(FIGHTER_GLOBALS[entry_id as usize].stick_x == 0.0 && FIGHTER_GLOBALS[entry_id as usize].stick_y == 0.0) {
        //uair
    //    if FIGHTER_GLOBALS[entry_id as usize].stick_x.abs() < 0.46 && FIGHTER_GLOBALS[entry_id as usize].stick_y > 0.46 {
    //        FIGHTER_GLOBALS[entry_id as usize].buffered_aerial = *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI;
    //    }
        //fair
    //    if FIGHTER_GLOBALS[entry_id as usize].stick_x > 0.46 && FIGHTER_GLOBALS[entry_id as usize].stick_y.abs() < 0.46 {
    //        FIGHTER_GLOBALS[entry_id as usize].buffered_aerial = *FIGHTER_COMMAND_ATTACK_AIR_KIND_F;
    //    }
        //bair
    //    if FIGHTER_GLOBALS[entry_id as usize].stick_x < -0.46 && FIGHTER_GLOBALS[entry_id as usize].stick_y.abs() < 0.46 {
    //        FIGHTER_GLOBALS[entry_id as usize].buffered_aerial = *FIGHTER_COMMAND_ATTACK_AIR_KIND_B;
    //    }
        //dair
    //    if FIGHTER_GLOBALS[entry_id as usize].stick_x.abs() < 0.46 && FIGHTER_GLOBALS[entry_id as usize].stick_y < -0.46 {
    //        FIGHTER_GLOBALS[entry_id as usize].buffered_aerial = *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW;
    //    }
    //}

    if !(WorkModule::get_float(boma,FIGHTER_INSTANCE_WORK_ID_FLOAT_SUBSTICK_X) == 0.0 && WorkModule::get_float(boma,FIGHTER_INSTANCE_WORK_ID_FLOAT_SUBSTICK_Y) == 0.0) {
        if WorkModule::get_float(boma,FIGHTER_INSTANCE_WORK_ID_FLOAT_SUBSTICK_X).abs() < 0.46 && WorkModule::get_float(boma,FIGHTER_INSTANCE_WORK_ID_FLOAT_SUBSTICK_Y) > 0.46 {
            WorkModule::set_int(boma, *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI, FIGHTER_INSTANCE_WORK_ID_INT_SUBSTICK_AIR_ATTACK);
        }
        //fair
        if WorkModule::get_float(boma,FIGHTER_INSTANCE_WORK_ID_FLOAT_SUBSTICK_X) > 0.46 && WorkModule::get_float(boma,FIGHTER_INSTANCE_WORK_ID_FLOAT_SUBSTICK_Y).abs() < 0.46 {
            WorkModule::set_int(boma, *FIGHTER_COMMAND_ATTACK_AIR_KIND_F, FIGHTER_INSTANCE_WORK_ID_INT_SUBSTICK_AIR_ATTACK);
        }
        //bair
        if WorkModule::get_float(boma,FIGHTER_INSTANCE_WORK_ID_FLOAT_SUBSTICK_X) < -0.46 && WorkModule::get_float(boma,FIGHTER_INSTANCE_WORK_ID_FLOAT_SUBSTICK_Y).abs() < 0.46 {
            WorkModule::set_int(boma, *FIGHTER_COMMAND_ATTACK_AIR_KIND_B, FIGHTER_INSTANCE_WORK_ID_INT_SUBSTICK_AIR_ATTACK);
        }
        //dair
        if WorkModule::get_float(boma,FIGHTER_INSTANCE_WORK_ID_FLOAT_SUBSTICK_X).abs() < 0.46 && WorkModule::get_float(boma,FIGHTER_INSTANCE_WORK_ID_FLOAT_SUBSTICK_Y) < -0.46 {
            WorkModule::set_int(boma, *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW, FIGHTER_INSTANCE_WORK_ID_INT_SUBSTICK_AIR_ATTACK);
        }
    }

    if WorkModule::get_int(boma,FIGHTER_INSTANCE_WORK_ID_INT_SUBSTICK_AIR_ATTACK) != 0 {
        return WorkModule::get_int(boma,FIGHTER_INSTANCE_WORK_ID_INT_SUBSTICK_AIR_ATTACK);
    }

    //if FIGHTER_GLOBALS[entry_id as usize].buffered_aerial != 0 {
        //return FIGHTER_GLOBALS[entry_id as usize].buffered_aerial;
    //}

    original!()(boma)
}

// #[skyline::hook(offset=INT_OFFSET)]
// pub unsafe fn get_param_int_offset_replace(
// boma: u64,
// param_type: u64,
// param_hash: u64) -> i32 {
//     let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
//     let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
//     let ret = original!()(boma, param_type, param_hash);

//     //unused, hook disabled

//     return ret;
// }

#[skyline::hook(offset=FLOAT_OFFSET)]
pub unsafe fn get_param_float_offset_replace(
boma: u64, 
param_type: u64, 
param_hash : u64) -> f32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let curr_motion_kind = MotionModule::motion_kind(module_accessor);

    let ret = original!()(boma, param_type, param_hash);

    if is_fighter(module_accessor) {
        //increases the knockback required to make sakurai angle lift fighters in the air. TODO: test if this does anything?
        if [*FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_SQUAT_F, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_SQUAT_WAIT].contains(&prev_status_kind) {
            if param_type == hash40("battle_object") {
                if param_hash == hash40("damage_angle_ground_reaction_min") {
                    return ret * 1.33; //should be working :) gotta do the math
                }
            }
        }

        if fighter_kind == *FIGHTER_KIND_PURIN && param_hash == 0 && param_type == hash40("weight") {
            if [*FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HIT_END, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD_MAX,
                *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL_AIR, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_TURN].contains(&status_kind) {
                return ret * 2.0;
            }
            else {
                return ret;
            }
        }

        if param_type == hash40("param_private") && param_hash == hash40("super_special_damage") {
            if FIGHTER_GLOBALS[entry_id as usize].terry_allow_super {
                return 100.;
            }
            else {
                return 1000.;
            }
        }

        //add up b landing lag to other stuff
        if fighter_kind == *FIGHTER_KIND_SIMON || fighter_kind == *FIGHTER_KIND_RICHTER {
            if WorkModule::is_flag(module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI) {
                if [hash40("landing_attack_air_frame_hi"), hash40("landing_attack_air_frame_b"), hash40("landing_attack_air_frame_n"), hash40("landing_attack_air_frame_f"), hash40("landing_attack_air_frame_lw"), hash40("landing_frame"), hash40("landing_frame_light")].contains(&param_type) {
                    return ret + 20.0;
                }
            }
        }

        if fighter_kind == *FIGHTER_KIND_SHEIK {
            if !FIGHTER_GLOBALS[entry_id as usize].sheik_heavy_variant && param_type == hash40("landing_attack_air_frame_lw") {
                return 13.0;
            }
        }

        if fighter_kind == *FIGHTER_KIND_MARIOD && WorkModule::get_int(module_accessor, FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_GA_MEDECINE_TIMER) >= 0 {
            if [hash40("air_accel_x_mul"), hash40("air_speed_x_stable"), hash40("walk_speed_max"), hash40("run_speed_max"), hash40("dash_speed")].contains(&param_type) {
                return ret * 1.66;
            }
        }

        //removing jostling for non heavies
        for i in 0 .. TOTAL_FIGHTER {
            let o = i as usize;
            let status_kind_i = StatusModule::status_kind(get_boma(i));
            let curr_motion_kind_i = MotionModule::motion_kind(get_boma(i));
            let fighter_kind_i = get_kind(&mut *sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(i)));
            let mut jostle_on = false;

            if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_FURAFURA_STAND, *FIGHTER_STATUS_KIND_SLEEP, *FIGHTER_STATUS_KIND_BIND, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_TURN].contains(&status_kind_i) 
              || [hash40("attack11"), hash40("attack_100_start"), hash40("attack100"), hash40("attack_100_end"), hash40("attack12"), hash40("attack13"), hash40("attacks3"), hash40("attacks3hi"), hash40("attacks3lw"), hash40("attacks32"), hash40("attacks33"), hash40("attackdash")].contains(&curr_motion_kind_i)
              || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_FURAFURA_STAND, *FIGHTER_STATUS_KIND_SLEEP, *FIGHTER_STATUS_KIND_BIND, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_TURN].contains(&status_kind)
              || [hash40("attack11"), hash40("attack_100_start"), hash40("attack100"), hash40("attack_100_end"), hash40("attack12"), hash40("attack13"), hash40("attacks3"), hash40("attacks3hi"), hash40("attacks3lw"), hash40("attacks32"), hash40("attacks33"), hash40("attackdash")].contains(&curr_motion_kind) {
                jostle_on = true;
            }
            if [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_GANON, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_SNAKE].contains(&fighter_kind) 
              || [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_GANON, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_SNAKE].contains(&fighter_kind_i) {
                jostle_on = true;
            }

            if (PostureModule::pos_x(module_accessor) - PostureModule::pos_x(get_boma(i))).abs() < 14.0 && (PostureModule::pos_y(module_accessor) - PostureModule::pos_y(get_boma(i))).abs() < 20.0 && o != get_player_number(module_accessor) {
                if jostle_on {
                    if param_hash == 0 {
                        // if param_type == hash40("jostle_weight") {
                        //     return 999.0;
                        // }
                        if param_type == hash40("jostle_front") || param_type == hash40("jostle_back") {
                            return ret * 0.25;
                        }
                        // if param_type =
                        // if param_type == hash40("jostle_front") || param_type == hash40("jostle_back") {
                        //     //println!("jostle boundaries");
                        //     return ret * 2.0;
                        // }
                    }
                }
                else {
                    if param_hash == 0 {
                        if param_type == hash40("jostle_front") || param_type == hash40("jostle_back") {
                            return -6.0;
                        }
                    }
                }
            }
        }
    }

    original!()(boma, param_type, param_hash)
}

#[skyline::main(name = "galeforce")]
pub fn main() {
    unsafe {
        // LookupSymbol(
        //     &mut FIGHTER_MANAGER,
        //     "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}".as_bytes().as_ptr(),
        // );
        LookupSymbol(
            &mut FIGHTER_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}".as_bytes().as_ptr(),
        );
        LookupSymbol(
            &mut FIGHTER_CUTIN_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}".as_bytes().as_ptr(),
        );
    //status
    //nro::add_hook(nro_main).unwrap();

        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);

        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
        // if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
        //     INT_OFFSET = offset;
        // }
        // if let Some(offset) = find_subsequence(text, INT64_SEARCH_CODE) {
        //     INT64_OFFSET = offset;
        // }
    }

    //hooks
    skyline::install_hook!(is_enable_transition_term_replace); //many GAs, some special moves disabling
    skyline::install_hook!(on_flag_replace); //short hop macro removal
    skyline::install_hook!(get_attack_air_kind_replace); //c stick fix
    skyline::install_hook!(change_status_request_from_script_replace); //Dash roll disabling
    skyline::install_hook!(get_active_num_replace); //Dash roll disabling
    //skyline::install_hook!(pass_floor_replace); //platform dairdodge up snap, doesnt work
    //skyline::install_hook!(is_enable_cancel_replace);
    skyline::install_hook!(get_param_float_offset_replace);
    //skyline::install_hook!(get_param_int_offset_replace);

    //code edits
    fighters::install();
}