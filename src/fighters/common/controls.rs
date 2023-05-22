
use super::*;

pub fn run(fighter : &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = get_kind(&mut *fighter.module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let prev_status_kind: i32 = StatusModule::prev_status_kind(fighter.module_accessor, 0);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        
        //C stick fixes cleanup, must be here because is_attack won't run in get_atack_air_kind at the correct time
        if is_status_damage(&mut *fighter.module_accessor) {
            VarModule::set_int(fighter.battle_object, commons::instance::int::SUBSTICK_AIR_ATTACK, 0);
        }
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) { //don't clear if c stick is held
            if situation_kind == *SITUATION_KIND_GROUND {
                if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4,*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
                *FIGHTER_TANTAN_STATUS_KIND_ATTACK_COMBO, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_RV, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_WAIT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WAIT].contains(&status_kind)
                  && (fighter.global_table[MOTION_FRAME].get_i32() >= 4 /*reset if it's not attack canceled and we're not near endlag*/ && FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(curr_motion_kind), false) - MotionModule::frame(fighter.module_accessor) >= 6.0) {
                    VarModule::set_int(fighter.battle_object, commons::instance::int::SUBSTICK_AIR_ATTACK, 0);
                }
                if fighter_kind == *FIGHTER_KIND_DEMON && curr_motion_kind == hash40("attack_lw3") && MotionModule::frame(fighter.module_accessor) <= 15.0 { //kazuya dtilt fix
                    VarModule::set_int(fighter.battle_object, commons::instance::int::SUBSTICK_AIR_ATTACK, 0);
                }
            }
            if situation_kind == *SITUATION_KIND_AIR {
                //if AttackModule::is_attack( fighter.module_accessor, 0, false) || AttackModule::is_attack( fighter.module_accessor, 1, false) //snake back air doesn't have a hitbox id 0, so checking 1 is necessary in case this also affects other characters
                //  || (MotionModule::frame(fighter.module_accessor) < 20.0 && [*FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_MURABITO, *FIGHTER_KIND_MIIGUNNER, *FIGHTER_KIND_TANTAN].contains(&fighter_kind)) {
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
                    VarModule::set_int(fighter.battle_object, commons::instance::int::SUBSTICK_AIR_ATTACK, 0);
                }
            }
        }
        else {
            ControlModule::get_attack_air_kind(fighter.module_accessor); //needs to be called here. can probably be moved to status?
        }
        //dtilt pivots, move to status or remove
        if ![*FIGHTER_STATUS_KIND_SQUAT_B].contains(&prev_status_kind) {
            if curr_motion_kind == hash40("attack_lw3") && curr_motion_kind != hash40("squat_b") && !AttackModule::is_attack(fighter.module_accessor, 0, false) {
                if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) < -0.25
                && VarModule::is_flag(fighter.battle_object, commons::instance::flag::ALLOW_REVERSE_ATTACK_LW3)  {
                    if fighter.global_table[MOTION_FRAME].get_i32() <= 3 {
                        VarModule::off_flag(fighter.battle_object, commons::instance::flag::ALLOW_REVERSE_ATTACK_LW3);
                        let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
                        FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, -1.0);
                        PostureModule::reverse_lr(fighter.module_accessor);
                        PostureModule::update_rot_y_lr(fighter.module_accessor);
                    }
                }
            }
            else {
                VarModule::on_flag(fighter.battle_object, commons::instance::flag::ALLOW_REVERSE_ATTACK_LW3);
            }
        }

        //aerial turn around
        // could still be simplified, but the current implementation seems mostly bug free.
        if situation_kind == SITUATION_KIND_AIR {
            let stick_x = fighter.global_table[STICK_X].get_f32();
            // println!("air_turn_invalid?: {}", VarModule::is_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INVALID));
            // println!("aerial_turn_input_method?: {}", VarModule::get_int(fighter.battle_object, commons::instance::int::AIR_TURN_INITIATE_METHOD));

            //this block is just for clarity. air_turn_invalid is in theory not necessary but it makes the code more readable
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) > 12 //because get_stick_flick runs when fighters are on the ground (of course) we don't want to forbid aerial turn if a player does a rar or something
             && (VarModule::get_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME) > 12 || VarModule::get_int(fighter.battle_object, commons::instance::int::AIR_TURN_COUNT) > 0) {
                VarModule::on_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INVALID);
            }
            else {
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INVALID);
            }

            if !VarModule::is_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INVALID) {
                //checks for input method
                if VarModule::get_int(fighter.battle_object, commons::instance::int::AIR_TURN_INITIATE_METHOD) == 0 {
                    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                        //second flick. its placed after the first check to avoid triggering both checks on the same frame
                        if VarModule::get_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME) != 0 && ControlModule::get_flick_x(fighter.module_accessor) < 1 && stick_x * PostureModule::lr(fighter.module_accessor) < -0.5 {
                            VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INITIATE_METHOD, 1) ;
                        }
                        //Acknowledge first stick flick. must be after the get_flick_x check so they won't both happen on the same frame
                        if (stick_x * PostureModule::lr(fighter.module_accessor)) < -0.5 {
                            //VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INITIATE_METHOD, 1) ;
                            VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME, 1);
                        } 
                        //turn with taunt
                        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) && PostureModule::lr(fighter.module_accessor) > 0.0
                         || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) && PostureModule::lr(fighter.module_accessor) < 0.0 {
                            VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INITIATE_METHOD, 2);
                            VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME, 1);
                        }
                    }
                }
                //turn here
                else if VarModule::get_int(fighter.battle_object, commons::instance::int::AIR_TURN_INITIATE_METHOD) == 1
                 || (VarModule::get_int(fighter.battle_object, commons::instance::int::AIR_TURN_INITIATE_METHOD) == 2 && VarModule::get_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME) > 4) { //method check
                    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) && MotionModule::end_frame(fighter.module_accessor) - fighter.global_table[MOTION_FRAME].get_f32() > 5.0 {
                        if !(fighter_kind == *FIGHTER_KIND_PICKEL && ([*FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP_AERIAL, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP_SQUAT].contains(&status_kind))) {
                            VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INITIATE_METHOD, 0) ;
                            VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME, 0);
                            VarModule::add_int(fighter.battle_object, commons::instance::int::AIR_TURN_COUNT, 1);
                            PostureModule::reverse_lr(fighter.module_accessor);
                            PostureModule::update_rot_y_lr(fighter.module_accessor);
                            //break off from tumble
                            if is_status_damage(&mut *fighter.module_accessor) {
                                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
                            }
                        }
                    }
                }
                if VarModule::get_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME) != 0 { //if we enter here it means we initiated either input method
                    VarModule::add_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME, 1);
                }
            }
            //resets on failed attempt (while airborne), allows to reenter initial flick check
            else if ControlModule::get_flick_x(fighter.module_accessor) < 1 {
                VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INITIATE_METHOD, 0) ;
                VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME, 0);
            }
        }
        //not in the air, reset
        else {
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INVALID);
            VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INITIATE_METHOD, 0) ;
            VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME, 0);
            VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_COUNT, 0);
        }

        //some buffer clears!
            //TODO move to status
        //clear on damage
        if StopModule::is_damage(fighter.module_accessor) && is_status_damage(&mut *fighter.module_accessor) { //clear in hitlag, while actually being hit (not on shield)
            ControlModule::clear_command(fighter.module_accessor, true);
        }

        let status_next = StatusModule::status_kind_next(fighter.module_accessor);
        if ([*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE_F].contains(&status_next) //clear when using roll / spotdodge or airdodge, and buffering another
        && ![*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&status_kind))
        && !((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0
          || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_SMASH) != 0
          || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0) {
            ControlModule::clear_command(fighter.module_accessor, true);
        }
    
        //var clearing
        if ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN].contains(&status_kind) {
            //perfect pivots fixes
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::SMASH_TURN);
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::ALLOW_PERFECT_PIVOT);
        }
    }
}