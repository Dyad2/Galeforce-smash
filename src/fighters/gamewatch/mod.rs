//use smash::phx::Hash40;
use smash::lib::{
    lua_const::*,
    L2CValue,
};
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

use crate::fighters::common::galeforce::*;
use galeforce_utils::{vars::*, utils::*, table_const::*};
use custom_var::*;

mod acmd;
mod status;

#[fighter_frame( agent = FIGHTER_KIND_GAMEWATCH )]
fn cardboard_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        //fixme: doesnt work? the exact same code works for the belmonts
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            VarModule::on_flag(fighter.battle_object,commons::instance::flag::DISABLE_SPECIAL_HI)
        }
        if is_special_reset(&mut *fighter.module_accessor) {
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::DISABLE_SPECIAL_HI);
        }

        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            if !VarModule::is_flag(fighter.battle_object,commons::instance::flag::GALEFORCE_ATTACK_ON) {
                //if ControlModule::get_flick_y(fighter.module_accessor) < 1 {
                    VarModule::on_flag(fighter.battle_object,commons::instance::flag::GALEFORCE_ATTACK_ON);
                    galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                    println!("gnw stored judge! ");
                    if fighter.global_table[STICK_Y].get_f32() >= 0.15  {
                        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND) != 9 {
                            VarModule::set_int(fighter.battle_object, WorkModule::get_int(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND) + 1, gamewatch::instance::int::JUDGE_STORED_KIND);
                        }
                    }
                    else if fighter.global_table[STICK_Y].get_f32() >= -0.15 {
                        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND) != 1 {
                            VarModule::set_int(fighter.battle_object, WorkModule::get_int(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND) - 1, gamewatch::instance::int::JUDGE_STORED_KIND);
                        }
                    }
                //}
            }
        }
    }
}

pub fn install() {
    acmd::install();
    status::install();
}