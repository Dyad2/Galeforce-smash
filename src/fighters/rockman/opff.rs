use super::*;

#[fighter_frame( agent = FIGHTER_KIND_ROCKMAN )]
fn RockNRoll_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

        //GA - Sliding jump
            //type: cancel
            //hit an opponent with down tilt and press jump!
        if !is_operation_cpu(fighter.module_accessor) {
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_lw3") {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && !is_hitlag(fighter.module_accessor) {
                    VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                }
                if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) && VarModule::get_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER) < 20 {
                    VarModule::add_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER, 1);
                    if (ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
                        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
                        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
                        fighter.sub_transition_group_check_ground_jump_mini_attack();
                        fighter.sub_transition_group_check_ground_jump();
                        galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                        VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                    }
                }
            }
            else {
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                VarModule::set_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER, 0);
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        RockNRoll_frame
    );
}