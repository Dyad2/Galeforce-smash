use {
    smash_script::*,
    std::mem,
    smash::{
        hash40,
        app::{
            utility::get_kind,
            BattleObjectModuleAccessor,
            FighterSpecializer_Reflet,
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
    }
};

pub mod galeforce;
pub mod controls;
pub mod edge_cancels;
pub mod ecb_shifts;
pub mod opff;
mod common_status;

pub fn install() {
    common_status::install();
}