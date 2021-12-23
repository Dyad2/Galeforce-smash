use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::{
        fighters::{
            demon::agent_init::cmd_input_check,
        }
    }
};

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);

        if fighter_kind == *FIGHTER_KIND_DEMON {
            fighter.global_table[CMD_INPUT_CHECK].assign(&L2CValue::Ptr(cmd_input_check as *const () as _));
        }
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}