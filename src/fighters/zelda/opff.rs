use super::*;

#[fighter_frame( agent = FIGHTER_KIND_ZELDA )]
fn best_princess_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        //let fighter.module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        //let status_kind = StatusModule::status_kind(fighter.module_accessor);

        //GA - Hylia's helping hand
        // type: cancel / restriction lift
        //  allows zelda to stop charging phantom by shielding / airdodging. The phantom will continue charging
        if !is_operation_cpu(fighter.module_accessor) {
            if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
                zelda_buff_effect(fighter);
                if [hash40("special_lw"), hash40("special_air_lw")].contains(&curr_motion_kind) && MotionModule::frame(fighter.module_accessor) > 10.0 {
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD_HOLD) {
                        if situation_kind == *SITUATION_KIND_AIR {
                            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                            galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                            VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                        }
                        else if situation_kind == *SITUATION_KIND_GROUND {
                            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, false);
                            galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
                            VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                        }
                    }
                }
            }
            //TODO should be in status script
            // if situation_kind == *SITUATION_KIND_GROUND && status_kind == *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2 && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            //     //StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, false);
            //     fighter.change_status(FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3.into(), false.into());
            //     VarModule::on_flag(fighter.battle_object, zelda::instance::flag::SPECIAL_HI_CANCEL);
            // }
            if ![hash40("special_hi_start"), hash40("special_air_hi_start"), hash40("special_hi"), hash40("special_air_hi")].contains(&curr_motion_kind) {
                VarModule::off_flag(fighter.battle_object, zelda::instance::flag::SPECIAL_HI_CANCEL);
            }
        }
    }
}


pub fn install() {
    smashline::install_agent_frames!(
        best_princess_frame
    );
}