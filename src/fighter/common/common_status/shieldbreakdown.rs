use super::*; 

#[skyline::hook(replace = L2CFighterCommon_status_pre_ShieldBreakDown)]
unsafe extern "C" fn status_ShieldBreakDown_pre(fighter: &mut L2CFighterCommon) -> L2CValue {

    let mask = (*FIGHTER_STATUS_ATTR_DAMAGE | *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY) as u64;
    let mask2 = *FS_SUCCEEDS_KEEP_EFFECT | *FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_SOUND;
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
        mask2
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false,
        *FIGHTER_TREADED_KIND_DISABLE, 
        false, 
        false, 
        false, 
        mask,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0
    );
    //fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_ShieldBreakDown as *const () as _));
    return 0.into();
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_ShieldBreakDown)]
unsafe extern "C" fn bac_status_ShieldBreakDown(fighter: &mut L2CFighterCommon) {
    fighter.status_ShieldBreakDown();
}

#[skyline::hook(replace = L2CFighterCommon_status_ShieldBreakDown)]
unsafe extern "C" fn status_ShieldBreakDown(fighter: &mut L2CFighterCommon) {

    //smash::app::sv_fighter_util::is
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DOWN_FLAG_UP);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("damage_hi_3"), 0.0, 1.0, false, 0.0, false, false); //motion becomes damage_hi_3, we want to change it here so the main script works
    MotionModule::set_rate(fighter.module_accessor, 0.66);
    
    let shield_reset_hp = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_reset")); //
    WorkModule::set_float(fighter.module_accessor, shield_reset_hp, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHECK_DEAD_AREA_FORCE);

    if !FighterUtil::check_melee_rule_time(300.0, smash::app::FighterCheckMeleeRuleTime(0), true) { //might not be right for training mode idk
        HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
    }
    macros::PLAY_SE(fighter, Hash40::new_raw(0x14fe0eb7e3 as u64)); //shield break sound
    fighter.sub_shift_status_main(L2CValue::Ptr(status_ShieldBreakDown_main as *const () as _));
}

#[skyline::hook(replace = L2CFighterCommon_status_ShieldBreakDown_Main)]
unsafe extern "C" fn status_ShieldBreakDown_main(fighter: &mut L2CFighterCommon) -> L2CValue {
   
    if fighter.global_table[MOTION_FRAME].get_i32() >= 25 {
        HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FURAFURA.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    return 0.into();
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_ShieldBreakDown_pre,
            bac_status_ShieldBreakDown,
            status_ShieldBreakDown,
            status_ShieldBreakDown_main,
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}