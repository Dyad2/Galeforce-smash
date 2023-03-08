use super::*;

#[fighter_frame( agent = FIGHTER_KIND_LINK )]
fn cowardlydog_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        //GA Idea: Armor of Courage
        //  Link's % get reset to 0 for a short time, allowing him to tank hits and get his beam attack back. 
        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            println!("damage_storage: {}", VarModule::get_float(fighter.battle_object, link::instance::float::DAMAGE_STORAGE));
            VarModule::sub_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER, 1);
            DamageModule::set_damage_lock(fighter.module_accessor, true); //does it need to be set to true every frame? tf

            if !VarModule::is_flag(fighter.battle_object, commons::instance::flag::DO_ONCE) {
                //let cmb_vec1 = Vector4f{x: 0.2, y: 0.60, z: 0.25, w: 1.0}; //rgba format? w doesnt seem to do anything
                //let cmb_vec2 = Vector4f{x: 0.1, y: 0.125, z: 0.1, w: 1.0}; //the two colors seem to be added together? might be shadow/highlight
                //ColorBlendModule::set_main_color(fighter.module_accessor, &cmb_vec1, &cmb_vec2, 2.5 /*glow range?*/, 1.5 /*glow strength?*/, 1, true);
                galeforce_apply_effect(&mut *fighter.module_accessor, 0.66);
                VarModule::on_flag(fighter.battle_object, commons::instance::flag::DO_ONCE);
                VarModule::set_float(fighter.battle_object, link::instance::float::DAMAGE_STORAGE, DamageModule::damage(fighter.module_accessor, 0));
                DamageModule::heal(fighter.module_accessor, -DamageModule::damage(fighter.module_accessor, 0), 0);
            }

            if is_status_grabbed(&mut *fighter.module_accessor) || VarModule::get_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER) <= 0 {
                VarModule::on_flag(fighter.battle_object, link::instance::flag::RESTORE_DAMAGE);
            }
            if VarModule::is_flag(fighter.battle_object, link::instance::flag::RESTORE_DAMAGE) {
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::DO_ONCE);
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                VarModule::off_flag(fighter.battle_object, link::instance::flag::RESTORE_DAMAGE);
                DamageModule::set_damage_lock(fighter.module_accessor, false);
                DamageModule::add_damage(fighter.module_accessor, VarModule::get_float(fighter.battle_object, link::instance::float::DAMAGE_STORAGE), 0);
                //ColorBlendModule::cancel_main_color(fighter.module_accessor, 1);
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        cowardlydog_frame
    );
}