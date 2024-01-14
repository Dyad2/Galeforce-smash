use super::*;

unsafe extern "C" fn fx_run(_fighter: &mut L2CAgentBase) {

    //keep empty, removes step smoke. maybe add dark effects instead?
}

unsafe extern "C" fn effectescapen(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        }
}

unsafe extern "C" fn soundescapen(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_ganon_escapeair"));
            macros::PLAY_SE(fighter, Hash40::new("vc_ganon_appeal_s01"));
        }
    wait(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SE(fighter, Hash40::new("se_ganon_landing01"));
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_run", fx_run,);
    agent.effect_acmd("effect_escapen", effectescapen,);
    agent.sound_acmd("sound_escapen", soundescapen,);
}