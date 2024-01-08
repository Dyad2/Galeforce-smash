use super::*;
use crate::fighters::common::opff::common_fighter_frame;

unsafe extern "C" fn byleth_failnaught_to_airn(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    
    if curr_motion_kind == hash40("special_air_n_start") 
       && MotionModule::frame(fighter.module_accessor) <= 28.0 
       && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0
       && ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_N {
            VarModule::on_flag(fighter.module_accessor, master::instance::flag::FAILNAUGHT_TO_AIRN);
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
    }
    
    if ![hash40("special_air_n_start"), hash40("attack_air_n")].contains(&curr_motion_kind) {
        VarModule::off_flag(fighter.module_accessor, master::instance::flag::FAILNAUGHT_TO_AIRN);
    }
}

//GA - Atrocity
// type: new move
//  cancels forward smash (not angled down) with an altered side b that combos
unsafe extern "C" fn byleth_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);

    if ![*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT_DASH, *FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) {
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
    if [hash40("attack_s4_hi"), hash40("attack_s4_s")].contains(&curr_motion_kind) && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if MotionModule::frame(fighter.module_accessor) <= 30.0 {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT, false);
            StopModule::end_stop(fighter.module_accessor);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, -1);
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
            MotionModule::set_frame(fighter.module_accessor, 8.0, false);
            galeforce_apply_effect(&mut *fighter.module_accessor, 0.66);
        }
    }
}

unsafe extern "C" fn master_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    byleth_galeforce_attack(fighter);
    byleth_failnaught_to_airn(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, master_frame);
}