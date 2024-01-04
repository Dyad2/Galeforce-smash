use super::*;
use crate::fighters::common::opff::common_fighter_frame;

unsafe extern "C" fn rosetta_stuff(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

    //todo: allow rosa to use this also in the air
    if curr_motion_kind == hash40("special_n_charge") || curr_motion_kind == hash40("special_air_n_charge") {
        if (ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
            VarModule::on_flag(fighter.battle_object, rosetta::instance::flag::TICO_RECALL);
            if situation_kind == SITUATION_KIND_GROUND {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
                fighter.sub_transition_group_check_ground_jump_mini_attack();
                fighter.sub_transition_group_check_ground_jump();
            }
            else if situation_kind == SITUATION_KIND_AIR {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
                fighter.sub_transition_group_check_air_jump_aerial();
            }
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            VarModule::on_flag(fighter.battle_object, rosetta::instance::flag::TICO_RECALL);
            if situation_kind == SITUATION_KIND_GROUND {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, false);
            }
            else if situation_kind == SITUATION_KIND_AIR {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
            }
        }
    }
}

unsafe extern "C" fn rosetta_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    rosetta_stuff(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, rosetta_frame);
}