use super::*;

#[status_script(agent="pitb", status = FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn pitb_specialhirush_pre(fighter: &mut L2CFighterCommon) -> L2CValue {

    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND as u32, 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS), 
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
        0,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    return 0.into();
}

pub fn install() {
    install_status_scripts!(
        pitb_specialhirush_pre,
    );
}