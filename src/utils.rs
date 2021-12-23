use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::{BattleObjectModuleAccessor, utility::get_kind, BattleObject, sv_battle_object};
use smash::*;

use crate::FIGHTER_MANAGER_ADDR;

//Training modpack utils
pub fn get_category(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    (module_accessor.info >> 28) as u8 as i32
}

pub fn is_fighter(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    get_category(module_accessor) == BATTLE_OBJECT_CATEGORY_FIGHTER
}

pub fn is_operation_cpu(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        if !is_fighter(module_accessor) {
            return false;
        }

        let entry_id_int = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;
        let entry_id = app::FighterEntryID(entry_id_int);
        let mgr = *(FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);
        let fighter_information = FighterManager::get_fighter_information(mgr, entry_id) as *mut app::FighterInformation;

        FighterInformation::is_operation_cpu(fighter_information)
    }
}

//author: ayerbe
pub unsafe fn get_attacker_number(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> usize {
	let attacker_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ATTACKER_COLOR) as usize;
	return attacker_number;
}

//author: ayerbe
pub unsafe fn get_boma(entry_id: i32) -> *mut smash::app::BattleObjectModuleAccessor {
    let boma = &mut *sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(entry_id));
    return boma;
}

pub unsafe fn get_player_number(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> usize {
    let player_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    return player_number;
}

//author: dyad
pub unsafe fn is_special_reset(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor);
    let situation_kind = StatusModule::situation_kind(module_accessor);

        if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF, *SITUATION_KIND_WATER].contains(&situation_kind)
            || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, 
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP,
                *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_SWING_GAOGAEN_FAILURE,
                *FIGHTER_STATUS_KIND_SWING_GAOGAEN_LARIAT, *FIGHTER_STATUS_KIND_SWING_GAOGAEN_RETURN, *FIGHTER_STATUS_KIND_SWING_GAOGAEN_SHOULDER, *FIGHTER_STATUS_KIND_SWING_GAOGAEN_THROWN,
                *FIGHTER_STATUS_KIND_SWALLOWED_THROWN, *FIGHTER_STATUS_KIND_SWALLOWED_THROWN_STAR, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN, *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_THROWN,
                *FIGHTER_STATUS_KIND_MEWTWO_THROWN, *FIGHTER_STATUS_KIND_MIIFIGHTER_COUNTER_THROWN, *FIGHTER_STATUS_KIND_CLUNG_THROWN_BLANK_DIDDY, *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY,
                *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *FIGHTER_STATUS_KIND_CLUNG_DAMAGE_DIDDY, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *FIGHTER_STATUS_KIND_CLUNG_GANON].contains(&status_kind) 
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

    if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_SWING_GAOGAEN_FAILURE,
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
    let status_kind = StatusModule::status_kind(module_accessor);
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

pub unsafe fn is_status_grabbed(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor);

    if [*FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_SWING_GAOGAEN_FAILURE,
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