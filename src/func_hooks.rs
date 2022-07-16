use {
    custom_var::*,
    skyline::{
        //nro::{
            //self,
            //NroInfo
        //},
        hooks::{
            getRegionAddress,
            Region
        },
    },
    smash::{
        hash40,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    galeforce_utils::{
        table_const::*,
        utils::*,
        vars,
        vars::*
    },
    crate::ecb_shifts::ecb_shifts
};

#[skyline::from_offset(0x3ac540)]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

//author: wuboy! custom_var stuff. resets status vars when status changes
#[skyline::hook( replace = StatusModule::init_settings )]
pub unsafe fn init_settings_replace(
    boma: *mut BattleObjectModuleAccessor,
    situation: SituationKind,
    kinetic: i32,
    correct: u32,
    cliff_check: GroundCliffCheckKind,
    jostle: bool,
    keep_flag: i32,
    keep_int: i32,
    keep_float: i32,
    arg10: i32,
) {
    let mut mask = 0;
    if keep_flag == *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG {
        mask += VarModule::RESET_STATUS_FLAG;
    }
    if keep_int == *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT {
        mask += VarModule::RESET_STATUS_INT;
        mask += VarModule::RESET_STATUS_INT64;
    }
    if keep_float == *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT {
        mask += VarModule::RESET_STATUS_FLOAT;
    }
    let object_id = (*boma).battle_object_id;
    let object = get_battle_object_from_id(object_id);
    VarModule::reset(object, mask);

    // teleports fighters on landing
    // Because the aerial ECB shift code currently runs in opff, it runs a frame "late"
    // which causes characters to appear stuck halfway into the ground on the first frame they land
    // so we need to re-shift your character back up to the proper height on that single frame
    if is_fighter(boma) {
        if !((&[
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_CAPTURE_CUT,
            *FIGHTER_STATUS_KIND_THROWN
        ]).contains(&StatusModule::prev_status_kind(boma, 1))
        && !(&[
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_CAPTURE_CUT,
            *FIGHTER_STATUS_KIND_ENTRY,
            *FIGHTER_STATUS_KIND_THROWN,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_BURY,
            *FIGHTER_STATUS_KIND_BURY_WAIT
        ]).contains(&StatusModule::prev_status_kind(boma, 0))
        && !(WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR | *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)))
        && VarModule::get_float(object, vars::commons::instance::float::ECB_OFFSET_Y) != 0.0 {
            if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                if StatusModule::prev_situation_kind(boma) != *SITUATION_KIND_GROUND {
                    let fighter_pos = smash::phx::Vector3f {x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma) + VarModule::get_float(object, vars::commons::instance::float::ECB_OFFSET_Y), z: PostureModule::pos_z(boma)};
                    PostureModule::set_pos(boma, &fighter_pos);
                }
            }
        }
    }

    original!()(boma, situation, kinetic, correct, cliff_check, jostle, keep_flag, keep_int, keep_float, arg10)
}

//author: ayerbe
#[skyline::hook(replace = smash::app::lua_bind::WorkModule::on_flag)]
pub unsafe fn on_flag_replace(
boma: &mut smash::app::BattleObjectModuleAccessor,
flag: i32) -> u64 {
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

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::get_float)]
pub unsafe fn get_float_replace(
boma: &mut smash::app::BattleObjectModuleAccessor,
flag: i32) -> f32 {
    let object_id = boma.battle_object_id;
    let object = get_battle_object_from_id(object_id);
    let fighter_kind = smash::app::utility::get_kind(boma);

    if fighter_kind == *FIGHTER_KIND_LUCARIO {
        if [*FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_DEBUG_AURAPOWER, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_EFFECT_SCALE, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_PREV_AURAPOWER, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER].contains(&flag) && VarModule::get_int(object, lucario::instance::int::MAX_AURA_TIMER) > 0 {
            return 1.33
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
    let ret = original!()(boma, term);
    let object_id = boma.battle_object_id;
    let object = get_battle_object_from_id(object_id);
    let module_accessor = (*object).module_accessor;
    let curr_motion_kind = MotionModule::motion_kind(module_accessor);
    let status_kind = StatusModule::status_kind(module_accessor);
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    let prev_status_kind_1 = StatusModule::prev_status_kind(module_accessor, 1);
    let situation_kind = StatusModule::situation_kind(module_accessor);

    //falling from platforms
    //when hit, check if character is falling off
    if ![*FIGHTER_STATUS_KIND_CATCHED_CUT_GANON, *FIGHTER_STATUS_KIND_CLUNG_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *FIGHTER_STATUS_KIND_CATCHED_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON].contains(&prev_status_kind_1) {
        if StopModule::is_stop(module_accessor) && situation_kind == *SITUATION_KIND_GROUND {
            VarModule::on_flag(object,commons::instance::flag::PLATFORM_FALL_STUN);
        }
        if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_DAMAGE].contains(&prev_status_kind) && situation_kind == *SITUATION_KIND_AIR && VarModule::is_flag(object,commons::instance::flag::PLATFORM_FALL_STUN) {
            VarModule::off_flag(object,commons::instance::flag::PLATFORM_FALL_STUN);
        }
        if curr_motion_kind == 50017544460 {
            if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 30 && situation_kind != *SITUATION_KIND_GROUND {
                return false;
            }
            //allow characters to land :)
            else {
                return ret;
            }
        }
    }
    
    if fighter_kind == *FIGHTER_KIND_ROY && VarModule::is_flag(object, commons::instance::flag::GALEFORCE_ATTACK_ON) && status_kind == *FIGHTER_STATUS_KIND_WAIT {
        if term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U {
            return false;
        }
    }
    else if fighter_kind == *FIGHTER_KIND_CHROM {
        if VarModule::is_flag(object, commons::instance::flag::GALEFORCE_ATTACK_ON) && status_kind == *FIGHTER_STATUS_KIND_WAIT {
            if term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR {
                return false;
            }
        }
        if VarModule::get_int(object, commons::instance::int::FRAME_COUNTER) >= 0 {
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S {
                return false;
            }
        }
    }
    else if fighter_kind == *FIGHTER_KIND_GEKKOUGA {
        if VarModule::get_int(object, commons::instance::int::FRAME_COUNTER) > 0 
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
    else if fighter_kind == *FIGHTER_KIND_FALCO {
        //TODO: update FAF?
        if curr_motion_kind == hash40("escape_air_slide") {
            if MotionModule::frame(module_accessor) <= 1.0 {
                VarModule::set_float(object, falco::instance::float::STICK_X, ControlModule::get_stick_x(module_accessor));
                VarModule::set_float(object, falco::instance::float::STICK_Y, ControlModule::get_stick_y(module_accessor));
            }
            //up
            if VarModule::get_float(object, falco::instance::float::STICK_Y) >= 0.46 && VarModule::get_float(object, falco::instance::float::STICK_X).abs() < 0.46 {
                if 93. <= MotionModule::frame(module_accessor) {
                    VarModule::on_flag(object, falco::instance::flag::DIRECTIONAL_AIR_ESCAPE_FAF);
                }
            }
            //side
            if VarModule::get_float(object, falco::instance::float::STICK_X).abs() >= 0.46 && VarModule::get_float(object, falco::instance::float::STICK_Y).abs() < 0.46 {
                if 73. <= MotionModule::frame(module_accessor) {
                    VarModule::on_flag(object, falco::instance::flag::DIRECTIONAL_AIR_ESCAPE_FAF);
                }
            }
            //down
            if VarModule::get_float(object, falco::instance::float::STICK_Y) <= -0.46 && VarModule::get_float(object, falco::instance::float::STICK_X).abs() < 0.46 {
                if 61. <= MotionModule::frame(module_accessor) {
                    VarModule::on_flag(object, falco::instance::flag::DIRECTIONAL_AIR_ESCAPE_FAF);
                }
            }
            if !VarModule::is_flag(object, falco::instance::flag::AIRDASH) {
                if VarModule::is_flag(object, falco::instance::flag::DIRECTIONAL_AIR_ESCAPE_FAF) {
                    return ret;
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
            VarModule::set_float(object, falco::instance::float::STICK_X, 0.0);
            VarModule::set_float(object, falco::instance::float::STICK_Y, 0.0);
            VarModule::off_flag(object, falco::instance::flag::DIRECTIONAL_AIR_ESCAPE_FAF);
        }
    }
    else if fighter_kind == *FIGHTER_KIND_BAYONETTA && situation_kind == *SITUATION_KIND_AIR && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) < 4 {
        if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND].contains(&term) {
            return false;
        }
    }

    //aerial turn
    if VarModule::is_flag(object,commons::instance::flag::AIR_TURN_APPEAL_METHOD_INITIATE) && VarModule::get_int(object,commons::instance::int::AIR_TURN_INPUT_FRAME) <= 3 {
        if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND].contains(&term) {
            return false;
        }
    }

    //special moves disabling
    if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW].contains(&term) && VarModule::is_flag(object,commons::instance::flag::DISABLE_SPECIAL_ALL) {
        return false;
    }
    else if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI && VarModule::is_flag(object,commons::instance::flag::DISABLE_SPECIAL_HI) {
        return false;
    }
    else if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S && VarModule::is_flag(object,commons::instance::flag::DISABLE_SPECIAL_S) {
        return false;
    }

    //prevents doing some actions before wavedash lag is over
    if curr_motion_kind == hash40("landing_heavy") && VarModule::is_flag(object,commons::instance::flag::WAVEDASH) {
        if get_fighter_common_from_accessor(&mut *module_accessor).global_table[MOTION_FRAME].get_f32() >= 10. {
        //if MotionModule::frame(module_accessor) >= 10. {
            if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON].contains(&term) {
                return true;
            }
            else {
                return false;
            }
        }
    }
    //Dash roll disabling, should be done in status script but uuuh lazy
    if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_RYU_STATUS_KIND_DASH_BACK, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK].contains(&status_kind) {
        if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE].contains(&term) {
            return false;
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

    if fighter_kind == *FIGHTER_KIND_SAMUS && article_kind != *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB { //<-- without this samus can't spawn bombs with missiles out (needs testing)
        if ret_missile == 1 || ret_super == 1 {
            return 9; //random high number so another missile won't be generated
        }
    }

    original!()(boma, article_kind)
}

//C stick fixes!
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_attack_air_kind)]
pub unsafe fn get_attack_air_kind_replace(
    boma: &mut smash::app::BattleObjectModuleAccessor,
) -> i32 {
    let object_id = boma.battle_object_id;
    let object = get_battle_object_from_id(object_id);

    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
        VarModule::set_float(object, commons::instance::float::SUBSTICK_X, ControlModule::get_stick_x(boma) * PostureModule::lr(boma));
        VarModule::set_float(object, commons::instance::float::SUBSTICK_Y, ControlModule::get_stick_y(boma));
    }
    else {
        VarModule::set_float(object, commons::instance::float::SUBSTICK_X, 0.0);
        VarModule::set_float(object, commons::instance::float::SUBSTICK_Y, 0.0);
    }

    if !(VarModule::get_float(object, commons::instance::float::SUBSTICK_X) == 0.0 && VarModule::get_float(object, commons::instance::float::SUBSTICK_Y) == 0.0) {
        if VarModule::get_float(object, commons::instance::float::SUBSTICK_X).abs() < 0.46 && VarModule::get_float(object,commons::instance::float::SUBSTICK_Y) > 0.46 {
            VarModule::set_int(object, commons::instance::int::SUBSTICK_AIR_ATTACK, *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI);
        }
        //fair
        if VarModule::get_float(object, commons::instance::float::SUBSTICK_X) > 0.46 && VarModule::get_float(object, commons::instance::float::SUBSTICK_Y).abs() < 0.46 {
            VarModule::set_int(object, commons::instance::int::SUBSTICK_AIR_ATTACK, *FIGHTER_COMMAND_ATTACK_AIR_KIND_F,);
        }
        //bair
        if VarModule::get_float(object, commons::instance::float::SUBSTICK_X) < -0.46 && VarModule::get_float(object, commons::instance::float::SUBSTICK_Y).abs() < 0.46 {
            VarModule::set_int(object, commons::instance::int::SUBSTICK_AIR_ATTACK, *FIGHTER_COMMAND_ATTACK_AIR_KIND_B);
        }
        //dair
        if VarModule::get_float(object, commons::instance::float::SUBSTICK_X).abs() < 0.46 && VarModule::get_float(object, commons::instance::float::SUBSTICK_Y) < -0.46 {
            VarModule::set_int(object, commons::instance::int::SUBSTICK_AIR_ATTACK, *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW);
        }
    }

    if VarModule::get_int(object,  commons::instance::int::SUBSTICK_AIR_ATTACK) != 0 {
        return VarModule::get_int(object, commons::instance::int::SUBSTICK_AIR_ATTACK);
    }

    original!()(boma)
}

// #[skyline::hook(offset=INT_OFFSET)]
// pub unsafe fn get_param_int_offset_replace(
// boma: u64,
// param_type: u64,
// param_hash: u64) -> i32 {
//     let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
//     let ret = original!()(boma, param_type, param_hash);

//     //unused, hook disabled

//     return ret;
// }

#[skyline::hook(offset = FLOAT_OFFSET)]
pub unsafe fn get_param_float_offset_replace(
boma: u64, 
param_type: u64, 
param_hash : u64) -> f32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let object_id = module_accessor.battle_object_id;
    let object = get_battle_object_from_id(object_id);
    let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(&mut *module_accessor);
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);

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

        //modifies gravity while in directional airdodges to use the LSI gravity average (a bit higher actually)
        if VarModule::is_flag(object,commons::instance::flag::ESCAPE_AIR_IS_SLIDE) {
            if param_type == hash40("air_accel_y") && !WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL) {
                //return 0.086;
                return 0.1;
            }
        }

        if [*FIGHTER_KIND_POPO, *FIGHTER_KIND_NANA].contains(&fighter_kind) && (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
            //println!("link_no: {}", )
            //let partner_object = get_battle_object_from_id(*FIGHTER_POPO_INSTANCE_WORK_ID_INT_PARTNER_OBJECT_ID);
            if param_hash == hash40("nana_opt_dst") {
                println!("control!");
                return 999.0;
            }
        }

        //slows down dash speed for characters faster than steve, who has the worst dash speed. faster characters receive a bigger penalty
        //move this to status when u know how :)
        if param_type == hash40("dash_speed") {
            let dash_speed = ret;
            let reduce = dash_speed - 1.45;
            let slower_dash = 1.45 + (reduce * 0.67);

            return slower_dash;
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
        //add up b landing lag to other stuff
        else if fighter_kind == *FIGHTER_KIND_SIMON || fighter_kind == *FIGHTER_KIND_RICHTER {
            if VarModule::is_flag(object, commons::instance::flag::DISABLE_SPECIAL_HI) {
                if [hash40("landing_attack_air_frame_hi"), hash40("landing_attack_air_frame_b"), hash40("landing_attack_air_frame_n"), hash40("landing_attack_air_frame_f"), hash40("landing_attack_air_frame_lw"), hash40("landing_frame"), hash40("landing_frame_light")].contains(&param_type) {
                    return ret + 20.0;
                }
            }
        }
        else if fighter_kind == *FIGHTER_KIND_SHEIK {
            if !VarModule::is_flag(object, sheik::instance::flag::ATTACK_AIR_LW_S) && param_type == hash40("landing_attack_air_frame_lw") {
                return 13.0;
            }
        }
        else if fighter_kind == *FIGHTER_KIND_MARIOD && VarModule::get_int(object, mariod::instance::int::GA_MEDECINE_TIMER) >= 0 {
            if [hash40("air_accel_x_mul"), hash40("air_speed_x_stable"), hash40("walk_speed_max"), hash40("run_speed_max"), hash40("dash_speed")].contains(&param_type) {
                return ret * 1.66;
            }
        }
    }

    original!()(boma, param_type, param_hash)
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

pub fn install() {
    unsafe{
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, vars::FLOAT_SEARCH_CODE) {
            vars::FLOAT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, vars::INT_SEARCH_CODE) {
            vars::INT_OFFSET = offset;
        }
        // if let Some(offset) = find_subsequence(text, vars::NOTIFY_LOG_EVENT_COLLISION_HIT_SEARCH_CODE) {
        //     vars::NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        // }
    }
    skyline::install_hooks!(
        init_settings_replace,
        is_enable_transition_term_replace, //many GAs, some special moves disabling
        on_flag_replace, //short hop macro removal
        get_float_replace,
        get_attack_air_kind_replace, //c stick fix
        //change_status_request_from_script_replace, //Dash roll disabling
        get_active_num_replace, //Samus missile stuff
        get_param_float_offset_replace,
    );
}