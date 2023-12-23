use super::*;
use crate::fighters::common::opff::common_fighter_frame;

unsafe extern "C" fn metaknight_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let status_kind: i32 = StatusModule::status_kind(fighter.module_accessor);
    
    if //[*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_END, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END].contains(&prev_status_kind)
        status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL
      && VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON)
      && situation_kind == *SITUATION_KIND_AIR {
        VarModule::on_flag(fighter.battle_object, commons::instance::flag::DISABLE_SPECIAL_ALL);
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
        galeforce_apply_effect(&mut *fighter.module_accessor, 0.66);
    }
    if [hash40("special_air_n_end"), hash40("special_n_spin"),
        hash40("special_air_lw"), hash40("special_air_lw_f"), hash40("special_air_lw_b"),
        hash40("special_hi_loop")].contains(&curr_motion_kind) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
    }
    else {
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
}
unsafe extern "C" fn metaknight_cleanup(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);

    if is_special_reset(&mut *fighter.module_accessor) {
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::DISABLE_SPECIAL_ALL);
    }
    if ![hash40("special_air_n_end"), hash40("special_n_end")].contains(&curr_motion_kind) {
        VarModule::off_flag(fighter.battle_object, metaknight::instance::flag::MACH_TORNADO_HIT);
    }
}

unsafe extern "C" fn metaknight_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    metaknight_galeforce_attack(fighter);
    metaknight_cleanup(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, metaknight_frame);
}