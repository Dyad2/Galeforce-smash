use super::*;
use crate::fighter::common::opff::common_fighter_frame;

//GA Idea: Armor of Courage
//  Link's % get reset to 0 for a short time, allowing him to tank hits and get his beam attack back. 
unsafe extern "C" fn link_galeforce_attack(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {            
        VarModule::sub_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER, 1);
        DamageModule::set_damage_lock(fighter.module_accessor, true); //does it need to be set to true every frame? tf
        //let cmb_vec1 = Vector4f{x: 0.2, y: 0.60, z: 0.25, w: 1.0}; //rgba format? w doesnt seem to do anything
        //let cmb_vec2 = Vector4f{x: 0.2, y: 0.125, z: 0.25, w: 1.0}; //the two colors seem to be added together? might be shadow/highlight
        //ColorBlendModule::set_main_color(fighter.module_accessor, &cmb_vec1, &cmb_vec2, 20.5 /*glow range?*/, 10.5 /*glow strength?*/, 0, false);
        macros::BURN_COLOR(fighter, 0.26, 1.5, 0.7, 0.7);
        macros::BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
    
        //do_once prevents reactivation from hitting up b again
        if !VarModule::is_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE) {
            galeforce_apply_effect(&mut *fighter.module_accessor, 0.66);
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE);
            VarModule::set_float(fighter.module_accessor, link::instance::float::DAMAGE_STORAGE, DamageModule::damage(fighter.module_accessor, 0));
            DamageModule::heal(fighter.module_accessor, -DamageModule::damage(fighter.module_accessor, 0), 0);
        }
    
        if is_status_grabbed(&mut *fighter.module_accessor) || VarModule::get_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER) <= 0 {
            VarModule::on_flag(fighter.module_accessor, link::instance::flag::RESTORE_DAMAGE);
        }
        if VarModule::is_flag(fighter.module_accessor, link::instance::flag::RESTORE_DAMAGE) && !is_status_grabbed(&mut *fighter.module_accessor) { //restore damage only after link was thrown. evil :)
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE);
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON);
            VarModule::off_flag(fighter.module_accessor, link::instance::flag::RESTORE_DAMAGE);
            DamageModule::set_damage_lock(fighter.module_accessor, false);
            DamageModule::add_damage(fighter.module_accessor, VarModule::get_float(fighter.module_accessor, link::instance::float::DAMAGE_STORAGE), 0);
            //ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
            macros::BURN_COLOR_NORMAL(fighter);
        }
    }    
}

unsafe extern "C" fn link_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    link_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, link_frame);
}