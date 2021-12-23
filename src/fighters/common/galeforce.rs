use smash::lib::lua_const::*;
use smash::app::{BattleObjectModuleAccessor, utility::get_kind, BattleObject, sv_system::*, lua_bind::*};
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;
use smash::phx::{Hash40, Vector3f};

use crate::fighters::common::FIGHTER_GLOBALS;
use crate::utils::*;
use crate::var::*;

static mut HANDLE: u32 = 0;

static mut v: [i32; 9] = [255; 9];

pub unsafe fn galeforce_apply_effect(boma : &mut BattleObjectModuleAccessor, size : f32) {

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

    
    HANDLE = EffectModule::req_follow(boma,
        smash::phx::Hash40{hash: hash40("sys_counter_flash")},
        smash::phx::Hash40{hash: hash40("top")}, 
        &pos, &rot, size /* *(duration_left / duration_total)*/, false, 0, 
        0, 0, 0, 0, false, false) as u32;
        
    EffectModule::set_rgb(boma, HANDLE, 5., 0., 2.);
}

pub unsafe fn robin_ignis_effect(boma : &mut BattleObjectModuleAccessor) {
    let pos = Vector3f  {x : 1.5, y : 0.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let mut handle: u32 = 0;
    let mut once : bool = false;

    if FIGHTER_GLOBALS[entry_id as usize].ga_on {
        handle = EffectModule::req_follow(boma,
            smash::phx::Hash40{hash: hash40("sys_aura_light")},
            smash::phx::Hash40{hash: hash40("handr")}, 
            &pos, &rot, 1.33, false, 0, 
            0, 0, 0, 0, false, false) as u32;

        EffectModule::set_rgb(boma, handle, 4.5, 0.75, 3.5);
    }
    else {
        EffectModule::kill_kind(boma, smash::phx::Hash40{hash: hash40("sys_aura_light")}, false, true);
    }
}

pub unsafe fn zelda_buff_effect(boma : &mut BattleObjectModuleAccessor) {
    let pos = Vector3f  {x : 1.5, y : 0.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};

    let handle: u32 = EffectModule::req_follow(boma,
        smash::phx::Hash40{hash: hash40("sys_damage_aura")},
        smash::phx::Hash40{hash: hash40("handl")}, 
        &pos, &rot, 0.2 /* *(duration_left / duration_total)*/, false, 0, 
        0, 0, 0, 0, false, false) as u32;

    EffectModule::set_rgb(boma, handle, 5.0, 0.0, 2.5);
    EffectModule::kill_kind(boma, smash::phx::Hash40{hash: hash40("sys_damage_aura")}, false, true);
}

pub unsafe fn kamui_debuff_effect(boma : &mut BattleObjectModuleAccessor) {
    let pos = Vector3f  {x : 0.0, y : 0.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    if FIGHTER_GLOBALS[entry_id as usize].kamui_debuff_timer > 0 {
        if FIGHTER_GLOBALS[entry_id as usize].kamui_debuff_timer % 10 == 1 {
        //if FIGHTER_GLOBALS[entry_id as usize].once {
            //FIGHTER_GLOBALS[entry_id as usize].once = false;
            let handle = EffectModule::req_follow(boma,
                smash::phx::Hash40{hash: hash40("sys_damage_curse")},
                smash::phx::Hash40{hash: hash40("waist")}, 
                &pos, &rot, 0.25 /* *(duration_left / duration_total)*/, false, 0, 
                0, 0, 0, 0, false, false) as u32;

            EffectModule::set_rgb(boma, handle, 5., 1., 2.);
            EffectModule::set_alpha(boma, handle, 0.5);
        }
    }
    else {
        EffectModule::kill_kind(boma, smash::phx::Hash40{hash: hash40("sys_damage_curse")}, false, true);
    }
}

pub unsafe fn chrom_disable_dance_effect(boma : &mut BattleObjectModuleAccessor) {
    let pos = Vector3f  {x : 0.0, y : 0.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    if FIGHTER_GLOBALS[entry_id as usize].chrom_dance_disable_time > 0 && !FIGHTER_GLOBALS[entry_id as usize].once {
        FIGHTER_GLOBALS[entry_id as usize].once = true;
        let handle: u32 = EffectModule::req_follow(boma,
        smash::phx::Hash40{hash: hash40("sys_steam")},
        smash::phx::Hash40{hash: hash40("waist")}, 
        &pos, &rot, 1.25 /* *(duration_left / duration_total)*/, false, 0, 
        0, 0, 0, 0, false, false) as u32;

        EffectModule::set_rgb(boma, handle, 0.35, 0.35, 0.35);
    }
    else {
        EffectModule::kill_kind(boma, smash::phx::Hash40{hash: hash40("sys_steam")}, false, true);
        FIGHTER_GLOBALS[entry_id as usize].once = false;
    }
}

pub unsafe fn sheik_ga_buff(boma : &mut BattleObjectModuleAccessor) {
    let pos = Vector3f  {x : 0.0, y : 0.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    if FIGHTER_GLOBALS[entry_id as usize].ga_on && !FIGHTER_GLOBALS[entry_id as usize].once {
        FIGHTER_GLOBALS[entry_id as usize].once = true;
        let handle: u32 = EffectModule::req_follow(boma,
        smash::phx::Hash40{hash: hash40("sys_steam")},
        smash::phx::Hash40{hash: hash40("waist")}, 
        &pos, &rot, 1.25 /* *(duration_left / duration_total)*/, false, 0, 
        0, 0, 0, 0, false, false) as u32;

        EffectModule::set_rgb(boma, handle, -0.35, -0.35, -0.35);
    }
    else {
        EffectModule::kill_kind(boma, smash::phx::Hash40{hash: hash40("sys_steam")}, false, true);
        FIGHTER_GLOBALS[entry_id as usize].once = false;
    }
}

pub unsafe fn mariod_buff_effect(boma : &mut BattleObjectModuleAccessor) {

    let pos = Vector3f  {x : 0.0, y : 5.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

    if WorkModule::get_int(boma, FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_GA_MEDECINE_TIMER) > 0 && !FIGHTER_GLOBALS[entry_id as usize].once {
        FIGHTER_GLOBALS[entry_id as usize].once = true;
        let handle: u32 = EffectModule::req_follow(boma,
            smash::phx::Hash40{hash: hash40("sys_aura_light")},
            smash::phx::Hash40{hash: hash40("top")}, 
            &pos, &rot, 6.0 /* *(duration_left / duration_total)*/, false, 0, 
            0, 0, 0, 0, false, false) as u32;
        EffectModule::set_rgb(boma, handle, 4.0, 0.0, 2.5);
    }
    else {
        EffectModule::kill_kind(boma, smash::phx::Hash40{hash: hash40("sys_aura_light")}, false, true);
        FIGHTER_GLOBALS[entry_id as usize].once = false;
    }
}

//Some of them might be in acmd files instead!
pub unsafe fn attacks(lua_state: u64, status_kind: i32, situation_kind: i32, fighter_kind: i32, curr_motion_kind: u64, entry_id: i32) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let status_kind_prev = StatusModule::prev_status_kind(boma, 0);
    let cat1 = ControlModule::get_command_flag_cat(boma, 0);

    //corrin
    //as long as any player has the debuff, corrin has ga_on. kamui_entry checks that the correct corrin gets its buff
    for entry in 0..8 {
        v[entry] = FIGHTER_GLOBALS[entry as usize].kamui_entry;
        if v.contains(&entry_id) {
            if fighter_kind == *FIGHTER_KIND_KAMUI { //probably useless check
                FIGHTER_GLOBALS[entry_id as usize].ga_on = true;
            }
        }
        else {
            if fighter_kind == *FIGHTER_KIND_KAMUI {
                FIGHTER_GLOBALS[entry_id as usize].ga_on = false;
            }
        }
    }
    //Type: debuff / buff
    if status_kind == *FIGHTER_STATUS_KIND_KAMUI_PIERCE {
        if get_attacker_number(boma) < 8 {
            if smash::app::utility::get_kind(&mut *get_boma(get_attacker_number(boma) as i32)) == *FIGHTER_KIND_KAMUI {
                FIGHTER_GLOBALS[entry_id as usize].kamui_entry = get_attacker_number(boma) as i32; //keeps track of who has given the debuff
                FIGHTER_GLOBALS[entry_id as usize].kamui_debuff = true; 
                FIGHTER_GLOBALS[entry_id as usize].kamui_debuff_timer = 1; //hold timer while the opponent is pinned
            }
        }
    }
    if FIGHTER_GLOBALS[entry_id as usize].kamui_debuff && FIGHTER_GLOBALS[entry_id as usize].kamui_debuff_timer <= 600 {
        AttackModule::set_power_mul(boma, 0.75);
        kamui_debuff_effect(boma);
        FIGHTER_GLOBALS[entry_id as usize].kamui_debuff_timer += 1;
    }
    else {
        AttackModule::set_power_mul(boma, 1.0);
        FIGHTER_GLOBALS[entry_id as usize].kamui_debuff_timer = 0;
        FIGHTER_GLOBALS[entry_id as usize].kamui_debuff = false;
        FIGHTER_GLOBALS[entry_id as usize].kamui_entry = 255;
    }

    //terry (dolly)
        //type: buff (go! meter)
        //hitting with power wave above 100% unlocks super specials
    //check if pw hits someone
    if StopModule::is_damage(boma) && get_attacker_number(boma) < 8 {
        if smash::app::utility::get_kind(&mut *get_boma(get_attacker_number(boma) as i32)) == *FIGHTER_KIND_DOLLY &&
         StatusModule::status_kind(&mut *get_boma(get_attacker_number(boma) as i32)) == *FIGHTER_STATUS_KIND_SPECIAL_N {
            FIGHTER_GLOBALS[(get_attacker_number(boma) as i32) as usize].ga_on = true;
        }
    }
    
    //terry: check if super specials hit someone. deactivate on hit
    if StopModule::is_damage(boma) && get_attacker_number(boma) < 8 {
        let terry_motion = MotionModule::motion_kind(&mut *get_boma(get_attacker_number(boma) as i32));
        if smash::app::utility::get_kind(&mut *get_boma(get_attacker_number(boma) as i32)) == *FIGHTER_KIND_DOLLY &&
         [hash40("super_special"), hash40("super_special2_blow")].contains(&terry_motion) {
            FIGHTER_GLOBALS[(get_attacker_number(boma) as i32) as usize].terry_allow_super = false;
        }
    }

    //sheik: checks if vanish hits an opponent
    if StopModule::is_damage(boma) && get_attacker_number(boma) < 8 {
        let sheik_motion = MotionModule::motion_kind(&mut *get_boma(get_attacker_number(boma) as i32));
        if smash::app::utility::get_kind(&mut *get_boma(get_attacker_number(boma) as i32)) == *FIGHTER_KIND_SHEIK &&
         [hash40("special_hi_start"), hash40("special_air_hi_start")].contains(&sheik_motion) {
            FIGHTER_GLOBALS[(get_attacker_number(boma) as i32) as usize].ga_on = true;
        }
    }

    //Dr.Mario
    //type: buff
    //if a supervitamin pill hits an opponent while drMario is still in endlag, he gains a temporary buff to movement
    //the speed buff is in lib.rs
    //detecting if drMario's pills hit someone
    if StopModule::is_damage(boma) && get_attacker_number(boma) < 8 {
        let attacker_number = get_attacker_number(boma) as i32;
        if smash::app::utility::get_kind(&mut *get_boma(attacker_number)) == *FIGHTER_KIND_MARIOD &&
         [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) && //only gives doc his ga on hit
         StatusModule::status_kind(&mut *get_boma(attacker_number)) == *FIGHTER_STATUS_KIND_SPECIAL_N &&
         WorkModule::get_int(&mut *get_boma(attacker_number), FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_GA_MEDECINE_TIMER) <= 0 && MotionModule::frame(&mut *get_boma(attacker_number)) <= 30.0 { //frame 30 is actually frame x, because of acmd speed modifier
            FIGHTER_GLOBALS[attacker_number as usize].ga_on = true;
        }
    }

    //Ganon - Warlock's Darkest Flight
        //type: cancel
        //cancels aerial side b with up b to regrab
    if status_kind_prev == FIGHTER_STATUS_KIND_CATCHED_AIR_GANON {
        //if get_attacker_number(boma) < 8 {
            if FIGHTER_GLOBALS[(get_attacker_number(boma) as i32) as usize].ga_on {
                FIGHTER_GLOBALS[entry_id as usize].ganon_ga_is_victim = true;
                FIGHTER_GLOBALS[entry_id as usize].ganon_ga_victim_duration = 60;
            }
        //}
    }
    if FIGHTER_GLOBALS[entry_id as usize].ganon_ga_is_victim {
        StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_FALL, true);
        MotionModule::change_motion_kind(boma, Hash40{hash: hash40("fall")});
    }
    if FIGHTER_GLOBALS[entry_id as usize].ganon_ga_victim_duration >= 0 {
        FIGHTER_GLOBALS[entry_id as usize].ganon_ga_victim_duration -= 1;
    }
    if FIGHTER_GLOBALS[entry_id as usize].ganon_ga_victim_duration <= 0 {
        FIGHTER_GLOBALS[entry_id as usize].ganon_ga_is_victim = false;
    }

    //ness GA projectile code. FIXME doesn't work.
    if StopModule::is_hit(boma) && get_attacker_number(boma) < 8 {
        let ness_status = StatusModule::status_kind(&mut *get_boma(get_attacker_number(boma) as i32));
        if smash::app::utility::get_kind(&mut *get_boma(get_attacker_number(boma) as i32)) == *FIGHTER_KIND_NESS &&
         [*FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_AGAIN, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD].contains(&ness_status) &&
         StatusModule::situation_kind(&mut *get_boma(get_attacker_number(boma) as i32)) == *SITUATION_KIND_AIR {
            FIGHTER_GLOBALS[(get_attacker_number(boma) as i32) as usize].ga_on = true;
        }
    }

    //Counter hit GAs
    if StopModule::is_hit(boma) && get_attacker_number(boma) < 8 {
        let attacker_curr_motion = MotionModule::motion_kind(&mut *get_boma(get_attacker_number(boma) as i32));
        let prev_status = StatusModule::prev_status_kind(boma, 0);
        if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100,
                *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_HI3,
                *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_LW4,
                *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&prev_status) {
            //palu GA
            if smash::app::utility::get_kind(&mut *get_boma(get_attacker_number(boma) as i32)) == *FIGHTER_KIND_PALUTENA && attacker_curr_motion == hash40("attack_dash") {
                FIGHTER_GLOBALS[(get_attacker_number(boma) as i32) as usize].ga_on = true;
            }
            //Kazuya Abolishing Fist
            // if smash::app::utility::get_kind(&mut *get_boma(get_attacker_number(boma) as i32)) == *FIGHTER_KIND_DEMON && attacker_curr_motion == hash40("abolishingfist") {
            //     FIGHTER_GLOBALS[(get_attacker_number(boma) as i32) as usize].counterhit = true;
            // }
        }
        // if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
        //     FIGHTER_GLOBALS[entry_id as usize].counterhit = false;
        // }
    }

    //Puff (purin)
        // when rest is used vs an opponent without the mark, reduces endlag on rest and heals
        //gives opponent the mark when hit by up special
        if StopModule::is_hit(boma) && get_attacker_number(boma) < 8 {
            let puff_curr_motion = MotionModule::motion_kind(&mut *get_boma(get_attacker_number(boma) as i32));
            if smash::app::utility::get_kind(&mut *get_boma(get_attacker_number(boma) as i32)) == *FIGHTER_KIND_PURIN &&
             [hash40("special_hi_r"), hash40("special_hi_l"), hash40("special_air_hi_r"), hash40("special_air_hi_l")].contains(&puff_curr_motion) &&
             !FIGHTER_GLOBALS[entry_id as usize].puff_mark {
                FIGHTER_GLOBALS[entry_id as usize].puff_mark = true;
                FIGHTER_GLOBALS[entry_id as usize].puff_mark_duration = 420;
            }
        }
        //mark decay and cleanup
        if FIGHTER_GLOBALS[entry_id as usize].puff_mark {
            zelda_buff_effect(boma);
            FIGHTER_GLOBALS[entry_id as usize].puff_mark_duration -= 1;
            if FIGHTER_GLOBALS[entry_id as usize].puff_mark_duration <= 0 {
                FIGHTER_GLOBALS[entry_id as usize].puff_mark = false;
                FIGHTER_GLOBALS[entry_id as usize].puff_mark_duration = 420;
            }
        }

        //checks if victim doesn't have sing's mark, then gives puff their ga. TODO: Currently gives puff their GA on shield
        if StopModule::is_hit(boma) && get_attacker_number(boma) < 8 {
            let puff_curr_motion = MotionModule::motion_kind(&mut *get_boma(get_attacker_number(boma) as i32));

            if ![*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&status_kind) &&
                 smash::app::utility::get_kind(&mut *get_boma(get_attacker_number(boma) as i32)) == *FIGHTER_KIND_PURIN {
                if [hash40("special_lw_r"), hash40("special_lw_l"), hash40("special_air_lw_r"), hash40("special_air_lw_l")].contains(&puff_curr_motion) {
                    if !FIGHTER_GLOBALS[entry_id as usize].puff_mark && !FIGHTER_GLOBALS[(get_attacker_number(boma) as i32) as usize].once {
                        FIGHTER_GLOBALS[(get_attacker_number(boma) as i32) as usize].ga_on = true;
                        FIGHTER_GLOBALS[(get_attacker_number(boma) as i32) as usize].puff_rest_hit = true;
                    }
                }
                else {
                    FIGHTER_GLOBALS[(get_attacker_number(boma) as i32) as usize].once = false;
                    FIGHTER_GLOBALS[(get_attacker_number(boma) as i32) as usize].puff_rest_hit = false;
                }
            }
        }
}