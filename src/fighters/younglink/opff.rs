use super::*;
use crate::fighters::common::opff::common_fighter_frame;

//GA - ?
// type: new move 
//  When Young link hits an opponent with Z-Air, he is moved to the hookshot's position and allowed to press the offense
unsafe extern "C" fn  younglink_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    
    if curr_motion_kind == hash40("air_catch") {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
        if !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
                //MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("air_catch_move"), 1.0, 1.0, 1.0);
                //MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("air_catch_move"), 1.0, 1.0, 0.0, true, false);
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("air_catch_move"), 0.0, 1.0, false, 0.0, false, false);
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
                VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
            }
        }
    }
    else {
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
    //prevents landing while young link is moving forward
    if curr_motion_kind == hash40("air_catch_move") {
        if fighter.global_table[MOTION_FRAME].get_i32() <= 25 {
            StatusModule::set_keep_situation_air(fighter.module_accessor, true);
            GroundModule::pass_floor(fighter.module_accessor);
        }
        else {
            StatusModule::set_keep_situation_air(fighter.module_accessor, false);
        }
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && !VarModule::is_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE) {
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE);
            let stop  = smash::phx::Vector3f { x: 0.33, y: 1.0, z: 1.0 };
            KineticModule::mul_speed(fighter.module_accessor, &stop, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    else {
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE);
    }
}

unsafe extern "C" fn younglink_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    younglink_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, younglink_frame);
}