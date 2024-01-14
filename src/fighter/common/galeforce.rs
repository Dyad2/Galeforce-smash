use super::*;

pub unsafe extern "C" fn galeforce_apply_effect(boma : &mut BattleObjectModuleAccessor, size : f32) {
    let pos = Vector3f  {x : 0., y : 4., z : 0.};
    let rot = Vector3f  {x : 0., y : 90., z : 0.};
    
    // Known effects
    //      hash40("sys_damage_aura")   <-- flame / smoke
    //      hash40("sys_aura_light")    <-- transparent sun?
    //      0x0e27bc68a2                <-- raygun?
    //      hash40("sys_counter_flash") <-- light rays and expanding circle
    //      hash40("sys_hit_magic")     <-- spiky lights
    //      hash40("sys_bg_vortex")     <-- Chaos control-like
    //      hash40("sys_guard_mark")    <-- rotating shield impact
    //      hash40("sys_steam")         <-- steam. you know

    let handle = EffectModule::req_follow(boma,
        smash::phx::Hash40{hash: hash40("sys_counter_flash")},
        smash::phx::Hash40{hash: hash40("top")}, 
        &pos, &rot, size /* *(duration_left / duration_total)*/, false, 0, 
        0, 0, 0, 0, false, false) as u32;
        
    EffectModule::set_rgb(boma, handle, 5., 0.05, 2.25);
    flash_eye_info(boma);
}

pub unsafe extern "C"  fn luigi_charge_effect(boma : &mut BattleObjectModuleAccessor, size : f32) {
    let pos = Vector3f  {x : 0., y : 4., z : 0.};
    let rot = Vector3f  {x : 0., y : 90., z : 0.};

    let handle = EffectModule::req_follow(boma,
        smash::phx::Hash40{hash: hash40("sys_sp_flash")},
        smash::phx::Hash40{hash: hash40("handl")}, 
        &pos, &rot, size, false, 0, 
        0, 0, 0, 0, false, false) as u32;
        
    EffectModule::detach_kind(boma, Hash40::new("sys_sp_flash"), -1);
    EffectModule::set_rgb(boma, handle, 1.0, 1.0, 1.5);
    flash_eye_info(boma);
}

pub unsafe extern "C" fn robin_ignis_effect(fighter: &mut L2CFighterCommon) {
    
    let pos = Vector3f  {x : 1.5, y : 0.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};

    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) {
        let handle = EffectModule::req_follow(fighter.module_accessor,
            smash::phx::Hash40{hash: hash40("sys_aura_light")},
            smash::phx::Hash40{hash: hash40("handr")}, 
            &pos, &rot, 1.33, false, 0, 
            0, 0, 0, 0, false, false) as u32;

        EffectModule::set_rgb(fighter.module_accessor, handle, 4.5, 0.75, 3.5);
        EffectModule::set_alpha(fighter.module_accessor, handle, 0.75);
    }
    else {
        EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_aura_light")}, false, true);
    }
}

pub unsafe extern "C" fn zelda_buff_effect(fighter: &mut L2CFighterCommon) {
    let pos = Vector3f  {x : 1.5, y : 0.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};

    let handle = EffectModule::req_follow(fighter.module_accessor,
        smash::phx::Hash40{hash: hash40("sys_damage_aura")},
        smash::phx::Hash40{hash: hash40("handl")}, 
        &pos, &rot, 0.2 /* *(duration_left / duration_total)*/, false, 0, 
        0, 0, 0, 0, false, false) as u32;

    EffectModule::set_rgb(fighter.module_accessor, handle, 5.0, 0.0, 2.5);
    EffectModule::set_alpha(fighter.module_accessor, handle, 0.75);
    EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_damage_aura")}, false, true);
}

pub unsafe extern "C" fn kamui_debuff_effect(fighter: &mut L2CFighterCommon) {
    let pos = Vector3f  {x : 0.0, y : 0.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};

    if VarModule::get_int(fighter.module_accessor, commons::instance::int::KAMUI_DRAGON_HEX_DURATION) % 15 == 0 {
        let handle = EffectModule::req_follow(fighter.module_accessor,
            smash::phx::Hash40{hash: hash40("sys_damage_curse")},
            smash::phx::Hash40{hash: hash40("waist")}, 
            &pos, &rot, 0.33 /* *(duration_left / duration_total)*/, false, 0, 
            0, 0, 0, 0, false, false) as u32;

        EffectModule::set_rgb(fighter.module_accessor, handle, 5., 1., 2.);
        EffectModule::set_alpha(fighter.module_accessor, handle, 0.33);
    }
    // else {
    //     EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_damage_curse")}, false, true);
    // }
}

pub unsafe extern "C" fn chrom_disable_dance_effect(fighter: &mut L2CFighterCommon) {
    let pos = Vector3f  {x : 0.0, y : 0.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};

    if VarModule::get_int(fighter.module_accessor, commons::instance::int::FRAME_COUNTER) % 10 == 1  {
        let handle: u32 = EffectModule::req_follow(fighter.module_accessor,
        smash::phx::Hash40{hash: hash40("sys_steam")},
        smash::phx::Hash40{hash: hash40("waist")}, 
        &pos, &rot, 1.25, false, 0, 
        0, 0, 0, 0, false, false) as u32;

        EffectModule::set_rgb(fighter.module_accessor, handle, 0.35, 0.35, 0.35);
    }
    // else {
    //     EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_steam")}, false, true);
    // }
}

pub unsafe extern "C" fn sheik_ga_buff(fighter: &mut L2CFighterCommon) {
    let pos = Vector3f  {x : 0.0, y : 0.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};

    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::GALEFORCE_ATTACK_ON) && !VarModule::is_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE) {
        VarModule::on_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE);
        let handle: u32 = EffectModule::req_follow(fighter.module_accessor,
        smash::phx::Hash40{hash: hash40("sys_steam")},
        smash::phx::Hash40{hash: hash40("waist")}, 
        &pos, &rot, 1.25 /* *(duration_left / duration_total)*/, false, 0, 
        0, 0, 0, 0, false, false) as u32;

        EffectModule::set_rgb(fighter.module_accessor, handle, -0.35, -0.35, -0.35);
    }
    else {
        EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_steam")}, false, true);
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE);
    }
}

pub unsafe extern "C" fn mariod_buff_effect(fighter: &mut L2CFighterCommon) {
    let pos = Vector3f  {x : 0.0, y : 5.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};

    if VarModule::get_int(fighter.module_accessor, mariod::instance::int::GA_MEDECINE_TIMER) > 0 && !VarModule::is_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE) {
        VarModule::on_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE);
        let handle: u32 = EffectModule::req_follow(fighter.module_accessor,
            smash::phx::Hash40{hash: hash40("sys_aura_light")},
            smash::phx::Hash40{hash: hash40("top")}, 
            &pos, &rot, 6.0 /* *(duration_left / duration_total)*/, false, 0, 
            0, 0, 0, 0, false, false) as u32;
        EffectModule::set_rgb(fighter.module_accessor, handle, 4.0, 0.0, 2.5);
    }
    else {
        EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_aura_light")}, false, true);
        VarModule::off_flag(fighter.module_accessor, commons::instance::flag::DO_ONCE);
    }
}

//Some of them might be in acmd files instead!
pub unsafe extern "C" fn run(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let attacker_number = get_attacker_number(&mut *fighter.module_accessor);
    
    //Corrin
    if status_kind == *FIGHTER_STATUS_KIND_KAMUI_PIERCE && attacker_number < 8 {
        if smash::app::utility::get_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KIND_KAMUI /*&& StatusModule::status_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL*/ {
            VarModule::on_flag(get_boma(attacker_number as i32), commons::instance::flag::GALEFORCE_ATTACK_ON); //gives corrin their buff
            VarModule::set_int(fighter.module_accessor, commons::instance::int::KAMUI_DRAGON_HEX_DURATION, 600); //hold timer while the opponent is pinned
        }
    }
    else if VarModule::get_int(fighter.module_accessor, commons::instance::int::KAMUI_DRAGON_HEX_DURATION) > 0 {
        AttackModule::set_power_mul(fighter.module_accessor, 0.75);
        VarModule::add_int(fighter.module_accessor, commons::instance::int::KAMUI_DRAGON_HEX_DURATION, -1);
        galeforce::kamui_debuff_effect(fighter);
    }
    else {
        AttackModule::set_power_mul(fighter.module_accessor, 1.0);
    }
    //Puff (purin)
        // when rest is used vs an opponent without the mark, reduces endlag on rest and heals
        //gives opponent the mark when hit by up special
    if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START && StopModule::is_hit(fighter.module_accessor) && attacker_number < 8 {
        let puff_curr_motion = MotionModule::motion_kind(&mut *get_boma(attacker_number as i32));
        if smash::app::utility::get_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KIND_PURIN &&
         [hash40("special_hi_r"), hash40("special_hi_l"), hash40("special_air_hi_r"), hash40("special_air_hi_l")].contains(&puff_curr_motion) &&
         !VarModule::is_flag(fighter.module_accessor, commons::instance::flag::PURIN_MARK) {
            VarModule::on_flag(fighter.module_accessor, commons::instance::flag::PURIN_MARK);
            VarModule::set_int(fighter.module_accessor, commons::instance::int::PURIN_MARK_DURATION, 420);
        }
    }
    //mark decay and cleanup
    if VarModule::is_flag(fighter.module_accessor, commons::instance::flag::PURIN_MARK) {
        zelda_buff_effect(fighter);
        VarModule::sub_int(fighter.module_accessor, commons::instance::int::PURIN_MARK_DURATION, 1);
        if VarModule::get_int(fighter.module_accessor, commons::instance::int::PURIN_MARK_DURATION) <= 0 {
            VarModule::off_flag(fighter.module_accessor, commons::instance::flag::PURIN_MARK);
        }
    }
}