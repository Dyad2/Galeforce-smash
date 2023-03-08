use smash::app::{BattleObjectModuleAccessor, lua_bind::*, FighterUtil::*};
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;
use smash::phx::Vector3f;

use galeforce_utils::vars::*;
use custom_var::*;

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

    let handle = EffectModule::req_follow(boma,
        smash::phx::Hash40{hash: hash40("sys_counter_flash")},
        smash::phx::Hash40{hash: hash40("top")}, 
        &pos, &rot, size /* *(duration_left / duration_total)*/, false, 0, 
        0, 0, 0, 0, false, false) as u32;
        
    EffectModule::set_rgb(boma, handle, 5., 0.05, 2.25);
    flash_eye_info(boma);
}

pub unsafe fn robin_ignis_effect(fighter: &mut L2CFighterCommon) {
    
    let pos = Vector3f  {x : 1.5, y : 0.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};

    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
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

pub unsafe fn zelda_buff_effect(fighter: &mut L2CFighterCommon) {
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

pub unsafe fn kamui_debuff_effect(fighter: &mut L2CFighterCommon) {
    let pos = Vector3f  {x : 0.0, y : 0.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};

    if VarModule::get_int(fighter.battle_object, commons::instance::int::KAMUI_DRAGON_HEX_DURATION) % 15 == 0 {
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

pub unsafe fn chrom_disable_dance_effect(fighter: &mut L2CFighterCommon) {
    let pos = Vector3f  {x : 0.0, y : 0.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};

    if VarModule::get_int(fighter.battle_object, commons::instance::int::FRAME_COUNTER) % 10 == 1  {
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

pub unsafe fn sheik_ga_buff(fighter: &mut L2CFighterCommon) {
    let pos = Vector3f  {x : 0.0, y : 0.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};

    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) && !VarModule::is_flag(fighter.battle_object, commons::instance::flag::DO_ONCE) {
        VarModule::on_flag(fighter.battle_object, commons::instance::flag::DO_ONCE);
        let handle: u32 = EffectModule::req_follow(fighter.module_accessor,
        smash::phx::Hash40{hash: hash40("sys_steam")},
        smash::phx::Hash40{hash: hash40("waist")}, 
        &pos, &rot, 1.25 /* *(duration_left / duration_total)*/, false, 0, 
        0, 0, 0, 0, false, false) as u32;

        EffectModule::set_rgb(fighter.module_accessor, handle, -0.35, -0.35, -0.35);
    }
    else {
        EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_steam")}, false, true);
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::DO_ONCE);
    }
}

pub unsafe fn mariod_buff_effect(fighter: &mut L2CFighterCommon) {
    let pos = Vector3f  {x : 0.0, y : 5.0, z : 0.0};
    let rot = Vector3f  {x : 0.0, y : 90.0, z : 0.0};

    if VarModule::get_int(fighter.battle_object, mariod::instance::int::GA_MEDECINE_TIMER) > 0 && !VarModule::is_flag(fighter.battle_object, commons::instance::flag::DO_ONCE) {
        VarModule::on_flag(fighter.battle_object, commons::instance::flag::DO_ONCE);
        let handle: u32 = EffectModule::req_follow(fighter.module_accessor,
            smash::phx::Hash40{hash: hash40("sys_aura_light")},
            smash::phx::Hash40{hash: hash40("top")}, 
            &pos, &rot, 6.0 /* *(duration_left / duration_total)*/, false, 0, 
            0, 0, 0, 0, false, false) as u32;
        EffectModule::set_rgb(fighter.module_accessor, handle, 4.0, 0.0, 2.5);
    }
    else {
        EffectModule::kill_kind(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_aura_light")}, false, true);
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::DO_ONCE);
    }
}

//Some of them might be in acmd files instead!
//pub unsafe fn attacks(fighter : &mut L2CFighterCommon, status_kind: i32) {
//     let status_kind_prev = StatusModule::prev_status_kind(fighter.module_accessor, 0);
//     //let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
//     let attacker_number = get_attacker_number(&mut *fighter.module_accessor);

    //corrin, TODO doesnt work? probably doesnt run at correct frame for the opponent to be in the pierce status?
    // if attacker_kind == *WEAPON_KIND_KAMUI_SPEARHAND {
    //     if StatusModule::status_kind(defender_boma) == *FIGHTER_STATUS_KIND_KAMUI_PIERCE {
    //         println!("corrin");
    //         VarModule::on_flag(owner_object, commons::instance::flag::GALEFORCE_ATTACK_ON); //gives corrin their buff
    //         VarModule::set_int(defender_object, commons::instance::int::KAMUI_DRAGON_HEX_DURATION, 600); //hold timer while the opponent is pinned
    //     }
    // }

    //corrin
    // if status_kind == *FIGHTER_STATUS_KIND_KAMUI_PIERCE && attacker_number < 8 {
    //     let attacker_object_id =  (*get_boma(attacker_number as i32)).battle_object_id;
    //     let attacker_object = get_battle_object_from_id(attacker_object_id);
    //     if smash::app::utility::get_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KIND_KAMUI /*&& StatusModule::status_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL*/ {
    //         VarModule::on_flag(attacker_object, commons::instance::flag::GALEFORCE_ATTACK_ON); //gives corrin their buff
    //         VarModule::set_int(fighter.battle_object, commons::instance::int::KAMUI_DRAGON_HEX_DURATION, 600); //hold timer while the opponent is pinned
    //     }
    // }
    // if VarModule::get_int(fighter.battle_object, commons::instance::int::KAMUI_DRAGON_HEX_DURATION) > 0 {
    //     AttackModule::set_power_mul(fighter.module_accessor, 0.75);
    //     VarModule::add_int(fighter.battle_object, commons::instance::int::KAMUI_DRAGON_HEX_DURATION, -1);
    //     kamui_debuff_effect(fighter);
    // }
    // else {
    //     AttackModule::set_power_mul(fighter.module_accessor, 1.0);
    // }

//     //terry (dolly)
//         //type: buff (go! meter)
//         //hitting with power wave above 100% unlocks super specials
//     //check if pw hits someone
//     if StopModule::is_damage(fighter.module_accessor) && attacker_number < 8 {
//         let attacker_object_id =  (*get_boma(attacker_number as i32)).battle_object_id;
//         let attacker_object = get_battle_object_from_id(attacker_object_id);
//         if smash::app::utility::get_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KIND_DOLLY && StatusModule::status_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_STATUS_KIND_SPECIAL_N {
//             VarModule::on_flag(attacker_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
//         }
//     }
    
//     //terry: check if super specials hit someone. deactivate on hit
//     if StopModule::is_damage(fighter.module_accessor) && attacker_number < 8 {
//         let attacker_object_id =  (*get_boma(attacker_number as i32)).battle_object_id;
//         let attacker_object = get_battle_object_from_id(attacker_object_id);
//         let terry_motion = MotionModule::motion_kind(&mut *get_boma(attacker_number as i32));
//         if smash::app::utility::get_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KIND_DOLLY &&
//          [hash40("super_special"), hash40("super_special2_blow")].contains(&terry_motion) {
//             VarModule::off_flag(attacker_object, commons::instance::flag::GALEFORCE_ATTACK_CONFIRM);
//         }
//     }

//     //sheik: checks if vanish hits an opponent
//     if StopModule::is_damage(fighter.module_accessor) && attacker_number < 8 {
//         let attacker_object_id =  (*get_boma(attacker_number as i32)).battle_object_id;
//         let attacker_object = get_battle_object_from_id(attacker_object_id);
//         let sheik_motion = MotionModule::motion_kind(&mut *get_boma(attacker_number as i32));
//         if smash::app::utility::get_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KIND_SHEIK &&
//          [hash40("special_hi_start"), hash40("special_air_hi_start")].contains(&sheik_motion) {
//             VarModule::on_flag(attacker_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
//         }
//     }

//     //Dr.Mario
//     //type: buff
//     //if a supervitamin pill hits an opponent while drMario is close, he gains a temporary buff to movement
//     //the speed buff is in lib.rs
//     //detecting if drMario's pills hit someone
//     if StopModule::is_damage(fighter.module_accessor) && attacker_number < 8 {
//         //let attacker_number = attacker_number as i32;
//         let attacker_object_id =  (*get_boma(attacker_number as i32)).battle_object_id;
//         let attacker_object = get_battle_object_from_id(attacker_object_id);
//         if smash::app::utility::get_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KIND_MARIOD
//           && [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) //only gives doc his ga on hit
//           && StatusModule::status_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_STATUS_KIND_SPECIAL_N
//           && VarModule::get_int(attacker_object, mariod::instance::int::GA_MEDECINE_TIMER) <= 0
//           && (PostureModule::pos_x(fighter.module_accessor) - PostureModule::pos_x(get_boma(attacker_number as i32))).abs() < 33.0
//           && (PostureModule::pos_y(fighter.module_accessor) - PostureModule::pos_y(get_boma(attacker_number as i32))).abs() < 28.0 {
//             VarModule::on_flag(attacker_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
//         }
//     }

//     //Ganon - Warlock's Darkest Flight
//         //type: cancel
//         //cancels aerial side b with up b to regrab
//     if status_kind_prev == FIGHTER_STATUS_KIND_CATCHED_AIR_GANON && attacker_number < 8 {
//         let attacker_object_id =  (*get_boma(attacker_number as i32)).battle_object_id;
//         let attacker_object = get_battle_object_from_id(attacker_object_id);
//         if VarModule::is_flag(attacker_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
//             VarModule::on_flag(fighter.battle_object, commons::instance::flag::IS_VICTIM_GANON_GA);
//         }
//     }
//     if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_VICTIM_GANON_GA) 
//     && ![*FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *FIGHTER_STATUS_KIND_CATCHED_GANON, *FIGHTER_STATUS_KIND_CLUNG_GANON, *FIGHTER_STATUS_KIND_FALL].contains(&status_kind)
//     && attacker_number < 8 {
//         StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
//         MotionModule::change_motion_kind(fighter.module_accessor, Hash40{hash: hash40("fall")});
//         let fighter_pos = Vector3f { x: PostureModule::pos_x(&mut *get_boma(attacker_number as i32)) + (7.5 * PostureModule::lr(&mut *get_boma(attacker_number as i32))), y: PostureModule::pos_y(&mut *get_boma(attacker_number as i32)) + 11.0, z: PostureModule::pos_z(&mut *get_boma(attacker_number as i32))};
//         PostureModule::set_pos(fighter.module_accessor, &fighter_pos);
//     }
//     else {
//         VarModule::off_flag(fighter.battle_object, commons::instance::flag::IS_VICTIM_GANON_GA);
//     }

//     //Counter hit GAs
//     if StopModule::is_hit(fighter.module_accessor) && attacker_number < 8 {
//         let attacker_curr_motion = MotionModule::motion_kind(&mut *get_boma(attacker_number as i32));
//         let prev_status = StatusModule::prev_status_kind(fighter.module_accessor, 0);
//         let attacker_object_id =  (*get_boma(attacker_number as i32)).battle_object_id;
//         let attacker_object = get_battle_object_from_id(attacker_object_id);
//         if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100,
//                 *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_HI3,
//                 *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_LW4,
//                 *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&prev_status) {
//             //palu GA
//             if smash::app::utility::get_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KIND_PALUTENA && attacker_curr_motion == hash40("attack_dash") {
//                 VarModule::on_flag(attacker_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
//             }
//             //Kazuya Abolishing Fist
//             // if smash::app::utility::get_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KIND_DEMON && attacker_curr_motion == hash40("abolishingfist") {
//             //     FIGHTER_GLOBALS[(attacker_number as i32) as usize].counterhit = true;
//             // }
//         }
//     }

//     //Puff (purin)
//         // when rest is used vs an opponent without the mark, reduces endlag on rest and heals
//         //gives opponent the mark when hit by up special
//         if StopModule::is_hit(fighter.module_accessor) && attacker_number < 8 {
//             let puff_curr_motion = MotionModule::motion_kind(&mut *get_boma(attacker_number as i32));
//             if smash::app::utility::get_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KIND_PURIN &&
//              [hash40("special_hi_r"), hash40("special_hi_l"), hash40("special_air_hi_r"), hash40("special_air_hi_l")].contains(&puff_curr_motion) &&
//              !VarModule::is_flag(fighter.battle_object, commons::instance::flag::PURIN_MARK) {
//                 VarModule::on_flag(fighter.battle_object, commons::instance::flag::PURIN_MARK);
//                 VarModule::set_int(fighter.battle_object, commons::instance::int::PURIN_MARK_DURATION, 420);
//             }
//         }
//         //mark decay and cleanup
//         if VarModule::is_flag(fighter.battle_object, commons::instance::flag::PURIN_MARK) {
//             zelda_buff_effect(fighter);
//             VarModule::sub_int(fighter.battle_object, commons::instance::int::PURIN_MARK_DURATION, 1);
//             if VarModule::get_int(fighter.battle_object, commons::instance::int::PURIN_MARK_DURATION) <= 0 {
//                 VarModule::off_flag(fighter.battle_object, commons::instance::flag::PURIN_MARK);
//             }
//         }

//         //checks if victim doesn't have sing's mark, then gives puff their ga.
//         if StopModule::is_hit(fighter.module_accessor) && attacker_number < 8 {
//             let attacker_object_id =  (*get_boma(attacker_number as i32)).battle_object_id;
//             let attacker_object = get_battle_object_from_id(attacker_object_id);
//             if is_status_damage(&mut *fighter.module_accessor) && smash::app::utility::get_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KIND_PURIN {
//                 let puff_curr_motion = MotionModule::motion_kind(&mut *get_boma(attacker_number as i32));
//                 if [hash40("special_lw_r"), hash40("special_lw_l"), hash40("special_air_lw_r"), hash40("special_air_lw_l")].contains(&puff_curr_motion) {
//                     if !VarModule::is_flag(fighter.battle_object, commons::instance::flag::PURIN_MARK) && !VarModule::is_flag(attacker_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
//                         VarModule::on_flag(attacker_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
//                     }
//                 }
//             }
//         }
// }