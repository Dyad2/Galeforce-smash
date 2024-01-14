use {
    smashline::*,
    smash_script::*,
    std::mem,
    smash::{
        hash40,
        app::{
            utility::get_kind,
            BattleObject,
            BattleObjectModuleAccessor,
            FighterSpecializer_Reflet,
            sv_battle_object,
            FighterUtil::*,
            FighterUtil,
            lua_bind::*,
        },
        phx::{
            Vector2f,
            Vector3f,
            Hash40
        },
        lua2cpp::{
            L2CFighterCommon,
            *
        },
        lib:: {
            L2CValue,
            lua_const::*
        }
    },
    custom_var::*,
    galeforce_utils::{
        table_const::*,
        vars::*,
        utils::*,
        utils::get_battle_object_from_id,
    }
};

pub static mut TOTAL_FIGHTER: i32 = 0;

pub mod galeforce;
pub mod controls;
pub mod edge_cancels;
pub mod ecb_shifts;
pub mod opff;
mod common_status;

pub fn install() {
    common_status::install();
}