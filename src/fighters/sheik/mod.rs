use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

use crate::fighters::common::galeforce::sheik_ga_buff;
use galeforce_utils::{vars::*, table_const::*, utils::*};
use custom_var::*;

pub mod acmd;
pub mod effect;

#[fighter_frame( agent = FIGHTER_KIND_SHEIK )]
fn sheikah_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);

        if status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.battle_object,commons::instance::flag::GALEFORCE_ATTACK_ON);
        }

        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            sheik_ga_buff(fighter);
            if is_status_damage(&mut *fighter.module_accessor) || [*FIGHTER_STATUS_KIND_PASSIVE, *FIGHTER_STATUS_KIND_PASSIVE_CEIL, *FIGHTER_STATUS_KIND_PASSIVE_FB, *FIGHTER_STATUS_KIND_PASSIVE_WALL, *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind){
                VisibilityModule::set_whole(fighter.module_accessor, true);
            }
            if is_status_grabbed(&mut *fighter.module_accessor) {
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
            }
        }

        if curr_motion_kind == hash40("attack_air_lw") {
            if fighter.global_table[MOTION_FRAME].get_i32() < 5 {
                if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    VarModule::on_flag(fighter.battle_object, sheik::instance::flag::ATTACK_AIR_LW_W);
                }
                else {
                    VarModule::off_flag(fighter.battle_object, sheik::instance::flag::ATTACK_AIR_LW_W);
                }
            }
        }
        else if !curr_motion_kind == hash40("landing_air_lw") {
            VarModule::off_flag(fighter.battle_object, sheik::instance::flag::ATTACK_AIR_LW_W);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        sheikah_frame
    );
    acmd::install();
    effect::install();
}