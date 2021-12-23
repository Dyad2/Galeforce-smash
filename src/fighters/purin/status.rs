#[status_script(agent = "purin", status = FIGHTER_STATUS_KIND_SPECIAL_N_ROLL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_EXEC)]
unsafe fn lucina_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    WorkModule::get_float(boma, *FIGHTER_PURIN_STATUS_SPECIAL_N_WORK_FLOAT_MOVE_DIR);
}