#![allow(unused_must_use)]

use {
	super::*,
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    galeforce_utils::vars::*,
};

#[skyline::hook(replace = L2CFighterCommon_status_pre_EscapeAir)]
unsafe fn pre_escapeair(fighter: &mut L2CFighterCommon) -> L2CValue {
    //fighter.status_pre_EscapeAir();
    if ControlModule::get_stick_y(fighter.module_accessor).abs() >= 0.2 || ControlModule::get_stick_x(fighter.module_accessor).abs() >= 0.2 {
        VarModule::on_flag(fighter.battle_object, commons::instance::flag::ESCAPE_AIR_IS_SLIDE);
    }
    else {
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::ESCAPE_AIR_IS_SLIDE);
    }
	if ControlModule::get_stick_y(fighter.module_accessor) >= 0.60 {
		StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_GROUND as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        VarModule::on_flag(fighter.battle_object,  commons::instance::flag::ESCAPE_AIR_UP);
    }
	else {
		StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
	}
	FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, 0, 0, 0, 0);
	0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            pre_escapeair,
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}