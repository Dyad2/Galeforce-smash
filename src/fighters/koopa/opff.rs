use super::*;
use crate::fighters::common::opff::common_fighter_frame;

//Galeforce attack - down b jump cancel
unsafe extern "C" fn koopa_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    
    if curr_motion_kind == hash40("special_lw") {
        if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) && check_jump_input(fighter.module_accessor) {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
            galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
    }
    else {
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
}

//slowing down side special grab startup
//now in acmd
//unsafe extern "C" fn koopa_specials_slow(fighter: &mut L2CFighterCommon) {
//    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    
//    if curr_motion_kind == hash40("special_s_catch") && MotionModule::frame(fighter.module_accessor) <= 5. {
//        MotionModule::set_rate(fighter.module_accessor, 0.75);
//    }
//    else if curr_motion_kind == hash40("special_s_air_catch") && MotionModule::frame(fighter.module_accessor) <= 5. {
//        MotionModule::set_rate(fighter.module_accessor, 0.45);
//    }
//}

unsafe extern "C" fn koopa_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    koopa_galeforce_attack(fighter);
    //koopa_specials_slow(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, koopa_frame);
}