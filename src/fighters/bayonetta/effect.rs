use super::*;

#[acmd_script( agent = "bayonetta", script = "effect_attack11", category = ACMD_EFFECT, low_priority)]
unsafe fn effectattack11(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_attack12", category = ACMD_EFFECT, low_priority)]
unsafe fn effectattack12(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_attack13", category = ACMD_EFFECT, low_priority)]
unsafe fn effectattack13(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_attack100end", category = ACMD_EFFECT, low_priority)]
unsafe fn effectattack100end(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_attackhi3", category = ACMD_EFFECT, low_priority)]
unsafe fn effectattackhi3(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_attackdash", category = ACMD_EFFECT, low_priority)]
unsafe fn effectattackdash(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_attackairf", category = ACMD_EFFECT, low_priority)]
unsafe fn effectattackairf(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_attackairlw", category = ACMD_EFFECT, low_priority)]
unsafe fn effectattackairlw(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_landingairlw", category = ACMD_EFFECT, low_priority)]
unsafe fn effectlandingairlw(fighter: &mut L2CAgentBase) {
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
#[acmd_script( agent = "bayonetta", script = "effect_specialhi", category = ACMD_EFFECT, low_priority)]
unsafe fn effectspecialhi(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_specialairhi", category = ACMD_EFFECT, low_priority)]
unsafe fn effectspecialairhi(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_specials", category = ACMD_EFFECT, low_priority)]
unsafe fn effectspecials(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_specialairsdlanding", category = ACMD_EFFECT, low_priority)]
unsafe fn effectspecialairsdlanding(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_throwf", category = ACMD_EFFECT, low_priority)]
unsafe fn effectthrowf(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_throwlw", category = ACMD_EFFECT, low_priority)]
unsafe fn effectthrowlw(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_walkfast", category = ACMD_EFFECT, low_priority)]
unsafe fn walkfast(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_walkmiddle", category = ACMD_EFFECT, low_priority)]
unsafe fn walkmiddle(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_walkslow", category = ACMD_EFFECT, low_priority)]
unsafe fn walkslow(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_run", category = ACMD_EFFECT, low_priority)]
unsafe fn fx_run(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_dash", category = ACMD_EFFECT, low_priority)]
unsafe fn fxdash(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_runbrake", category = ACMD_EFFECT, low_priority)]
unsafe fn fxrunbrake(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_runbrakel", category = ACMD_EFFECT, low_priority)]
unsafe fn fxrunbrakel(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_runbraker", category = ACMD_EFFECT, low_priority)]
unsafe fn fxrunbraker(fighter: &mut L2CAgentBase) {
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

#[acmd_script( agent = "bayonetta", script = "effect_landingheavy", category = ACMD_EFFECT, low_priority)]
unsafe fn fx_landingheavy(fighter: &mut L2CAgentBase) {
    
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
}

#[acmd_script( agent = "bayonetta", script = "effect_landinglight", category = ACMD_EFFECT, low_priority)]
unsafe fn fx_landinglight(fighter: &mut L2CAgentBase) {
    
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
}

#[acmd_script( agent = "bayonetta", script = "effect_landingfallspecial", category = ACMD_EFFECT, low_priority)]
unsafe fn fx_landingfallspecial(fighter: &mut L2CAgentBase) {
    
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
}

#[acmd_script( agent = "bayonetta", scripts = ["effect_landingairn", "effect_landingairhi", "effect_landingairf", "effect_landingairf2", "effect_landingairf3", "effect_landingairb"], category = ACMD_EFFECT, low_priority)]
unsafe fn fx_landingairall(fighter: &mut L2CAgentBase) {
    
        if macros::is_excute(fighter)
        {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effectattack11,
        effectattack12,
        effectattack13,
        effectattack100end,
        effectattackhi3,
        effectattackdash,
        //effectattackairhi,
        effectattackairf,
        effectattackairlw,
        effectlandingairlw,
        effectspecialhi,
        effectspecialairhi,
        effectspecials,
        effectspecialairsdlanding,
        effectthrowlw,
        effectthrowf,
        walkfast,
        walkmiddle,
        walkslow,
        fxdash,
        fx_run,
        fxrunbrake,
        fxrunbrakel,
        fxrunbraker,
        fx_landingheavy,
        fx_landinglight,
        fx_landingfallspecial,
        fx_landingairall,
    );
}