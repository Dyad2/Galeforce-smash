use {
    smash::{
        lua2cpp::*
    },
    smashline::*,
    custom_var::*
};

#[fighter_reset]
fn fighter_reset(fighter: &mut L2CFighterCommon) {
    CustomVarManager::reset_var_module(fighter.battle_object);
}

#[agent_reset]
fn agent_reset(fighter: &mut L2CFighterBase) {
    CustomVarManager::reset_var_module(fighter.battle_object);
}

pub fn install() {
    install_agent_resets!(
        fighter_reset,
        agent_reset
    );
}