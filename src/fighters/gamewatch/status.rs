use super::*;

#[status_script(agent="gamewatch", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn specials_setkind(fighter: &mut L2CFighterCommon) -> L2CValue {

    let ret = original!(fighter);

    println!("game and watch!");

    if VarModule::is_flag(fighter.battle_object,commons::instance::flag::GALEFORCE_ATTACK_ON) {
        WorkModule::set_int(fighter.module_accessor, VarModule::get_int(fighter.battle_object, gamewatch::instance::int::JUDGE_STORED_KIND), *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND);
        //WorkModule::set_int64(
    }

    println!("gnw special_s_kind: {}", WorkModule::get_int(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND));

    return ret;
}

pub fn install() {
    install_status_scripts!(
        specials_setkind,
    );
}