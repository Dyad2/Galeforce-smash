use super::*;
use crate::fighter::common::opff::common_fighter_frame;

unsafe extern "C" fn rosetta_stuff(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

    //todo: allow rosa to use this also in the air
    if curr_motion_kind == hash40("special_n_charge") || curr_motion_kind == hash40("special_air_n_charge") {
        if (ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
            VarModule::on_flag(fighter.module_accessor, rosetta::instance::flag::TICO_RECALL);
            if situation_kind == SITUATION_KIND_GROUND {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
                fighter.sub_transition_group_check_ground_jump_mini_attack();
                fighter.sub_transition_group_check_ground_jump();
            }
            else if situation_kind == SITUATION_KIND_AIR {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
                fighter.sub_transition_group_check_air_jump_aerial();
            }
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            VarModule::on_flag(fighter.module_accessor, rosetta::instance::flag::TICO_RECALL);
            if situation_kind == SITUATION_KIND_GROUND {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, false);
            }
            else if situation_kind == SITUATION_KIND_AIR {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
            }
        }
    }
    //recall out of shield?
}

unsafe extern "C" fn rosetta_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    rosetta_stuff(fighter);
}

unsafe extern "C" fn tico_callback(fighter_base : &mut L2CFighterBase) {
    if get_kind(&mut *fighter_base.module_accessor) != WEAPON_KIND_ROSETTA_TICO {
        return
    }
    let status_kind = StatusModule::status_kind(fighter_base.module_accessor);
    
    let weapon =  mem::transmute::<&mut BattleObject, &mut smash::app::Weapon>(smash::app::sv_system::battle_object(fighter_base.lua_state_agent));
    let owner_id = smash::app::lua_bind::Weapon::get_founder_id(weapon) as u32;
    
    //weapons using owner_fighter.module_accessor stuff
    if !smash::app::sv_battle_object::is_null(owner_id) && smash::app::sv_battle_object::is_active(owner_id) {
        let owner_boma = &mut *sv_battle_object::module_accessor(owner_id);
        let owner_status = StatusModule::status_kind(owner_boma);            
    
        //return if rosa cancels special n charge
        if VarModule::is_flag(owner_boma, rosetta::instance::flag::TICO_RECALL) {
            WorkModule::on_flag(owner_boma, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_RETURN);
            VarModule::off_flag(owner_boma, rosetta::instance::flag::TICO_RECALL);
        }
        //recover from helplessness
        if status_kind == *WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FALL {
            if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&owner_status) {
                WorkModule::on_flag(fighter_base.module_accessor, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_RETURN);
            }
        }
        if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&owner_status) || WorkModule::is_flag(owner_boma, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
            HitModule::set_whole(fighter_base.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, rosetta_frame);

    smashline::Agent::new("rosetta_tico")
        .on_line(smashline::Main, tico_callback)
        .install();
}