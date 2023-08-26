// use super::*;

// #[status_script(agent="peach", status = FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn status_floatstart(fighter: &mut L2CFighterCommon) -> L2CValue {

//     if VarModule::is_flag(fighter.battle_object, peach::instance::flag::ALLOW_FLOAT) {
//         original!(fighter);
//     }
//     return 0.into()
// }

// #[status_script(agent="peach", status = FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn status_float(fighter: &mut L2CFighterCommon) -> L2CValue {

//     if VarModule::is_flag(fighter.battle_object, peach::instance::flag::ALLOW_FLOAT) {
//         original!(fighter);
//     }
//     return 0.into()
// }

// pub fn install() {
//     install_status_scripts!(
//         status_floatstart,
//         status_float
//     );
// }