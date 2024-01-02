use super::*;

//Author: Wuboy (i presume)
unsafe extern "C" fn bayonetta_reset_landing(fighter: &mut L2CFighterCommon) {
    bayonetta_reset_abk(fighter);
    bayonetta_reset_witchtwist(fighter);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME);
}

unsafe extern "C" fn bayonetta_reset_abk(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_REUSE_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
}

unsafe extern "C" fn bayonetta_reset_witchtwist(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USED_COUNT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AFTER_ACTION);
}

unsafe extern "C" fn bayo_specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
      && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
      && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
      && fighter.global_table[MOTION_FRAME].get_i32() <= 32 {
        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_S_HOLD_END.into(), false.into());
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
            if !MotionModule::is_end(fighter.module_accessor) {
                handle_special_s(fighter);
                return 0.into();
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT_CANCEL_OK);
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), false.into());
        }
    }
    else {
        if PostureModule::lr(fighter.module_accessor) <= 0.0 {
            if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) {
                fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END.into(), false.into());
            }
        }
        else {
            if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
                fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END.into(), false.into());
            }
        }
    }
    return 0.into();
}

unsafe extern "C" fn handle_special_s(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);

    if status != *FIGHTER_STATUS_KIND_SPECIAL_S {
        if status == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_S_HOLD_END {
            somefunc(fighter, false);
            return;
        }
        if status != *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U {
            return;
        }
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_INT_STEP) == *FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP) != *FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP) != *FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING {
                    let motion_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLOAT_MOTION_SPEED_X) * WorkModule::get_param_float(fighter.module_accessor, 0xfea97fe73, hash40("ab_u_shooting_speed_x_mul"));
                    let motion_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLOAT_MOTION_SPEED_Y) * WorkModule::get_param_float(fighter.module_accessor, 0xfea97fe73, hash40("ab_u_shooting_speed_y_mul"));
                    let kinetic_energy_stop = std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP));
                    let kinetic_energy_stop_normal = std::mem::transmute::<u64, &mut smash::app::KineticEnergyNormal>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP));

                    KineticEnergy::reset_energy(kinetic_energy_stop, 0, &Vector2f{x: 0.0, y: 0.0}, &Vector3f{x: 0.0, y: 0.0, z:0.0}, fighter.module_accessor);
                    KineticEnergyNormal::set_accel(kinetic_energy_stop_normal, &Vector2f{x: motion_x, y: motion_y});

                    let ab_u_shoot_brake = WorkModule::get_param_float(fighter.module_accessor, 0xfea97fe73, hash40("ab_u_shooting_brake_speed_x"));
                    KineticEnergyNormal::set_brake(kinetic_energy_stop_normal, &Vector2f{x: ab_u_shoot_brake, y: 0.0});
                    
                    let ab_u_stable_speed = WorkModule::get_param_float(fighter.module_accessor, 0xfea97fe73, hash40("ab_u_shooting_stable_speed_x"));
                    KineticEnergyNormal::set_stable_speed(kinetic_energy_stop_normal, &Vector2f{x: ab_u_stable_speed, y: 0.0});
                    KineticEnergyNormal::set_limit_speed(kinetic_energy_stop_normal, &Vector2f{x: -1.0, y: -1.0});
                    KineticEnergy::enable(kinetic_energy_stop);

                    let kinetic_energy_grav = std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY));
                    let fighter_kinetic_energy_grav = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyGravity>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY));
                    KineticEnergy::reset_energy(kinetic_energy_grav, 0, &Vector2f{x: 0.0, y: 0.0}, &Vector3f{x: 0.0, y: 0.0, z:0.0}, fighter.module_accessor);
                    
                    let ab_u_gravity = WorkModule::get_param_float(fighter.module_accessor, 0xfea97fe73, hash40("ab_u_shooting_accel_y"));
                    FighterKineticEnergyGravity::set_accel(fighter_kinetic_energy_grav, ab_u_gravity);
                    let ab_u_gravity_stable = WorkModule::get_param_float(fighter.module_accessor, 0xfea97fe73, hash40("ab_u_shooting_max_speed_y"));
                    FighterKineticEnergyGravity::set_stable_speed(fighter_kinetic_energy_grav, ab_u_gravity_stable);
                    KineticEnergy::enable(kinetic_energy_grav);

                    let kinetic_energy_motion = std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
                    KineticEnergy::unable(kinetic_energy_motion);
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING, WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_INT_STEP));
                }
                else {
                    WorkModule::set_float(fighter.module_accessor, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN), *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLOAT_MOTION_SPEED_X);
                    WorkModule::set_float(fighter.module_accessor, KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN), *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_FLOAT_MOTION_SPEED_X);
                }
            }
        }
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_INT_STEP) == *FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP) != *FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING {
                let kinetic_energy_stop = std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP));
                let speed_x = KineticEnergy::get_speed_x(kinetic_energy_stop).abs();
                let speed_y = KineticEnergy::get_speed_y(kinetic_energy_stop);

                let ab_u_stable_speed= WorkModule::get_param_float(fighter.module_accessor, 0xfea97fe73, hash40("ab_u_shooting_stable_speed_x"));

                if ab_u_stable_speed <= speed_x {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT_END, WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_U_INT_STEP));
                }
            }
        }
        else {
            somefunc(fighter, true);
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT) {
                //idk wtf happens here?
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT_CANCEL_OK) {
                return;
            }
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_INT_ENABLE_HIT_CANCEL_FRAME) <= WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_INT_FRAME) {
                CancelModule::enable_cancel(fighter.module_accessor);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT_CANCEL_OK);
            }
        }
    }
}

unsafe extern "C" fn somefunc(fighter: &mut L2CFighterCommon, check_cliff: bool) {
    let pos = PostureModule::pos(fighter.module_accessor);

    //these are very likely not right idk ghidra is a fuck
    let vec2fa = Vector2f{x: (*pos).x, y: (*pos).y + 0.1};
    let vec2fb = Vector2f{x: 0.0, y: 0.0};
    let mut vec2fc = Vector2f{x: 0.0, y: 0.0};
    let mut vec2fd = Vector2f{x: 0.0, y: 0.0};

    //handles bayo's rotation on slopes maybe?
    if GroundModule::ray_check_hit_pos_normal(fighter.module_accessor, &vec2fa, &vec2fb, &mut vec2fc, &mut vec2fd, true) == 1 {
        if !(smash::app::sv_math::vec2_distance(vec2fa.x, vec2fa.y, vec2fc.x,vec2fc.y) <= 1.0) {
            let posture = PostureModule::lr(fighter.module_accessor);
            let mut angle = 0.0;
            if !BattleObjectWorld::is_gravity_normal(singletons::BattleObjectWorld()) {
                //there's stuff missing here but i dont understand it
                angle = smash::app::SlopeModuleSimple::gravity_angle(fighter.module_accessor);
            }
            WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLOAT_GROUND_ANGLE);
        }
    }
    if check_cliff {
        if !GroundModule::is_ottotto(fighter.module_accessor, 10.0) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_NEAR_CLIFF);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_NEAR_CLIFF);
        }
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP) == *FIGHTER_BAYONETTA_SHOOTING_STEP_SHOOTING {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_SHOOTING_SPEED_MUL);
    }
    return;
}

unsafe extern "C" fn status_specialsmain(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_BAYONETTA_SPECIAL_S);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0x976c3b29b), 0.0, 1.0, false, 0.0, false, false);
    bayonetta_reset_landing(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(bayo_specials_main_loop as *const () as _))
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_S, status_specialsmain);
}