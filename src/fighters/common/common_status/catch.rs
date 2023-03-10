use super::*;

//most of this is a gift from wuboy :)
#[skyline::hook(replace = L2CFighterCommon_status_Catch)]
unsafe fn status_catch(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    fighter.sub_status_Catch();
    GrabModule::set_rebound(fighter.module_accessor, true);

    // disabled for now, reactivate when i'm done with archetypes?
    // if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) < -0.25 {
    //     if fighter.global_table[MOTION_FRAME].get_i32() <= 1.0 {
    //         let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
    //         FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, -1.0);
    //         PostureModule::reverse_lr(fighter.module_accessor);
    //         PostureModule::update_rot_y_lr(fighter.module_accessor);
    //     }
    // }

    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Catch_Main as *const () as _))
}


#[skyline::hook(replace = L2CFighterCommon_status_CatchDash)]
unsafe fn status_catchdash(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    fighter.sub_status_CatchDash();
    GrabModule::set_rebound(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchDash_Main as *const () as _))
}


#[skyline::hook(replace = L2CFighterCommon_status_CatchTurn)]
unsafe fn status_catchturn(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    fighter.sub_status_CatchTurn();
    GrabModule::set_rebound(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchTurn_Main as *const () as _))
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_catch,
            status_catchdash,
            status_catchturn,
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook).unwrap();
    // install_hooks!(
    //     bac_status_catch,
    //     bac_status_catchdash,
    //     bac_status_catchturn,
    //     status_catch,
    //     status_catchdash,
    //     status_catchturn,
    // );
}

// #[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon34bind_address_call_status_CatchEPN3lib8L2CAgentE")]
// unsafe fn bac_status_catch(fighter: &mut L2CFighterCommon, _arg: L2CAgent) {
//     fighter.status_Catch();
// }
// #[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon12status_CatchEv")]
// unsafe fn status_catch(fighter: &mut L2CFighterCommon) {

// #[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon34bind_address_call_status_CatchDashEPN3lib8L2CAgentE")]
// unsafe fn bac_status_catchdash(fighter: &mut L2CFighterCommon, _arg: L2CAgent) {
//     fighter.status_CatchDash();
// }
// #[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon16status_CatchDashEv")]
// unsafe fn status_catchdash(fighter: &mut L2CFighterCommon) {

// #[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon34bind_address_call_status_CatchTurnEPN3lib8L2CAgentE")]
// unsafe fn bac_status_catchturn(fighter: &mut L2CFighterCommon, _arg: L2CAgent) {
//     fighter.status_CatchTurn();
// }
// #[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon16status_CatchTurnEv")]
// unsafe fn status_catchturn(fighter: &mut L2CFighterCommon) {