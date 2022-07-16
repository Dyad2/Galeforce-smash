use std::arch::asm;
use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::sv_battle_object;
use smash::lua2cpp::L2CAgentBase;
use smash::{phx::Vector3f, lua2cpp::L2CFighterCommon};
use smash::app::sv_animcmd::*;
use smash::app::sv_animcmd;
use smashline::*;
use smash_script::*;

use crate::fighters::common::galeforce::*;
use galeforce_utils::{vars::*, table_const::*, utils::*};
use custom_var::*;

#[fighter_frame( agent = FIGHTER_KIND_BAYONETTA )]
fn bayo_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        
        //GA - Witches' Ascent
        // type: restriction lift
        //  after using afterburner kick once, hitting with dabk allows an additional use of upwards abk
        if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U 
          && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
        //allows one more abk after hitting with a second dabk
        if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D 
          && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
          && VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM);
        }
        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) && VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM) {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
            if (status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U || status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D) && MotionModule::frame(fighter.module_accessor) < 3.0 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM);
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.66);
            }
        }
        if situation_kind == *SITUATION_KIND_GROUND {
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM);
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }

        //remove gravity on bats startup
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR && MotionModule::frame(fighter.module_accessor) <= 10. {
                KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            else {
                KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }

        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            if VarModule::is_flag(fighter.battle_object, bayonetta::instance::flag::DODGE_OFFSET) {
                if VarModule::get_int(fighter.battle_object, bayonetta::instance::int::DODGE_OFFSET_NUM) == 2 {
                    VarModule::set_int(fighter.battle_object, bayonetta::instance::int::DODGE_OFFSET_NUM, 0);
                    VarModule::off_flag(fighter.battle_object, bayonetta::instance::flag::DODGE_OFFSET);
                    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40{hash: hash40("attack_s3_s2")}, 0.0, 0.0, 0.0);
                }
                if VarModule::get_int(fighter.battle_object, bayonetta::instance::int::DODGE_OFFSET_NUM) == 3 {
                    VarModule::set_int(fighter.battle_object, bayonetta::instance::int::DODGE_OFFSET_NUM, 0);
                    VarModule::off_flag(fighter.battle_object, bayonetta::instance::flag::DODGE_OFFSET);
                    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40{hash: hash40("attack_s3_s3")}, 0.0, 0.0, 0.0);
                }
            }
            if VarModule::get_int(fighter.battle_object, bayonetta::instance::int::DODGE_OFFSET_NUM) != 0 {
                VarModule::on_flag(fighter.battle_object, bayonetta::instance::flag::DODGE_OFFSET);
                if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) >= 0.2 {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40{hash: hash40("escape_f")}, 0.0, 0.0, 0.0);
                    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_F, false);
                }
                if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) < 0.2 && ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) > -0.2 {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40{hash: hash40("escape")}, 0.0, 0.0, 0.0);
                    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE, false);
                }
                if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) <= -0.2 {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40{hash: hash40("escape_b")}, 0.0, 0.0, 0.0);
                    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_B, false);
                }
            }
        }
        if ![hash40("attack_s3_s"), hash40("attack_s3_s2"), hash40("escape_b"), hash40("escape"), hash40("escape_f")].contains(&curr_motion_kind) && fighter.global_table[MOTION_FRAME].get_f32() == 14.0 {
            VarModule::set_int(fighter.battle_object, bayonetta::instance::int::DODGE_OFFSET_NUM, 0);
            VarModule::off_flag(fighter.battle_object, bayonetta::instance::flag::DODGE_OFFSET);
        }

        //not touching the jab finisher sound script with a 100ft long pole
        if curr_motion_kind == hash40("attack_100_end") && ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVEARM) {
            macros::PLAY_SE(fighter, Hash40::new("se_bayonetta_wecked_chargefull"));
        }

        //removes wicked weave costume
        if is_status_damage(&mut *fighter.module_accessor) {
            smash_script::notify_event_msc_cmd!(fighter, 0x25fd66ecef as u64, 30, 0, 1);
        }
        if [hash40("special_hi"), hash40("special_air_hi")].contains(&curr_motion_kind) {
            if fighter.global_table[MOTION_FRAME].get_f32() <= 35.0 {
                StatusModule::set_keep_situation_air(fighter.module_accessor, true);
                GroundModule::pass_floor(fighter.module_accessor);
            }
            else {
                StatusModule::set_keep_situation_air(fighter.module_accessor, false);
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        bayo_frame
    );
}