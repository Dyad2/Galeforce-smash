use super::*;
use crate::fighter::common::opff::common_fighter_frame;

//GA - Hitfall
// hit an opponent with fair, then tap down while in hitlag to fastfall
unsafe extern "C" fn m2_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);

    if situation_kind == *SITUATION_KIND_AIR && curr_motion_kind == hash40("attack_air_f") { 
        if is_hitlag(fighter.module_accessor) && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            let stick_y = fighter.global_table[STICK_Y].get_f32();
            if stick_y < 0.0 && ControlModule::get_flick_y(fighter.module_accessor) < 1 {
                StopModule::cancel_hit_stop(fighter.module_accessor);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                galeforce_apply_effect(&mut *fighter.module_accessor, 1.0);
            }
        }
    }
}

unsafe extern "C" fn m2_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    m2_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, m2_frame);
}