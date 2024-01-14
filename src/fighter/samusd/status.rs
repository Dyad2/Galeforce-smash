use super::*;

//manages speed, and the shoot flag for some reason
unsafe extern "C" fn samusd_specialnf_exec(fighter: &mut L2CFighterCommon) -> L2CValue {

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT) {
        //let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f32;
        //let lr = PostureModule::lr(fighter.module_accessor);
        //let min_spd = WorkModule::get_param_float(fighter.module_accessor, 0xf899192aa, hash40("cshot_shot_spd_min"));
        //let max_spd = WorkModule::get_param_float(fighter.module_accessor, 0xf899192aa, hash40("cshot_shot_spd_max"));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT);
    }
    return 0.into();
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Exec, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, samusd_specialnf_exec);
}