use super::*;

//effects
unsafe extern "C" fn effect_attackairn(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pikachu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        }
    frame(fighter.lua_state_agent, 3.0);
        for _ in 0..2 {
                if macros::is_excute(fighter)
                {
                    macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pikachu_elec_spark"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1, true);
                    //EffectModule::set_scale_last(fighter.module_accessor, &scale);
                    //let handle = EffectModule::get_last_handle(fighter.module_accessor) as u32;
                    //EffectModule::set_rgb(fighter.module_accessor, handle, 0.5, 0.5, 1.33);
                    // macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pikachu_kaminari_hit"), Hash40::new("top"), 0, -2, 0, 0, 90, 0, 1, true);
                    // EffectModule::set_scale_last(fighter.module_accessor, &scale);
                    // EffectModule::set_rate_last(fighter.module_accessor, 1.25);
                    // EffectModule::set_rgb_partial_last(fighter.module_accessor, 0.2, 0.2, 1.0);
                }
            wait(fighter.lua_state_agent, 6.0);
                if macros::is_excute(fighter)
                {
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new("pikachu_elec_spark"), false, false);
                }
            wait(fighter.lua_state_agent, 10.0);
        }
    frame(fighter.lua_state_agent, 26.0);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("pikachu_cheek"), false, true);
        }
}

unsafe extern "C" fn sound_attackairn(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter)
        {
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_pikachu_rnd_attack"));
            macros::PLAY_STATUS(fighter, Hash40::new("se_pikachu_attackair_n01"));
        }
    // wait(fighter.lua_state_agent, 6.0);
    //     if macros::is_excute(fighter)
    //     {
    //         macros::STOP_SE(fighter, Hash40::new("se_pikachu_attackair_n01"));
    //     }
    wait(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter)
        {
            macros::PLAY_STATUS(fighter, Hash40::new("se_pikachu_attackair_n01"));
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_attackairn", effect_attackairn,);
    agent.sound_acmd("sound_attackairn", sound_attackairn,);
}