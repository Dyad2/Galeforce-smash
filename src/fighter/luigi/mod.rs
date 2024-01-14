use smash::phx::Hash40;
use smash::hash40;
use smash::lib::{lua_const::*, L2CValue};
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

mod acmd;
mod opff;
mod specials;
mod weapons;
mod status;

use crate::fighter::common::galeforce::*;
use galeforce_utils::{vars::*, table_const::*};
use custom_var::*;

//not in opff, used in acmd scripts only
unsafe fn check_add_charge(fighter: &mut L2CAgentBase) {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        VarModule::add_int(fighter.module_accessor, luigi::instance::int::ELEC_CHARGE, 1);
        luigi_charge_effect(fighter, 1.25);
    }
}

pub fn install() {
    let agent = &mut smashline::Agent::new("luigi");
    acmd::install(agent);
    opff::install(agent);
    specials::install(agent);
    status::install(agent);
    weapons::install(agent);
    agent.install();
}