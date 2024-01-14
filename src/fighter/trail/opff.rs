use super::*;
use crate::fighter::common::opff::common_fighter_frame;

unsafe extern "C" fn trail_stall_prevent(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N3_HOP)
      || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N2_HOP)
      || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N1_HOP) {
          WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N3_HOP);
          WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N2_HOP);
          WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N1_HOP);
      }
    else {
          WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N3_HOP);
          WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N2_HOP);
          WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N1_HOP);
    }
}

unsafe extern "C" fn trail_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    trail_stall_prevent(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, trail_frame);
}