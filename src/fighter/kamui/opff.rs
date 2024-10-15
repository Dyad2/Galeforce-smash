use super::*;
use crate::fighter::common::opff::common_fighter_frame;

unsafe extern "C" fn kamui_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    
    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
        if VarModule::get_int(fighter.module_accessor, kamui::instance::int::GA_DURATION) > 0 {
            VarModule::sub_int(fighter.module_accessor, kamui::instance::int::GA_DURATION, 1);
            if [hash40("special_s_wall_attack_b"), hash40("special_s_wall_attack_f")].contains(&curr_motion_kind) {
                MotionModule::set_rate(fighter.module_accessor, 1.4);
            }
            if !VarModule::is_flag(fighter.module_accessor, kamui::instance::flag::KAMUI_TRANSFORMED) {
                VarModule::on_flag(fighter.module_accessor, kamui::instance::flag::KAMUI_TRANSFORMED);
                VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_horn") as i64);
                VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_hide") as i64);
            }
        }
        else {
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
            VarModule::off_flag(fighter.module_accessor, kamui::instance::flag::KAMUI_TRANSFORMED);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("dragon") as i64, hash40("dragon_none") as i64);
            VisibilityModule::set_int64(fighter.module_accessor, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
        }
    }
}

unsafe extern "C" fn kamui_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    kamui_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, kamui_frame);
}