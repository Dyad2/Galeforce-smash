use super::*;
use crate::fighter::common::opff::common_fighter_frame;

//GA: Shoot the duck!
// When hitting an opponent, DuckHunt can command the gunman to shoot towards the duck. Foes should be careful, the gunman's aim is dodgy
// part of the code is in weapon's opff, another in function_hooks (get_param_float)
pub unsafe extern "C" fn duckhunt_galeforce_attack(fighter : &mut L2CFighterCommon) {
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_GUNMAN) && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && !VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
            galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
        }
    }
    else {
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
    }
}
unsafe extern "C" fn duckhunt_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    duckhunt_galeforce_attack(fighter);
}

pub unsafe extern "C" fn gunman_callback(fighter_base : &mut L2CFighterBase) {    
    if get_kind(&mut *fighter_base.module_accessor) != WEAPON_KIND_DUCKHUNT_GUNMAN {
        return
    }
    let weapon =  mem::transmute::<&mut BattleObject, &mut smash::app::Weapon>(smash::app::sv_system::battle_object(fighter_base.lua_state_agent));
    let owner_id = smash::app::lua_bind::Weapon::get_founder_id(weapon) as u32;
    
    //weapons using owner_fighter.module_accessor stuff
    if !smash::app::sv_battle_object::is_null(owner_id) && smash::app::sv_battle_object::is_active(owner_id) {
        let owner_boma = &mut *sv_battle_object::module_accessor(owner_id);
            
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
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, duckhunt_frame);

    smashline::Agent::new("duckhunt_gunman")
        .on_line(smashline::Main, gunman_callback)
        .install();
}
