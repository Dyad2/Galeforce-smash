//tap jump checks
//if ControlModule::is_enable_flick_jump(boma) && ControlModule::get_stick_y(boma) > 0.7 && ControlModule::get_flick_y(boma) < 3

//related to damage turn?
//FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR

//jump armor is a flag woo
//FIGHTER_YOSHI_INSTANCE_WORK_ID_FLAG_JUMP_AERIAL_ARMOR

//let hitstun = (WorkModule::get_int(boma, *FIGHTER_STATUS_DAMAGE_WORK_INT_FRAME) - WorkModule::get_int(boma, *FIGHTER_STATUS_DAMAGE_WORK_INT_HIT_STOP_FRAME)) as f32;

pub mod galeforce;

use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::{BattleObjectModuleAccessor, SituationKind, utility::get_kind, BattleObject, sv_battle_object, GroundCliffCheckKind};
use smash::{phx::{Vector3f, Hash40}, lua2cpp::{L2CFighterCommon, L2CFighterBase}};
//use smash::*;
use skyline::nn::ro::LookupSymbol;
use skyline::nro::{self, NroInfo};
use smash::lib::LuaConst;
use smash::hash40;
use smashline::*;
use std::mem;

use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::FIGHTER_MANAGER_ADDR;
use crate::ecb_shifts::ecb_shifts;
use crate::edge_cancels::edge_cancels;
use crate::utils::*;
use crate::var::*;

pub static mut TOTAL_FIGHTER: i32 = 0;

//GA related / universal features checks
#[derive(Clone, Copy, Debug)]
pub struct PlayersParameters {
    // C-stick
    pub stick_x: f32, pub stick_y: f32, pub buffered_aerial: i32,
    // ECB and platforms
    pub ecb_offset: f32, pub push_off_disabled: bool, pub push_off_hit: bool,
    // other
    pub edge_cancel_stuff: bool, pub frame_counter: i32, pub jostle: bool, pub wavedash: bool, pub wavedash_cancel: bool, pub wavedash_cancel_frame_counter: i32,
    dtilt_reverse_once: bool, pub blocked_attack: bool,
    // ga stuff
    pub ga_on: bool,  pub ga_on_confirm: bool,  pub once: bool, pub ganon_ga_is_victim: bool, pub ganon_ga_victim_duration: i32, pub ike_ga_xlu: i32, pub falco_airdash: bool, 
    pub revenge_frames: i32, pub doc_buff: i32, pub roy_ga_timer: f32, pub reflet_ga_motion_frame: f32, pub reflet_attack_occur: bool, pub reflet_thunder_fx_handle: u32, pub terry_allow_super: bool,
    pub puff_mark: bool, pub puff_mark_duration: i32, pub puff_rest_hit: bool, pub puff_rest_time: i32, pub chrom_dance_disable_time: i32, pub kamui_debuff: bool,
    pub kamui_debuff_timer: i32, pub kamui_entry: i32, pub greninja_ga_xlu: i32,
    //specific character vars
    pub mariod_nair_damage: f32, pub lucario_aura_charge: bool, pub lucario_aura_max_timer: i32, pub lucario_aura_scale: f32, pub luci_up_b_landing: bool, pub banjump: bool, pub kirby_last_hat: i32, pub krool_has_crown: bool, pub falco_stick_x: f32, pub falco_stick_y: f32,
    pub falco_dairdodge_faf: bool, pub onehat: bool, pub mach_tornado_hit: bool, pub terry_specialn_charge: f32, pub greninja_shuricharge: f32,
    pub bayo_dodge_offset: bool, pub bayo_has_used_dodge_offset_2nd : bool, pub bayo_dodge_offset_ftilt_num: i32, pub rosa_tico_recall: bool, pub ink_arts: bool, pub ink_arts_delay: i32, pub sheik_heavy_variant: bool, pub zelda_farore_cancel: bool
}

impl PlayersParameters {
    const fn new() -> Self {
        PlayersParameters
        {
            // C-stick
            stick_x: 0., stick_y: 0., buffered_aerial: 0,
            // ECB and platforms
            ecb_offset: 0., push_off_disabled: false, push_off_hit: false, 
            // Others
            edge_cancel_stuff: false, frame_counter: 0, jostle: false, wavedash: false,
            wavedash_cancel: false,  wavedash_cancel_frame_counter: 0, dtilt_reverse_once: false, blocked_attack: false,
            //ga stuff
            ga_on: false, ga_on_confirm: false, once: false, ganon_ga_is_victim: false, ganon_ga_victim_duration: 0, ike_ga_xlu: 0,
            falco_airdash: false, revenge_frames: 0, doc_buff: 0,  reflet_ga_motion_frame: 0., reflet_attack_occur: false, reflet_thunder_fx_handle: 0,
            roy_ga_timer: 999.0, terry_allow_super: false, terry_specialn_charge: 0.0,  puff_mark: false, puff_mark_duration: 420,
            puff_rest_hit: false, puff_rest_time: 0, chrom_dance_disable_time: 0, kamui_debuff: false,  kamui_debuff_timer: 0, kamui_entry: 255,
            greninja_ga_xlu: 0,
            //specific character vars
            mariod_nair_damage: 4.0, lucario_aura_charge: false, lucario_aura_max_timer: 0, lucario_aura_scale: 1.0, luci_up_b_landing: false, banjump: false, kirby_last_hat: -1, krool_has_crown: false,
            falco_stick_x: 0., falco_stick_y: 0., falco_dairdodge_faf: false, onehat: true, mach_tornado_hit: false, greninja_shuricharge: 0.0,
            bayo_dodge_offset: false, bayo_has_used_dodge_offset_2nd : false, bayo_dodge_offset_ftilt_num: 0,
            rosa_tico_recall: false, ink_arts: false, ink_arts_delay: 0, sheik_heavy_variant: false, zelda_farore_cancel: false,
        }
    }
}

pub static mut FIGHTER_GLOBALS: [PlayersParameters; 9] = [PlayersParameters::new(); 9]; //reducing to 8 causes crashes

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let fighter_kind = get_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let curr_motion_kind = MotionModule::motion_kind(boma);
        let FIGHTER_MANAGER =  *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
        let cat1 = ControlModule::get_command_flag_cat(boma, 0);
        let cat2 = ControlModule::get_command_flag_cat(boma, 1);
        TOTAL_FIGHTER = FighterManager::total_fighter_num(FIGHTER_MANAGER);

        let fighter_entry_id = smash::app::FighterEntryID(entry_id);
        let fighter_manager =  *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
        let fighter_information = FighterManager::get_fighter_information(fighter_manager, fighter_entry_id) as *mut smash::app::FighterInformation;
        let stock_count = FighterInformation::stock_count(fighter_information);

        //reset parameters when match ends or is reset
        if !FighterManager::is_ready_go(FIGHTER_MANAGER) || status_kind == *FIGHTER_STATUS_KIND_DEAD || KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_RESET {
            let save_hat = FIGHTER_GLOBALS[entry_id as usize].kirby_last_hat;
            let one_hat = FIGHTER_GLOBALS[entry_id as usize].onehat;
            FIGHTER_GLOBALS[entry_id as usize] = PlayersParameters::new();
            FIGHTER_GLOBALS[entry_id as usize].kirby_last_hat = save_hat; //restore hats
            FIGHTER_GLOBALS[entry_id as usize].onehat = one_hat;
            //levin when match begins
            if fighter_kind == *FIGHTER_KIND_REFLET {
                WorkModule::set_int(boma, 8, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
            }
        }

        edge_cancels(boma, status_kind, fighter_kind);
        ecb_shifts(boma, status_kind, situation_kind, fighter_kind);
        galeforce::attacks(lua_state, status_kind, situation_kind, fighter_kind, curr_motion_kind, entry_id);

        if status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, false);
        }

        if !is_operation_cpu(boma) {
            //C stick fixes cleanup, must be here because is_attack won't run in get_atack_air_kind at the correct time
            if situation_kind == *SITUATION_KIND_GROUND {
                if is_status_damage(boma) {
                    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SUBSTICK_AIR_ATTACK);
                    //FIGHTER_GLOBALS[entry_id as usize].buffered_aerial = 0; //interrupted
                }
                if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) { //don't clear if c stick is held
                    if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4,*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
                    *FIGHTER_TANTAN_STATUS_KIND_ATTACK_COMBO, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_RV, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_WAIT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WAIT].contains(&status_kind)
                      && (MotionModule::frame(boma) >= 4.0 /*reset if it's not attack canceled and we're not near endlag*/ && FighterMotionModuleImpl::get_cancel_frame(boma, Hash40::new_raw(curr_motion_kind), false) - MotionModule::frame(boma) >= 6.0) {
                        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SUBSTICK_AIR_ATTACK);
                        //FIGHTER_GLOBALS[entry_id as usize].buffered_aerial = 0; //interrupted
                        } /*needs to be corrected frame i guess?*/
                    }
                if (fighter_kind == *FIGHTER_KIND_DEMON && curr_motion_kind == hash40("attack_lw3") && MotionModule::frame(boma) <= 15.0) { //kazuya dtilt fix
                    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SUBSTICK_AIR_ATTACK);
                    //FIGHTER_GLOBALS[entry_id as usize].buffered_aerial = 0;
                }
            }
            if situation_kind == *SITUATION_KIND_AIR {
                if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) 
                  || is_status_damage(boma) //interrupted
                  || AttackModule::is_attack( boma, 0, false) || AttackModule::is_attack( boma, 1, false) //snake back air doesn't have a hitbox id 0, so checking 1 is necessary in case this also affects other characters
                  || (MotionModule::frame(boma) < 20.0 && [*FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_MURABITO, *FIGHTER_KIND_MIIGUNNER, *FIGHTER_KIND_TANTAN].contains(&fighter_kind)) {
                    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SUBSTICK_AIR_ATTACK);
                    //FIGHTER_GLOBALS[entry_id as usize].buffered_aerial = 0;
                }
            }
            //c stick fix 
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                ControlModule::get_attack_air_kind(boma);
            }

            //dtilt pivots
            if curr_motion_kind == hash40("attack_lw3") {
                if ControlModule::get_stick_x(boma) * PostureModule::lr(boma) < -0.25
                && MotionModule::frame(boma) < 4.0 && !FIGHTER_GLOBALS[entry_id as usize].dtilt_reverse_once {
                    FIGHTER_GLOBALS[entry_id as usize].dtilt_reverse_once = true;
                    let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
                    FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, -1.0);
                    PostureModule::reverse_lr(boma);
                    PostureModule::update_rot_y_lr(boma);
                }
            }
            else {
                FIGHTER_GLOBALS[entry_id as usize].dtilt_reverse_once = false;
            }

            //wavedashes during jumpsquat and footstools
            if (status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT || status_kind == *FIGHTER_STATUS_KIND_TREAD_JUMP) && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::get_stick_x(boma) != 0.0 {
                StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                MotionModule::change_motion_kind(boma, smash::phx::Hash40{hash: hash40("escape_air_slide")});
            }
            //aerial turn around
            //character fixes
            if !(fighter_kind == *FIGHTER_KIND_METAKNIGHT && status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH) 
              && !(fighter_kind == *FIGHTER_KIND_PZENIGAME && status_kind == *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_CHARGE)
              && fighter_kind != FIGHTER_KIND_NANA
              && !(fighter_kind == *FIGHTER_KIND_SHULK && [*FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP].contains(&status_kind)) {
                if situation_kind == SITUATION_KIND_AIR {
                    //Acknowledge first stick flick
                    if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_INITIATE) && (ControlModule::get_stick_x(boma) < 0.15 && ControlModule::get_stick_x(boma) > -0.15) {
                        WorkModule::on_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_STICK_RELEASED);
                    }
                    //only turn in certain status
                    if [*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL, 
                      *FIGHTER_STATUS_JUMP_FLY_NEXT, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3, *FIGHTER_STATUS_KIND_DETACH_WALL_JUMP, 
                      *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_JUMP, *FIGHTER_EDGE_STATUS_KIND_ATTACK_AIR_F_WALL_JUMP].contains(&status_kind)
                      || CancelModule::is_enable_cancel(boma)
                      || [hash40("jump_aerial_f1"), hash40("jump_aerial_f2"), hash40("jump_aerial_f3"), hash40("jump_aerial_f4"), hash40("jump_aerial_f5"), ].contains(&curr_motion_kind) {
                        //turn with stick
                        if (ControlModule::get_stick_x(boma) * PostureModule::lr(boma)) < 0.0 {
                            WorkModule::on_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_INITIATE);
                            WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_INPUT_FRAME);
                        }
                        //turn! 
                        if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_ATTACK)
                          && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_INITIATE) 
                          && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_STICK_RELEASED) 
                          && WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_COUNT) < 2 
                          && (ControlModule::get_stick_x(boma) * PostureModule::lr(boma)) < 0.0  {
                            PostureModule::reverse_lr(boma);
                            PostureModule::update_rot_y_lr(boma);
                            WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_STICK_RELEASED);
                            WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_INITIATE);
                            WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_INPUT_FRAME);
                            WorkModule::set_int(boma, WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_COUNT) +1, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_COUNT);
                            if is_status_damage(boma) {
                                StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
                            }
                        }
                        //turn with taunt
                        if !WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_APPEAL_METHOD_INITIATE) {
                            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) && PostureModule::lr(boma) > 0.0 {
                                WorkModule::on_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_APPEAL_METHOD_INITIATE);
                            }
                            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) && PostureModule::lr(boma) < 0.0 {
                                WorkModule::on_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_APPEAL_METHOD_INITIATE);
                            }
                        }
                        else {
                            WorkModule::set_int(boma, WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_APPEAL_METHOD_DELAY_FRAME) +1, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_APPEAL_METHOD_DELAY_FRAME);
                        }
                        if WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_APPEAL_METHOD_DELAY_FRAME) > 3 
                           && WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_COUNT) < 2 
                           && !ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                            PostureModule::reverse_lr(boma);
                            PostureModule::update_rot_y_lr(boma);
                            WorkModule::set_int(boma, WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_COUNT) +1, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_COUNT);
                            WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_APPEAL_METHOD_DELAY_FRAME);
                            WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_APPEAL_METHOD_INITIATE);
                            if is_status_damage(boma) {
                                StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
                            }
                        }
                    }
                    if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_INITIATE) {
                        WorkModule::set_int(boma, WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_INPUT_FRAME) + 1, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_INPUT_FRAME);
                    }
                }
                //don't allow turn around if input is too slow
                if WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_INPUT_FRAME) > 7 || situation_kind == *SITUATION_KIND_GROUND || status_kind == *FIGHTER_STATUS_KIND_CLIFF_CATCH {
                    WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_INITIATE);
                    WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_STICK_RELEASED);
                    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_INPUT_FRAME);
                    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_COUNT);
                    WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_APPEAL_METHOD_INITIATE);
                    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_APPEAL_METHOD_DELAY_FRAME);
                }
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && !WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_APPEAL_METHOD_INITIATE) {
                    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_AIR_TURN_APPEAL_METHOD_DELAY_FRAME);
                    WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_TURN_APPEAL_METHOD_INITIATE);
                }
            }
            //shield dropping, use shield + b + down to drop
                //TODO move to lib.rs
            if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS) != 0 && GroundModule::is_passable_ground(boma) && 
            [hash40("guard"), hash40("guard_on"), hash40("shield_guard")].contains(&curr_motion_kind) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
            }
            //some buffer clears!
                //TODO move to lib.rs
            let status_next = StatusModule::status_kind_next(boma);
            //clear on damage
            if StopModule::is_damage(boma) && is_status_damage(boma) { //clear in hitlag, while actually being hit (not on shield)
                ControlModule::clear_command(boma, true);
            }

            if //clear when using dairdodge, to prevent buffering rolls when wavedashing
            ((curr_motion_kind == hash40("landing_heavy") 
            && FIGHTER_GLOBALS[entry_id as usize].wavedash) 
            || //clear when using roll / spotdodge and buffering another
            ([*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE_F].contains(&status_next) 
            && ![hash40("guard"), hash40("guard_on"), hash40("shield_guard")].contains(&curr_motion_kind)))

            && !((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0
              || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_SMASH) != 0
              || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0) {
                ControlModule::clear_command(boma, true);
            }

            //kirby hat in victory screen, cannot be in kirby_frame?
            if fighter_kind == *FIGHTER_KIND_KIRBY {
                if FighterManager::is_ready_go(fighter_manager) && stock_count > 0 {
                    FIGHTER_GLOBALS[entry_id as usize].kirby_last_hat = WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
                }
                if [hash40("lose"), hash40("win_1"), hash40("win_1_wait"), hash40("win_2"), hash40("win_2_wait"), hash40("win_3"), hash40("win_3_wait")].contains(&curr_motion_kind) {
                    WorkModule::set_int(boma, FIGHTER_GLOBALS[entry_id as usize].kirby_last_hat, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
                    //if FIGHTER_GLOBALS[entry_id as usize].onehat {
                    //    FIGHTER_GLOBALS[entry_id as usize].onehat = false;
                        ArticleModule::generate_article(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAT, false, 0);
                        //ArticleModule::set_visibility_whole(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAT, true, smash::app::ArticleOperationTarget{0: *ARTICLE_OPE_TARGET_ALL});
                        if [*FIGHTER_KIND_FALCO, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_WIIFIT, *FIGHTER_KIND_PICKEL, *FIGHTER_KIND_DEMON].contains(&FIGHTER_GLOBALS[entry_id as usize].kirby_last_hat) {
                            let kirby_pos = smash::phx::Vector3f { x:0., y:-3.5, z:0.};
                            VisibilityModule::set_model_visible(boma, false);
                            PostureModule::set_pos(boma, &kirby_pos);
                            ArticleModule::set_visibility_whole(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAT, false, smash::app::ArticleOperationTarget{0: *ARTICLE_OPE_TARGET_ALL});
                        }
                        if /*FIGHTER_GLOBALS[entry_id as usize].kirby_last_hat == -1 &&*/ FIGHTER_GLOBALS[entry_id as usize].ga_on {
                            ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAT, smash::app::ArticleOperationTarget{0: *ARTICLE_OPE_TARGET_ALL});
                            ArticleModule::generate_article(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_ULTRASWORDHAT, false, 0);
                        }
                    //}
                }
                // else {
                //     FIGHTER_GLOBALS[entry_id as usize].onehat = true; //makes sure we generate only one hat. might be useless, this is a test to see if it fixes things
                // }
            }

        }

        //Wavedash jump/shield cancel
        if curr_motion_kind == hash40("escape_air_slide") {
            FIGHTER_GLOBALS[entry_id as usize].wavedash = true;
        }
        if ![hash40("escape_air_slide"), hash40("landing_heavy")].contains(&curr_motion_kind) {
            FIGHTER_GLOBALS[entry_id as usize].wavedash = false;
        }
        //wavedash cancels, jump works even without change_status_request
        if curr_motion_kind == hash40("landing_heavy") && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD) {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, false);
                MotionModule::change_motion_kind(boma, Hash40::new("guard"));
            }
        }

        //running taunts, disabled for now. should probably be rewritten, with transition terms?
        // if [*FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE].contains(&status_kind) {
        //     //up taunt
        //     if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
        //         StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        //         if PostureModule::lr(boma) == -1.0 {
        //             ControlModule::reset_main_stick(boma);
        //             MotionModule::change_motion_kind(boma, smash::phx::Hash40{hash: hash40("appeal_hi_l")});
        //         }
        //         else {
        //             ControlModule::reset_main_stick(boma);
        //             MotionModule::change_motion_kind(boma, smash::phx::Hash40{hash: hash40("appeal_hi_r")});
        //         }
        //     }
        //     //down taunt
        //     if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
        //         StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        //         if PostureModule::lr(boma) == -1.0 {
        //             ControlModule::reset_main_stick(boma);
        //             MotionModule::change_motion_kind(boma, smash::phx::Hash40{hash: hash40("appeal_lw_l")});
        //         }
        //         else {
        //             ControlModule::reset_main_stick(boma);
        //             MotionModule::change_motion_kind(boma, smash::phx::Hash40{hash: hash40("appeal_lw_r")});
        //         }
        //     }
        //     //side taunt
        //     if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) ||
        //         ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
        //         StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        //         if PostureModule::lr(boma) == -1.0 {
        //             ControlModule::reset_main_stick(boma);
        //             MotionModule::change_motion_kind(boma, smash::phx::Hash40{hash: hash40("appeal_s_l")});
        //         }
        //         else {
        //             ControlModule::reset_main_stick(boma);
        //             MotionModule::change_motion_kind(boma, smash::phx::Hash40{hash: hash40("appeal_s_r")});
        //         }
        //     }
        // }
    }
}

//Use this for general per-frame weapon-level hooks
#[weapon_frame_callback]
fn global_weapon_frame(fighter_base : &mut L2CFighterBase) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
        let weapon_kind = get_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let curr_motion_kind = MotionModule::motion_kind(boma);

        let weapon =  mem::transmute::<&mut BattleObject, &mut smash::app::Weapon>(smash::app::sv_system::battle_object(fighter_base.lua_state_agent));
        let owner_id = smash::app::lua_bind::Weapon::get_founder_id(weapon) as u32;

        //weapons using owner_boma stuff
        //let owner_id = smash::app::lua_bind::WorkModule::get_int(module_accessor, WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
        if !smash::app::sv_battle_object::is_null(owner_id) && smash::app::sv_battle_object::is_active(owner_id) {
            let owner_boma = &mut *sv_battle_object::module_accessor(owner_id);
            let owner_status = StatusModule::status_kind(owner_boma);
            let owner_entry_id = WorkModule::get_int(owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
            //let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);

            //rosalina
            if weapon_kind == *WEAPON_KIND_ROSETTA_TICO {
                //return if rosa cancels special n charge
                if FIGHTER_GLOBALS[owner_entry_id as usize].rosa_tico_recall {
                    WorkModule::on_flag(boma, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_RETURN);
                    FIGHTER_GLOBALS[owner_entry_id as usize].rosa_tico_recall = false;
                }
                //recover from helplessness
                if status_kind == *WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FALL {
                    if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&owner_status) {
                        WorkModule::on_flag(boma, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_RETURN);
                    }
                }
                if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&owner_status) || WorkModule::is_flag(owner_boma, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
                    HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                }
            }

            //zelda
            if !is_operation_cpu(owner_boma) {
                if weapon_kind == *WEAPON_KIND_ZELDA_DEIN_S {
                    if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                        FIGHTER_GLOBALS[owner_entry_id as usize].ga_on = true; 
                    }
                }
                if weapon_kind == *WEAPON_KIND_ZELDA_PHANTOM {
                    ecb_shifts(boma, status_kind, situation_kind, weapon_kind);
                    if ![*WEAPON_ZELDA_PHANTOM_STATUS_KIND_ATTACK, *WEAPON_ZELDA_PHANTOM_STATUS_KIND_DISAPPEAR].contains(&status_kind)
                       && WorkModule::is_flag(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_FLAG_CANCEL) 
                       && !is_status_damage(owner_boma)
                       && WorkModule::get_float(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_FLOAT_HP) > 0.0 {
                        StatusModule::change_status_request_from_script(boma, *WEAPON_ZELDA_PHANTOM_STATUS_KIND_BUILD, false);
                    }
                    if is_status_damage(owner_boma) || KineticModule::get_kinetic_type(boma) == *WEAPON_KINETIC_TYPE_RESET {
                        WorkModule::set_float(boma, 0.0, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_FLOAT_HP);
                        WorkModule::on_flag(boma, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_FLAG_CANCEL);
                    }
                }
            }

            //inkling
        //     if weapon_kind == *WEAPON_KIND_INKLING_INKBULLET {
        //         if MotionModule::motion_kind(owner_boma) == hash40("attack_air_b") {
        //             println!("bullet in back air");
        //                 println!("bullet life: {}", WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE));
        //                 //println!("bullet life (init): {}", WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE));
        //             if WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE) == 11 { //do it once
        //                 println!("bullet revert");
        //                 let revert  = smash::phx::Vector3f { x: KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * -2., y: 0.0, z: 0.0 };
        //                 KineticModule::add_speed(boma, &revert);
        //                 PostureModule::set_lr(boma, -1.0 * PostureModule::lr(owner_boma));
        //                 PostureModule::update_rot_y_lr(boma);
        //             }
        //         }
        //         if MotionModule::motion_kind(owner_boma) == hash40("attack_air_lw") {
        //             println!("bullet in down air");
        //             if WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE) == 11 {
        //                 println!("bullet drop");
        //                 let drop  = smash::phx::Vector3f { x: KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * -1., y: KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN), z: 0.0 };
        //                 KineticModule::add_speed(boma, &drop);
        //             }
        //         }
        //     }
        // }

        // //simple weapon edits
        // //isabelle
        // if weapon_kind == *WEAPON_KIND_SHIZUE_CLAYROCKET {
        //     if status_kind == *WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_FLY {
        //         MotionModule::set_rate(boma, 1.2); 
        //     }
        // }
        }
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame, global_weapon_frame
    );
}