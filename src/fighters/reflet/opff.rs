use super::*;
use crate::fighters::common::opff::common_fighter_frame;

//unsafe extern "C" fn reflet_galeforce_attack(fighter: &mut L2CFighterCommon) {
//    let lua_state = fighter.lua_state_agent;
//    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
//    let status_kind = StatusModule::status_kind(fighter.module_accessor);
//    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
//    let bo = smash::app::sv_system::battle_object(lua_state);
//    let instance_bo = mem::transmute::<&mut smash::app::BattleObject, &mut smash::app::Fighter>(bo);
//    let instance_boma = mem::transmute::<&mut smash::app::BattleObjectModuleAccessor, &mut smash::app::FighterModuleAccessor>(&mut *fighter.module_accessor);
    
//    //Robin (reflet) Galeforce attack: Ignis
//    // type: buff
//    //  when thunder is charged up to arcthunder and thoron, Robin can sacrifice the charge to empower a normal, non-smash attack. 
//    if !is_operation_cpu(fighter.module_accessor) {
//        robin_ignis_effect(fighter);
//        if AttackModule::is_attack(fighter.module_accessor, 0, false) && VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
//            VarModule::on_flag(fighter.module_accessor, reflet::instance::flag::GALEFORCE_ATTACK_ATTACK_OCCUR);
//        }
//        //if robin tries to activate their GA and the frame # is lower than it was last frame, cleanup
//        if MotionModule::frame(fighter.module_accessor) - VarModule::get_float(fighter.module_accessor, reflet::instance::float::GALEFORCE_ATTACK_INPUT_WINDOW) < 0.0 {
//            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
//            VarModule::off_flag(fighter.module_accessor, reflet::instance::flag::GALEFORCE_ATTACK_ATTACK_OCCUR);
//            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE);
//            VarModule::set_float(fighter.module_accessor, reflet::instance::float::GALEFORCE_ATTACK_INPUT_WINDOW, 0.0);
//            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
//            FighterSpecializer_Reflet::exit_special_n_tron(instance_boma);
//        }
//        //activating the GA before the first active frame
//        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND) >= 2 {
//            if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3].contains(&status_kind) 
//            && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && !VarModule::is_flag(fighter.module_accessor,  reflet::instance::float::GALEFORCE_ATTACK_INPUT_WINDOW) {
//                VarModule::on_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE);
//                VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
//                VarModule::set_float(fighter.module_accessor, reflet::instance::float::GALEFORCE_ATTACK_INPUT_WINDOW, MotionModule::frame(fighter.module_accessor));
//                FighterSpecializer_Reflet::change_hud_kind(instance_bo, 3);
//                FighterSpecializer_Reflet::change_grimoire(instance_boma, 3);
//                FighterSpecializer_Reflet::get_active_thunder_num(instance_boma, 0);
//            }
//        }
//        //applies power modifier throughout an entire move
//        if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
//            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND) == 2 {
//                if !VarModule::is_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE) {
//                    WorkModule::set_int(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT) - 5, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT);
//                }
//                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
//                    AttackModule::set_power_mul(fighter.module_accessor, 1.1);
//                }
//                else {
//                    AttackModule::set_power_mul(fighter.module_accessor, 1.25);
//                }
//            }
//            else { //necessarily at 3
//                if !VarModule::is_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE) {
//                    WorkModule::set_int(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT) - 8, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT);
//                }
//                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
//                    AttackModule::set_power_mul(fighter.module_accessor, 1.2);
//                }
//                else {
//                    AttackModule::set_power_mul(fighter.module_accessor, 1.4);
//                }
//            }
//            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
//                galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
//            }
//        }
//    }    
//    if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_CANCEL && THUNDER_FX_HANDLE[entry_id as usize] == 0 {
//        THUNDER_FX_HANDLE[entry_id as usize] = EffectModule::get_last_handle(fighter.module_accessor) as u32;
//    }    
//    if THUNDER_FX_HANDLE[entry_id as usize] != 0 && !VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
//        EffectModule::kill(fighter.module_accessor, THUNDER_FX_HANDLE[entry_id as usize], false, true);
//        THUNDER_FX_HANDLE[entry_id as usize] = 0;
//    }    
//    //remove thunder charge with taunt
//    if curr_motion_kind == hash40("appeal_lw_r") || curr_motion_kind == hash40("appeal_lw_l") {
//        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
//    }
//}

unsafe extern "C" fn reflet_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    //reflet_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, reflet_frame);
}