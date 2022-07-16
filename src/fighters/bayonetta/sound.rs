use std::arch::asm;
use smash::phx::{Hash40, Vector3f};
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::{
    lua_bind::*,
    sv_animcmd::*,
    sv_animcmd,
    sv_system,
};
use smashline::*;
use smash_script::*;

//that one is weeeeeird
// #[acmd_script( agent = "bayonetta", script = "sound_attack100end_default", category = ACMD_SOUND, low_priority)]
// unsafe fn soundthrowf(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;

// if(0x1cb400(FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND, FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1)){
// 	frame(Frame=1)
// 	if(is_excute){
// 		sound(MA_MSC_CMD_SOUND_STOP_SE_STATUS)
// 	}
// }
// frame(Frame=8)
// if(is_excute){
// 	PLAY_SEQUENCE(hash40("seq_bayonetta_rnd_attack_smash_s01_jp"))
// }
// frame(Frame=10)
// if(is_excute){
// 	PLAY_SE(hash40("se_bayonetta_attack100_02"))
// }
// frame(Frame=22)
// if(is_excute){
// 	PLAY_SE(hash40("se_bayonetta_loveisblue_spin"))
// }
// frame(Frame=55)
// if(is_excute){
// 	PLAY_STEP(hash40("se_bayonetta_step_right_s"))
// }
// frame(0, 1)
// if(is_excute){
// 	sound(MA_MSC_CMD_SOUND_STOP_SE_STATUS)
// }
// frame(Frame=8)
// if(is_excute){
// 	PLAY_SEQUENCE(hash40("seq_bayonetta_rnd_attack_smash_s01_en"))
// }
// frame(Frame=10)
// if(is_excute){
// 	PLAY_SE(hash40("se_bayonetta_attack100_02"))
// }
// frame(Frame=22)
// if(is_excute){
// 	PLAY_SE(hash40("se_bayonetta_loveisblue_spin"))
// }
// frame(Frame=55)
// if(is_excute){
// 	PLAY_STEP(hash40("se_bayonetta_step_right_s"))
// }
// }

#[acmd_script( agent = "bayonetta", script = "sound_throwf", category = ACMD_SOUND, low_priority)]
unsafe fn soundthrowf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_bayonetta_rnd_attack_throw_f"));
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_bayonetta_smash_s02"));
            macros::PLAY_SE(fighter, Hash40::new("se_bayonetta_wecked_chargefull"));
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_bayonetta_throw_f01"));
        }
    frame(lua_state, 28.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_bayonetta_smash_h03"));
        }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        soundthrowf,
    );
}