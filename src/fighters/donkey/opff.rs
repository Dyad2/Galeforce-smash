use super::*;
use crate::fighters::common::opff::common_fighter_frame;

pub unsafe extern "C" fn donkey_get_barrell(fighter : &mut L2CFighterCommon) {
    //dash attack to neutral air when diddy falls offstage
    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S &&
     StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND &&
     VarModule::is_flag(fighter.module_accessor, donkey::instance::flag::GET_BARREL) {
        VarModule::off_flag(fighter.module_accessor, donkey::instance::flag::GET_BARREL);
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), false.into());
        ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_BARREL), 0, 0, false, false);
    }
}

unsafe extern "C" fn donkey_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    donkey_get_barrell(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, donkey_frame);
}