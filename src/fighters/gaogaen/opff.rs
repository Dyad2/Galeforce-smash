use super::*;
use crate::fighters::common::opff::common_fighter_frame;

unsafe extern "C" fn gaogaen_revenge_rework(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);

    //has revenge
    if WorkModule::get_float(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE) > 1.0 {
        VarModule::add_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER, 1);
        //hits something. don't remove revenge bonus if it's the counter hit.
        if status_kind == FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_HIT {
            VarModule::set_float(fighter.module_accessor, gaogaen::instance::float::REVENGE_BONUS_PRESERVE, WorkModule::get_float(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE));
            //reset grace period when revenge is successful
            VarModule::set_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER, 0);
        }
        //preserving revenge stats.
        else if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
        && ![hash40("attack_11"), hash40("attack_12")].contains(&curr_motion_kind) { //<-- these attacks don't remove revenge. up b initial hits are missing
            WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_ATTACK_HIT_REVENGE);
            VarModule::set_float(fighter.module_accessor, gaogaen::instance::float::REVENGE_BONUS_PRESERVE, WorkModule::get_float(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE));
            
            if VarModule::is_flag(fighter.module_accessor, gaogaen::instance::flag::REVENGE_REDUCE_ONCE) && WorkModule::get_float(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE) > 1.5 {
                VarModule::off_flag(fighter.module_accessor, gaogaen::instance::flag::REVENGE_REDUCE_ONCE);
                VarModule::add_float(fighter.module_accessor, gaogaen::instance::float::REVENGE_BONUS_PRESERVE, -0.5);
                //set the true revenge var
                WorkModule::set_float(fighter.module_accessor, VarModule::get_float(fighter.module_accessor, gaogaen::instance::float::REVENGE_BONUS_PRESERVE), *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE);
            }
            else if VarModule::is_flag(fighter.module_accessor, gaogaen::instance::flag::REVENGE_REDUCE_ONCE) {
                VarModule::off_flag(fighter.module_accessor, gaogaen::instance::flag::REVENGE_REDUCE_ONCE);
                VarModule::set_float(fighter.module_accessor, gaogaen::instance::float::REVENGE_BONUS_PRESERVE, 1.0);
                WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE);
                //attempt at removing the visual effect
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_ATTACK_HIT_REVENGE);
            }
        }
        else {
            VarModule::on_flag(fighter.module_accessor, gaogaen::instance::flag::REVENGE_REDUCE_ONCE);
        }
        //5 seconds grace before revenge begins decaying, then decay once per second
        if VarModule::get_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER) % 60 == 0 && VarModule::get_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER) >= 300 {
            VarModule::add_float(fighter.module_accessor, gaogaen::instance::float::REVENGE_BONUS_PRESERVE, -0.025);
            WorkModule::set_float(fighter.module_accessor, VarModule::get_float(fighter.module_accessor, gaogaen::instance::float::REVENGE_BONUS_PRESERVE), *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE);
        }
    }
    //revenge is off
    else {
        VarModule::set_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER, 0);
        VarModule::set_float(fighter.module_accessor, gaogaen::instance::float::REVENGE_BONUS_PRESERVE, 1.0);
        WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE);
        WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_ATTACK_HIT_REVENGE);
    }    
}

//unsafe extern "C" fn gaogaen_frame(fighter: &mut L2CFighterCommon) {
//    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
//    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    
//    //changing motion rates here to avoid acmd's command grabs
//    //TODO reimplement in acmd, command grabs work now its 2023 ffs
//    if curr_motion_kind == hash40("special_s_start") || curr_motion_kind == hash40("special_air_s_start"){
//        if MotionModule::frame(fighter.module_accessor) <= 16.0 {
//            MotionModule::set_rate(fighter.module_accessor, 0.75);
//        }
//        else {
//            MotionModule::set_rate(fighter.module_accessor, 1.425);
//        }
//    }
//}

unsafe extern "C" fn gaogaen_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    gaogaen_revenge_rework(fighter);
    //gaogaen_frame(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, gaogaen_frame);
}