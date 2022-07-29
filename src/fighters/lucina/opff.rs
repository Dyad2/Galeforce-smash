use super::*;

#[fighter_frame( agent = FIGHTER_KIND_LUCINA )]
fn masked_marth_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);

        if curr_motion_kind == hash40("special_hi") {
            VarModule::on_flag(fighter.battle_object, marcina::instance::flag::LUCINA_SPECIAL_HI_LANDING);
            AttackModule::clear_all(fighter.module_accessor);
            if MotionModule::frame(fighter.module_accessor) >= 12. {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_2"), 13.0, 1.0, 0.0, false, false);
            }
        }
        else if curr_motion_kind == hash40("special_hi_2") {
            if MotionModule::frame(fighter.module_accessor) >= 40. {
                let speedhi2 = smash::phx::Vector3f { x: 0.085 * ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor), y: 0., z: 0.0 };
                KineticModule::add_speed(fighter.module_accessor, &speedhi2);
                KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            }
            if MotionModule::frame(fighter.module_accessor) >= 62. && MotionModule::frame(fighter.module_accessor) < 63. {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_3"), 63.0, 1.0, 0.0, false, false);
            }
        }
        else if curr_motion_kind == hash40("special_hi_3") {
            let speedhi3 = smash::phx::Vector3f { x: 0.085 * ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor), y: 0., z: 0.0 };
            KineticModule::add_speed(fighter.module_accessor, &speedhi3);
            if MotionModule::frame(fighter.module_accessor) >= 65. {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_3"), 63.0, 1.0, 0.0, true, false);
            }
        }
        if curr_motion_kind == hash40("landing_fall_special") && VarModule::is_flag(fighter.battle_object, marcina::instance::flag::LUCINA_SPECIAL_HI_LANDING) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_aether"), 1.0, 1.0, false, 0.0, false, false);
        }
        if ![hash40("special_hi"), hash40("special_hi_2"), hash40("special_hi_3"), hash40("landing_fall_special"), hash40("landing_aether"), 51563581894].contains(&curr_motion_kind) {
            VarModule::off_flag(fighter.battle_object, marcina::instance::flag::LUCINA_SPECIAL_HI_LANDING);
        }
        if is_status_damage(&mut *fighter.module_accessor) {
            macros::AFTER_IMAGE_OFF(fighter, 1);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        masked_marth_frame
    );
}