use super::*;
use crate::fighter::common::opff::common_fighter_frame;

//GA - Hylia's helping hand
// type: cancel / restriction lift
//  allows zelda to stop charging phantom by shielding / airdodging. The phantom will continue charging
unsafe extern "C" fn zelda_galeforce_attack(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    
    if !is_operation_cpu(fighter.module_accessor) {
        if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            zelda_buff_effect(fighter);
            if [hash40("special_lw"), hash40("special_air_lw")].contains(&curr_motion_kind) && MotionModule::frame(fighter.module_accessor) > 10.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD_HOLD) {
                    if situation_kind == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                    }
                    else if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, false);
                        galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
                    }
                }
            }
        }
        if ![hash40("special_hi_start"), hash40("special_air_hi_start"), hash40("special_hi"), hash40("special_air_hi")].contains(&curr_motion_kind) {
            VarModule::off_flag(fighter.module_accessor, zelda::instance::flag::SPECIAL_HI_CANCEL);
        }
    }
}

unsafe extern "C" fn zelda_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    zelda_galeforce_attack(fighter);
}

unsafe extern "C" fn dein_s_callback(fighter_base : &mut L2CFighterBase) {    
    let weapon =  mem::transmute::<&mut BattleObject, &mut smash::app::Weapon>(smash::app::sv_system::battle_object(fighter_base.lua_state_agent));
    let owner_id = smash::app::lua_bind::Weapon::get_founder_id(weapon) as u32;
    
    //weapons using owner_fighter.module_accessor stuff
    if !smash::app::sv_battle_object::is_null(owner_id) && smash::app::sv_battle_object::is_active(owner_id) {
        let owner_boma = &mut *sv_battle_object::module_accessor(owner_id);
        
        if AttackModule::is_infliction(fighter_base.module_accessor, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(owner_boma, commons::instance::flag::GALEFORCE_ATTACK_ON);
        }
    }
}

unsafe extern "C" fn phantom_callback(fighter_base : &mut L2CFighterBase) {
    if get_kind(&mut *fighter_base.module_accessor) != WEAPON_KIND_ZELDA_PHANTOM {
        return
    }
    let status_kind = StatusModule::status_kind(fighter_base.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter_base.module_accessor);
    
    let weapon =  mem::transmute::<&mut BattleObject, &mut smash::app::Weapon>(smash::app::sv_system::battle_object(fighter_base.lua_state_agent));
    let owner_id = smash::app::lua_bind::Weapon::get_founder_id(weapon) as u32;
    
    //weapons using owner_fighter.module_accessor stuff
    if !smash::app::sv_battle_object::is_null(owner_id) && smash::app::sv_battle_object::is_active(owner_id) {
        let owner_boma = &mut *sv_battle_object::module_accessor(owner_id);
        
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

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, zelda_frame);

    smashline::Agent::new("zelda_phantom")
        .on_line(smashline::Main, phantom_callback)
        .install();
    smashline::Agent::new("zelda_dein_s")
        .on_line(smashline::Main, dein_s_callback)
        .install();
}