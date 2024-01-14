use super::*;
use crate::fighter::common::opff::common_fighter_frame;

unsafe extern "C" fn gekkouga_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    
    //GA - Protean
    // type: Cancel, then Buff
    // Greninja can decide to not use the substitute counter attack to get the protean buff instead. while the buff is active, the next use of a water or dark move is more powerful
    if status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_HIT {
        if MotionModule::frame(fighter.module_accessor) <= 3.0 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
            StopModule::cancel_hit_stop(fighter.module_accessor);
            galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
        }
    }
    //allows greninja to disappear, then cancel the attack
    else if status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_ATTACK {
        if MotionModule::frame(fighter.module_accessor) >= 1.0 /*attempt to fix a speed bug that teleports greninja upwards */ && VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            VarModule::set_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER, 10);
            if situation_kind == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
            }
            else {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
            }
        }
    }
    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
        //cleanup
        if VarModule::get_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER) > 0 {
            VarModule::sub_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER, 1);
            HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
        }
        else {
            HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
        }
        //water shuriken
        if WorkModule::get_float(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE) > 0.1 {
            VarModule::set_float(fighter.module_accessor, gekkouga::instance::float::SHURICHARGE, WorkModule::get_float(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE));
        }
        if VarModule::get_float(fighter.module_accessor, gekkouga::instance::float::SHURICHARGE) >= 0.2 {
            if [*FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_SHOT, *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_MAX_SHOT].contains(&status_kind) {
                if MotionModule::frame(fighter.module_accessor) >= 16.0 {
                    VarModule::set_float(fighter.module_accessor, gekkouga::instance::float::SHURICHARGE, VarModule::get_float(fighter.module_accessor, gekkouga::instance::float::SHURICHARGE) / 1.66);
                    StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_HOLD, true);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_N_WORK_FLAG_RELEASE_HOLD_BUTTON);
                    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_SHURIKEN, false, -1);
                    WorkModule::set_float(fighter.module_accessor, VarModule::get_float(fighter.module_accessor, gekkouga::instance::float::SHURICHARGE), *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE);
                }
            }
            else if ![*FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_MAX_START].contains(&status_kind) {
                VarModule::set_float(fighter.module_accessor, gekkouga::instance::float::SHURICHARGE, 0.0);
            }
        }
    }    
}

unsafe extern "C" fn gekkouga_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    gekkouga_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, gekkouga_frame);
}