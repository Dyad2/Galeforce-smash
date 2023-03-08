use super::*;

//up tilt
#[status_script(agent="rockman", status = FIGHTER_STATUS_KIND_ATTACK_HI3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn rocky_attackhi3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {

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
#[status_script(agent="rockman", status = FIGHTER_STATUS_KIND_ATTACK_HI3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn rocky_attackhi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackHi3()
}

//up smash
unsafe extern "C" fn rockman_attack_hi4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed = MotionModule::trans_move_speed(fighter.module_accessor);
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if !MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND && !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                return 0.into()
            }
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR && !fighter.sub_air_check_fall_common().get_bool() {
                return 0.into()
            }
            if speed.y < -0.001 {
                if fighter.sub_transition_group_check_air_landing().get_bool() {
                    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
                    return 0.into()
                }
            }
        }
        if !fighter.status_AttackHi4Start_Main().get_bool() {
            if !MotionModule::is_end(fighter.module_accessor) {
                return 0.into()
            }
            else {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return 0.into()
            }
        }
        else {
            //VarModule::off_flag(fighter.battle_object, rockman::status::flag::ATTACK_HI4_LANDING);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ATTACK_HI3_LANDING);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
            return 1.into()
        }
    }
    return 1.into()
}

#[status_script(agent="rockman", status = FIGHTER_STATUS_KIND_ATTACK_HI4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn rocky_attackhi4_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    fighter.status_AttackHi4_common(L2CValue::U64(0xa5598d745)); //the u64 should be attack_hi4 ? vanilla script had attack_hi3
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ATTACK_HI3_LANDING);
    //VarModule::on_flag(fighter.battle_object, rockman::status::flag::ATTACK_HI4_LANDING);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(rockman_attack_hi4_main_loop as *const () as _))
}

#[status_script(agent="rockman", status = FIGHTER_STATUS_KIND_ATTACK_HI4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn rocky_attackhi4_pre(fighter: &mut L2CFighterCommon) -> L2CValue {

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

#[status_script(agent="rockman", status = FIGHTER_STATUS_KIND_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn status_landing(fighter: &mut L2CFighterCommon) -> L2CValue {

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

pub fn install() {
    install_status_scripts!(
        status_landing,
        rocky_attackhi3_pre,
        rocky_attackhi3_main,
        rocky_attackhi4_pre,
        rocky_attackhi4_main,
    );
}