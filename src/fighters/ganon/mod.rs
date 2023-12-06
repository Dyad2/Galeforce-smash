use smash::phx::{Hash40, Vector3f};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;
use std::mem;

use crate::fighters::common::galeforce::*;
use galeforce_utils::{vars::*, table_const::*};
use custom_var::*;

mod acmd;
mod effects;
mod opff;
mod status;

pub fn install() {
    acmd::install();
    effects::install();
    opff::install();
    status::install();
}