//The wavedash code is modified from HDR and mostly not mine

#![allow(unused_must_use)]

use {
    super::*,
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    galeforce_utils::vars::*,
};

#[skyline::hook(replace = L2CFighterCommon_status_pre_EscapeAir)]
unsafe extern "C" fn status_EscapeAir_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
	println!("status_EscapeAir_pre");

    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::WAVEDASH) {
        GroundModule::attach_ground(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }

    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, 0, 0, 0, 0);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_EscapeAir)]
unsafe extern "C" fn status_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
	println!("status_EscapeAir");

    //TODO: re-check vanilla script for identical behavior.
    ControlModule::reset_trigger(fighter.module_accessor);
    fighter.sub_escape_air_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_slide"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air"), 0.0, 1.0, false, 0.0, false, false);
    }
    //airdodge staling? 
    // let mut motion_rate = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_FLOAT_MOTION_RATE_PENALTY);
    // let start_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
    // if 0 < start_frame {
    //     let intan_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_XLU_FRAME);
    //     let add_xlu_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
    //     motion_rate = 1.0 / ((intan_frame as f32) / ((intan_frame - add_xlu_frame) as f32));
    // }
    // MotionModule::set_rate(fighter.module_accessor, motion_rate);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(status_EscapeAir_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_EscapeAir_Main)]
unsafe extern "C" fn status_EscapeAir_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
	println!("status_EscapeAir_Main");

    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::WAVEDASH) {
        return 1.into();
    }
    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_escape_air_common_main)]
unsafe extern "C" fn sub_escape_air_common_main(fighter: &mut L2CFighterCommon) -> L2CValue {
	println!("sub_escape_air_common_main");

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return true.into();
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
      || (!fighter.sub_wait_ground_check_common(false.into()).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        if fighter.sub_escape_air_common_strans_main().get_bool() {
            return true.into();
        }
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return true.into();
        }
        if !MotionModule::is_end(fighter.module_accessor) {
            return false.into();
        } 
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    return true.into();
}

#[skyline::hook(replace = L2CFighterCommon_sub_escape_air_common_strans_main)]
unsafe extern "C" fn sub_escape_air_common_strans_main(fighter: &mut L2CFighterCommon) -> L2CValue {
	println!("sub_escape_air_common_strans_main");

    let escape_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
    let escape_throw_item_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("escape_throw_item_frame"));
    
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW)
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && ItemModule::is_have_item(fighter.module_accessor, 0)
    && {
      fighter.clear_lua_stack();
      smash_script::lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
      smash::app::sv_module_access::item(fighter.lua_state_agent);
      !fighter.pop_lua_stack(1).get_bool()
    }
    && escape_frame <= escape_throw_item_frame
    && !fighter.can_entry_cliff_air_lasso().get_bool() {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return 1.into();
    }

    let air_lasso_type = WorkModule::get_param_int(fighter.module_accessor, hash40("air_lasso_type"), 0);
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO)
    && air_lasso_type != *FIGHTER_AIR_LASSO_TYPE_NONE
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && !LinkModule::is_link(fighter.module_accessor, *FIGHTER_LINK_NO_CONSTRAINT) {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return 1.into();
    }

    let trigger_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("air_escape_passive_trigger_frame")) as f32;
    let passive_trigger_frame_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("passive_trigger_frame_mul"), 0);
    let passive_frame = trigger_frame as f32 * passive_trigger_frame_mul;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();

    let passive_fb_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value"));
    if situation_kind == *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB)
            && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
            && passive_fb_cont_value <= fighter.global_table[STICK_X].get_f32().abs()
            && (escape_frame as f32) < passive_frame {
                fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_FB.into(), true.into());
                return 1.into();
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE)
            && situation_kind == *SITUATION_KIND_GROUND
            && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
            && (escape_frame as f32) < passive_frame {
                fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE.into(), false.into());
                return 1.into();
            }
        }
        //new
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::WAVEDASH);
        }
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 1.into();
        //
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_AIR) {
        let jump_trigger_count = ControlModule::get_trigger_count(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP as u8);
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32)
        && ((jump_trigger_count & 0xff) as f32) < passive_frame
        && (escape_frame as f32) < passive_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP.into(), true.into());
            return 1.into();
        }

        let jump_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y"));
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32)
        && jump_stick_y <= fighter.global_table[STICK_Y].get_f32()
        && (escape_frame as f32) < passive_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP.into(), true.into());
            return 1.into();
        }

        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32)
        && (escape_frame as f32) < passive_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL.into(), false.into());
            return 1.into();
        }

        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32)
        && (escape_frame as f32) < passive_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_CEIL.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_EscapeAir)]
unsafe extern "C" fn status_EscapeAir_end(fighter: &mut L2CFighterCommon) -> L2CValue {
	println!("status_EscapeAir_end");
    
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_LANDING {
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::WAVEDASH);
    }
    call_original!(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_EscapeAir_pre,
            status_EscapeAir,
            status_EscapeAir_Main,
            sub_escape_air_common_main,
            sub_escape_air_common_strans_main,
            status_EscapeAir_end,
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}