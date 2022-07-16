use super::*;
use smash::lua2cpp::{L2CFighterCommon, *};
//use galeforce_utils::table_const::*;
use smashline::*;

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon17status_DashCommonEv")]
unsafe fn status_DashCommon(fighter: &mut L2CFighterCommon) {

    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS);
    //WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW);

    call_original!(fighter)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon16status_Dash_MainEv")]
unsafe extern "C" fn status_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_dash_main_common(fighter, 0.into());
    0.into()
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon23status_Dash_Main_commonEN3lib8L2CValueE")]
unsafe extern "C" fn status_dash_main_common(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {

    if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U) && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0)
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW) && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0)
    || (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) && (fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0 || fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0))
    {
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_APPEAL, false);
    }
    
    if ControlModule::get_stick_y(fighter.module_accessor) < -0.6875 { //the literal is common - pass_stick_y
        if GroundModule::is_passable_ground(fighter.module_accessor) {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_PASS, false);
        }
        // else {
        //     StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_SQUAT, false);
        // }
    }

    call_original!(fighter, arg)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon15status_Dash_SubEv")]
unsafe fn status_Dash_sub(fighter: &mut L2CFighterCommon) {
    let value: f32 = if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_LANDING || fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_LANDING_LIGHT {
        6.0
    } else {
        0.0
    };

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("dash"), 0.0, 1.0, false, value, false, false);
    status_DashCommon(fighter);
}

#[common_status_script(status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon11status_DashEv")]
unsafe fn status_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_Dash_sub(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_dash_main as *const () as _))
}

pub fn install() {
    install_hooks!(
        status_DashCommon,
        status_Dash_sub,
        status_dash_main,
        status_dash_main_common
    );
    install_status_scripts!(
        status_dash
    );
}