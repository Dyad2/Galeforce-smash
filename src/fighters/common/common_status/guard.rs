use super::*;

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon42FighterStatusGuard__landing_effect_controlEv")]
pub unsafe fn landing_effect_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_LANDING_EFFECT_FRAME);
    
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_LANDING_EFFECT_FRAME) <= 0 {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_guardlandingeffect"), -1);
        WorkModule::set_int(fighter.module_accessor, 8, *FIGHTER_STATUS_GUARD_ON_WORK_INT_LANDING_EFFECT_FRAME);
    }
    L2CValue::I32(0)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon17sub_guard_on_uniqEN3lib8L2CValueE")]
pub unsafe fn sub_guard_on_uniq(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    if !arg.get_bool() {
        landing_effect_control(fighter);
    } else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_LOCK) {
            let shield_dec1 = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_dec1"));
            let shield_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_frame"), 0);
            let shield_dec_rate = shield_dec1 / shield_frame;
            WorkModule::sub_float(fighter.module_accessor, shield_dec_rate, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        }
        let shield = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        let minimum_shield = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MIN);
        if shield < minimum_shield {
            WorkModule::set_float(fighter.module_accessor, minimum_shield, * FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MIN);
        }
        let min_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        if 0 < min_frame {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        }
        let catch_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
        if 0 < catch_frame {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
        } else {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
        }
    }
    L2CValue::I32(0)
}

//the following code is mine
//this status used to check min shield hold time, it was removed. shield drop was added.
#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon24status_guard_main_commonEv")]
unsafe fn status_guard_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let somebool;

    if WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD) > 0.0 {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), true.into());
                somebool = true;
                return somebool.into();
            }
            else {
                if (ControlModule::get_stick_y(fighter.module_accessor) < -0.33 && ControlModule::get_flick_y(fighter.module_accessor) <= 3)
                  && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
                  && GroundModule::is_passable_ground(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
                }
            }
        }
        somebool = false;
    }
    else {
        somebool = true;
        fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY.into(), false.into());
    }
    return somebool.into();
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon17status_Guard_MainEv")]
unsafe fn status_Guard_Main(fighter: &mut L2CFighterCommon) -> L2CValue {

    if !fighter.status_guard_main_common_air().get_bool() {
        if !fighter.sub_guard_cont().get_bool() {
            fighter.status_guard_main_common();
        }
    }
    return 0.into();
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon23sub_status_guard_commonEv")]
unsafe fn sub_status_guard_common(fighter: &mut L2CFighterCommon) {
    fighter.sub_guard_cont_pre();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_guard_on_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_guard_on_uniq as *const () as _));
}

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon12status_GuardEv")]
unsafe fn status_Guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_guard_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(status_Guard_Main as *const () as _))
}

pub fn install() {
    install_hooks!(
        status_guard_main_common,
        status_Guard_Main,
        sub_guard_on_uniq,
        sub_status_guard_common
    );
    install_status_scripts!(
        status_Guard
    );
}