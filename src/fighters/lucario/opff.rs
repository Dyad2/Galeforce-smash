use super::*; 

static mut STICK_DIR : [i32; 9] = [0; 9];
static mut COMMAND_FRAME : [i32; 9] = [0; 9];

unsafe extern "C" fn lucario_tail3_intangibility(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    
    if [hash40("attackhi3"), hash40("attacklw3"), hash40("attacks4"), 
      hash40("attackairhi"), hash40("attackairb"), hash40("attackairn"), hash40("attackairf"), hash40("attackairlw"),
      hash40("specialnstart"), hash40("specialnhold"), hash40("specialnmax"), hash40("specialnshoot"),
      hash40("specialairnstart"), hash40("specialairnhold"), hash40("specialairnmax"), hash40("specialairnshoot")].contains(&curr_motion_kind) 
      || curr_motion_kind == hash40("speciallw") && MotionModule::frame(fighter.module_accessor) < 4. && MotionModule::frame(fighter.module_accessor) > 10. {
        macros::HIT_NODE(fighter, Hash40::new("s_tail3"), *HIT_STATUS_NORMAL);
    }
}

//aura charge taunt
unsafe extern "C" fn lucario_aura_charge(fighter: &mut L2CFighterCommon) {
    let curr_motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if curr_motion_kind == hash40("appeal_hi_r") || curr_motion_kind == hash40("appeal_hi_l") {
        if fighter.global_table[MOTION_FRAME].get_i32() == 36 {
            if PostureModule::lr(fighter.module_accessor) == -1.0 { //posture check to get the correct motion below
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    DamageModule::add_damage(fighter.module_accessor, 5.0, 0);
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40{hash: hash40("appeal_hi_r")}, 17.0, 1.1, 0.0, true, false);
                }
                else {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40{hash: hash40("appeal_hi_r")}, 81.0, 1.0, 0.0, true, false);
                }
            }
            else {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    DamageModule::add_damage(fighter.module_accessor, 5.0, 0);
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40{hash: hash40("appeal_hi_l")}, 17.0, 1.1, 0.0, true, false);
                }
                else {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40{hash: hash40("appeal_hi_l")}, 81.0, 1.0, 0.0, true, false);
                }
            }
        }
        if fighter.global_table[MOTION_FRAME].get_i32() == 75 {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
        }
    }
}

//charge input dair, hold down for 10 frames and it becomes a spike
unsafe extern "C" fn lucario_dair_charge(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR 
    && (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) || [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind)) {
        STICK_DIR[entry_id as usize] = get_stick_dir(&mut *fighter.module_accessor);
        if [1,2,3].contains(&STICK_DIR[entry_id as usize]) {
            COMMAND_FRAME[entry_id as usize] = COMMAND_FRAME[entry_id as usize] +1;
        }
        else {
            COMMAND_FRAME[entry_id as usize] = 0;
            VarModule::off_flag(fighter.battle_object,  lucario::instance::flag::ATTACK_AIR_LW_CHARGED);
        }
        if COMMAND_FRAME[entry_id as usize] > 10 {
            VarModule::on_flag(fighter.battle_object, lucario::instance::flag::ATTACK_AIR_LW_CHARGED);
        }
    }
    else {
        COMMAND_FRAME[entry_id as usize] = 0;
        VarModule::off_flag(fighter.battle_object, lucario::instance::flag::ATTACK_AIR_LW_CHARGED);
    }
}

//GA - Aura Burst
// type : buff
//  hit someone with charged up taunt hitbox to gain maximum aura for a short time
unsafe extern "C" fn lucario_galeforce_attack(fighter: &mut L2CFighterCommon) {
    if !is_operation_cpu(fighter.module_accessor) {
        if VarModule::get_int(fighter.battle_object, lucario::instance::int::MAX_AURA_TIMER) > 0 {
            //WorkModule::set_float(fighter.module_accessor, 1.33, FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_AURA_SCALE);
            if DamageModule::damage(fighter.module_accessor, 0) < 150.0 {
                AttackModule::set_power_mul(fighter.module_accessor, (1.33 - (100.0 + 0.22 * DamageModule::damage(fighter.module_accessor, 0)) / 100.0) + 1.0);
            }
            else {
                AttackModule::set_power_mul(fighter.module_accessor, 1.0);
            }
            VarModule::sub_int(fighter.battle_object, lucario::instance::int::MAX_AURA_TIMER, 1);
    
            if VarModule::get_int(fighter.battle_object, lucario::instance::int::MAX_AURA_TIMER) % 20 == 1 {
                let pos = Vector3f  {x : 0., y : 3.5, z : 0.};
                let rot = Vector3f  {x : 0., y : 0., z : 0.};
    
                let handle = EffectModule::req_follow(fighter.module_accessor,
                    smash::phx::Hash40{hash: hash40("lucario_aura")},
                    smash::phx::Hash40{hash: hash40("top")}, 
                    &pos, &rot, 0.4, false, 0, 
                    0, 0, 0, 0, false, false) as u32;
    
                EffectModule::set_rgb(fighter.module_accessor, handle, 5., 0., 2.);
            }
        }
        
        //visual effect
        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
            galeforce_apply_effect(&mut *fighter.module_accessor, 0.5);
            VarModule::set_int(fighter.battle_object, lucario::instance::int::MAX_AURA_TIMER, 480);
        }
    }
}

unsafe extern "C" fn lucario_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    lucario_tail3_intangibility(fighter);
    lucario_dair_charge(fighter);
    lucario_aura_charge(fighter);
    lucario_galeforce_attack(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, lucario_frame);
}