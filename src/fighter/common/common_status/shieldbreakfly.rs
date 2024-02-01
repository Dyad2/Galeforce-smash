use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_ShieldBreakFly)]
unsafe extern "C" fn status_ShieldBreakFly_Main(fighter: &mut L2CFighterCommon) {
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_DOWN.into(), true.into()); //made custom
        } 
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_FALL.into(), true.into());
        }
    }
    return;
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_ShieldBreakFly)]
unsafe extern "C" fn status_ShieldBreakFly_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_GROUND), 
        *FIGHTER_KINETIC_TYPE_DAMAGE_GROUND,
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
        0,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0
    );

    //stops shieldstun from applying speed after break
    fighter.clear_lua_stack();
    smash_script::lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE, 0.0, 0.0);
    smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    
    return 0.into();
}

// #[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon34sub_status_shield_break_fly_commonEN3lib8L2CValueE")]
// unsafe fn substatus_shield_break_fly_common(fighter: &mut L2CFighterCommon, param1: bool) {
//     let shield_reset_hp = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_reset")); //
//     WorkModule::set_float(fighter.module_accessor, shield_reset_hp, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
//     WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHECK_DEAD_AREA_FORCE);
//     //stops shieldstun from applying speed after break
//     fighter.clear_lua_stack();
//     smash_script::lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE, 0.0, 0.0);
//     smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
//     WorkModule::set_float(fighter.module_accessor, 50.0, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_REACTION_FRAME); //doesnt work
//     fighter.ftStatusUniqProcessDamage_init_common();
//     if !FighterUtil::check_melee_rule_time(300.0, smash::app::FighterCheckMeleeRuleTime(0), true) { //might not be right for training mode idk
//         HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
//     }
//     if param1 { //crit?
//         macros::PLAY_SE(fighter, Hash40::new_raw(0x14fe0eb7e3 as u64)); //shield break sound
//     }
//     let another_some_param = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), 0x16856bbe5); //hash not cracked yet, but has a value of 300. number of frames before victim xlu ends?
//     WorkModule::set_int(fighter.module_accessor, another_some_param, *FIGHTER_STATUS_FURAFURA_STAND_WORK_INT_FRAME_COUNT); //was FIGHTER_STATUS_FURAFURA_STAND_WORK_INT_TERMINATE_XLU_COUNT, but it doesn't exists?
//     notify_event_msc_cmd!(fighter, 0x20cbc92683 as u64, 1, *FIGHTER_LOG_DATA_INT_SHIELD_BREAK_FLY_NUM);
//     return
// }

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_ShieldBreakFly_Main,
            status_ShieldBreakFly_pre,
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}