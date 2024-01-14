use super::*;

unsafe extern "C" fn expression_landingheavy(fighter: &mut L2CAgentBase) {    
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        if !VarModule::is_flag(fighter.module_accessor, commons::instance::flag::WAVEDASH) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("expression_landingheavy", expression_landingheavy,);
}