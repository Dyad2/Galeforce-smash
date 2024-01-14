use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
//use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut smashline::Agent::new("lucas");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}

// #[status_script(agent = "lucas", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn lucas_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     GroundModule::set_correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
//     //KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
//     if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
//         MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
//     }
//     else {
//         MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
//     }
//     fighter.sub_shift_status_main(L2CValue::Ptr(lucas_specials_loop as *const () as _))
// }

// unsafe extern "C" fn lucas_specials_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if StatusModule::is_situation_changed(fighter.module_accessor) {
//         if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
//             MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
//         }
//         else {
//             MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
//         }
//     }
//     if MotionModule::is_end(fighter.module_accessor) {
//         //KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
//         if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
//             fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
//         }
//         else {
//             fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
//         }
//     }
//     L2CValue::I32(0)
// }
