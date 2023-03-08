use smash::phx::{Hash40, Vector2f};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

mod acmd_fighter;
//mod acmd_weapons;
mod opff;

pub fn install() {
    acmd_fighter::install();
    //acmd_weapons::install();
    opff::install();
}