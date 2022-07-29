
use super::*;

#[fighter_frame_callback]
fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = get_kind(&mut *fighter.module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let prev_status_kind: i32 = StatusModule::prev_status_kind(fighter.module_accessor, 0);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        TOTAL_FIGHTER = FighterManager::total_fighter_num(singletons::FighterManager());

        //reset parameters when match ends or is reset
        // if !FighterManager::is_ready_go(FIGHTER_MANAGER) || status_kind == *FIGHTER_STATUS_KIND_DEAD || KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_RESET {
        //     WorkModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
        //     WorkModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM);
        //     WorkModule::off_flag(fighter.module_accessor, commons::instance::flag::DISABLE_SPECIAL_S);
        //     WorkModule::off_flag(fighter.module_accessor, commons::instance::flag::DISABLE_SPECIAL_HI);
        //     WorkModule::off_flag(fighter.module_accessor, commons::instance::flag::DISABLE_SPECIAL_ALL);
        //     WorkModule::off_flag(fighter.module_accessor, commons::instance::flag::PURIN_MARK);
        //     //levin when match begins
        //     // bugged? robin gets sword back for no reason sometimes
        //     if fighter_kind == *FIGHTER_KIND_REFLET {
        //         WorkModule::set_int(fighter.module_accessor, 8, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
        //     }
        //     if fighter_kind == *FIGHTER_KIND_MARIOD {
        //         WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_GA_MEDECINE_TIMER);
        //     }
        // }
        
        if !FighterManager::is_ready_go(singletons::FighterManager()) || status_kind == *FIGHTER_STATUS_KIND_DEAD || KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_RESET {
            CustomVarManager::reset_var_module(fighter.battle_object);
            //levin when match begins. maybe bugged? robin gets sword back for no reason sometimes
            if fighter_kind == *FIGHTER_KIND_REFLET {
                WorkModule::set_int(fighter.module_accessor, 8, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
            }
        }

        edge_cancels(fighter, status_kind, situation_kind, fighter_kind);
        ecb_shifts(fighter);
        galeforce::attacks(fighter, status_kind);

        if status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
        }
        
        //"thou shalt not crossup with thy shit burst"
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
            FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, 0.25);
        }

        //let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        //removing roll movement
        // if curr_motion_kind == hash40("escape_f") || curr_motion_kind == hash40("escape_b") {
        //     FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, 0.0);
        // }
        
        if entry_id == 0 { //run only once
            for fighter1 in 0 .. TOTAL_FIGHTER {
                let mut jostle_on = false;

                let fighter1_boma = get_boma(fighter1);
                let status_kind1 = StatusModule::status_kind(fighter1_boma);
                let curr_motion_kind1 = MotionModule::motion_kind(fighter1_boma);
                let fighter_kind1 = get_kind(&mut *sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(fighter1)));
                for fighter2 in 0 .. TOTAL_FIGHTER {
                    let fighter2_boma = get_boma(fighter2);
                    let status_kind2 = StatusModule::status_kind(fighter2_boma);
                    let curr_motion_kind2 = MotionModule::motion_kind(fighter2_boma);
                    let fighter_kind2 = get_kind(&mut *sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(fighter2)));

                    if fighter2 > fighter1 { //if fighter 2 is smaller, the loop has already ran for that player pair. skip.
                        if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_FURAFURA_STAND, *FIGHTER_STATUS_KIND_SLEEP, *FIGHTER_STATUS_KIND_BIND, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_TURN].contains(&status_kind1) 
                          || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_FURAFURA_STAND, *FIGHTER_STATUS_KIND_SLEEP, *FIGHTER_STATUS_KIND_BIND, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_TURN].contains(&status_kind2)
                          || [hash40("attack11"), hash40("attack_100_start"), hash40("attack100"), hash40("attack_100_end"), hash40("attack12"), hash40("attack13"), hash40("attacks3"), hash40("attacks3hi"), hash40("attacks3lw"), hash40("attacks32"), hash40("attacks33"), hash40("attackdash")].contains(&curr_motion_kind1)
                          || [hash40("attack11"), hash40("attack_100_start"), hash40("attack100"), hash40("attack_100_end"), hash40("attack12"), hash40("attack13"), hash40("attacks3"), hash40("attacks3hi"), hash40("attacks3lw"), hash40("attacks32"), hash40("attacks33"), hash40("attackdash")].contains(&curr_motion_kind2)
                          || [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_GANON, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_SNAKE].contains(&fighter_kind1) 
                          || [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_GANON, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_SNAKE].contains(&fighter_kind2) {
                            jostle_on = true;
                        }
                        if jostle_on {
                            if (PostureModule::pos_x(fighter1_boma) - PostureModule::pos_x(fighter2_boma)).abs() < 14.0 && (PostureModule::pos_y(fighter1_boma) - PostureModule::pos_y(fighter2_boma)).abs() < 20.0 {
                                JostleModule::set_team(fighter1_boma,  fighter1 + 1);
                                JostleModule::set_team(fighter2_boma,  fighter2 + 1);
                            }
                        }
                        else {
                            JostleModule::set_team(fighter1_boma, 0);
                            JostleModule::set_team(fighter2_boma, 0);
                        }
                    }
                }
            }
        }

        if !is_operation_cpu(fighter.module_accessor) {
            //C stick fixes cleanup, must be here because is_attack won't run in get_atack_air_kind at the correct time
            if is_status_damage(&mut *fighter.module_accessor) {
                VarModule::set_int(fighter.battle_object, commons::instance::int::SUBSTICK_AIR_ATTACK, 0);
            }
            if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) { //don't clear if c stick is held
                if situation_kind == *SITUATION_KIND_GROUND {
                    if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4,*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
                    *FIGHTER_TANTAN_STATUS_KIND_ATTACK_COMBO, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_RV, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_WAIT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WAIT].contains(&status_kind)
                      && (fighter.global_table[MOTION_FRAME].get_f32() >= 4.0 /*reset if it's not attack canceled and we're not near endlag*/ && FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(curr_motion_kind), false) - MotionModule::frame(fighter.module_accessor) >= 6.0) {
                        VarModule::set_int(fighter.battle_object, commons::instance::int::SUBSTICK_AIR_ATTACK, 0);
                    }
                    if fighter_kind == *FIGHTER_KIND_DEMON && curr_motion_kind == hash40("attack_lw3") && MotionModule::frame(fighter.module_accessor) <= 15.0 { //kazuya dtilt fix
                        VarModule::set_int(fighter.battle_object, commons::instance::int::SUBSTICK_AIR_ATTACK, 0);
                    }
                }
                if situation_kind == *SITUATION_KIND_AIR {
                    if AttackModule::is_attack( fighter.module_accessor, 0, false) || AttackModule::is_attack( fighter.module_accessor, 1, false) //snake back air doesn't have a hitbox id 0, so checking 1 is necessary in case this also affects other characters
                      || (MotionModule::frame(fighter.module_accessor) < 20.0 && [*FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_MURABITO, *FIGHTER_KIND_MIIGUNNER, *FIGHTER_KIND_TANTAN].contains(&fighter_kind)) {
                        VarModule::set_int(fighter.battle_object, commons::instance::int::SUBSTICK_AIR_ATTACK, 0);
                      }
                }
            }
            //c stick fix 
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                ControlModule::get_attack_air_kind(fighter.module_accessor);
            }

            //dtilt pivots
            if ![*FIGHTER_STATUS_KIND_SQUAT_B].contains(&prev_status_kind) {
                if curr_motion_kind == hash40("attack_lw3") && curr_motion_kind != hash40("squat_b") && !AttackModule::is_attack(fighter.module_accessor, 0, false) {
                    if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) < -0.25
                    && VarModule::is_flag(fighter.battle_object, commons::instance::flag::ALLOW_REVERSE_ATTACK_LW3)  {
                        if fighter.global_table[MOTION_FRAME].get_f32() <= 4.0 {
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

            //wavedashes during jumpsquat and footstools
            if (status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT || status_kind ==  *FIGHTER_STATUS_KIND_TREAD_JUMP) && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && (ControlModule::get_stick_x(fighter.module_accessor).abs() >= 0.15 || ControlModule::get_stick_y(fighter.module_accessor).abs() >= 0.15) {
                StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                //StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                MotionModule::change_motion_kind(fighter.module_accessor, smash::phx::Hash40{hash: hash40("escape_air_slide")});
            }
            
            //aerial turn around
            if situation_kind == SITUATION_KIND_AIR {
                //checks if the fighter should be allowed to turn
                if VarModule::get_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME) > 10 || VarModule::get_int(fighter.battle_object, commons::instance::int::AIR_TURN_COUNT) >= 2 {
                    VarModule::on_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INVALID);
                }
                //is okey
                else {
                    VarModule::off_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INVALID);
                }
                //turn with stick
                if VarModule::is_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INITIATE) && !VarModule::is_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_APPEAL_METHOD_INITIATE) && !VarModule::is_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INVALID) {
                    if ControlModule::get_flick_x(fighter.module_accessor) < 1 {
                        VarModule::on_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_STICK_RELEASED);
                    }
                    //turn!
                    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
                      && VarModule::is_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_STICK_RELEASED) 
                      && ((ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor)) < -0.25)
                      && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
                        PostureModule::reverse_lr(fighter.module_accessor);
                        PostureModule::update_rot_y_lr(fighter.module_accessor);
                        VarModule::off_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_STICK_RELEASED);
                        VarModule::off_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INITIATE);
                        VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME, 0);
                        VarModule::add_int(fighter.battle_object, commons::instance::int::AIR_TURN_COUNT, 1);
                        if is_status_damage(&mut *fighter.module_accessor) {
                            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
                        }
                    }
                }
                //Acknowledge first stick flick. must be after the get_flick_x check so they won't both happen on the same frame
                if (ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor)) < -0.25 && !VarModule::is_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INITIATE) {
                    VarModule::on_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INITIATE);
                    VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME, 0);
                }
                //turn with taunt
                if !VarModule::is_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_APPEAL_METHOD_INITIATE) {
                    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) && PostureModule::lr(fighter.module_accessor) > 0.0
                     || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) && PostureModule::lr(fighter.module_accessor) < 0.0 {
                        VarModule::on_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_APPEAL_METHOD_INITIATE);
                        VarModule::off_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INITIATE);
                    }
                }
                //turn!
                else { //necessary check so air_input int check won't trigger at the wrong time
                    if VarModule::get_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME) > 3 
                       && !VarModule::is_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INVALID)
                       && !ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
                       && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
                        PostureModule::reverse_lr(fighter.module_accessor);
                        PostureModule::update_rot_y_lr(fighter.module_accessor);
                        VarModule::add_int(fighter.battle_object, commons::instance::int::AIR_TURN_COUNT, 1);
                        VarModule::off_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_APPEAL_METHOD_INITIATE);
                        VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME, 0);
                        if is_status_damage(&mut *fighter.module_accessor) {
                            StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
                        }
                    }
                }
                if VarModule::is_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INITIATE) || VarModule::is_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_APPEAL_METHOD_INITIATE) {
                    VarModule::add_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME, 1);
                }
                //resets on failed attempt, allows to reenter initial flick check
                if VarModule::is_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INVALID) && ControlModule::get_flick_x(fighter.module_accessor) < 1 {
                    VarModule::off_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INITIATE);
                }
            }
            //reset
            else {
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INVALID);
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_INITIATE);
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_STICK_RELEASED);
                VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME, 0);
                VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_COUNT, 0);
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_APPEAL_METHOD_INITIATE);
            }
            //prevents turning while attacking with standard method. shouldn't be required?
            // if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && !VarModule::is_flag(fighter.battle_object, commons::instance::flag::AIR_TURN_APPEAL_METHOD_INITIATE) {
            //     VarModule::set_int(fighter.battle_object, commons::instance::int::AIR_TURN_INPUT_FRAME, 0);
            //     VarModule::off_flag(fighter.battle_object,commons::instance::flag::AIR_TURN_APPEAL_METHOD_INITIATE);
            // }

            //shield dropping, use shield + b + down to drop
            if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS) != 0 && GroundModule::is_passable_ground(fighter.module_accessor) && 
            [hash40("guard"), hash40("guard_on"), hash40("shield_guard")].contains(&curr_motion_kind) {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
            }
            //some buffer clears!
                //TODO move to lib.rs
            //clear on damage
            if StopModule::is_damage(fighter.module_accessor) && is_status_damage(&mut *fighter.module_accessor) { //clear in hitlag, while actually being hit (not on shield)
                ControlModule::clear_command(fighter.module_accessor, true);
            }
            
            let status_next = StatusModule::status_kind_next(fighter.module_accessor);
            if ((VarModule::is_flag(fighter.battle_object, commons::instance::flag::WAVEDASH)) //clear when using dairdodge, to prevent buffering rolls when wavedashing
            || ([*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE_F].contains(&status_next) //clear when using roll / spotdodge and buffering another
            && ![hash40("guard"), hash40("guard_on"), hash40("shield_guard")].contains(&curr_motion_kind)))
            && !((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0
              || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_SMASH) != 0
              || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0) {
                ControlModule::clear_command(fighter.module_accessor, true);
            }
        }

        //Wavedash jump/shield cancel var
        if curr_motion_kind == hash40("landing_heavy") && VarModule::is_flag(fighter.battle_object, commons::instance::flag::ESCAPE_AIR_IS_SLIDE) {
            VarModule::on_flag(fighter.battle_object, commons::instance::flag::WAVEDASH);
        }
        else if status_kind != *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::WAVEDASH);
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::ESCAPE_AIR_IS_SLIDE)
        }
    }
}

#[weapon_frame_callback]
fn global_weapon_frame(fighter_base : &mut L2CFighterBase) {
    unsafe {
        let weapon_kind = get_kind(&mut *fighter_base.module_accessor);
        let status_kind = StatusModule::status_kind(fighter_base.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter_base.module_accessor);

        let weapon =  mem::transmute::<&mut BattleObject, &mut smash::app::Weapon>(smash::app::sv_system::battle_object(fighter_base.lua_state_agent));
        let owner_id = smash::app::lua_bind::Weapon::get_founder_id(weapon) as u32;

        //weapons using owner_fighter.module_accessor stuff
        //let owner_id = smash::app::lua_bind::WorkModule::get_int(module_accessor, WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
        if !smash::app::sv_battle_object::is_null(owner_id) && smash::app::sv_battle_object::is_active(owner_id) {
            let owner_boma = &mut *sv_battle_object::module_accessor(owner_id);
            let owner_status = StatusModule::status_kind(owner_boma);
            //let fighter = get_fighter_common_from_accessor(owner_boma);
            
            let owner_object_id = owner_boma.battle_object_id;
            let owner_object = get_battle_object_from_id(owner_object_id);

            //rosalina
            if weapon_kind == *WEAPON_KIND_ROSETTA_TICO {
                //return if rosa cancels special n charge
                if VarModule::is_flag(owner_object, rosetta::instance::flag::TICO_RECALL) {
                    WorkModule::on_flag(owner_boma, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_RETURN);
                    VarModule::off_flag(owner_object, rosetta::instance::flag::TICO_RECALL);
                }
                //recover from helplessness
                if status_kind == *WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FALL {
                    if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&owner_status) {
                        WorkModule::on_flag(fighter_base.module_accessor, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_RETURN);
                    }
                }
                if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&owner_status) || WorkModule::is_flag(owner_boma, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
                    HitModule::set_whole(fighter_base.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                }
            }

            //zelda
            if !is_operation_cpu( owner_boma) {
                if weapon_kind == *WEAPON_KIND_ZELDA_DEIN_S {
                    if AttackModule::is_infliction(fighter_base.module_accessor, *COLLISION_KIND_MASK_HIT) {
                        VarModule::on_flag(owner_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                    }
                }
                if weapon_kind == *WEAPON_KIND_ZELDA_PHANTOM {
                    //attempt at restoring 9.0.1 behavior. base offset was changed to detect ground when the phantom is higher up, and then it is set back down here
                    if situation_kind == *SITUATION_KIND_GROUND {
                        VarModule::set_float(fighter_base.battle_object, commons::instance::float::ECB_OFFSET_Y, 7.0);
                        GroundModule::set_rhombus_offset(fighter_base.module_accessor, &Vector2f{x : 0.0, y : VarModule::get_float(fighter_base.battle_object, commons::instance::float::ECB_OFFSET_Y)});
                    }
                    //ecb_shifts(fighter, status_kind, situation_kind, weapon_kind);
                    if !ReflectModule::is_reflect(fighter_base.module_accessor) {
                        if ![*WEAPON_ZELDA_PHANTOM_STATUS_KIND_ATTACK, *WEAPON_ZELDA_PHANTOM_STATUS_KIND_DISAPPEAR].contains(&status_kind)
                           && WorkModule::is_flag(fighter_base.module_accessor, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_FLAG_CANCEL) 
                           && !is_status_damage(owner_boma)
                           && WorkModule::get_float(fighter_base.module_accessor, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_FLOAT_HP) > 0.0 {
                            StatusModule::change_status_request(fighter_base.module_accessor, *WEAPON_ZELDA_PHANTOM_STATUS_KIND_BUILD, true);
                        }
                        if is_status_damage(owner_boma) || KineticModule::get_kinetic_type(fighter_base.module_accessor) == *WEAPON_KINETIC_TYPE_RESET {
                            WorkModule::set_float(fighter_base.module_accessor, 0.0, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_FLOAT_HP);
                            WorkModule::on_flag(fighter_base.module_accessor, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_FLAG_CANCEL);
                        }
                    }
                }
            }

            //inkling
        //     if weapon_kind == *WEAPON_KIND_INKLING_INKBULLET {
        //         if MotionModule::motion_kind(owner_fighter.module_accessor) == hash40("attack_air_b") {
        //             println!("bullet in back air");
        //                 println!("bullet life: {}", WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE));
        //                 //println!("bullet life (init): {}", WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE));
        //             if WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE) == 11 { //do it once
        //                 println!("bullet revert");
        //                 let revert  = smash::phx::Vector3f { x: KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * -2., y: 0.0, z: 0.0 };
        //                 KineticModule::add_speed(fighter.module_accessor, &revert);
        //                 PostureModule::set_lr(fighter.module_accessor, -1.0 * PostureModule::lr(owner_fighter.module_accessor));
        //                 PostureModule::update_rot_y_lr(fighter.module_accessor);
        //             }
        //         }
        //         if MotionModule::motion_kind(owner_fighter.module_accessor) == hash40("attack_air_lw") {
        //             println!("bullet in down air");
        //             if WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE) == 11 {
        //                 println!("bullet drop");
        //                 let drop  = smash::phx::Vector3f { x: KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * -1., y: KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN), z: 0.0 };
        //                 KineticModule::add_speed(fighter.module_accessor, &drop);
        //             }
        //         }
        //     }
        // }
        }
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame, global_weapon_frame
    );
    status::install();
}