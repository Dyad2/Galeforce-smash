use smash::phx::{Hash40, Vector4f};
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

use crate::fighters::common::galeforce::*;
use galeforce_utils::{vars::*, utils::*};
use custom_var::*;

mod acmd;
mod opff;
mod weapon;

pub fn install() {
    acmd::install();
    opff::install();
    weapon::install();
}