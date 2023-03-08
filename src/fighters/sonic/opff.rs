use super::*;

#[fighter_frame( agent = FIGHTER_KIND_SONIC )]
fn coldsteel_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        //Sonic GA
        // jump cancel down air on hit when jump is held / pressed in hitlag
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") 
          && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
          && check_jump_input(fighter.module_accessor) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
            if fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        coldsteel_frame
    );
}