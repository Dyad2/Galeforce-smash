use super::*;

#[fighter_frame( agent = FIGHTER_KIND_FOX )]
fn space_furry_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        //GA: The Melee Special tm
        // type: cancel
        //  on hit, cancel shine with jump!
        if !is_operation_cpu(fighter.module_accessor) {
            if [*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) && VarModule::get_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER) < 20 {
                //check if the shine hits
                if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                    VarModule::on_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                }
            }
            //cleanup
            else {
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                VarModule::set_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER, 0);
            }
            //actual cancel happens here. Fox has jumps as long as shine hits opponents. Cannot change motions while in hitlag (hopefully)
            if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) /*&& !is_hitlag(fighter.module_accessor)*/ {
                VarModule::add_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER, 1);
                if check_jump_input(fighter.module_accessor) {
                    VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
                        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
                        if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() || fighter.sub_transition_group_check_ground_jump().get_bool() {
                            galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                        }
                    }
                    else {
                        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
                        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
                        if fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
                            galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                        }
                    }
                }
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        space_furry_frame
    );
}