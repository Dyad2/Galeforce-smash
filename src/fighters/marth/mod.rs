use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::{lua_bind::*, sv_animcmd::*};
use smashline::*;
use smash_script::*;

use galeforce_utils::{vars::*, table_const::*, utils::*};
use crate::fighters::common::galeforce::*;
use custom_var::*;

pub mod acmd;
pub mod effects;
pub mod opff;
pub mod specials;

pub fn install() {
    acmd::install();
    effects::install();
    opff::install();
    specials::install();
}