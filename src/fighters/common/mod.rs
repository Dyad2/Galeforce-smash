
pub mod galeforce;
mod status;
mod opff;

use {
    smashline::*,
    std::mem,
    smash::{
        hash40,
        app::{
            BattleObjectModuleAccessor,
            utility::get_kind,
            BattleObject,
            sv_battle_object,
            GroundCliffCheckKind,
            FighterUtil,
            lua_bind::*
        },
        phx::{
            Vector2f,
            Vector3f,
            Hash40
        },
        lua2cpp::{
            L2CFighterCommon,
            L2CFighterBase
        },
        lib:: {
            L2CValue,
            lua_const::*
        }
    },
    skyline::{
        nn::ro::LookupSymbol,
        nro::{self, NroInfo}
    },
    custom_var::*,
    galeforce_utils::{
        table_const::*,
        vars::*,
        utils::*
    }
};

use crate::func_hooks::get_battle_object_from_id;
use crate::edge_cancels::*;
use crate::ecb_shifts::*;

pub static mut TOTAL_FIGHTER: i32 = 0;

pub fn install() {
    opff::install();
}