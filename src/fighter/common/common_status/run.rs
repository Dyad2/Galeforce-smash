use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_Run_Sub)]
unsafe extern "C" fn status_run_sub(fighter: &mut L2CFighterCommon) {
    let value: f32 = if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_DASH || fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_TURN {
        WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_RUN_WORK_FLOAT_START_FRAME)
    } else {
        0.0
    };
 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("run"), value, 1.0, false, 0.0, false, false);
    
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_GEKIKARA_RUN_BRAKE);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW);
}

#[skyline::hook(replace = L2CFighterCommon_status_Run_Main)]
unsafe extern "C" fn status_run_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U) && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0)
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW) && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0)
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) && (fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0 || fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0))
    {
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_APPEAL, false);
    }
    
    if ControlModule::get_stick_y(fighter.module_accessor) < -0.66 {
        if GroundModule::is_passable_ground(fighter.module_accessor) {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_PASS, false);
        }
        else {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_SQUAT, false);
        }
    }

    call_original!(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_status_Run)]
unsafe extern "C" fn status_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_run_sub(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_run_main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_RunBrake)]
unsafe fn status_runbrake(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_RunBrake();
    fighter.sub_shift_status_main(L2CValue::Ptr(status_runbrake_main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_RunBrake_Main)]
unsafe extern "C" fn status_runbrake_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U) && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0)
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW) && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0)
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S)  && (fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0 || fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0))
    {
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_APPEAL, false);
    }
    
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), false.into());
    }
    
    if ControlModule::get_stick_y(fighter.module_accessor) < -0.66 {
        if GroundModule::is_passable_ground(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT.into(), false.into());
        }
    }

    call_original!(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_run_sub,
            status_run_main,
            status_run,
            status_runbrake,
            status_runbrake_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}