use smash::phx::{Hash40, Vector3f};
use smash::hash40;
use smash::lib::{lua_const::*, L2CValue};
use smash::app::lua_bind::*;
use smash::{lua2cpp::L2CFighterCommon, lua2cpp::L2CAgentBase};
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

mod acmd;
mod status;

#[fighter_frame( agent = FIGHTER_KIND_PITB )]
fn pitb_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);

        if curr_motion_kind != hash40("attack_air_n") {
            EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_blackball_attack")}, false, true);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        pitb_frame
    );
    acmd::install();
    status::install();
}