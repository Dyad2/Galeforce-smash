use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_AttackDash)]
unsafe extern "C" fn status_attack_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_dash"), 0.0, 1.0, false, 0.0, false, false);

    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_FLAG_ATTACK_HI4_DISABLE);

    let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_CATCH_FRAME);

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
        let param_jump_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("jump_mini_attack_enable_frame"));
        WorkModule::set_int(fighter.module_accessor, param_jump_frame + 1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
    let log_attack_kind = fighter.status_attack()["log_infos"]["attack_dash"].get_i64(); //got that line from hdr
    WorkModule::set_int64(fighter.module_accessor, log_attack_kind, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let log_ak = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);

    if log_ak > 0 && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log_ak as u64);
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack_dash_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack_dash_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(status_AttackDash_Main as *const () as _))
}

// #[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon20sub_attack_dash_uniqEN3lib8L2CValueE")]
// unsafe extern "C" fn sub_attack_dash_uniq(fighter: &mut L2CFighterCommon, arg2: bool) -> L2CValue {
//     if !arg2 {
//         let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_FRAME);
//         if frame <= WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("item_catch_frame_attack_dash")) {
//             fighter.sub_GetLightItemImm(false.into()); 
//         }
//         if frame >= WorkModule::get_param_int(fighter.module_accessor, 0x1095b52773, 0) {
//             if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_FLAG_ATTACK_HI4_DISABLE) {
//                 WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
//                 WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_FLAG_ATTACK_HI4_DISABLE);
//             }
//         }
//     }
//     else {
//         let catch_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_CATCH_FRAME);
//         if 0 <= catch_frame {
//             WorkModule::dec_int(fighter.module_accessor,  *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_CATCH_FRAME);
//             if catch_frame == 0 {
//                 WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
//                 WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
//             }
//         }
//         if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) || WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME, 0) != 0 {
//             WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
//             WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
//             WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
//         }
//         WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_FRAME);
//     }
//     return 0.into()
// }

#[skyline::hook(replace = L2CFighterCommon_status_AttackDash_Main)]
unsafe extern "C" fn status_AttackDash_Main(fighter: &mut L2CFighterCommon) -> L2CValue {

    //prevents crossups with dash attack
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, 0.4); //this should probably be handled by jostling?
    }

    //move this to robin's custom dash attack status when you are less lazy    
    if smash::app::utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_REFLET {
        let bo = smash::app::sv_system::battle_object(fighter.lua_state_agent);
        let instance_bo = mem::transmute::<&mut smash::app::BattleObject, &mut smash::app::Fighter>(bo);
        let instance_boma = mem::transmute::<&mut smash::app::BattleObjectModuleAccessor, &mut smash::app::FighterModuleAccessor>(&mut *fighter.module_accessor);

        if fighter.global_table[MOTION_FRAME].get_i32() <= 5 && !VarModule::is_flag(fighter.module_accessor, reflet::status::flag::ATTACK_BUTTON_RELEASED) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0 {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                }
            }
            else {
                VarModule::on_flag(fighter.module_accessor, reflet::status::flag::ATTACK_BUTTON_RELEASED);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
        //after frame 5, thunder_sword is on
        if fighter.global_table[MOTION_FRAME].get_i32() == 6 {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                FighterSpecializer_Reflet::change_hud_kind(instance_bo, *FIGHTER_REFLET_MAGIC_KIND_SWORD);
                VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_thunder") as i64);
                //WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_REFLET_STATUS_ATTACK_AIR_INT_THUNDER_SWORD); //idk what this does, lets not
                WorkModule::dec_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                    FighterSpecializer_Reflet::set_flag_to_table(instance_boma, *FIGHTER_REFLET_MAGIC_KIND_SWORD, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
                }
            }
            //after frame 5, thunder_sword is off
            else {
                VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
                // let levin_restore_pts = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("thunder_sword_smash_attack_revival_time"));
                // let thunder_revival_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_REVIVAL_COUNT);
                // WorkModule::set_int(fighter.module_accessor, thunder_revival_count - levin_restore_pts,*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_REVIVAL_COUNT); //restore levin points
            }
        }
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    let jump_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if 0 < jump_frame {
        if !StopModule::is_stop(fighter.module_accessor) && fighter.sub_check_button_jump().get_bool() {
            let motion = MotionModule::motion_kind(fighter.module_accessor);

            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(motion), -1);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 1.into();
        }
    }
    let reserve_log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if jump_frame == 1 && !fighter.global_table[IS_STOP].get_bool() && reserve_log_attack_kind > 0 {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, reserve_log_attack_kind);
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) {
        let stick = fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor);
        if stick <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x")) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND && !ItemModule::is_have_item(fighter.module_accessor, 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
                return 0.into();
            }
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND && !ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else if WorkModule::get_param_int(fighter.module_accessor, 0x17e10662a4, 0) == *FIGHTER_ATTACK_DASH_TYPE_SQUAT_WAIT {
            fighter.change_status(FIGHTER_ATTACK_DASH_TYPE_SQUAT_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    return 0.into();
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_attack_dash,
            status_AttackDash_Main,
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}

//pub fn install() {
//    install_status_scripts!(
//        status_attack_dash,
//    );
//    install_hooks!(
//        //sub_attack_dash_uniq,
//        status_AttackDash_Main
//    );
//}