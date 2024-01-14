use super::*;
use crate::fighter::common::opff::common_fighter_frame;

//Marth's GA: Noble's Pursuit
//  type: cancel
//      Allows marth to follow up after landing a late hit up special. Disables up special until Marth either lands or grabs a ledge, getting counterhit won't save him if done offstage!
unsafe extern "C" fn marth_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    
    if curr_motion_kind == hash40("special_hi") || curr_motion_kind == hash40("special_air_hi") {
        if fighter.global_table[MOTION_FRAME].get_i32() > 8 && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
        if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) && MotionModule::frame(fighter.module_accessor) > 13. {
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::DISABLE_SPECIAL_HI);
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
            galeforce_apply_effect(&mut *fighter.module_accessor, 0.66);
        }
    }
    else {
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
    if is_special_reset(&mut *fighter.module_accessor) {
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::DISABLE_SPECIAL_HI);
    }    
}

unsafe extern "C" fn marth_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    marth_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, marth_frame);
}