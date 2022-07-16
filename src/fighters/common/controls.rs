            // //C stick fixes cleanup, must be here because is_attack won't run in get_atack_air_kind at the correct time
            // if is_status_damage(&mut *fighter.module_accessor) {
            //     WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_INSTANCE_WORK_ID_INT_SUBSTICK_AIR_ATTACK);
            // }
            // if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) { //don't clear if c stick is held
            //     if situation_kind == *SITUATION_KIND_GROUND {
            //         if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4,*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            //         *FIGHTER_TANTAN_STATUS_KIND_ATTACK_COMBO, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_RV, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_WAIT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WAIT].contains(&status_kind)
            //           && (MotionModule::frame(fighter.module_accessor) >= 4.0 /*reset if it's not attack canceled and we're not near endlag. should be corrected_frame probably*/ && FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(curr_motion_kind), false) - MotionModule::frame(fighter.module_accessor) >= 6.0) {
            //             WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_INSTANCE_WORK_ID_INT_SUBSTICK_AIR_ATTACK);
            //         }
            //         if fighter_kind == *FIGHTER_KIND_DEMON && curr_motion_kind == hash40("attack_lw3") && MotionModule::frame(fighter.module_accessor) <= 15.0 { //kazuya dtilt fix
            //             WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_INSTANCE_WORK_ID_INT_SUBSTICK_AIR_ATTACK);
            //         }
            //     }
            //     if situation_kind == *SITUATION_KIND_AIR {
            //         if AttackModule::is_attack( fighter.module_accessor, 0, false) || AttackModule::is_attack( fighter.module_accessor, 1, false) //snake back air doesn't have a hitbox id 0, so checking 1 is necessary in case this also affects other characters
            //           || (MotionModule::frame(fighter.module_accessor) < 20.0 && [*FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_MURABITO, *FIGHTER_KIND_MIIGUNNER, *FIGHTER_KIND_TANTAN].contains(&fighter_kind)) {
            //             WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_INSTANCE_WORK_ID_INT_SUBSTICK_AIR_ATTACK);
            //           }
            //     }
            // }
            // //c stick fix 
            // if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            //     ControlModule::get_attack_air_kind(fighter.module_accessor);
            // }

            // //dtilt pivots
            // if ![*FIGHTER_STATUS_KIND_SQUAT_B].contains(&prev_status_kind) {
            //     if curr_motion_kind == hash40("attack_lw3") && curr_motion_kind != hash40("squat_b") {
            //         if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) < -0.25
            //         && MotionModule::frame(fighter.module_accessor) < 4.0 && WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_ALLOW_REVERSE_ATTACK_LW3) && !AttackModule::is_attack( fighter.module_accessor, 0, false) {
            //             WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_ALLOW_REVERSE_ATTACK_LW3);
            //             let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
            //             FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, -1.0);
            //             PostureModule::reverse_lr(fighter.module_accessor);
            //             PostureModule::update_rot_y_lr(fighter.module_accessor);
            //         }
            //     }
            //     else {
            //         VarModule::on_flag(fighter.battle_object, _INSTANCE_WORK_ID_FLAG_ALLOW_REVERSE_ATTACK_LW3);
            //     }
            // }