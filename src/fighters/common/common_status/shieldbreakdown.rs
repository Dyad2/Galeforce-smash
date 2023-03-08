//FIXME: shieldbreaks freeze fighters

use super::*; 

#[common_status_script(status = FIGHTER_STATUS_KIND_SHIELD_BREAK_DOWN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon26status_pre_ShieldBreakDownEv")]
unsafe fn status_ShieldBreakDown_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("status_ShieldBreakDown_pre");

    let mask = (*FIGHTER_STATUS_ATTR_DAMAGE | *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY) as u64;
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
        *FIGHTER_TREADED_KIND_DISABLE, 
        false, 
        false, 
        false, 
        mask,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(bac_status_ShieldBreakDown as *const () as _));
    return 0.into();
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_ShieldBreakDown_Main)]
unsafe fn bac_status_ShieldBreakDown_Main(fighter: &mut L2CFighterCommon, _agent : L2CAgent) {
    println!("bac_status_ShieldBreakDown_Main");
    fighter.status_ShieldBreakDown_Main();
}

#[common_status_script(status = FIGHTER_STATUS_KIND_SHIELD_BREAK_DOWN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon26status_pre_ShieldBreakDownEv")]
unsafe fn status_ShieldBreakDown_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("status_ShieldBreakDown_main");
    
    if fighter.global_table[MOTION_FRAME].get_f32() >= 20.0 {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FURAFURA.into(), false.into());
        }
    }
    return 0.into();
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_ShieldBreakDown)]
unsafe fn bac_status_ShieldBreakDown(fighter: &mut L2CFighterCommon, _agent : L2CAgent) {
    println!("bac_status_ShieldBreakDown");
    fighter.status_ShieldBreakDown();
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon22status_ShieldBreakDownEv")]
unsafe fn status_ShieldBreakDown(fighter: &mut L2CFighterCommon) {
    println!("status_ShieldBreakDown");

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("damage_hi_3"), 0.0, 1.0, false, 0.0, false, false); //motion becomes damage_hi_3, we want to change it here so the main script works
    MotionModule::set_rate(fighter.module_accessor, 0.75);
    fighter.sub_shift_status_main(L2CValue::Ptr(bac_status_ShieldBreakDown_Main as *const () as _));
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            bac_status_ShieldBreakDown_Main,
            bac_status_ShieldBreakDown,
        );
    }
}

pub fn install() {
    install_status_scripts!(
        status_ShieldBreakDown_pre,
        status_ShieldBreakDown_main
    );
    install_hooks!(
        status_ShieldBreakDown,
    );
    skyline::nro::add_hook(nro_hook);
}