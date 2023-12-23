use super::*;

unsafe extern "C" fn luigi_specials_main(fighter: &mut L2CFighterCommon) {

    handle_situation(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_specials_main_loop as *const () as _));
}

unsafe extern "C" fn luigi_specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let elec_charge = VarModule::get_int(fighter.battle_object, luigi::instance::int::ELEC_CHARGE);

    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        VarModule::on_flag(fighter.battle_object, luigi::status::flag::SPECIAL_S_SPECIAL_BUTTON_RELEASED);
    }

    if elec_charge != 0
      && VarModule::is_flag(fighter.battle_object, luigi::status::flag::SPECIAL_S_SPECIAL_BUTTON_RELEASED)
      && !VarModule::is_flag(fighter.battle_object, luigi::status::flag::SPECIAL_S_CHARGE_USED)
      && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
      && fighter.global_table[MOTION_FRAME].get_i32() < 14 {

        VarModule::on_flag(fighter.battle_object, luigi::status::flag::SPECIAL_S_CHARGE_USED);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_BONUS);
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_FLAG_REVERSE_LR) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_FLAG_REVERSE_LR);
            let stick_x = fighter.global_table[STICK_X].get_f32();
            let param_reverse_lr = WorkModule::get_param_float(fighter.module_accessor, 0xfea97fe73, hash40("lr_stick_x"));

            if param_reverse_lr < stick_x {
                PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
            }
            let mut situation_changed = false;
            if !StatusModule::is_changing(fighter.module_accessor) {
                let mtrans = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_INT_MTRANS);
                if mtrans != fighter.global_table[PREV_SITUATION_KIND] { //is that right?
                    if mtrans == fighter.global_table[SITUATION_KIND] {
                        situation_changed = true;
                    }
                }
            }
            if situation_changed {
                handle_situation(fighter);
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE.into(), false.into());
    }
    return 0.into()
}

pub unsafe fn handle_situation(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S);
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_FLAG_FIRST) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_FLAG_FIRST);
        }
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_LUIGI_STATUS_SPECIAL_S_INT_MTRANS);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_AIR_S);
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_FLAG_FIRST) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_FLAG_FIRST);
        }
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_LUIGI_STATUS_SPECIAL_S_INT_MTRANS);
    }
}

unsafe extern "C" fn luigi_specialscharge_end(fighter: &mut L2CFighterCommon) -> L2CValue {

    if VarModule::get_int(fighter.battle_object, luigi::instance::int::ELEC_CHARGE) >= 4 && VarModule::is_flag(fighter.battle_object, luigi::status::flag::SPECIAL_S_CHARGE_USED) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE);
    }
    EffectModule::remove_common(fighter.module_accessor, Hash40::new("0xaec2db62e")); //charge effect probably
    if StatusModule::status_kind_next(fighter.module_accessor) == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_BREATH {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM.into(), false.into());
    }
    return 0.into();
}

unsafe extern "C" fn luigi_specialsram_end(fighter: &mut L2CFighterCommon) -> L2CValue {

    if VarModule::is_flag(fighter.battle_object, luigi::status::flag::SPECIAL_S_CHARGE_USED) {
        VarModule::set_int(fighter.battle_object, luigi::instance::int::ELEC_CHARGE, 0);
    }
    return 0.into();
}

unsafe extern "C" fn luigi_specialsram_init(fighter: &mut L2CFighterCommon) -> L2CValue {

    if VarModule::is_flag(fighter.battle_object, luigi::status::flag::SPECIAL_S_CHARGE_USED) {
        galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
    }
    GroundModule::set_test_coll_stop_status(fighter.module_accessor, true);
    return 0.into();
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_S, luigi_specials_main);
    agent.status(smashline::End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, luigi_specialscharge_end);
    agent.status(smashline::End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_specialsram_end);
    agent.status(smashline::Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_specialsram_init);
}