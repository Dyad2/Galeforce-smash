use skyline_smash::app::utility;
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

unsafe extern "C" fn global_weapon_frame(fighter_base : &mut L2CFighterBase) {
    if smash::app::utility::get_category(&mut *fighter_base.module_accessor) == *BATTLE_OBJECT_CATEGORY_WEAPON {
        let weapon_kind = get_kind(&mut *fighter_base.module_accessor);
        let status_kind = StatusModule::status_kind(fighter_base.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter_base.module_accessor);
        
        let weapon =  mem::transmute::<&mut BattleObject, &mut smash::app::Weapon>(smash::app::sv_system::battle_object(fighter_base.lua_state_agent));
        let owner_id = smash::app::lua_bind::Weapon::get_founder_id(weapon) as u32;
        
        //weapons using owner_fighter.module_accessor stuff
        if !smash::app::sv_battle_object::is_null(owner_id) && smash::app::sv_battle_object::is_active(owner_id) {
            let owner_boma = &mut *sv_battle_object::module_accessor(owner_id);
            let owner_status = StatusModule::status_kind(owner_boma);
                
            //duck!
            if weapon_kind == *WEAPON_KIND_DUCKHUNT_GUNMAN {
                //check if dh has their GA
                if StatusModule::status_kind(fighter_base.module_accessor) == *WEAPON_DUCKHUNT_GUNMAN_STATUS_KIND_READY {
                    if VarModule::is_flag(owner_boma, commons::instance::flag::GALEFORCE_ATTACK_ON) {
                        StatusModule::change_status_request(fighter_base.module_accessor, *WEAPON_DUCKHUNT_GUNMAN_STATUS_KIND_READY, false);
                        //gunman is to the left of dh
                        if PostureModule::pos_x(fighter_base.module_accessor) < PostureModule::pos_x(owner_boma) {
                            PostureModule::set_lr(fighter_base.module_accessor, 1.0);
                        }
                        //gunman is to the right of dh
                        else {
                            PostureModule::set_lr(fighter_base.module_accessor, -1.0);
                        }
                        PostureModule::update_rot_y_lr(fighter_base.module_accessor);
                        VarModule::off_flag(owner_boma, commons::instance::flag::GALEFORCE_ATTACK_ON);
                    }
                    if VarModule::is_flag(owner_boma, duckhunt::instance::flag::GUNMAN_REACTIVATE) {
                        StatusModule::change_status_request(fighter_base.module_accessor, *WEAPON_DUCKHUNT_GUNMAN_STATUS_KIND_READY, false);
                        VarModule::off_flag(owner_boma, duckhunt::instance::flag::GUNMAN_REACTIVATE);
                    }
                }
            }
        
            //rosalina
            if weapon_kind == *WEAPON_KIND_ROSETTA_TICO {
                //return if rosa cancels special n charge
                if VarModule::is_flag(owner_boma, rosetta::instance::flag::TICO_RECALL) {
                    WorkModule::on_flag(owner_boma, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_RETURN);
                    VarModule::off_flag(owner_boma, rosetta::instance::flag::TICO_RECALL);
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
            if weapon_kind == *WEAPON_KIND_ZELDA_DEIN_S {
                if AttackModule::is_infliction(fighter_base.module_accessor, *COLLISION_KIND_MASK_HIT) {
                    VarModule::on_flag(owner_boma, commons::instance::flag::GALEFORCE_ATTACK_ON);
                }
            }
            if weapon_kind == *WEAPON_KIND_ZELDA_PHANTOM {
                //attempt at restoring 9.0.1 behavior. base offset was changed to detect ground when the phantom is higher up, and then it is set back down here
                if situation_kind == *SITUATION_KIND_GROUND {
                    VarModule::set_float(fighter_base.module_accessor, commons::instance::float::ECB_OFFSET_Y, 7.0);
                    GroundModule::set_rhombus_offset(fighter_base.module_accessor, &Vector2f{x : 0.0, y : VarModule::get_float(fighter_base.module_accessor, commons::instance::float::ECB_OFFSET_Y)});
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
    }
}

// Use this for general per-frame fighter-level hooks
pub unsafe extern "C" fn common_fighter_frame(fighter: &mut L2CFighterCommon) {
    if utility::get_category(&mut *fighter.module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        cancel_effect(fighter);
        fighter_reset(fighter);
        Jostling(fighter);
        ecb_shifts::run(fighter);
        edge_cancels::run(fighter);
        controls::run(fighter);
        galeforce::run(fighter); //crashes the game rn
    }
    else if utility::get_category(&mut *fighter.module_accessor) == *BATTLE_OBJECT_CATEGORY_WEAPON {
        global_weapon_frame(fighter);
    }
}