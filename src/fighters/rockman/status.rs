use super::*;

//up tilt
unsafe extern "C" fn rocky_attackhi3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {

    let mask = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_HI3 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64;

    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_GROUND), 
        *FIGHTER_KINETIC_TYPE_MOTION, 
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false,
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        false, 
        mask,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32,
        0
    );
    return 0.into();
}

//removes workmodule::on_flag for FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ATTACK_HI3_LANDING
unsafe extern "C" fn rocky_attackhi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackHi3()
}

//up smash
unsafe extern "C" fn rockman_attack_hi4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed = MotionModule::trans_move_speed(fighter.module_accessor);
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                return 0.into()
            }
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR && fighter.sub_air_check_fall_common().get_bool() {
                return 0.into()
            }
        }
        if speed.y < -0.001 {
            if fighter.sub_transition_group_check_air_landing().get_bool() {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
                //fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                return 0.into()
            }
        }
        if status_AttackHi4_Main_minjump(fighter) { //was status_AttackHi3_Main_minjump() in vanilla, refer to fn below
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ATTACK_HI3_LANDING);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
            return 1.into()
        }
        else {
            if MotionModule::is_end(fighter.module_accessor) {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return 0.into()
            }
            else {
                return 0.into()
            }
        }
    }
    return 1.into()
}

pub unsafe extern "C" fn status_AttackHi4_Main_minjump(fighter: &mut L2CFighterCommon) -> bool { //not an actual status fn, reimplements status_AttackHi3_Main_minjump()
    let jump_mini_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    
    if 0 < jump_mini_attack_frame {
        if !StopModule::is_stop(fighter.module_accessor) {
            if fighter.sub_check_button_jump().get_bool() {
                let motion = MotionModule::motion_kind(fighter.module_accessor);
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(motion), -1);
                WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                fighter.change_status_jump_mini_attack(true.into());
                return true;
            }
        }
    }
    else if 1 == jump_mini_attack_frame {
        if fighter.global_table[IS_STOP].get_bool() == false {
            let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            if 0 < attack_kind {
                FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
                WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                return false;
            }
        }
    }
    return false;
}

unsafe extern "C" fn rocky_attackhi4_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    //fighter.status_AttackHi4Start_Main();
    fighter.status_AttackHi4_common(L2CValue::U64(0xa5598d745)); //the u64 should be attack_hi4 ? vanilla script had attack_hi3
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ATTACK_HI3_LANDING);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(rockman_attack_hi4_main_loop as *const () as _))
}

unsafe extern "C" fn rocky_attackhi4_pre(fighter: &mut L2CFighterCommon) -> L2CValue {

    let mask = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_HI4 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64;
    let mask2 = *FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_NO_REACTION;

    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR), 
        *FIGHTER_KINETIC_TYPE_MOTION_AIR, 
        *GROUND_CORRECT_KIND_AIR as u32, 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), 
        true, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLAG, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_INT, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLOAT,
        mask2
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false,
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        false, 
        mask,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_4 as u32,
        0
    );
    return 0.into();
}

unsafe extern "C" fn status_landing(fighter: &mut L2CFighterCommon) -> L2CValue {

    fighter.sub_landing_uniq_process_exec();
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_LANDING {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ATTACK_HI3_LANDING) {
            let landing_lag = WorkModule::get_param_int(fighter.module_accessor, 0xdf05c072b, hash40("attack_hi3_landing_frame"));
            if MotionModule::frame(fighter.module_accessor) < landing_lag as f32 {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_LANDING_FLAG_STIFF_CANCEL);
            }
        }
    }
    return 0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Exec, *FIGHTER_STATUS_KIND_LANDING, status_landing);

    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_ATTACK_HI4, rocky_attackhi4_pre);
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ATTACK_HI4, rocky_attackhi4_main);

    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_ATTACK_HI3, rocky_attackhi3_pre);
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ATTACK_HI3, rocky_attackhi3_main);
}