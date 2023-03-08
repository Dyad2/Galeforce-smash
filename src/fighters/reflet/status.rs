use super::*;

#[status_script(agent="reflet", status = FIGHTER_STATUS_KIND_ATTACK_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_attackdash(fighter: &mut L2CFighterCommon) -> L2CValue {

    // let bo = smash::app::sv_system::battle_object(fighter.lua_state_agent);
    // let instance_bo = mem::transmute::<&mut smash::app::BattleObject, &mut smash::app::Fighter>(bo);

    // if fighter.global_table[MOTION_FRAME].get_i32() == 6.0 {
    //     if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
    //         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
    //         FighterSpecializer_Reflet::change_hud_kind(instance_bo, *FIGHTER_REFLET_MAGIC_KIND_SWORD);
    //         VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    //     }
    //     else {
    //         WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
    //         VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    //     }
    // }

    //calls the standard dash attack status instead of robin's custom one. the levin stuff is managed in opff for now
    fighter.status_AttackDash();
    return 0.into()
}

// #[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_AttackAir_Main)]
// unsafe fn bac_status_attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.status_AttackAir_Main()
// }

#[status_script(agent="reflet", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_attackair(fighter: &mut L2CFighterCommon) -> L2CValue {

    fighter.sub_attack_air();
    fighter.sub_shift_status_main(L2CValue::Ptr(reflet_attackair_main_loop as *const () as _));
    return 0.into()
}

unsafe extern "C" fn reflet_attackair_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_custom(fighter);
    fighter.status_AttackAir_Main();
    0.into()
}

unsafe fn attack_air_custom(fighter: &mut L2CFighterCommon) {
    let bo = smash::app::sv_system::battle_object(fighter.lua_state_agent);
    let instance_bo = mem::transmute::<&mut smash::app::BattleObject, &mut smash::app::Fighter>(bo);
    let instance_boma = mem::transmute::<&mut smash::app::BattleObjectModuleAccessor, &mut smash::app::FighterModuleAccessor>(&mut *fighter.module_accessor);

    //up to frame 5, check is player is holding A
    if fighter.global_table[MOTION_FRAME].get_i32() <= 5 && !VarModule::is_flag(fighter.battle_object, reflet::status::flag::ATTACK_BUTTON_RELEASED) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
        else {
            VarModule::on_flag(fighter.battle_object, reflet::status::flag::ATTACK_BUTTON_RELEASED);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
        }
    }
    //after frame 5, thunder_sword is on
    if fighter.global_table[MOTION_FRAME].get_i32() == 6 {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            FighterSpecializer_Reflet::change_hud_kind(instance_bo, *FIGHTER_REFLET_MAGIC_KIND_SWORD);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_thunder") as i64);
            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_REFLET_STATUS_ATTACK_AIR_INT_THUNDER_SWORD); //idk what this does
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 {
                FighterSpecializer_Reflet::set_flag_to_table(instance_boma, *FIGHTER_REFLET_MAGIC_KIND_SWORD, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
        }
        //after frame 5, thunder_sword is off
        else {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
            let levin_restore_pts = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("thunder_sword_smash_attack_revival_time"));
            let thunder_revival_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_REVIVAL_COUNT);
            WorkModule::set_int(fighter.module_accessor, thunder_revival_count - levin_restore_pts,*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_REVIVAL_COUNT); //restore levin points
        }
    }
}

pub fn install() {
    install_status_scripts!(
        status_attackdash,
        status_attackair
    );
}