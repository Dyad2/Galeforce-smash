use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;
use std::mem;

use galeforce_utils::{vars::*, table_const::*};
use custom_var::*;

mod effects;
mod acmd;
mod specials;

#[fighter_frame( agent = FIGHTER_KIND_CAPTAIN )]
fn captain_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);

        if [hash40("special_air_lw"), hash40("special_lw")].contains(&curr_motion_kind) && fighter.global_table[MOTION_FRAME].get_i32() <= 4 {
            if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) < 0.0 && !VarModule::is_flag(fighter.battle_object, commons::instance::flag::DO_ONCE) {
                VarModule::on_flag(fighter.battle_object, commons::instance::flag::DO_ONCE);
                PostureModule::reverse_lr(fighter.module_accessor);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
                let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
                FighterKineticEnergyMotion::reverse_chara_dir(fighter_kinetic_energy_motion);
            }
        }
        else {
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::DO_ONCE);
        }
    }
}

pub fn install() {
    let agent = &mut smashline::Agent::new("captain");
    acmd::install(agent);
    effects::install(agent);
    specials::install(agent);
}
