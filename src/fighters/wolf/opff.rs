use super::*;
use crate::fighters::common::opff::common_fighter_frame;

//GA - Calculated Recklessness
// type: restriction lift
//  allows wolf to not enter helplessness state if side b hits an opponent
unsafe extern "C" fn  wolf_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);

    if !is_operation_cpu(fighter.module_accessor) {
        if curr_motion_kind == hash40("special_s_end") || curr_motion_kind == hash40("special_air_s_end") {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                if MotionModule::frame(fighter.module_accessor) >= 27.0 && MotionModule::frame(fighter.module_accessor) <= 30.0{
                    galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                    VarModule::on_flag(fighter.module_accessor, commons::instance::flag::DISABLE_SPECIAL_S);
                    StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
                }
            }
        }
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND || StopModule::is_damage(fighter.module_accessor) {
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::DISABLE_SPECIAL_S);
        }
    }
}

unsafe extern "C" fn wolf_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    wolf_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, wolf_frame);
}