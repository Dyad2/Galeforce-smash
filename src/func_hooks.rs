use smash::phx::Vector4f;

use {
    custom_var::*,
    skyline::{
        hooks::{
            getRegionAddress,
            Region
        },
    },
    smash::{
        hash40,
        app::{lua_bind::*, FighterManager, *},
        lib::{lua_const::*},
        phx::{Vector3f, Hash40},
    },
    galeforce_utils::{
        utils::*,
        utils::get_battle_object_from_id,
        vars,
        vars::*,
    },
};

// #[skyline::hook( replace = L2CFighterBase::change_status)]
// pub unsafe fn change_status_replace(
// fighter: L2CFighterBase,
// status: L2CValue,
// flag: L2CValue) {
//     println!("in change_status");

//     if fighter.global_table[KIND].get_i32() == *FIGHTER_KIND_PEACH && !VarModule::is_flag(fighter.battle_object, peach::instance::flag::ALLOW_FLOAT) {
//         let status_i32 = status.get_i32();
//         if [*FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT].contains(&status_i32) {
//             return;
//         }
//         else {
//             call_original!(fighter.clone(), status.clone(), flag.clone());
//         }
//     }
//     call_original!(fighter.clone(), status.clone(), flag.clone());
// }

#[skyline::hook( replace = StatusModule::change_status_request_from_script)]
pub unsafe fn change_status_request_replace(
boma: &mut smash::app::BattleObjectModuleAccessor,
status: i32,
flag: i32) {
    println!("in change_status_request");
    let fighter_kind = smash::app::utility::get_kind(boma);
    let object_id = boma.battle_object_id;
    let object = get_battle_object_from_id(object_id);

    if fighter_kind == *FIGHTER_KIND_PEACH && !VarModule::is_flag(object, peach::instance::flag::ALLOW_FLOAT) {
        if [*FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT].contains(&status) {
            return;
        }
        else {
            call_original!(boma, status, flag);
        }
    }
    call_original!(boma, status, flag);
}

#[skyline::hook( replace = StatusModule::change_status_force)]
pub unsafe fn change_status_force_replace(
boma: &mut smash::app::BattleObjectModuleAccessor,
status: i32,
flag: i32) {
    println!("in change_status_force");
    let fighter_kind = smash::app::utility::get_kind(boma);
    let object_id = boma.battle_object_id;
    let object = get_battle_object_from_id(object_id);

    if fighter_kind == *FIGHTER_KIND_PEACH && !VarModule::is_flag(object, peach::instance::flag::ALLOW_FLOAT) {
        if [*FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT].contains(&status) {
            return;
        }
        else {
            call_original!(boma, status, flag);
        }
    }
    call_original!(boma, status, flag);
}

#[skyline::hook( replace = StatusModule::set_status_kind_interrupt)]
pub unsafe fn set_status_interrupt_replace(
boma: &mut smash::app::BattleObjectModuleAccessor,
status: i32) {
    println!("in set_status_interrupt");
    let fighter_kind = smash::app::utility::get_kind(boma);
    let object_id = boma.battle_object_id;
    let object = get_battle_object_from_id(object_id);

    if fighter_kind == *FIGHTER_KIND_PEACH && !VarModule::is_flag(object, peach::instance::flag::ALLOW_FLOAT) {
        if [*FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT].contains(&status) {
            return;
        }
        else {
            call_original!(boma, status);
        }
    }
    call_original!(boma, status);
}
//author: 
// var reset code: wuboy! custom_var stuff. resets status vars when status changes.
// ecb teleport is adapted from hdr code
#[skyline::hook( replace = StatusModule::init_settings )]
pub unsafe fn init_settings_replace(
    boma: *mut BattleObjectModuleAccessor,
    situation: SituationKind,
    kinetic: i32,
    correct: u32,
    cliff_check: GroundCliffCheckKind,
    jostle: bool,
    keep_flag: i32,
    keep_int: i32,
    keep_float: i32,
    arg10: i32,
) {
    let mut mask = 0;
    if keep_flag == *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG {
        mask += VarModule::RESET_STATUS_FLAG;
    }
    if keep_int == *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT {
        mask += VarModule::RESET_STATUS_INT;
        mask += VarModule::RESET_STATUS_INT64;
    }
    if keep_float == *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT {
        mask += VarModule::RESET_STATUS_FLOAT;
    }
    let object_id = (*boma).battle_object_id;
    let object = get_battle_object_from_id(object_id);
    VarModule::reset(object, mask);

    // teleports fighters on landing
    // The ECB shift code runs in opff, so it runs a frame late,
    // which causes characters to be stuck halfway into the ground on the frame they land
    // so they need to moved back up on that frame
    if is_fighter(boma) {
        if ![*FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_CAPTURE_CUT,
            *FIGHTER_STATUS_KIND_THROWN,
        ].contains(&StatusModule::prev_status_kind(boma, 1))
        && ![
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_CAPTURE_CUT,
            *FIGHTER_STATUS_KIND_ENTRY,
            *FIGHTER_STATUS_KIND_THROWN,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_BURY,
            *FIGHTER_STATUS_KIND_BURY_WAIT
        ].contains(&StatusModule::prev_status_kind(boma, 0)/*&status_kind*/)
        && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
        && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
        && VarModule::get_float(object, vars::commons::instance::float::ECB_OFFSET_Y) != 0.0 {
            if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                if StatusModule::prev_situation_kind(boma) != *SITUATION_KIND_GROUND {
                    let fighter_pos = smash::phx::Vector3f {x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma) + VarModule::get_float(object, vars::commons::instance::float::ECB_OFFSET_Y), z: PostureModule::pos_z(boma)};
                    PostureModule::set_pos(boma, &fighter_pos);
                }
            }
        }
    }

    original!()(boma, situation, kinetic, correct, cliff_check, jostle, keep_flag, keep_int, keep_float, arg10)
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_flag)]
pub unsafe fn is_flag_replace(
boma: &mut smash::app::BattleObjectModuleAccessor,
flag: i32) -> bool {
    
    if is_training_mode() {
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            let kind = smash::app::utility::get_kind(boma);
            let ret_autocancel = original!()(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            let mut ret_autocancel_pickel = false;
            if kind == *FIGHTER_KIND_PICKEL {
                ret_autocancel_pickel = original!()(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_HI_ENABLE_LANDING);
            }

            //autocancel overlay. should be in status, can't find a valid status for it
            if !ret_autocancel || (!ret_autocancel_pickel && kind == *FIGHTER_KIND_PICKEL) {
                let cmb_vec1 = Vector4f{x: 0.35, y: 0.35, z: 0.66, w: 1.0};
                let cmb_vec2 = Vector4f{x: 0.35, y: 0.35, z: 0.66, w: 1.0};
                ColorBlendModule::set_main_color(boma, &cmb_vec1, &cmb_vec2, 1.0, 0.33, 0, true);
            }
            else {
                ColorBlendModule::cancel_main_color(boma, 0);
            }
        }
        else {
            ColorBlendModule::cancel_main_color(boma, 0);
        }
    }
    original!()(boma, flag)
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::get_float)]
pub unsafe fn get_float_replace(
boma: &mut smash::app::BattleObjectModuleAccessor,
float: i32) -> f32 {
    let object_id = boma.battle_object_id;
    let object = get_battle_object_from_id(object_id);

    if smash::app::utility::get_kind(boma) == *FIGHTER_KIND_LUCARIO
      && VarModule::get_int(object, lucario::instance::int::MAX_AURA_TIMER) > 0 
      && [*FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_DEBUG_AURAPOWER,
          /* *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_EFFECT_SCALE, */
          *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_PREV_AURAPOWER, 
          *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER].contains(&float) {
            return 1.33
    }
    else {
        original!()(boma, float)
    }
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_replace(
boma: &mut smash::app::BattleObjectModuleAccessor,
term: i32
) -> bool {
    let fighter_kind = smash::app::utility::get_kind(boma);
    let object_id = boma.battle_object_id;
    let object = get_battle_object_from_id(object_id);
    let module_accessor = (*object).module_accessor;
    let curr_motion_kind = MotionModule::motion_kind(module_accessor);
    let status_kind = StatusModule::status_kind(module_accessor);
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);
    let prev_status_kind_1 = StatusModule::prev_status_kind(module_accessor, 1);
    let situation_kind = StatusModule::situation_kind(module_accessor);

    //falling from platforms
    //when hit, check if character is falling off
    if ![*FIGHTER_STATUS_KIND_CATCHED_CUT_GANON, *FIGHTER_STATUS_KIND_CLUNG_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *FIGHTER_STATUS_KIND_CATCHED_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON].contains(&prev_status_kind_1) {
        if is_hitlag(module_accessor) && situation_kind == *SITUATION_KIND_GROUND {
            VarModule::on_flag(object,commons::instance::flag::PLATFORM_FALL_STUN);
        }
        if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_DAMAGE].contains(&prev_status_kind) && situation_kind == *SITUATION_KIND_AIR && VarModule::is_flag(object,commons::instance::flag::PLATFORM_FALL_STUN) {
            VarModule::off_flag(object,commons::instance::flag::PLATFORM_FALL_STUN);
        }
        if curr_motion_kind == hash40("fall_damage") {
            if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 30 && situation_kind != *SITUATION_KIND_GROUND {
                return false;
            }
            //allow characters to land :)
            else {
                //fix for ai
                // FIXME causes fighters to never enter tumble
                // if situation_kind != *SITUATION_KIND_GROUND {
                //     StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                // }
                return original!()(boma, term);
            }
        }
    }
    
    if fighter_kind == *FIGHTER_KIND_LUCINA {
        if VarModule::get_int(object, commons::instance::int::FRAME_COUNTER) >= 0 {
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S {
                return false;
            }
        }
    }
    else if fighter_kind == *FIGHTER_KIND_GEKKOUGA {
        if VarModule::get_int(object, commons::instance::int::FRAME_COUNTER) > 0 
        && [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR,
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4,
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4,
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4,
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH,
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S].contains(&term) {
            return false;
        }
    }
    else if fighter_kind == *FIGHTER_KIND_FALCO {
        //TODO: update FAF?
        if WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
            if MotionModule::frame(module_accessor) <= 1.0 {
                VarModule::set_float(object, falco::instance::float::STICK_X, ControlModule::get_stick_x(module_accessor));
                VarModule::set_float(object, falco::instance::float::STICK_Y, ControlModule::get_stick_y(module_accessor));
            }
            //up
            if VarModule::get_float(object, falco::instance::float::STICK_Y) >= 0.46 && VarModule::get_float(object, falco::instance::float::STICK_X).abs() < 0.46 {
                if 93. <= MotionModule::frame(module_accessor) {
                    VarModule::on_flag(object, falco::instance::flag::DIRECTIONAL_AIR_ESCAPE_FAF);
                }
            }
            //side
            if VarModule::get_float(object, falco::instance::float::STICK_X).abs() >= 0.46 && VarModule::get_float(object, falco::instance::float::STICK_Y).abs() < 0.46 {
                if 73. <= MotionModule::frame(module_accessor) {
                    VarModule::on_flag(object, falco::instance::flag::DIRECTIONAL_AIR_ESCAPE_FAF);
                }
            }
            //down
            if VarModule::get_float(object, falco::instance::float::STICK_Y) <= -0.46 && VarModule::get_float(object, falco::instance::float::STICK_X).abs() < 0.46 {
                if 61. <= MotionModule::frame(module_accessor) {
                    VarModule::on_flag(object, falco::instance::flag::DIRECTIONAL_AIR_ESCAPE_FAF);
                }
            }
            if !VarModule::is_flag(object, falco::instance::flag::AIRDASH) {
                if VarModule::is_flag(object, falco::instance::flag::DIRECTIONAL_AIR_ESCAPE_FAF) {
                    return original!()(boma, term);
                }
                else {
                    if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL,
                         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
                         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
                         *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND].contains(&term) {
                        return false;
                    }
                }
            }
        }
        else {
            VarModule::set_float(object, falco::instance::float::STICK_X, 0.0);
            VarModule::set_float(object, falco::instance::float::STICK_Y, 0.0);
            VarModule::off_flag(object, falco::instance::flag::DIRECTIONAL_AIR_ESCAPE_FAF);
        }
    }
    //todo might be useless, this is a param?
    // else if fighter_kind == *FIGHTER_KIND_BAYONETTA && situation_kind == *SITUATION_KIND_AIR && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) < 4 {
    //     if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND].contains(&term) {
    //         return false;
    //     }
    // }
    //bug fixing. the second one aims to remove pmlg
    else if fighter_kind == *FIGHTER_KIND_PICKEL {
        if [
             *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_FALL, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_FALL_AERIAL, 
             *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_AERIAL, 
             *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_SQUAT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_LANDING,
             *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_LANDING_LIGHT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WAIT, 
             *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK_BACK
           ].contains(&status_kind) 
           && [
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK, 
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR, 
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH, 
             *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH
           ].contains(&term) {
            return false;
        }
        if !VarModule::is_flag(object, pickel::instance::flag::ALLOW_SPECIAL_N) && term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N {
            return false;
        }
    }
    //aerial turn
    // if VarModule::get_int(object, commons::instance::int::AIR_TURN_INITIATE_METHOD) == 2 && VarModule::get_int(object,commons::instance::int::AIR_TURN_INPUT_FRAME) <= 3 {
    //     if ![*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR].contains(&term) {
    //         return false;
    //     }
    // }
    //special moves disabling
    if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW].contains(&term) && VarModule::is_flag(object,commons::instance::flag::DISABLE_SPECIAL_ALL) {
        return false;
    }
    else if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI && VarModule::is_flag(object,commons::instance::flag::DISABLE_SPECIAL_HI) {
        return false;
    }
    else if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S && VarModule::is_flag(object,commons::instance::flag::DISABLE_SPECIAL_S) {
        return false;
    }
    
    original!()(boma, term)
}

// samus missile disabling
#[skyline::hook(replace = smash::app::lua_bind::ArticleModule::get_active_num)]
pub unsafe fn get_active_num_replace(
    boma: &mut smash::app::BattleObjectModuleAccessor,
    article_kind: i32,
) -> u64 {
    let fighter_kind = smash::app::utility::get_kind(boma);
    let ret_missile = original!()(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_MISSILE);
    let ret_super = original!()(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_SUPERMISSILE);

    if fighter_kind == *FIGHTER_KIND_SAMUS && article_kind != *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB { //<-- without this samus can't spawn bombs with missiles out
        if ret_missile == 1 || ret_super == 1 {
            return 9; //random high number so another missile won't be generated
        }
    }

    original!()(boma, article_kind)
}

//C stick fixes!
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_attack_air_kind)]
pub unsafe fn get_attack_air_kind_replace(
    boma: &mut smash::app::BattleObjectModuleAccessor,
) -> i32 {
    let object_id = boma.battle_object_id;
    let object = get_battle_object_from_id(object_id);

    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
        VarModule::set_float(object, commons::instance::float::SUBSTICK_X, ControlModule::get_stick_x(boma) * PostureModule::lr(boma));
        VarModule::set_float(object, commons::instance::float::SUBSTICK_Y, ControlModule::get_stick_y(boma));
    }
    else {
        VarModule::set_float(object, commons::instance::float::SUBSTICK_X, 0.0);
        VarModule::set_float(object, commons::instance::float::SUBSTICK_Y, 0.0);
    }

    if !(VarModule::get_float(object, commons::instance::float::SUBSTICK_X) == 0.0 && VarModule::get_float(object, commons::instance::float::SUBSTICK_Y) == 0.0) {
        if VarModule::get_float(object, commons::instance::float::SUBSTICK_X).abs() < 0.46 && VarModule::get_float(object,commons::instance::float::SUBSTICK_Y) > 0.46 {
            VarModule::set_int(object, commons::instance::int::SUBSTICK_AIR_ATTACK, *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI);
        }
        //fair
        if VarModule::get_float(object, commons::instance::float::SUBSTICK_X) > 0.46 && VarModule::get_float(object, commons::instance::float::SUBSTICK_Y).abs() < 0.46 {
            VarModule::set_int(object, commons::instance::int::SUBSTICK_AIR_ATTACK, *FIGHTER_COMMAND_ATTACK_AIR_KIND_F,);
        }
        //bair
        if VarModule::get_float(object, commons::instance::float::SUBSTICK_X) < -0.46 && VarModule::get_float(object, commons::instance::float::SUBSTICK_Y).abs() < 0.46 {
            VarModule::set_int(object, commons::instance::int::SUBSTICK_AIR_ATTACK, *FIGHTER_COMMAND_ATTACK_AIR_KIND_B);
        }
        //dair
        if VarModule::get_float(object, commons::instance::float::SUBSTICK_X).abs() < 0.46 && VarModule::get_float(object, commons::instance::float::SUBSTICK_Y) < -0.46 {
            VarModule::set_int(object, commons::instance::int::SUBSTICK_AIR_ATTACK, *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW);
        }
    }

    if VarModule::get_int(object,  commons::instance::int::SUBSTICK_AIR_ATTACK) != 0 {
        return VarModule::get_int(object, commons::instance::int::SUBSTICK_AIR_ATTACK);
    }

    original!()(boma)
}


// #[skyline::hook(replace = smash::app::lua_bind::ArticleModule::generate_article)]
// pub unsafe fn generate_article_replace(
//     boma: &mut smash::app::BattleObjectModuleAccessor,
//     article_kind: i32,
//     unk1: bool,
//     unk2: i32,
// ) -> u64 {
//     let cat = utility::get_category(&mut *boma);
//     if cat == *BATTLE_OBJECT_CATEGORY_FIGHTER && smash::app::utility::get_kind(boma) == *FIGHTER_KIND_ROSETTA && article_kind == *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO {
//         return 0;
//     }
//     original!()(boma, article_kind, unk1, unk2)
// }

//Galeforce attacks and stuff
#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(
fighter_manager: &mut FighterManager,
attacker_object_id: u32,
defender_object_id: u32, 
move_type: f32,
arg5: i32,
move_type_again: bool) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_object_id);
    let attacker_object = get_battle_object_from_id(attacker_object_id);
    let attacker_cat = utility::get_category(&mut *attacker_boma);

    let defender_boma = sv_battle_object::module_accessor(defender_object_id);
    let defender_object = get_battle_object_from_id(defender_object_id);
    let defender_cat = utility::get_category(&mut *defender_boma);

    let attacker_kind = sv_battle_object::kind(attacker_object_id);

    if attacker_cat == *BATTLE_OBJECT_CATEGORY_WEAPON {
        if defender_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            let owner_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
            let owner_object = get_battle_object_from_id(owner_id);
            let owner_boma = get_boma_from_id(owner_id);

            //terry (dolly)
                //type: buff (go! meter)
                //hitting with power wave above 100% unlocks super specials
            // if attacker_kind == *WEAPON_KIND_DOLLY_WAVE {
            //     VarModule::on_flag(owner_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
            // }

            //sheik: checks if vanish hits an opponent
            if attacker_kind == *WEAPON_KIND_SHEIK_FUSIN {
                VarModule::on_flag(owner_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
            }

            //Dr.Mario
            //type: buff
            //if a supervitamin pill hits an opponent while drMario is close, he gains a temporary buff to movement
            //the speed buff is in float hook
            //detecting if drMario's pills hit someone
            if attacker_kind == *WEAPON_KIND_MARIOD_DRCAPSULE {
                if VarModule::get_int(owner_object, mariod::instance::int::GA_MEDECINE_TIMER) <= 0
                  && (PostureModule::pos_x(defender_boma) - PostureModule::pos_x(owner_boma)).abs() < 33.0
                  && (PostureModule::pos_y(defender_boma) - PostureModule::pos_y(owner_boma)).abs() < 28.0 {
                    VarModule::on_flag(owner_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                }
            }
        }
    }
    else if attacker_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if defender_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            let def_status_kind_prev = StatusModule::prev_status_kind(defender_boma, 0);
            //let attacker_motion = MotionModule::motion_kind(attacker_boma);
            let defender_status = StatusModule::status_kind(defender_boma);
            let attacker_status = StatusModule::status_kind(attacker_boma);

            // if attacker_kind == *FIGHTER_KIND_MEWTWO {
            //     VarModule::on_flag(defender_object, commons::instance::flag::MEWTWO_PRESSURED);
            // }

            //pikachu: for some reason is_infliction_status doesnt work. trying here
            if attacker_kind == *FIGHTER_KIND_PIKACHU && attacker_status == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP {
                VarModule::on_flag(attacker_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
            }

            //Ganon - Warlock's Darkest Flight
                //type: cancel
                //cancels aerial side b with up b to regrab
            if def_status_kind_prev == FIGHTER_STATUS_KIND_CATCHED_AIR_GANON {
                if VarModule::is_flag(attacker_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
                    VarModule::on_flag(defender_object, commons::instance::flag::IS_VICTIM_GANON_GA);
                }
            }
            if VarModule::is_flag(defender_object, commons::instance::flag::IS_VICTIM_GANON_GA)
            && ![*FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *FIGHTER_STATUS_KIND_CATCHED_GANON, *FIGHTER_STATUS_KIND_CLUNG_GANON, *FIGHTER_STATUS_KIND_FALL].contains(&defender_status) {
                StatusModule::change_status_request(defender_boma, *FIGHTER_STATUS_KIND_FALL, true);
                MotionModule::change_motion_kind(defender_boma, Hash40{hash: hash40("fall")});
                let fighter_pos = Vector3f { x: PostureModule::pos_x(attacker_boma) + (7.5 * PostureModule::lr(attacker_boma)), y: PostureModule::pos_y(attacker_boma) + 11.0, z: PostureModule::pos_z(attacker_boma)};
                PostureModule::set_pos(defender_boma, &fighter_pos);
            }
            else {
                VarModule::off_flag(defender_object, commons::instance::flag::IS_VICTIM_GANON_GA);
            }

            //Counter hit GAs
            // if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100,
            //         *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_HI3,
            //         *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_LW4,
            //         *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&defender_status) {
            //     //palu GA
            //     if attacker_kind == *FIGHTER_KIND_PALUTENA {
            //         if attacker_motion == hash40("attack_dash") {
            //             VarModule::on_flag(attacker_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
            //         }
            //     }
                //Kazuya Abolishing Fist
                // if smash::app::utility::get_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KIND_DEMON && attacker_curr_motion == hash40("abolishingfist") {
                //     FIGHTER_GLOBALS[(attacker_number as i32) as usize].counterhit = true;
                // }
            //}

            //Puff (purin) TODO doesnt work
            // when rest is used vs an opponent without the mark, reduces endlag on rest and heals
            //gives opponent the mark when hit by up special
            // if attacker_kind == *FIGHTER_KIND_PURIN {
            //     if [hash40("special_hi_r"), hash40("special_hi_l"), hash40("special_air_hi_r"), hash40("special_air_hi_l")].contains(&attacker_motion) && !VarModule::is_flag(defender_object, commons::instance::flag::PURIN_MARK) {
            //         VarModule::on_flag(defender_object, commons::instance::flag::PURIN_MARK);
            //         VarModule::set_int(defender_object, commons::instance::int::PURIN_MARK_DURATION, 420);
            //     }
            //     //checks if victim doesn't have sing's mark, then gives puff their ga.
            //     if is_status_damage(&mut *defender_boma) {
            //         if [hash40("special_lw_r"), hash40("special_lw_l"), hash40("special_air_lw_r"), hash40("special_air_lw_l")].contains(&attacker_motion) {
            //             if !VarModule::is_flag(defender_object, commons::instance::flag::PURIN_MARK) && !VarModule::is_flag(attacker_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            //                 VarModule::on_flag(attacker_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
            //             }
            //         }
            //     }
            // }
        }
    }
    original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

// #[skyline::hook(offset=INT_OFFSET)]
// pub unsafe fn get_param_int_offset_replace(
// boma: u64,
// param_type: u64,
// param_hash: u64) -> i32 {
//     let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
//     let object_id = module_accessor.battle_object_id;
//     let object = get_battle_object_from_id(object_id);
//     let fighter_kind = smash::app::utility::get_kind(&mut *module_accessor);

//     let ret = original!()(boma, param_type, param_hash);

//     if is_fighter(module_accessor) {
//         if fighter_kind == *FIGHTER_KIND_CAPTAIN && param_hash == hash40("critical_frame") {
//             if !VarModule::is_flag(object, captain::status::flag::ALLOW_SPECIAL_N_CRITICAL) {
//                 return 0;
//             }
//             else {
//                 return ret;
//             }
//         }
//     }
//     return ret;
// }

#[skyline::hook(offset = FLOAT_OFFSET)]
pub unsafe fn get_param_float_offset_replace(
boma: u64, 
param_type: u64, 
param_hash : u64) -> f32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let object_id = module_accessor.battle_object_id;
    let object = get_battle_object_from_id(object_id);
    let status_kind = StatusModule::status_kind(module_accessor);
    let kind = smash::app::utility::get_kind(&mut *module_accessor);
    let prev_status_kind = StatusModule::prev_status_kind(module_accessor, 0);

    let ret = original!()(boma, param_type, param_hash);

    if is_fighter(module_accessor) {
        //increases the knockback required to make sakurai angle lift fighters in the air when they are crouching. TODO: test if this does anything?
        if [*FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_SQUAT_F, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_SQUAT_WAIT].contains(&prev_status_kind) {
            if param_type == hash40("battle_object") {
                if param_hash == hash40("damage_angle_ground_reaction_min") {
                    return ret * 1.33; //should be working :) gotta do the math
                }
            }
        }

        //modifies gravity while in directional airdodges to use the LSI gravity average (a bit higher actually)
        //it has to check for status otherwise it seems to enable the gravity change on other statuses. because the flag actual value is shared with a lot of other flags. 
        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR && WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) { 
            if param_type == hash40("air_accel_y") && !WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL) {
                //return 0.086;
                return 0.1; //this value is arbitrary but feels nice.
            }
        }

        // if [*FIGHTER_KIND_POPO, *FIGHTER_KIND_NANA].contains(&kind) && (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
        //     //let partner_object = get_battle_object_from_id(*FIGHTER_POPO_INSTANCE_WORK_ID_INT_PARTNER_OBJECT_ID);
        //     if param_hash == hash40("nana_opt_dst") {
        //         return 999.0;
        //     }
        // }
        // if kind == *FIGHTER_KIND_BAYONETTA {
        //     if param_hash == hash40("ab_u_rotate") {
        //         if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        //             return -21.0;
        //         }
        //         else {
        //             return -25.0;
        //         }
        //     }
        // }

        if kind == *FIGHTER_KIND_LUIGI {
            if param_hash == hash40("charge_bonus") { //normally this gives 20 bonus frames when side b is smash inputted, but in vl the param is 0, and then the value is set down here for the GA.
                let mul = VarModule::get_int(object, luigi::instance::int::ELEC_CHARGE) as f32;
                return 20.0 * mul;
            }
        }

        //slows down dash speed for characters faster than steve, who has the worst dash speed. faster characters receive a bigger penalty
        //now written in prcxml file
        // if param_type == hash40("dash_speed") {
        //     let dash_speed = ret;
        //     let reduce = dash_speed - 1.45;
        //     let slower_dash = 1.45 + (reduce * 0.67);

        //     return slower_dash;
        // }

        if kind == *FIGHTER_KIND_PURIN && param_hash == 0 && param_type == hash40("weight") {
            if [*FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HIT_END, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD_MAX,
                *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL_AIR, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_TURN].contains(&status_kind) {
                return ret * 2.0;
            }
            else {
                return ret;
            }
        }
        //add up b landing lag to other stuff
        else if [*FIGHTER_KIND_SIMON, *FIGHTER_KIND_RICHTER, *FIGHTER_KIND_GAMEWATCH].contains(&kind) {
            if VarModule::is_flag(object, commons::instance::flag::DISABLE_SPECIAL_HI) {
                if [hash40("landing_attack_air_frame_hi"), hash40("landing_attack_air_frame_b"), hash40("landing_attack_air_frame_n"), hash40("landing_attack_air_frame_f"), hash40("landing_attack_air_frame_lw"), hash40("landing_frame"), hash40("landing_frame_light")].contains(&param_type) {
                    return ret + 20.0;
                }
            }
        }
        else if kind == *FIGHTER_KIND_SHEIK {
            if VarModule::is_flag(object, sheik::instance::flag::ATTACK_AIR_LW_W) && param_type == hash40("landing_attack_air_frame_lw") {
                return 13.0;
            }
        }
        else if kind == *FIGHTER_KIND_MARIOD && VarModule::get_int(object, mariod::instance::int::GA_MEDECINE_TIMER) >= 0 {
            if [hash40("air_accel_x_mul"), hash40("air_speed_x_stable"), hash40("air_brake_x"), hash40("walk_speed_max"), hash40("run_speed_max"), hash40("dash_speed")].contains(&param_type) {
                return ret * 1.66;
            }
        }
    }
    else {
        let owner_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_object = get_battle_object_from_id(owner_id);
        //let owner_boma = get_boma_from_id(owner_id);

        if kind == *WEAPON_KIND_DUCKHUNT_GUNMAN && (VarModule::is_flag(owner_object, commons::instance::flag::GALEFORCE_ATTACK_ON) || VarModule::is_flag(owner_object, duckhunt::instance::flag::GUNMAN_REACTIVATE)) {
            if [hash40("ready_minutes_hige"), hash40("ready_minutes_noppo"), hash40("ready_minutes_kurofuku"), hash40("ready_minutes_sonburero"), hash40("ready_minutes_boss")].contains(&param_hash) {
                return 0.1;
            }
        }
    }

    original!()(boma, param_type, param_hash)
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

pub fn install() {
    unsafe{
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, vars::FLOAT_SEARCH_CODE) {
            vars::FLOAT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, vars::INT_SEARCH_CODE) {
            vars::INT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, vars::NOTIFY_LOG_EVENT_COLLISION_HIT_SEARCH_CODE) {
            vars::NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }
    skyline::install_hooks!(
        //change_status_replace,
        change_status_request_replace,
        change_status_force_replace,
        set_status_interrupt_replace,
        init_settings_replace,
        is_enable_transition_term_replace, //many GAs, some special moves disabling
        is_flag_replace, //training mode overlay
        get_float_replace,
        get_attack_air_kind_replace, //c stick fix
        get_active_num_replace, //Samus missile stuff
        get_param_float_offset_replace,
        //get_param_int_offset_replace, //removing cpt falcon special n critical. doesnt work
        notify_log_event_collision_hit_replace, //GA stuff
        //generate_article_replace, //rosalina but with no luma
    );
}

// from wubor
// #[skyline::hook(offset = 0x6310a0)]
// unsafe fn fighter_handle_damage_hook(object: *mut BattleObject, arg: *const u8) {