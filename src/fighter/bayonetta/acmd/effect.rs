use super::*;

unsafe extern "C" fn bayo_effectattack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            if PostureModule::lr(fighter.module_accessor) < 0.0 {
                let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_LINE1);
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new_raw(effect_kind), Hash40::new("top"), -2.5, 10.5, -3, 0, 0, 0, 1, false);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
            }
            else {
                let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_LINE1);
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new_raw(effect_kind), Hash40::new("top"), 2.5, 10.5, -3, 0, 0, 0, 1, false);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
            }
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_LINE1);
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, Hash40::new_raw(effect_kind), -1);
            sv_animcmd::EFFECT_DETACH_KIND_WORK(fighter.lua_state_agent);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            fighter.clear_lua_stack();
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1 { //blue
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true, 0.35, 0.7, 1.2);
            }
            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 { //purple
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true, 0.632, 0.145, 0.65);
            }
            else { //red
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true, 1.0, 0.1, 0.18);
            }
            sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
        }
    frame(lua_state, 30.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("bayonetta_beretta_rotation"), true, true);
        }
}

unsafe extern "C" fn bayo_effectattack12(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("bayonetta_beretta_rotation"), true, true);
        }
    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            if PostureModule::lr(fighter.module_accessor) < 0.0 {
                let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_ARC1);
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, effect_kind, Hash40::new("top"), 0, 10, 0, 3, 0, 175, 1, true);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
            }
            else
            {
                let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_ARC1);
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, effect_kind,  Hash40::new("top"), 0, 11, 0, 2, 0, 190, 1, true);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
            }
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_ARC1);
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, effect_kind, -1);
            sv_animcmd::EFFECT_DETACH_KIND_WORK(fighter.lua_state_agent);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            fighter.clear_lua_stack();
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1 { //blue
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true, 0.35, 0.7, 1.2);
            }
            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 { //purple
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true, 0.632, 0.145, 0.65);
            }
            else { //red
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true, 1.0, 0.1, 0.18);
            }
            sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
        }
    frame(lua_state, 32.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("bayonetta_beretta_rotation"), true, true);
        }
}

unsafe extern "C" fn bayo_effectattack13(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

            if macros::is_excute(fighter)
            {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("bayonetta_beretta_rotation"), true, true);
            }
        frame(lua_state, 6.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
                let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_ARC1);
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, effect_kind, Hash40::new("trans"), 1, 12, 0, 0, 5, 100, 1, true);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
                EffectModule::set_disable_render_offset_last(fighter.module_accessor);
            }
        frame(lua_state, 11.);
            if macros::is_excute(fighter)
            {
                let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_ARC1);
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, effect_kind, -1);
                sv_animcmd::EFFECT_DETACH_KIND_WORK(fighter.lua_state_agent);
            }
        wait(lua_state, 1.0);
            if macros::is_excute(fighter)
            {
                fighter.clear_lua_stack();
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1 { //blue
                    smash_script::lua_args!(fighter, Hash40::new("bayonetta_attack_scrape"), Hash40::new("top"), 7, 0.5, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 0.35, 0.7, 1.2);
                    sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
                }
                else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 { //purple
                    smash_script::lua_args!(fighter, Hash40::new("bayonetta_attack_scrape"), Hash40::new("top"), 7, 0.5, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 0.632, 0.145, 0.65);
                    sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
                }
                else { //red
                    smash_script::lua_args!(fighter, Hash40::new("bayonetta_attack_scrape"), Hash40::new("top"), 7, 0.5, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 1.0, 0.1, 0.18);
                    sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
                }
                macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), -2, 0, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            }
        wait(lua_state, 6.0);
            if macros::is_excute(fighter)
            {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1 { //blue
                    fighter.clear_lua_stack();
                    smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true, 0.35, 0.7, 1.2);
                    sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 0.35, 0.7, 1.2);
                    sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
                }
                else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 { //purple
                    fighter.clear_lua_stack();
                    smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true, 0.632, 0.145, 0.65);
                    sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 0.632, 0.145, 0.65);
                    sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
                }
                else { //red
                    fighter.clear_lua_stack();
                    smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true, 1.0, 0.1, 0.18);
                    sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 1.0, 0.1, 0.18);
                    sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
                }
            }
        frame(lua_state, 36.);
            if macros::is_excute(fighter)
            {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("bayonetta_beretta_rotation"), true, true);
            }
}

unsafe extern "C" fn bayo_effectattack100end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_100A);
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, effect_kind, false, false);
            sv_animcmd::EFFECT_OFF_KIND_WORK(fighter.lua_state_agent);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_100END);
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, effect_kind, Hash40::new("trans"), 1.5, 8, 0, 0, 0, 120, 1.25, true);
            sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
            EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        }
    frame(lua_state, 10.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 4, 0, 4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
    wait(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, Hash40::new("bayonetta_attack_wind"), Hash40::new("trans"), 0, 10, 20, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, 0.745, 0.941, 1);
            sv_animcmd::EFFECT_COLOR(fighter.lua_state_agent);
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    wait(lua_state, 11.0);
        if macros::is_excute(fighter)
        {
            fighter.clear_lua_stack();
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1 { //blue
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 0.35, 0.7, 1.2);
            }
            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 { //purple
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 0.632, 0.145, 0.65);
            }
            else { //red
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 1.0, 0.1, 0.18);
            }
            sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
        }
    frame(lua_state, 42.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter,  Hash40::new("bayonetta_beretta_rotation"), false, true);
        }
}

unsafe extern "C" fn bayo_effectattackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_ARC1);
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, Hash40::new_raw(effect_kind), Hash40::new("top"), 5, 14.5, 3, -49, 27, 86, 1.1, true);
            sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.0);
        }
    frame(lua_state, 7.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    frame(lua_state, 11.0);
        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, 27, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            fighter.clear_lua_stack();
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1 { //blue
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, -10, -60, -10, 1.0, true, 0.35, 0.7, 1.2);
            }
            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 { //purple
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, -10, -60, -10, 1.0, true, 0.632, 0.145, 0.65);
            }
            else { //red
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, -10, -60, -10, 1.0, true, 1.0, 0.1, 0.18);
            }
            sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
        }
    frame(lua_state, 36.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("bayonetta_beretta_rotation"), true, true);
        }
}

unsafe extern "C" fn bayo_effectattackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1 { //blue
        frame(lua_state, 0.);
            if macros::is_excute(fighter)
            {
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, -10, -60, -10, 1.0, true, 0.35, 0.7, 1.2);
                sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
                macros::PLAY_SE(fighter, Hash40::new("se_bayonetta_loveisblue_spin"));
            }
        frame(lua_state, 15.);
            if macros::is_excute(fighter)
            {
                macros::EFFECT_OFF_KIND(fighter,  Hash40::new("bayonetta_beretta_rotation"), true, true);
                //macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("armr"), 7, 0.5, 0, 0, 0, 0, 1.4, true);
                let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_STILETTO);
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, effect_kind, Hash40::new("armr"), 7, 0.5, 0, 0, 90, 0, 1.0, true);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
                EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
            }
        frame(lua_state, 16.);
            if macros::is_excute(fighter)
            {
                macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        frame(lua_state, 28.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            }
        frame(lua_state, 29.);
            if macros::is_excute(fighter)
            {
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true, 0.35, 0.7, 1.2);
                sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
            }
        frame(lua_state, 42.);
            if macros::is_excute(fighter)
            {
                macros::EFFECT_OFF_KIND(fighter,  Hash40::new("bayonetta_beretta_rotation"), true, true);
            }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 { //purple
        frame(lua_state, 0.);
            if macros::is_excute(fighter)
            {
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, -10, -60, -10, 1.0, true, 0.632, 0.145, 0.65);
                sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
                macros::PLAY_SE(fighter, Hash40::new("se_bayonetta_loveisblue_spin"));
            }
        frame(lua_state, 15.);
            if macros::is_excute(fighter)
            {
                macros::EFFECT_OFF_KIND(fighter,  Hash40::new("bayonetta_beretta_rotation"), true, true);
                //macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("armr"), 7, 0.5, 0, 0, 0, 0, 1.4, true);
                let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_STILETTO);
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, effect_kind, Hash40::new("armr"), 7, 0.5, 0, 0, 90, 0, 1.0, true);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
                EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
            }
        frame(lua_state, 16.);
            if macros::is_excute(fighter)
            {
                macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        frame(lua_state, 28.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            }
        frame(lua_state, 29.);
            if macros::is_excute(fighter)
            {
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true, 0.632, 0.145, 0.65);
                sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
            }
        frame(lua_state, 42.);
            if macros::is_excute(fighter)
            {
                macros::EFFECT_OFF_KIND(fighter,  Hash40::new("bayonetta_beretta_rotation"), true, true);
            }
    }
    else //red
    {
        frame(lua_state, 0.);
            if macros::is_excute(fighter)
            {
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, -10, -60, -10, 1.0, true, 1.0, 0.1, 0.18);
                sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
                macros::PLAY_SE(fighter, Hash40::new("se_bayonetta_loveisblue_spin"));
            }
        frame(lua_state, 15.);
            if macros::is_excute(fighter)
            {
                macros::EFFECT_OFF_KIND(fighter,  Hash40::new("bayonetta_beretta_rotation"), true, true);
                //macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("armr"), 7, 0.5, 0, 0, 0, 0, 1.4, true);
                let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_STILETTO);
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, effect_kind, Hash40::new("armr"), 7, 0.5, 0, 0, 90, 0, 1.0, true);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
                EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
            }
        frame(lua_state, 16.);
            if macros::is_excute(fighter)
            {
                macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        frame(lua_state, 28.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            }
        frame(lua_state, 29.);
            if macros::is_excute(fighter)
            {
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0, 0.1, 0.18);
                sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
            }
        frame(lua_state, 42.);
            if macros::is_excute(fighter)
            {
                macros::EFFECT_OFF_KIND(fighter,  Hash40::new("bayonetta_beretta_rotation"), true, true);
            }
    }
}

//air
unsafe extern "C" fn bayo_effectattackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1 //blue
    {
        frame(lua_state, 5.);
            if macros::is_excute(fighter)
            {
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_speedline"), Hash40::new("top"), 0, 9, -4, 0, 0, 0, 1.0, true, 0.632, 0.145, 0.65);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.35, 0.7, 1.2);
            }
        wait(lua_state, 1.);
            if macros::is_excute(fighter)
            {
                let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_LINE1);
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, effect_kind, Hash40::new("top"), 4, 13.0, -3, 0, -6, 0, 1.2, true);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
                macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
            }
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 { //purple
        frame(lua_state, 5.);
            if macros::is_excute(fighter)
            {
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_speedline"), Hash40::new("top"), 0, 9, -4, 0, 0, 0, 1.0, true, 0.632, 0.145, 0.65);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.632, 0.145, 0.65);
            }
        wait(lua_state, 1.);
            if macros::is_excute(fighter)
            {
                let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_LINE1);
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, effect_kind, Hash40::new("top"), 4, 13.0, -3, 0, -6, 0, 1.2, true);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
                macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
            }
    }
    else //red
    {
        frame(lua_state, 5.);
            if macros::is_excute(fighter)
            {
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, Hash40::new("bayonetta_speedline"), Hash40::new("top"), 0, 9, -4, 0, 0, 0, 1.0, true, 0.632, 0.145, 0.65);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.1, 0.18);
            }
        wait(lua_state, 1.);
            if macros::is_excute(fighter)
            {
                let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_LINE1);
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, effect_kind, Hash40::new("top"), 4, 13.0, -3, 0, -6, 0, 1.2, true);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
                macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
            }
    }
}

unsafe extern "C" fn bayo_effectattackairb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_WORK(fighter, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_ARC2, Hash40::new("top"), 2, 14, 0, 0, 170, 120, 1.1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_ARC2);
        fighter.clear_lua_stack();
        smash_script::lua_args!(fighter, Hash40::new_raw(effect_kind), -1);
        sv_animcmd::EFFECT_DETACH_KIND_WORK(fighter.lua_state_agent);
    }
}

unsafe extern "C" fn bayo_expressionattackairb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn bayo_effectattackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1
            {
                macros::EFFECT_FLW_POS(fighter, Hash40::new("bayonetta_speedline"), Hash40::new("top"), 0, 4, 8, 90, 0, 0, 1.2, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.35, 0.7, 1.2);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
            }
            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
                macros::EFFECT_FLW_POS(fighter, Hash40::new("bayonetta_speedline"), Hash40::new("top"), 0, 4, 8, 90, 0, 0, 1.2, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.632, 0.145, 0.65);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
            }
            else
            {
                macros::EFFECT_FLW_POS(fighter, Hash40::new("bayonetta_speedline"), Hash40::new("top"), 0, 4, 8, 90, 0, 0, 1.2, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.1, 0.18);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
            }
        }
}

unsafe extern "C" fn bayo_effectlandingairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("bayonetta_speedline"), false, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1
            {
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.1, 0.4, 1.0);
            }
            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.632, 0.145, 0.65);
            }
            else {
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.1, 0.18);
            }
        }
    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
}

//specials
unsafe extern "C" fn bayo_effectspecialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);

            let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_WIND);
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, effect_kind, Hash40::new("top"), 0, 30, 0, 0, 0, 0, 1, true);
            sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
            EffectModule::set_alpha_last(fighter.module_accessor, 0.6);

            let effect_kind2 = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_SPIRAL);
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, effect_kind2, Hash40::new("top"), 0, 30, 0, 0, 0, 0, 1, true);
            sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
            EffectModule::set_alpha_last(fighter.module_accessor, 0.4);

            //macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("bayonetta_afterburner_line2"), Hash40::new("top"), 0, 25, 0, -90, 0, 0, 1.2, true);
        }
    frame(lua_state, 32.);
        if macros::is_excute(fighter)
        {
            let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_WIND);
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, effect_kind, -1);
            sv_animcmd::EFFECT_DETACH_KIND_WORK(fighter.lua_state_agent);
        }
}

unsafe extern "C" fn bayo_effectspecialairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_REUSE as i32) {
                let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_WIND);
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, effect_kind, Hash40::new("top"), 0, 30, 0, 0, 0, 0, 1, true);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
                EffectModule::set_alpha_last(fighter.module_accessor, 0.6);

                let effect_kind2 = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_SPIRAL);
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, effect_kind2, Hash40::new("top"), 0, 30, 0, 0, 0, 0, 1, true);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
                EffectModule::set_alpha_last(fighter.module_accessor, 0.4);
            }
            else {
                let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_WIND);
                fighter.clear_lua_stack();
                smash_script::lua_args!(fighter, effect_kind, Hash40::new("top"), 0, 30, 0, 0, 0, 0, 1, true);
                sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
                EffectModule::set_alpha_last(fighter.module_accessor, 0.6);
            }

            //macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("bayonetta_afterburner_line2"), Hash40::new("top"), 0, 25, 0, -90, 0, 0, 1.2, true);
        }
    frame(lua_state, 32.);
        if macros::is_excute(fighter)
        {
            let effect_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_WIND);
            fighter.clear_lua_stack();
            smash_script::lua_args!(fighter, effect_kind, -1);
            sv_animcmd::EFFECT_DETACH_KIND_WORK(fighter.lua_state_agent);
        }
}

unsafe extern "C" fn bayo_effectspecials(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1 { //blue
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("bayonetta_heelslide_burst"), Hash40::new("kneer"), 9.5, 0, 0, 0, 90, 0, 1.2, true);
                EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.35, 0.7, 1.2);
            }
            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 { //purple
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("bayonetta_heelslide_burst"), Hash40::new("kneer"), 9.5, 0, 0, 0, 90, 0, 1.2, true);
                EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.675, 0.175, 0.7);
            }
            else { //red
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("bayonetta_heelslide_burst"), Hash40::new("kneer"), 9.5, 0, 0, 0, 90, 0, 1.2, true);
                EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.1, 0.18);
            }
        }
    wait(lua_state, 1.0);
        if macros::is_excute(fighter)
        {
            for _ in 0..5 {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_NEAR_CLIFF)
                {
                        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1 { //blue
                            fighter.clear_lua_stack();
                            smash_script::lua_args!(fighter, Hash40::new("bayonetta_heelslide_trace"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 0.35, 0.7, 1.2);
                            sv_animcmd::EFFECT_COLOR(fighter.lua_state_agent);
                        }
                        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 { //purple
                            fighter.clear_lua_stack();
                            smash_script::lua_args!(fighter, Hash40::new("bayonetta_heelslide_trace"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 0.675, 0.175, 0.7);
                            sv_animcmd::EFFECT_COLOR(fighter.lua_state_agent);
                        }
                        else { //red
                            fighter.clear_lua_stack();
                            smash_script::lua_args!(fighter, Hash40::new("bayonetta_heelslide_trace"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 1.0, 0.1, 0.18);
                            sv_animcmd::EFFECT_COLOR(fighter.lua_state_agent);
                        }
                    wait(lua_state, 2.0);
                        if macros::is_excute(fighter)
                        {
                            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1 { //blue
                                fighter.clear_lua_stack();
                                smash_script::lua_args!(fighter, Hash40::new("bayonetta_heelslide_trace"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 0.35, 0.7, 1.2);
                                sv_animcmd::EFFECT_COLOR(fighter.lua_state_agent);
                            }
                            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 { //purple
                                fighter.clear_lua_stack();
                                smash_script::lua_args!(fighter, Hash40::new("bayonetta_heelslide_trace"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 0.675, 0.175, 0.7);
                                sv_animcmd::EFFECT_COLOR(fighter.lua_state_agent);
                            }
                            else { //red
                                fighter.clear_lua_stack();
                                smash_script::lua_args!(fighter, Hash40::new("bayonetta_heelslide_trace"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 1.0, 0.1, 0.18);
                                sv_animcmd::EFFECT_COLOR(fighter.lua_state_agent);
                            }
                            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.4, 1, 0, 3, 0, 0, 0, false);
                        }
                }
                else 
                {
                        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1 { //blue
                            fighter.clear_lua_stack();
                            smash_script::lua_args!(fighter, Hash40::new("bayonetta_heelslide_trace"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 0.35, 0.7, 1.2);
                            sv_animcmd::EFFECT_COLOR(fighter.lua_state_agent);
                        }
                        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 { //purple
                            fighter.clear_lua_stack();
                            smash_script::lua_args!(fighter, Hash40::new("bayonetta_heelslide_trace"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 0.675, 0.175, 0.7);
                            sv_animcmd::EFFECT_COLOR(fighter.lua_state_agent);
                        }
                        else { //red
                            fighter.clear_lua_stack();
                            smash_script::lua_args!(fighter, Hash40::new("bayonetta_heelslide_trace"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 1.0, 0.1, 0.18);
                            sv_animcmd::EFFECT_COLOR(fighter.lua_state_agent);
                        }
                    wait(lua_state, 2.0);
                        if macros::is_excute(fighter)
                        {
                            fighter.clear_lua_stack();
                            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1 { //blue
                                fighter.clear_lua_stack();
                                smash_script::lua_args!(fighter, Hash40::new("bayonetta_heelslide_trace"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 0.35, 0.7, 1.2);
                                sv_animcmd::EFFECT_COLOR(fighter.lua_state_agent);
                            }
                            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 { //purple
                                fighter.clear_lua_stack();
                                smash_script::lua_args!(fighter, Hash40::new("bayonetta_heelslide_trace"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 0.675, 0.175, 0.7);
                                sv_animcmd::EFFECT_COLOR(fighter.lua_state_agent);
                            }
                            else { //red
                                fighter.clear_lua_stack();
                                smash_script::lua_args!(fighter, Hash40::new("bayonetta_heelslide_trace"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 1.0, 0.1, 0.18);
                                sv_animcmd::EFFECT_COLOR(fighter.lua_state_agent);
                            }
                            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.4, 1, 0, 3, 0, 0, 0, false);
                        }
                }
                wait(lua_state, 2.0);
            }
        }
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("bayonetta_heelslide_burst"), false, false);
        }
    frame(lua_state, 59.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("bayonetta_heelslide_burst"), false, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
}

unsafe extern "C" fn bayo_effectspecialairsdlanding(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            fighter.clear_lua_stack();
            let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
            smash_script::lua_args!(fighter, WorkModule::get_int(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_AFTERBURNER_LINE), -1); //keep boma, immutable borrow stuff
            sv_animcmd::EFFECT_DETACH_KIND_WORK(fighter.lua_state_agent);
            macros::EFFECT_DETACH_KIND(fighter, Hash40::new("bayonetta_afterburner_line2"), -1);
            macros::EFFECT(fighter, Hash40::new("bayonetta_quake"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1
            {
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.35, 0.7, 1.2);
            }
            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.632, 0.145, 0.65);
            }
            else
            {
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.1, 0.18);
            }
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn bayo_effectthrowf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 13.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 8.5, -3, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1
            {
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.35, 0.7, 1.2);
            }
            else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.632, 0.145, 0.65);
            }
            else
            {
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.2, 0.2, 0.2);
            }
        }
    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    // frame(lua_state, 15.);
    //     if macros::is_excute(fighter)
    //     {
    //         macros::EFFECT(Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    //     }
}

unsafe extern "C" fn bayo_effectthrowlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("footr"), 2, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    frame(lua_state, 19.);
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1
        {
            if macros::is_excute(fighter)
            {
                macros::EFFECT(fighter, Hash40::new("bayonetta_speedline"), Hash40::new("top"), 0, 8.5, -3, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            
                //macros::EFFECT_FLW_POS(fighter, Hash40::new("bayonetta_speedline"), Hash40::new("top"), 0, 7, 12, 90, 0, 0, 0.9, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.078, 0.451, 1);
                macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
            }
        }
        else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7
        {
            if macros::is_excute(fighter)
            {
                macros::EFFECT(fighter, Hash40::new("bayonetta_speedline"), Hash40::new("top"), 0, 8.5, -3, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            
                //macros::EFFECT_FLW_POS(fighter, Hash40::new("bayonetta_speedline"), Hash40::new("top"), 0, 7, 12, 90, 0, 0, 0.9, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.632, 0.145, 0.65);
                macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
            }
        }
        else {
            if macros::is_excute(fighter)
            {
                macros::EFFECT(fighter, Hash40::new("bayonetta_speedline"), Hash40::new("top"), 0, 8.5, -3, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            
                //macros::EFFECT_FLW_POS(fighter, Hash40::new("bayonetta_speedline"), Hash40::new("top"), 0, 7, 12, 90, 0, 0, 0.9, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.871, 0.145, 0.184);
                macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
            }
        }
    frame(lua_state, 20.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    frame(lua_state, 21.);
        if macros::is_excute(fighter)
        {
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
    frame(lua_state, 22.);
        if macros::is_excute(fighter)
        {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
}

unsafe extern "C" fn bayo_walkfast(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 4.5, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    for _ in 0..i32::MAX {
        frame(lua_state, 22.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 4.5, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        frame(lua_state, 47.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 4.5, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        frame(lua_state, 71.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 4.5, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        frame(lua_state, 95.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 4.5, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        wait_loop_sync_mot(lua_state);
    }
}

unsafe extern "C" fn bayo_walkmiddle(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 4.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    for _ in 0..i32::MAX {
        frame(lua_state, 24.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        frame(lua_state, 54.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        frame(lua_state, 80.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        frame(lua_state, 106.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        wait_loop_sync_mot(lua_state);
    }
}

unsafe extern "C" fn bayo_walkslow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 2.0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
        }
    for _ in 0..i32::MAX {
        frame(lua_state, 28.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
            }
        frame(lua_state, 58.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
            }
        frame(lua_state, 88.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        frame(lua_state, 118.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        wait_loop_sync_mot(lua_state);
    }
}

unsafe extern "C" fn bayo_fxrun(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    for _ in 0..i32::MAX {
        frame(lua_state, 15.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2.5, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            }
        frame(lua_state, 29.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4.5, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        frame(lua_state, 45.);
            if macros::is_excute(fighter)
            {
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2.5, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        wait_loop_sync_mot(lua_state);
    }
}

unsafe extern "C" fn bayo_fxdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 6.0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
}

unsafe extern "C" fn bayo_fxrunbrake(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 6.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 6.0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    wait(lua_state, 4.);
}

unsafe extern "C" fn bayo_fxrunbrakel(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 7.5, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 7.5, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    wait(lua_state, 4.);
}

unsafe extern "C" fn bayo_fxrunbraker(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    
    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 6.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    wait(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 6.0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    wait(lua_state, 4.);
}

unsafe extern "C" fn bayo_fxlandingheavy(fighter: &mut L2CAgentBase) {
    
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
}

unsafe extern "C" fn bayo_fxlandinglight(fighter: &mut L2CAgentBase) {
    
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
}

unsafe extern "C" fn bayo_fxlandingfallspecial(fighter: &mut L2CAgentBase) {
    
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
}

unsafe extern "C" fn bayo_fx_landingairall(fighter: &mut L2CAgentBase) {
    
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_attack11", bayo_effectattack11,)
    .effect_acmd("effect_attack12", bayo_effectattack12,)
    .effect_acmd("effect_attack13", bayo_effectattack13,)
    .effect_acmd("effect_attack100end", bayo_effectattack100end,)
    .effect_acmd("effect_attackhi3", bayo_effectattackhi3,)
    .effect_acmd("effect_attackdash", bayo_effectattackdash,)

    .effect_acmd("effect_attackairf", bayo_effectattackairf,)
    .effect_acmd("effect_attackairb", bayo_effectattackairb,)
    .expression_acmd("expression_attackairb", bayo_expressionattackairb,)
    .effect_acmd("effect_attackairlw", bayo_effectattackairlw,)
    .effect_acmd("effect_attacklandingairlw", bayo_effectlandingairlw,)

    .effect_acmd("effect_specialhi", bayo_effectspecialhi,)
    .effect_acmd("effect_specialairhi", bayo_effectspecialairhi,)
    .effect_acmd("effect_specials", bayo_effectspecials,)
    .effect_acmd("effect_specialairsdlanding", bayo_effectspecialairsdlanding,)

    .effect_acmd("effect_throwf", bayo_effectthrowf)
    .effect_acmd("effect_throwlw", bayo_effectthrowlw)
    
    .effect_acmd("effect_walkfast", bayo_walkfast,)
    .effect_acmd("effect_walkmiddle", bayo_walkmiddle,)
    .effect_acmd("effect_walkslow", bayo_walkslow,)
    .effect_acmd("effect_fun", bayo_fxrun,)
    .effect_acmd("effect_dash", bayo_fxdash,)
    .effect_acmd("effect_brake", bayo_fxrunbrake,)
    .effect_acmd("effect_brakel", bayo_fxrunbrakel,)
    .effect_acmd("effect_braker", bayo_fxrunbraker,)
    .effect_acmd("effect_landingheavy", bayo_fxlandingheavy,)
    .effect_acmd("effect_landinglight", bayo_fxlandinglight,)
    .effect_acmd("effect_landingfallspecial", bayo_fxlandingfallspecial,)

    .effect_acmd("effect_landingairn", bayo_fx_landingairall,)
    .effect_acmd("effect_landingairhi", bayo_fx_landingairall,)
    .effect_acmd("effect_landingairf", bayo_fx_landingairall,)
    .effect_acmd("effect_landingairf2", bayo_fx_landingairall,)
    .effect_acmd("effect_landingairf3", bayo_fx_landingairall,)
    .effect_acmd("effect_landingairb", bayo_fx_landingairall,)
    .install();
}