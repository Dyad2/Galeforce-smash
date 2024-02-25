use smash::app::{sv_battle_object, BattleObject};
//use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::*;

use crate::vars::*;

extern "C"{
    // gets whether we are in training mode
    #[link_name = "\u{1}_ZN3app9smashball16is_training_modeEv"]
    pub fn is_training_mode() -> bool;
}
// pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut app::BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
//     unsafe {
//         let lua_module = *(boma as *mut app::BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
//         std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
//     }
// }


pub unsafe fn is_hitlag(module_accessor: *mut smash::app::BattleObjectModuleAccessor) -> bool { //this might be offensive hitlag only? 
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) > 0 {
        return true;
    }
    return false;
}

//Training modpack utils
pub unsafe fn get_category(module_accessor: *mut app::BattleObjectModuleAccessor) -> i32 {
    ((*module_accessor).battle_object_id >> 28) as u8 as i32
}

pub unsafe fn is_fighter(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    get_category(module_accessor) == BATTLE_OBJECT_CATEGORY_FIGHTER
}

pub fn is_operation_cpu(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        if get_category(module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
            return false;
        }

        let entry_id_int = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;
        let entry_id = app::FighterEntryID(entry_id_int);
        let fighter_information = FighterManager::get_fighter_information(singletons::FighterManager(), entry_id);

        FighterInformation::is_operation_cpu(fighter_information)
    }
}

//author: ayerbe
pub unsafe fn get_attacker_number(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> usize {
	let attacker_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ATTACKER_COLOR) as usize;
	return attacker_number;
}

pub unsafe fn get_boma(entry_id: i32) -> *mut smash::app::BattleObjectModuleAccessor {
    let boma = &mut *sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(entry_id));
    return boma;
}

pub unsafe fn get_boma_from_id(id: u32) -> *mut smash::app::BattleObjectModuleAccessor {
    let boma = &mut *sv_battle_object::module_accessor(id);
    return boma;
}

// pub unsafe fn get_player_number(module_accessor: *mut smash::app::BattleObjectModuleAccessor) -> usize {
//     let player_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
//     return player_number;
// }

//author: dyad
pub unsafe fn check_jump_input(module_accessor: *mut smash::app::BattleObjectModuleAccessor) -> bool {
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);

    if (ControlModule::is_enable_flick_jump(module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
        return true;
    }
    return false;
}

pub unsafe fn is_special_reset(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    let situation_kind = StatusModule::situation_kind(module_accessor);

    if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF, *SITUATION_KIND_WATER].contains(&situation_kind)
        || is_status_damage(module_accessor)
        || is_status_grabbed(module_accessor)
        {
            return true;
        }
    else 
    {
        return false;
    }
}

pub unsafe fn is_status_damage(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor);

    if [*FIGHTER_STATUS_KIND_DAMAGE, 
        *FIGHTER_STATUS_KIND_DAMAGE_AIR, 
        *FIGHTER_STATUS_KIND_DAMAGE_FALL, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, 
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG, 
        *FIGHTER_STATUS_KIND_CAPTURE_CUT, 
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_THROWN, 
        *FIGHTER_STATUS_KIND_SWING_GAOGAEN_FAILURE,
        *FIGHTER_STATUS_KIND_SWING_GAOGAEN_LARIAT, 
        *FIGHTER_STATUS_KIND_SWING_GAOGAEN_RETURN, 
        *FIGHTER_STATUS_KIND_SWING_GAOGAEN_SHOULDER, 
        *FIGHTER_STATUS_KIND_SWING_GAOGAEN_THROWN,
        *FIGHTER_STATUS_KIND_SWALLOWED_THROWN, 
        *FIGHTER_STATUS_KIND_SWALLOWED_THROWN_STAR, 
        *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY,
        *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN, 
        *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_THROWN,
        *FIGHTER_STATUS_KIND_MIIFIGHTER_COUNTER_THROWN, 
        *FIGHTER_STATUS_KIND_MEWTWO_THROWN, 
        *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
        *FIGHTER_STATUS_KIND_CATCHED_REFLET,
        *FIGHTER_STATUS_KIND_CATCHED_GANON,
        *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON,
        *FIGHTER_STATUS_KIND_CATCHED_CUT_GANON,
        *FIGHTER_STATUS_KIND_CLUNG_THROWN_BLANK_DIDDY, 
        *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY,
        *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, 
        *FIGHTER_STATUS_KIND_CLUNG_DAMAGE_DIDDY, 
        *FIGHTER_STATUS_KIND_CLUNG_DIDDY, 
        *FIGHTER_STATUS_KIND_CLUNG_GANON].contains(&status_kind) {
            return true;
    }
    else {
        return false;
    }
}

pub unsafe fn is_status_grabbed(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor);

    if [*FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD,
     *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_THROWN,
      *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_DRIVER, *FIGHTER_STATUS_KIND_CAPTURE_ITEM, *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE, *FIGHTER_STATUS_KIND_CAPTURE_JUMP,
       *FIGHTER_STATUS_KIND_CAPTURE_KAWASAKI, *FIGHTER_STATUS_KIND_CAPTURE_MASTERCORE, *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND, *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD,
        *FIGHTER_STATUS_KIND_CAPTURE_MIMIKKYU, *FIGHTER_STATUS_KIND_CAPTURE_NABBIT, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *FIGHTER_STATUS_KIND_SWING_GAOGAEN_FAILURE,
        *FIGHTER_STATUS_KIND_SWING_GAOGAEN_LARIAT, *FIGHTER_STATUS_KIND_SWING_GAOGAEN_RETURN, *FIGHTER_STATUS_KIND_SWING_GAOGAEN_SHOULDER, *FIGHTER_STATUS_KIND_SWING_GAOGAEN_THROWN,
        *FIGHTER_STATUS_KIND_SWALLOWED_THROWN, *FIGHTER_STATUS_KIND_SWALLOWED_THROWN_STAR, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN, *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_THROWN,
        *FIGHTER_STATUS_KIND_MEWTWO_THROWN, *FIGHTER_STATUS_KIND_MIIFIGHTER_COUNTER_THROWN, *FIGHTER_STATUS_KIND_CLUNG_THROWN_BLANK_DIDDY, *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY,
        *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *FIGHTER_STATUS_KIND_CLUNG_DAMAGE_DIDDY, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *FIGHTER_STATUS_KIND_CLUNG_GANON].contains(&status_kind) {
        return true;
    }
    else {
        return false;
    }
}

pub unsafe fn get_stick_dir(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> i32 {
    let stick_x = ControlModule::get_stick_x(module_accessor) * PostureModule::lr(module_accessor);
    let stick_y = ControlModule::get_stick_y(module_accessor);

    if stick_x >= 0.45 {
        if stick_y >= 0.45 {
            return 9;
        }
        else if stick_y > -0.45 {
            return 6;
        }
        else {
            return 3;
        }
    }
    else if stick_x >= -0.45 {
        if stick_y >= 0.45 {
            return 8;
        }
        else if stick_y > -0.45 {
            return 5;
        }
        else {
            return 2;
        }
    }
    else {
        if stick_y >= 0.45 {
            return 7;
        }
        else if stick_y > -0.45 {
            return 4;
        }
        else {
            return 1;
        }
    }
}

#[skyline::from_offset(0x3ac560)]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

// 0x184c223f47,
// 0x193bdcb0cc,
// 0x1985267897,
// 0x19f2214801,
// hash40("collision_attr_aura"),
// hash40("collision_attr_bind_extra"),
// hash40("collision_attr_blaster_throw_down"),
// hash40("collision_attr_blaster_throw_up"),
// hash40("collision_attr_bury"),
// hash40("collision_attr_bury_r"),
// hash40("collision_attr_coin"),
// hash40("collision_attr_curse_poison"),
// hash40("collision_attr_cutup"),
// hash40("collision_attr_cutup_metal"),
// hash40("collision_attr_deathball"),
// hash40("collision_attr_dedede_hammer"),
// hash40("collision_attr_elec"),
// hash40("collision_attr_elec_whip"),
// hash40("collision_attr_fire"),
// hash40("collision_attr_flower"),
// hash40("collision_attr_ice"),
// hash40("collision_attr_ink_hit"),
// hash40("collision_attr_jack_bullet"),
// hash40("collision_attr_jack_final"),
// hash40("collision_attr_lay"),
// hash40("collision_attr_magic"),
// hash40("collision_attr_mario_local_coin"),
// hash40("collision_attr_marth_shield_breaker"),
// hash40("collision_attr_noamal"),
// hash40("collision_attr_none"),
// hash40("collision_attr_normal"),
// hash40("collision_attr_normal_bullet"),
// hash40("collision_attr_palutena_bullet"),
// hash40("collision_attr_paralyze"),
// hash40("collision_attr_pierce"),
// hash40("collision_attr_purple"),
// hash40("collision_attr_rush"),
// hash40("collision_attr_saving"),
// hash40("collision_attr_saving_ken"),
// hash40("collision_attr_search"),
// hash40("collision_attr_sleep"),
// hash40("collision_attr_sleep_ex"),
// hash40("collision_attr_slip"),
// hash40("collision_attr_stab"),
// hash40("collision_attr_sting"),
// hash40("collision_attr_sting_bowarrow"),
// hash40("collision_attr_sting_flash"),
// hash40("collision_attr_stop"),
// hash40("collision_attr_taiyo_hit"),
// hash40("collision_attr_turn"),
// hash40("collision_attr_water"),
// hash40("collision_attr_whip")

//tap jump checks
//if ControlModule::is_enable_flick_jump(fighter.module_accessor) && ControlModule::get_stick_y(fighter.module_accessor) > 0.7 && ControlModule::get_flick_y(fighter.module_accessor) < 3

//related to damage turn?
//FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR

//yoshi jump armor is a flag woo
//FIGHTER_YOSHI_INSTANCE_WORK_ID_FLAG_JUMP_AERIAL_ARMOR

//let hitstun = (WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_FRAME) - WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_HIT_STOP_FRAME)) as f32;
