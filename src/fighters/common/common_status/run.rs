use super::*;

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon14status_Run_SubEv")]
unsafe fn status_run_sub(fighter: &mut L2CFighterCommon) {
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

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon15status_Run_MainEv")]
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

#[common_status_script(status = FIGHTER_STATUS_KIND_RUN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon10status_RunEv")]
unsafe fn status_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_run_sub(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_run_main as *const () as _))
}

#[common_status_script(status = FIGHTER_STATUS_KIND_RUN_BRAKE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon15status_RunBrakeEv")]
unsafe fn status_runbrake(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_RunBrake();
    fighter.sub_shift_status_main(L2CValue::Ptr(status_runbrake_main as *const () as _))
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon20status_RunBrake_MainEv")]
unsafe fn status_runbrake_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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

pub fn install() {
    install_hooks!(
        status_run_sub,
        status_run_main,
        status_runbrake_main
    );
    install_status_scripts!(
        status_run,
        status_runbrake,
    );
}