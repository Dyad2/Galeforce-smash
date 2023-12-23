use super::*;
use crate::fighters::common::opff::common_fighter_frame;

// #[fighter_frame( agent = FIGHTER_KIND_LUIGI )]
// fn fingerguns_frame(fighter: &mut L2CFighterCommon) {
//     unsafe {
//         let status_kind = StatusModule::status_kind(fighter.module_accessor);
//         let elec_charge = VarModule::get_int(fighter.battle_object, luigi::instance::int::ELEC_CHARGE);
//         //rewrite in status, charge bonus is set frame 1
//         if status_kind == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE {
//             if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && fighter.global_table[MOTION_FRAME].get_i32() < 14 {
//                 if elec_charge <= 4 {
//                     WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_BONUS);
//                 }
//             }
//             else { //prevents from setting the flag to true if the player smash inputs
//                 WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_BONUS);
//             }
//         }
//         if status_kind == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM {
//             if elec_charge != 0 && VarModule::is_flag(fighter.battle_object, luigi::status::flag::SPECIAL_S_CHARGE_USED) {
//                 galeforce_apply_effect(&mut *fighter.module_accessor, 0.75);
//             }
//         }
//     }
// }

unsafe extern "C" fn luigi_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    //luigi_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, luigi_frame);
}