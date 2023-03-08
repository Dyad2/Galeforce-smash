//The wavedash code is modified from HDR and mostly not mine

//FIXME dark samus is bugged currently, might be ecb related

use super::*;
use galeforce_utils::{vars::*, table_const::*};
use custom_var::*;

#[common_status_script(status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon16status_JumpSquatEv")]
unsafe fn status_JumpSquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr_update = fighter.sub_status_JumpSquat_check_stick_lr_update();
    fighter.status_JumpSquat_common(lr_update);
    if (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
      || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && ControlModule::get_stick_y(fighter.module_accessor) >= 0.7)) && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        VarModule::on_flag(fighter.battle_object, commons::status::flag::JUMP_SQUAT_TO_ESCAPE_AIR);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(status_JumpSquat_Main as *const () as _))
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon21status_JumpSquat_MainEv")]
unsafe fn status_JumpSquat_Main(fighter: &mut L2CFighterCommon) -> L2CValue {

    let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[JUMP_SQUAT_MAIN_PRE].get_ptr());
    if fighter.global_table[JUMP_SQUAT_MAIN_PRE].get_bool() && callable(fighter).get_bool() {
        return 1.into();
    }
    
    //escape_air is new
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) {
        fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START) {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL)
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if !fighter.sub_transition_group_check_ground_item().get_bool() {
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI)
          && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0
          && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
        } 
        else if !fighter.sub_transition_specialflag_hoist().get_bool() {
            let cat2 = fighter.global_table[CMD_CAT2].get_i32();
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[ATTACK_HI4_PRE].get_ptr());
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START)
            && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                if fighter.global_table[ATTACK_HI4_PRE].get_bool() != false && callable(fighter).get_bool() {
                    return 0.into();
                }
                if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_ATTACK_DASH_ATTACK_HI4 != 0
                  && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_START.into(), true.into());
                }
            }
        }
    }
    0.into()
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon40uniq_process_JumpSquat_exec_status_paramEN3lib8L2CValueE")]
unsafe fn uniq_process_JumpSquat_exec_status_param(fighter: &mut L2CFighterCommon, _arg: L2CValue) {
    
    //had to read hdr code to understand what happens here. still not sure i got it all lol
    let should_check = if fighter.global_table[JUMP_SQUAT_MAIN_PRE].get_bool() {
        let custom_routine: *const extern "C" fn(&mut L2CFighterCommon) -> L2CValue = fighter.global_table[JUMP_SQUAT_MAIN_PRE].get_ptr() as _;
        if !custom_routine.is_null() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(custom_routine);
            callable(fighter);
            true
        }
        else {
            true
        }
    }
    else {
        true
    };
    if should_check {
        fighter.sub_jump_squat_uniq_check_sub(FIGHTER_STATUS_JUMP_FLAG_BUTTON.into());
        fighter.sub_jump_squat_uniq_check_sub_mini_attack();
    }

    let update_rate = MotionModule::update_rate(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor))) as u32 as f32;
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE != 0 || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD_HOLD))
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N == 0 {
        if !(fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_PICKEL 
        && [*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_SQUAT].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0))) {
            VarModule::on_flag(fighter.battle_object, commons::status::flag::JUMP_SQUAT_TO_ESCAPE_AIR);
        }
    }
    if end_frame <= (frame * update_rate) || VarModule::is_flag(fighter.battle_object, commons::status::flag::JUMP_SQUAT_TO_ESCAPE_AIR) /*to interrupt jumpsquat with airdodge*/ {
        StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), false);
        let situation_kind = fighter.global_table[SITUATION_KIND].clone();
        fighter.global_table[PREV_SITUATION_KIND].assign(&situation_kind);
        if VarModule::is_flag(fighter.battle_object, commons::status::flag::JUMP_SQUAT_TO_ESCAPE_AIR) {
            let stick = smash::app::sv_math::vec2_length(fighter.global_table[STICK_X].get_f32(), fighter.global_table[STICK_Y].get_f32());
            if stick >= 0.66 && fighter.global_table[STICK_Y].get_f32() <= 0.2
            {
                VarModule::on_flag(fighter.battle_object, commons::instance::flag::WAVEDASH);
                //GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_ESCAPE);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
            } 
            else {
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::WAVEDASH);
                GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
            }
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
        } 
        else {
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::WAVEDASH);
            GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FROM_SQUAT, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_JUMP_FROM);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
        }
    }
}

#[common_status_script(status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn status_exec_JumpSquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    uniq_process_JumpSquat_exec_status_param(fighter, L2CValue::Ptr(0 as _));
    0.into()
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon34uniq_process_JumpSquat_exec_statusEv")]
unsafe fn uniq_process_JumpSquat_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    uniq_process_JumpSquat_exec_status_param(fighter, L2CValue::Ptr(0 as _));
    0.into()
}

pub fn install() {
    install_status_scripts!(
        status_JumpSquat,
        status_exec_JumpSquat
    );

    install_hooks!(
        status_JumpSquat_Main,
        uniq_process_JumpSquat_exec_status_param,
        uniq_process_JumpSquat_exec_status
        
    );
}

//this is where FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI is set for short hop macro?
// void __thiscall
// lua2cpp::L2CFighterCommon::sub_jump_squat_uniq_check_sub(L2CFighterCommon *this,L2CValue param_1)

// {
//   long lVar1;
//   byte bVar2;
//   bool bVar3;
//   int iVar4;
//   ulong uVar5;
//   L2CValue *this_00;
//   ulong uVar6;
//   float fVar7;
//   L2CValue LStack160;
//   L2CValue LStack144;
//   L2CValue LStack128;
//   L2CValue LStack112;
//   L2CValue LStack96;
//   L2CValue LStack80;
//   L2CValue LStack64;

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon29sub_jump_squat_uniq_check_subEN3lib8L2CValueE")]
unsafe fn sub_jump_squat_uniq_check_sub(fighter: &mut L2CFighterCommon, param1: i32) {

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP) {
        if !WorkModule::is_flag(fighter.module_accessor, param1) {
            let stick_y = fighter.global_table[STICK_Y].get_f32();
            let param = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_y"));
            if stick_y < param {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
                            return;
                        }
                    }
                }
            }
        }
    }
}

//   lib::L2CValue::L2CValue(&LStack96,FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP);
//   iVar4 = lib::L2CValue::as_integer(&LStack96);
//   bVar2 = app::lua_bind::WorkModule__is_flag_impl(this->moduleAccessor,iVar4);
//   lib::L2CValue::L2CValue(&LStack80,(bool)(bVar2 & 1));
//   lib::L2CValue::L2CValue(&LStack64,false);
//   uVar5 = lib::L2CValue::operator==(&LStack80,(L2CValue *)&LStack64);
//   lib::L2CValue::~L2CValue(&LStack64);
//   lib::L2CValue::~L2CValue(&LStack80);
//   lib::L2CValue::~L2CValue(&LStack96);
//   if ((uVar5 & 1) == 0) {
//     return;
//   }
//   iVar4 = lib::L2CValue::as_integer((L2CValue *)(ulong)param_1);
//   bVar2 = app::lua_bind::WorkModule__is_flag_impl(this->moduleAccessor,iVar4);
//   lib::L2CValue::L2CValue(&LStack64,(bool)(bVar2 & 1));
//   bVar3 = lib::L2CValue::operator.cast.to.bool(&LStack64);
//   lib::L2CValue::~L2CValue(&LStack64);
//   if ((bVar3 & 1U) == 0) {
//     this_00 = (L2CValue *)lib::L2CValue::operator[](&this->globalTable,0x1b);
//     lib::L2CValue::L2CValue(&LStack80,0x6e5ec7051);
//     lib::L2CValue::L2CValue(&LStack96,0xe5fbcdc0e);
//     uVar5 = lib::L2CValue::as_integer(&LStack80);
//     uVar6 = lib::L2CValue::as_integer(&LStack96);
//     fVar7 = (float)app::lua_bind::WorkModule__get_param_float_impl(this->moduleAccessor,uVar5,uVar6)
//     ;
//     lib::L2CValue::L2CValue(&LStack64,fVar7);
//     uVar5 = lib::L2CValue::operator<(this_00,(L2CValue *)&LStack64);
//     if ((uVar5 & 1) == 0) {
//       lib::L2CValue::~L2CValue(&LStack64);
//       lib::L2CValue::~L2CValue(&LStack96);
//       lib::L2CValue::~L2CValue(&LStack80);
//     }
//     else {
//       lib::L2CValue::L2CValue(&LStack128,CONTROL_PAD_BUTTON_CSTICK_ON);
//       iVar4 = lib::L2CValue::as_integer(&LStack128);
//       bVar2 = app::lua_bind::ControlModule__check_button_on_impl(this->moduleAccessor,iVar4);
//       lib::L2CValue::L2CValue(&LStack112,(bool)(bVar2 & 1));
//       bVar3 = lib::L2CValue::operator.cast.to.bool(&LStack112);
//       if ((bVar3 & 1U) == 0) {
//         bVar2 = 1;
//       }
//       else {
//         lib::L2CValue::L2CValue(&LStack160,CONTROL_PAD_BUTTON_SPECIAL);
//         iVar4 = lib::L2CValue::as_integer(&LStack160);
//         bVar2 = app::lua_bind::ControlModule__check_button_on_impl(this->moduleAccessor,iVar4);
//         lib::L2CValue::L2CValue(&LStack144,(bool)(bVar2 & 1));
//         bVar3 = lib::L2CValue::operator.cast.to.bool(&LStack144);
//         bVar2 = bVar3 ^ 1;
//         lib::L2CValue::~L2CValue(&LStack144);
//         lib::L2CValue::~L2CValue(&LStack160);
//       }
//       lib::L2CValue::~L2CValue(&LStack112);
//       lib::L2CValue::~L2CValue(&LStack128);
//       lib::L2CValue::~L2CValue(&LStack64);
//       lib::L2CValue::~L2CValue(&LStack96);
//       lib::L2CValue::~L2CValue(&LStack80);
//       if ((bVar2 & 1) != 0) {
//         lib::L2CValue::L2CValue(&LStack64,_FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
//         iVar4 = lib::L2CValue::as_integer(&LStack64);
//         app::lua_bind::WorkModule__on_flag_impl(this->moduleAccessor,iVar4);
//         goto LAB_71001e1290;
//       }
//     }
//     lib::L2CValue::L2CValue(&LStack80,CONTROL_PAD_BUTTON_CSTICK_ON);
//     iVar4 = lib::L2CValue::as_integer(&LStack80);
//     bVar2 = app::lua_bind::ControlModule__check_button_trigger_impl(this->moduleAccessor,iVar4);
//     lib::L2CValue::L2CValue(&LStack64,(bool)(bVar2 & 1));
//     bVar3 = lib::L2CValue::operator.cast.to.bool(&LStack64);
//     if ((bVar3 & 1U) != 0) {
//       lib::L2CValue::L2CValue(&LStack112,CONTROL_PAD_BUTTON_ATTACK);
//       iVar4 = lib::L2CValue::as_integer(&LStack112);
//       bVar2 = app::lua_bind::ControlModule__check_button_trigger_impl(this->moduleAccessor,iVar4);
//       lib::L2CValue::L2CValue(&LStack96,(bool)(bVar2 & 1));
//       bVar3 = lib::L2CValue::operator.cast.to.bool(&LStack96);
//       if ((bVar3 & 1U) != 0) {
//         lib::L2CValue::L2CValue(&LStack144,CONTROL_PAD_BUTTON_SPECIAL);
//         iVar4 = lib::L2CValue::as_integer(&LStack144);
//         bVar2 = app::lua_bind::ControlModule__check_button_off_impl(this->moduleAccessor,iVar4);
//         lib::L2CValue::L2CValue(&LStack128,(bool)(bVar2 & 1));
//         bVar3 = lib::L2CValue::operator.cast.to.bool(&LStack128);
//         lib::L2CValue::~L2CValue(&LStack128);
//         lib::L2CValue::~L2CValue(&LStack144);
//         lib::L2CValue::~L2CValue(&LStack96);
//         lib::L2CValue::~L2CValue(&LStack112);
//         lib::L2CValue::~L2CValue(&LStack64);
//         lib::L2CValue::~L2CValue(&LStack80);
//         if ((bVar3 & 1U) == 0) {
//           return;
//         }
//         lib::L2CValue::L2CValue(&LStack64,_FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
//         iVar4 = lib::L2CValue::as_integer(&LStack64);
//         app::lua_bind::WorkModule__on_flag_impl(this->moduleAccessor,iVar4);
//         goto LAB_71001e1290;
//       }
//       lib::L2CValue::~L2CValue(&LStack96);
//       lib::L2CValue::~L2CValue(&LStack112);
//     }
//     lib::L2CValue::~L2CValue(&LStack64);
//     lVar1 = -0x40;
//   }
//   else {
//     lib::L2CValue::L2CValue(&LStack80,CONTROL_PAD_BUTTON_JUMP);
//     iVar4 = lib::L2CValue::as_integer(&LStack80);
//     bVar2 = app::lua_bind::ControlModule__check_button_off_impl(this->moduleAccessor,iVar4);
//     lib::L2CValue::L2CValue(&LStack64,(bool)(bVar2 & 1));
//     bVar3 = lib::L2CValue::operator.cast.to.bool(&LStack64);
//     if ((bVar3 & 1U) == 0) {
//       bVar2 = app::lua_bind::ControlModule__is_jump_mini_button_impl(this->moduleAccessor);
//       lib::L2CValue::L2CValue(&LStack96,(bool)(bVar2 & 1));
//       bVar3 = lib::L2CValue::operator.cast.to.bool(&LStack96);
//       lib::L2CValue::~L2CValue(&LStack96);
//       lib::L2CValue::~L2CValue(&LStack64);
//       lib::L2CValue::~L2CValue(&LStack80);
//       if ((bVar3 & 1U) == 0) {
//         return;
//       }
//     }
//     else {
//       lib::L2CValue::~L2CValue(&LStack64);
//       lib::L2CValue::~L2CValue(&LStack80);
//     }
//     lib::L2CValue::L2CValue(&LStack64,_FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
//     iVar4 = lib::L2CValue::as_integer(&LStack64);
//     app::lua_bind::WorkModule__on_flag_impl(this->moduleAccessor,iVar4);
// LAB_71001e1290:
//     lVar1 = -0x30;
//   }
//   lib::L2CValue::~L2CValue((L2CValue *)(&stack0xfffffffffffffff0 + lVar1));
//   return;
// }