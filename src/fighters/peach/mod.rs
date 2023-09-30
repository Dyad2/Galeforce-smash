use smash::phx::Hash40;
use smash::hash40;
use smash::lib::{lua_const::*};
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

use crate::fighters::common::galeforce::*;
use galeforce_utils::{vars::*, table_const::*};
use custom_var::*;

mod acmd;
//mod status;

#[fighter_frame( agent = FIGHTER_KIND_PEACH )]
fn parasoleil_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        //let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let prev_status = StatusModule::prev_status_kind(fighter.module_accessor, 0);

        //delayed float
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) >= 4 || prev_status != *FIGHTER_STATUS_KIND_JUMP_SQUAT {
                VarModule::on_flag(fighter.battle_object, peach::instance::flag::ALLOW_FLOAT);
                //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT);
                //WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_ENABLE_UNIQ);
            }
            else {
                VarModule::off_flag(fighter.battle_object, peach::instance::flag::ALLOW_FLOAT);
                //WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_PEACH_STATUS_UNIQ_FLOAT_WORK_INT_ENABLE_UNIQ);
            }
        }

        //GA - 
        // type: cancel
        //  allows to pull a turnip after landing forward throw. probably doesnt need to be in opff
        if curr_motion_kind == hash40("throw_f") {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
              && fighter.global_table[MOTION_FRAME].get_i32() >= 20 {
                VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
            }
            if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON)
              && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
            }
        }
        else {
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        parasoleil_frame
    );
    acmd::install();
    //status::install();
}