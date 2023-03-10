use super::*;
use galeforce_utils::table_const::*;

#[common_status_script(status = FIGHTER_STATUS_KIND_TURN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon11status_TurnEv")]
unsafe fn status_turn(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_pre_turncommon(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_turn_main as *const () as _))
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon21status_pre_TurnCommonEv")]
unsafe extern "C" fn status_pre_turncommon(fighter: &mut L2CFighterCommon) {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_TURN_ATTACK_S4_REV_PAD);

    //removed in galeforce:
    //WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
    
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("turn"), 0.0, 1.0, false, 0.0, false, false);
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon16status_Turn_MainEv")]
unsafe extern "C" fn status_turn_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[TURN_UNIQ].get_ptr());
    if fighter.global_table[TURN_UNIQ].get_bool() && callable(fighter).get_bool() {
        return 1.into();
    }
    
    let turn_work_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_TURN_WORK_FLOAT_LR);
    let dash_stick_x: f32 = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dash_stick_x"));
    let stick_x = fighter.global_table[STICK_X].get_f32();

    if stick_x == 0.0 {
        VarModule::off_flag(fighter.battle_object, commons::status::flag::DISABLE_BACKDASH);
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && stick_x * -1.0 * turn_work_lr < dash_stick_x // checks if stick is below dash threshold
    && VarModule::is_flag(fighter.battle_object, commons::instance::flag::SMASH_TURN)
    && VarModule::is_flag(fighter.battle_object, commons::instance::flag::ALLOW_PERFECT_PIVOT)
    && StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_DASH
    && MotionModule::frame(fighter.module_accessor) <= 1.0 {
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::ALLOW_PERFECT_PIVOT);
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::SMASH_TURN);
        let pivot_boost = -1.75 * PostureModule::lr(fighter.module_accessor);
        fighter.clear_lua_stack();
        smash_script::lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, pivot_boost);
        smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }
    if !status_turncommon(fighter).get_bool() {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0
            || fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0
            || VarModule::is_flag(fighter.battle_object, commons::instance::flag::SMASH_TURN) {
                if stick_x * turn_work_lr >= dash_stick_x && VarModule::is_flag(fighter.battle_object, commons::instance::flag::ALLOW_PERFECT_PIVOT) {
                    if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                        VarModule::on_flag(fighter.battle_object, commons::instance::flag::SMASH_TURN);
                        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN, false);
                    }
                }
                if !VarModule::is_flag(fighter.battle_object, commons::status::flag::DISABLE_BACKDASH) && stick_x * -1.0 * turn_work_lr >= dash_stick_x {
                    if MotionModule::frame(fighter.module_accessor) >= 1.0 {
                        VarModule::off_flag(fighter.battle_object, commons::instance::flag::SMASH_TURN);
                        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_DASH, false);
                    }
                }
            }
        }
        return 0.into();
    }
    1.into()
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon17status_TurnCommonEv")]
unsafe extern "C" fn status_turncommon(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if MotionModule::is_end(fighter.module_accessor) {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
            }
        }
        fighter.check_turn_attack_s4_pad_rev();
        if !fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
            if !fighter.sub_transition_group_check_ground_item().get_bool() {
                if !fighter.sub_transition_group_check_ground_catch().get_bool() {
                    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_FLAG_NO_TURN_TO_ESCAPE) {
                            if !fighter.sub_transition_group_check_ground_escape().get_bool() {
                                if !fighter.sub_transition_group_check_ground_guard().get_bool() {
                                    if !fighter.sub_transition_group_check_ground_special().get_bool() {
                                        if !fighter.sub_transition_group_check_ground_attack().get_bool() {
                                            if !fighter.sub_transition_group_check_ground_jump().get_bool() {
                                                if !fighter.sub_transition_group_check_ground(false.into()).get_bool() {
                                                    return false.into();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            return true.into();
                        }
                    }
                    if !fighter.sub_transition_group_check_ground_escape().get_bool() {
                        if !fighter.sub_transition_group_check_ground_guard().get_bool() {
                            if !fighter.sub_transition_group_check_ground_special().get_bool() {
                                if !fighter.sub_transition_group_check_ground_attack().get_bool() {
                                    if !fighter.sub_transition_group_check_ground_jump().get_bool() {
                                        if !fighter.sub_transition_group_check_ground(false.into()).get_bool() {
                                            return false.into();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    else {
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
    }
    return true.into()
}

#[common_status_script(status = FIGHTER_STATUS_KIND_TURN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon15status_end_TurnEv")]
unsafe fn status_end_turn(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, commons::instance::flag::SMASH_TURN) {
        VarModule::off_flag(fighter.battle_object, commons::status::flag::DISABLE_BACKDASH);
    }
    VarModule::off_flag(fighter.battle_object, commons::instance::flag::SMASH_TURN);
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_DASH {
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::ALLOW_PERFECT_PIVOT);
    }
    fighter.sub_exit_Turn();
    0.into()
}

pub fn install() {
    install_hooks!(
        status_pre_turncommon,
        status_turn_main,
        status_turncommon,
    );

    install_status_scripts!(
        status_turn,
        status_end_turn,
    );
}