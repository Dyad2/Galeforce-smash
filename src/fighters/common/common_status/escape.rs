use super::*;

#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon18status_pre_EscapeEv")]
unsafe fn status_Escape(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_GROUND), 
        *FIGHTER_KINETIC_TYPE_MOTION, 
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        false, 
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
        0, //*FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION as u64,
        0,
        0,
        0
    );
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        fighter.sub_transition_group_check_ground_jump_mini_attack();
        fighter.sub_transition_group_check_ground_jump();
        return 0.into();
    }
    return 0.into();
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon18status_pre_EscapeFEv")]
unsafe fn status_EscapeF(fighter: &mut L2CFighterCommon) -> L2CValue {
 
    let ret = fighter.sub_pre_escape_fb().get_bool();
 
    if !ret {
        StatusModule::init_settings(
            fighter.module_accessor, 
            smash::app::SituationKind(*SITUATION_KIND_GROUND), 
            *FIGHTER_KINETIC_TYPE_MOTION, 
            *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, 
            smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
            false, 
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ESCAPE_FLAG, 
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ESCAPE_INT, 
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ESCAPE_FLOAT,
            0
        );
        FighterStatusModuleImpl::set_fighter_status_data(
            fighter.module_accessor, 
            false,
            *FIGHTER_TREADED_KIND_NO_REAC, 
            false, 
            false, 
            false, 
            0,
            *FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION as u32,
            0,
            0
        );
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
            fighter.sub_transition_group_check_ground_jump_mini_attack();
            fighter.sub_transition_group_check_ground_jump();
        }
    }
    return ret.into();
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon18status_pre_EscapeBEv")]
unsafe fn status_EscapeB(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = fighter.sub_pre_escape_fb().get_bool();
    if !ret {
        StatusModule::init_settings(
            fighter.module_accessor, 
            smash::app::SituationKind(*SITUATION_KIND_GROUND), 
            *FIGHTER_KINETIC_TYPE_MOTION, 
            *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, 
            smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
            false, 
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ESCAPE_FLAG, 
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ESCAPE_INT, 
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ESCAPE_FLOAT,
            0
        );
        FighterStatusModuleImpl::set_fighter_status_data(
            fighter.module_accessor, 
            false,
            *FIGHTER_TREADED_KIND_NO_REAC, 
            false, 
            false, 
            false, 
            0,
            *FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION as u32,
            0,
            0
        );
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
            fighter.sub_transition_group_check_ground_jump_mini_attack();
            fighter.sub_transition_group_check_ground_jump();
        }
        return ret.into();
    }
    return ret.into();
}

pub fn install() {
    install_status_scripts!(
        status_Escape,
        status_EscapeF,
        status_EscapeB
    );
}