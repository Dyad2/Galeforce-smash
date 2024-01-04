use super::*;
use crate::fighters::common::opff::common_fighter_frame;

unsafe extern "C" fn yoshi_specials_jump(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    
    if curr_motion_kind == hash40("special_s_loop") || curr_motion_kind == hash40("special_air_s_loop") {
        if (ControlModule::is_enable_flick_jump(fighter.module_accessor) 
           && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 
           && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
            }
            else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_END, false);
            }
    }
}

unsafe extern "C" fn yoshi_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    yoshi_specials_jump(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, yoshi_frame);
}