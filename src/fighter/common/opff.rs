use super::*;

pub unsafe extern "C" fn cancel_effect(fighter : &mut L2CFighterCommon) {    
    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::HIT_CANCEL) {
        let frame = VarModule::get_int(fighter.module_accessor, commons::instance::int::HIT_CANCEL_FRAME_COUNTER);
        if frame < 24 {
            if frame % 4 == 0 {
                macros::FLASH(fighter, 0.5, 0.0, 1.0, 1.0);
            }
            else if frame % 2 == 0 {
                macros::COL_NORMAL(fighter);
            }
            VarModule::add_int(fighter.module_accessor, commons::instance::int::HIT_CANCEL_FRAME_COUNTER, 1);
        }
        else {
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::HIT_CANCEL);
            VarModule::set_int(fighter.module_accessor, commons::instance::int::HIT_CANCEL_FRAME_COUNTER, 0);
            //macros::FLASH(fighter, 1.0, 1.0, 1.0, 1.0);
        }
    }
}

unsafe extern "C" fn fighter_reset(fighter : &mut L2CFighterCommon) {    
    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_DEAD ||
      KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_RESET_NORMAL {
        CustomVarManager::reset_var_module(fighter.module_accessor, false);
    }
}

//Custom Jostling.
//  Jostling is enabled depending on fighter status. some fighters always have jostling enabled.
// it works because fighters who do not fulfill the requirement are all in the same jostling "team"
unsafe extern "C" fn Jostling(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);    
    
    if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, 
        *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, 
        *FIGHTER_STATUS_KIND_FURAFURA,*FIGHTER_STATUS_KIND_FURAFURA_STAND, 
        *FIGHTER_STATUS_KIND_SLEEP,*FIGHTER_STATUS_KIND_BIND, 
        *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, 
        *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE, 
        *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_TURN, 
        *FIGHTER_STATUS_KIND_SHIELD_BREAK_DOWN, *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY, 
        *FIGHTER_STATUS_KIND_SLIP_STAND_ATTACK, 
        *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_DASH, 
        *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, 
        *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, 
        *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_START, 
        *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind)
    || [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_GANON, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_SNAKE].contains(&fighter_kind) {
        JostleModule::set_team(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) + 1);
    }
    else {
        JostleModule::set_team(fighter.module_accessor, 0);
    }
}

// Use this for general per-frame fighter-level hooks
pub unsafe extern "C" fn common_fighter_frame(fighter: &mut L2CFighterCommon) {
    cancel_effect(fighter);
    fighter_reset(fighter);
    Jostling(fighter);
    ecb_shifts::run(fighter);
    edge_cancels::run(fighter);
    controls::run(fighter);
    galeforce::run(fighter); //crashes the game rn
}