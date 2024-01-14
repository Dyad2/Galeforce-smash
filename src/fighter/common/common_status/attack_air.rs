use super::*;
use smash::phx::Vector4f;
use smash::lib::L2CAgent;

#[skyline::hook(replace = L2CFighterCommon_status_AttackAir_Sub)]
unsafe extern "C" fn sub_attack_air(fighter: &mut L2CFighterCommon, param1 : L2CValue) {

    if is_training_mode() {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) {
            let cmb_vec1 = Vector4f{x: 0.35, y: 0.35, z: 0.66, w: 1.0};
            let cmb_vec2 = Vector4f{x: 0.35, y: 0.35, z: 0.66, w: 1.0};
            ColorBlendModule::set_main_color(fighter.module_accessor, &cmb_vec1, &cmb_vec2, 1.0, 0.33, 0, true);
        }
        else {
            ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
        }
    }

    fighter.sub_attack_air_common(true.into());
    return;
}

#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_common)]
unsafe extern "C" fn sub_attack_air_common(fighter: &mut L2CFighterCommon, param1 : L2CValue) {
    ControlModule::reset_trigger(fighter.module_accessor);
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    let whatever = 0xf3;
    fighter.global_table[FLICK_Y].assign(&whatever.into());
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if StopModule::is_stop(fighter.module_accessor) {
        fighter.attack_air_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_attack_air_uniq as *const () as _));
    if param1.get_bool() {
        fighter.sub_attack_air_kind();
    }
    ControlModule::reset_attack_air_kind(fighter.module_accessor);
    return;
}

// #[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon33bind_address_call_attack_air_uniqEPN3lib8L2CAgentENS1_8L2CValueE")]
// unsafe fn bac_attackair_uniq(fighter: &mut L2CFighterCommon, _param1: L2CAgent, param2: L2CValue) {
//     if is_training_mode() {
//         if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) {
//             let cmb_vec1 = Vector4f{x: 0.35, y: 0.35, z: 0.66, w: 1.0};
//             let cmb_vec2 = Vector4f{x: 0.35, y: 0.35, z: 0.66, w: 1.0};
//             ColorBlendModule::set_main_color(fighter.module_accessor, &cmb_vec1, &cmb_vec2, 1.0, 0.33, 0, true);
//         }
//         else {
//             ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
//         }
//     }
//     fighter.attack_air_uniq(param2);
//     return;
// }

// #[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon16status_AttackAirEv")]
// unsafe fn status_attackair(fighter: &mut L2CFighterCommon) {
//     if is_training_mode() {
//         if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) {
//             let cmb_vec1 = Vector4f{x: 0.35, y: 0.35, z: 0.66, w: 1.0};
//             let cmb_vec2 = Vector4f{x: 0.35, y: 0.35, z: 0.66, w: 1.0};
//             ColorBlendModule::set_main_color(fighter.module_accessor, &cmb_vec1, &cmb_vec2, 1.0, 0.33, 0, true);
//         }
//         else {
//             ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
//         }
//     }
//     fighter.sub_attack_air_common(true.into());
//     fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackAir_Main as *const () as _));
//     return
// }
// #[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon28status_AttackAir_Main_commonEv")]
// unsafe fn status_attackair_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
//     //this part is custom
//     if is_training_mode() {
//         if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) {
//             let cmb_vec1 = Vector4f{x: 0.35, y: 0.35, z: 0.66, w: 1.0};
//             let cmb_vec2 = Vector4f{x: 0.35, y: 0.35, z: 0.66, w: 1.0};
//             ColorBlendModule::set_main_color(fighter.module_accessor, &cmb_vec1, &cmb_vec2, 1.0, 0.33, 0, true);
//         }
//         else {
//             ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
//         }
//     }
//     //vanilla
//     if !fighter.attack_air_common_strans().get_bool() {
//         if CancelModule::is_enable_cancel(fighter.module_accessor) {
//             if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
//                 if !fighter.sub_air_check_fall_common().get_bool() {
//                     if !MotionModule::is_end(fighter.module_accessor) {
//                         return false.into();
//                     }
//                     ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
//                     fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
//                 }
//                 else {
//                     return false.into();
//                 }
//             }
//         }
//     }
//     return true.into();
// }

// #[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon24attack_air_common_stransEv")]
// unsafe fn status_attackair_common_strans(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_KEEP_AIR) {
//         return false.into()
//     }
//     else {
//         if !fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
//             if !fighter.sub_transition_group_check_air_landing().get_bool() {
//                 return false.into()
//             }
//         }
//         else {
//             ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
//             if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) {
//                 fighter.change_status(FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR.into(), false.into());
//             }
//             else {
//                 fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
//             }
//         }
//     }
//     return true.into();
// }

// #[skyline::hook(replace = L2CFighterCommon_attack_air_uniq)]
// unsafe fn status_attackair_uniq(fighter: &mut L2CFighterCommon, param1: L2CValue) -> L2CValue {
//     if is_training_mode() {
//         if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) {
//             let cmb_vec1 = Vector4f{x: 0.35, y: 0.35, z: 0.66, w: 1.0};
//             let cmb_vec2 = Vector4f{x: 0.35, y: 0.35, z: 0.66, w: 1.0};
//             ColorBlendModule::set_main_color(fighter.module_accessor, &cmb_vec1, &cmb_vec2, 1.0, 0.33, 0, true);
//         }
//         else {
//             ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
//         }
//     }
//     if !param1.get_bool() {
//         let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_FRAME);
//         let item_catch = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("item_air_catch_frame"));
//         if frame <= item_catch {
//             fighter.sub_GetLightItemImm(FIGHTER_STATUS_ATTACK_WORK_INT_100_HIT_NEAR_COUNT_CLIFF_STOP.into());
//         }
//     }
//     else {
//         WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_FRAME);
//     }
//     fighter.sub_fall_common_uniq(param1);
//     return 0.into();
// }

// #[common_status_script(status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
//     symbol = "_ZN7lua2cpp16L2CFighterCommon21status_AttackAir_MainEv")]
// unsafe fn status_attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if !fighter.status_AttackAir_Main_common().get_bool() {
//         fighter.sub_air_check_superleaf_fall_slowly(); 
//         if !fighter.global_table[IN_HITLAG].get_bool() {
//             FighterUtil::check_cloud_through_out(fighter.module_accessor);
//         }
//     }
//     return 0.into()
// }

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackAir)]
unsafe extern "C" fn status_AttackAir_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
    return 0.into();
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_attack_air,
            sub_attack_air_common,
            status_AttackAir_end,
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}


//pub fn install() {
//    install_status_scripts!(
//        //status_attackair_main,
//        status_AttackAir_end,
//    );
//    install_hooks!(
//        //bac_attackair_uniq,
//        sub_attack_air,
//        sub_attack_air_common,
//        //status_attackair,
//        //status_attackair_main_common,
//    );
//}