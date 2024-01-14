use super::*;
use crate::fighter::common::opff::common_fighter_frame;

//GA - Sliding jump
    //type: cancel
    //hit an opponent with down tilt and press jump!
unsafe extern "C" fn rockman_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    
    if !is_operation_cpu(fighter.module_accessor) {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_lw3") {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && !is_hitlag(fighter.module_accessor) {
                VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
            }
            if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) && VarModule::get_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER) < 20 {
                VarModule::add_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER, 1);
                if (ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
                    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
                    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
                    fighter.sub_transition_group_check_ground_jump_mini_attack();
                    fighter.sub_transition_group_check_ground_jump();
                    galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                    VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                }
            }
        }
        else {
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
            VarModule::set_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER, 0);
        }
    }
}

unsafe extern "C" fn rockman_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    rockman_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, rockman_frame);
}