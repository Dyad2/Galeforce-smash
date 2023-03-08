//The wavedash code is modified from HDR and mostly not mine

#![allow(unused_must_use)]

use {
    super::*,
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    galeforce_utils::vars::*,
};

unsafe fn check_attach_wavedash_to_ground(fighter : &mut L2CFighterCommon, _test: bool) -> bool {

    //would check for FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE, but it's not turned on by the status yet if entering from pre?
    let stick = smash::app::sv_math::vec2_length(fighter.global_table[STICK_X].get_f32(), fighter.global_table[STICK_Y].get_f32());
    if stick <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_slide_stick")) {
        return false;
    }
    // if test {
    //     println!("check_attach from escape_air_pre");
    // }
    // else {
    //     println!("check_attach from sub_escape_air_common_strans_main");
    // }

    let shift = VarModule::get_float(fighter.battle_object, commons::instance::float::ECB_OFFSET_Y);

    let ecb_bottom = *GroundModule::get_rhombus(fighter.module_accessor, true).add(1); //add1 gives the ecb we want, here it's ecb.bottom    
    let pos = *PostureModule::pos(fighter.module_accessor);
    let snap_leniency = if WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y) <= 0.0 {
            // For a downwards/horizontal airdodge, waveland snap threshold = the distance from your ECB bottom to your Top bone
            shift
        } else {
            // For an upwards airdodge, waveland snap threshold = 5 units below ECB bottom, if the distance from your ECB bottom to your Top bone is < 5
            (shift).max(6.0)
        };

    let line_bottom = Vector2f{x: ecb_bottom.x, y: pos.y + shift - snap_leniency /*waveland_threshold*/};
    let mut ground_pos_any = Vector2f{ x: 0.0, y:0.0};
    let mut ground_pos_stage = Vector2f{ x: 0.0, y:0.0};
    GroundModule::line_segment_check(fighter.module_accessor, &Vector2f{x: ecb_bottom.x, y: ecb_bottom.y}, &line_bottom, &Vector2f{x: 0.0, y:0.0}, &mut ground_pos_any, true);
    GroundModule::line_segment_check(fighter.module_accessor, &Vector2f{x: ecb_bottom.x, y: ecb_bottom.y}, &line_bottom, &Vector2f{x: 0.0, y:0.0}, &mut ground_pos_stage, false);
    let can_snap = ground_pos_any != Vector2f{ x: 0.0, y:0.0} && (ground_pos_stage == Vector2f{x: 0.0, y:0.0}
        || WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y) <= 0.0);
    if can_snap {
        //println!("can snap");
        VarModule::on_flag(fighter.battle_object, commons::instance::flag::WAVEDASH);
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos.x, y: ground_pos_any.y + 0.01, z: pos.z});
        GroundModule::attach_ground(fighter.module_accessor, true);
        true
    }
    else {
        false
    }
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon20status_pre_EscapeAirEv")]
pub unsafe fn status_EscapeAir_pre(fighter: &mut L2CFighterCommon) -> L2CValue {

    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::WAVEDASH) || check_attach_wavedash_to_ground(fighter, true) {
        GroundModule::attach_ground(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }

    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, 0, 0, 0, 0);
    0.into()
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon16status_EscapeAirEv")]
unsafe fn status_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {

    //TODO: re-check vanilla script for identical behavior. this script has been partially rewritten by looking at hdr code. 
    ControlModule::reset_trigger(fighter.module_accessor); //what does this do?
    fighter.sub_escape_air_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_slide"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air"), 0.0, 1.0, false, 0.0, false, false);
    }
    let mut motion_rate = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_FLOAT_MOTION_RATE_PENALTY);
    let start_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
    if 0 < start_frame {
        let intan_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_XLU_FRAME);
        let add_xlu_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
        motion_rate = 1.0 / ((intan_frame as f32) / ((intan_frame - add_xlu_frame) as f32));
    }
    MotionModule::set_rate(fighter.module_accessor, motion_rate);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(status_EscapeAir_Main as *const () as _))
}

unsafe extern "C" fn status_EscapeAir_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::WAVEDASH) {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 1.into();
    }
    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
    0.into()
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon26sub_escape_air_common_mainEv")]
unsafe extern "C" fn sub_escape_air_common_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return true.into();
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
      || (!fighter.sub_wait_ground_check_common(false.into()).get_bool() 
      && !fighter.sub_air_check_fall_common().get_bool()) {
        if fighter.sub_escape_air_common_strans_main().get_bool() {
            return true.into();
        }
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return true.into();
        }
        if !MotionModule::is_end(fighter.module_accessor) {
            return false.into();
        } 
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    return true.into();
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon33sub_escape_air_common_strans_mainEv")]
unsafe extern "C" fn sub_escape_air_common_strans_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let escape_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
    let escape_throw_item_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("escape_throw_item_frame"));
    
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW)
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && ItemModule::is_have_item(fighter.module_accessor, 0)
    && {
      fighter.clear_lua_stack();
      smash_script::lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
      smash::app::sv_module_access::item(fighter.lua_state_agent);
      !fighter.pop_lua_stack(1).get_bool()
    }
    && escape_frame <= escape_throw_item_frame
    && !fighter.can_entry_cliff_air_lasso().get_bool() {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return 1.into();
    }

    let air_lasso_type = WorkModule::get_param_int(fighter.module_accessor, hash40("air_lasso_type"), 0);
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO)
    && air_lasso_type != *FIGHTER_AIR_LASSO_TYPE_NONE
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && !LinkModule::is_link(fighter.module_accessor, *FIGHTER_LINK_NO_CONSTRAINT) {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return 1.into();
    }

    let trigger_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("air_escape_passive_trigger_frame")) as f32;
    let passive_trigger_frame_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("passive_trigger_frame_mul"), 0);
    let passive_frame = trigger_frame as f32 * passive_trigger_frame_mul;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();

    let passive_fb_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value"));
    if situation_kind == *SITUATION_KIND_GROUND || check_attach_wavedash_to_ground(fighter, false) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB)
            && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
            && passive_fb_cont_value <= fighter.global_table[STICK_X].get_f32().abs()
            && (escape_frame as f32) < passive_frame {
                fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_FB.into(), true.into());
                return 1.into();
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE)
            && situation_kind == *SITUATION_KIND_GROUND
            && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
            && (escape_frame as f32) < passive_frame {
                fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE.into(), false.into());
                return 1.into();
            }
        }
        //println!("everyday im wavedashing (sub)");
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_AIR) {
        let jump_trigger_count = ControlModule::get_trigger_count(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP as u8);
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32)
        && ((jump_trigger_count & 0xff) as f32) < passive_frame
        && (escape_frame as f32) < passive_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP.into(), true.into());
            return 1.into();
        }

        let jump_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y"));
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32)
        && jump_stick_y <= fighter.global_table[STICK_Y].get_f32()
        && (escape_frame as f32) < passive_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP.into(), true.into());
            return 1.into();
        }

        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32)
        && (escape_frame as f32) < passive_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_WALL.into(), false.into());
            return 1.into();
        }

        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL)
        && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32)
        && (escape_frame as f32) < passive_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_CEIL.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon20status_end_EscapeAirEv")]
unsafe fn status_EscapeAirEnd(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_LANDING {
        //println!("delet wavedash");
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::WAVEDASH);
    }
    call_original!(fighter)
}

//   pLVar8 = &this->globalTable;
//   pLVar5 = (L2CValue *)lib::L2CValue::operator[](pLVar8,0xb);
//   lib::L2CValue::L2CValue(&LStack96,FIGHTER_STATUS_KIND_FALL);
//   uVar6 = lib::L2CValue::operator==(pLVar5,(L2CValue *)&LStack96);
//   lib::L2CValue::~L2CValue(&LStack96);
//   if ((uVar6 & 1) == 0) {
//     pLVar5 = (L2CValue *)lib::L2CValue::operator[](pLVar8,0xb);
//     lib::L2CValue::L2CValue(&LStack96,FIGHTER_STATUS_KIND_LANDING);
//     uVar6 = lib::L2CValue::operator==(pLVar5,(L2CValue *)&LStack96);
//     lib::L2CValue::~L2CValue(&LStack96);
//     if ((uVar6 & 1) == 0) goto LAB_710015bbf0;
//   }
//   lib::L2CValue::L2CValue(&LStack128,_FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE);
//   iVar3 = lib::L2CValue::as_integer(&LStack128);
//   bVar2 = app::lua_bind::WorkModule__is_flag_impl(this->moduleAccessor,iVar3);
//   lib::L2CValue::L2CValue(&LStack112,(bool)(bVar2 & 1));
//   lib::L2CValue::L2CValue(&LStack96,false);
//   uVar6 = lib::L2CValue::operator==(&LStack112,(L2CValue *)&LStack96);
//   lib::L2CValue::~L2CValue(&LStack96);
//   lib::L2CValue::~L2CValue(&LStack112);
//   lib::L2CValue::~L2CValue(&LStack128);
//   if ((uVar6 & 1) == 0) {
//     lib::L2CValue::L2CValue(&LStack96,0xcad2ee25e);
//     lib::L2CValue::L2CValue(&LStack128,0x1e1805189d);
//     uVar6 = lib::L2CValue::as_integer(&LStack96);
//     uVar7 = lib::L2CValue::as_integer(&LStack128);
//     fVar9 = (float)app::lua_bind::WorkModule__get_param_float_impl(this->moduleAccessor,uVar6,uVar7)
//     ;
//     lib::L2CValue::L2CValue(&LStack112,fVar9);
//     lib::L2CValue::~L2CValue(&LStack128);
//     lib::L2CValue::~L2CValue(&LStack96);
//     lib::L2CValue::L2CValue(&LStack96,0xcad2ee25e);
//     lib::L2CValue::L2CValue(&LStack144,0x22e3199cd1);
//     uVar6 = lib::L2CValue::as_integer(&LStack96);
//     uVar7 = lib::L2CValue::as_integer(&LStack144);
//     fVar9 = (float)app::lua_bind::WorkModule__get_param_float_impl(this->moduleAccessor,uVar6,uVar7)
//     ;
//     lib::L2CValue::L2CValue(&LStack128,fVar9);
//     lib::L2CValue::~L2CValue(&LStack144);
//     lib::L2CValue::~L2CValue(&LStack96);
//     lib::L2CValue::L2CValue(&LStack176,(L2CValue *)&LStack128);
//     lib::L2CValue::L2CValue(&LStack192,(L2CValue *)&LStack112);
//     fVar9 = (float)app::lua_bind::MotionModule__frame_impl(this->moduleAccessor);
//     lib::L2CValue::L2CValue(&LStack96,fVar9);
//     uVar4 = app::lua_bind::MotionModule__end_frame_impl(this->moduleAccessor);
//     lib::L2CValue::L2CValue(&LStack160,uVar4);
//     this_00 = (L2CFighterBase *)lib::L2CValue::operator/(&LStack96,(L2CValue *)&LStack160);
//     iVar3 = (int)register0x00000008;
//     L2CFighterBase::lerp(this_00,iVar3 - 0xb0,iVar3 - 0xc0,iVar3 - 0xd0);
//     lib::L2CValue::~L2CValue(&LStack208);
//     lib::L2CValue::~L2CValue(&LStack160);
//     lib::L2CValue::~L2CValue(&LStack96);
//     lib::L2CValue::~L2CValue(&LStack192);
//     lib::L2CValue::~L2CValue(&LStack176);
//     lib::L2CValue::L2CValue(&LStack96,0.0);
//     lib::L2CValue::operator+(&LStack144,(L2CValue *)&LStack96);
//     lib::L2CValue::~L2CValue(&LStack96);
//     lib::L2CValue::L2CValue(&LStack96,FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
//     fVar9 = (float)lib::L2CValue::as_number(&LStack160);
//     iVar3 = lib::L2CValue::as_integer(&LStack96);
//     app::lua_bind::WorkModule__set_float_impl(this->moduleAccessor,fVar9,iVar3);
//     lib::L2CValue::~L2CValue(&LStack96);
//     lib::L2CValue::~L2CValue(&LStack160);
//     lib::L2CValue::L2CValue(&LStack96,0x6e5ec7051);
//     lib::L2CValue::L2CValue(&LStack224,0x22106d5854);
//     uVar6 = lib::L2CValue::as_integer(&LStack96);
//     uVar7 = lib::L2CValue::as_integer(&LStack224);
//     fVar9 = (float)app::lua_bind::WorkModule__get_param_float_impl(this->moduleAccessor,uVar6,uVar7)
//     ;
//     lib::L2CValue::L2CValue(&LStack160,fVar9);
//     lib::L2CValue::~L2CValue(&LStack224);
//     lib::L2CValue::~L2CValue(&LStack96);
//     lib::L2CValue::L2CValue(&LStack240,FIGHTER_KINETIC_ENERGY_ID_STOP);
//     lib::L2CAgent::clear_lua_stack((L2CAgent *)this);
//     lib::L2CAgent::push_lua_stack((L2CAgent *)this,(L2CValue *)&LStack240);
//     fVar9 = (float)app::sv_kinetic_energy::get_speed_x(this->luaStateAgent);
//     lib::L2CValue::L2CValue(&LStack96,fVar9);
//     lib::L2CValue::L2CValue(&LStack272,0xcad2ee25e);
//     lib::L2CValue::L2CValue(&LStack288,0x225cd067b1);
//     uVar6 = lib::L2CValue::as_integer(&LStack272);
//     uVar7 = lib::L2CValue::as_integer(&LStack288);
//     fVar9 = (float)app::lua_bind::WorkModule__get_param_float_impl(this->moduleAccessor,uVar6,uVar7)
//     ;
//     lib::L2CValue::L2CValue(&LStack256,fVar9);
//     lib::L2CValue::operator*(&LStack96,(L2CValue *)&LStack256);
//     lib::L2CValue::~L2CValue(&LStack256);
//     lib::L2CValue::~L2CValue(&LStack288);
//     lib::L2CValue::~L2CValue(&LStack272);
//     lib::L2CValue::~L2CValue(&LStack96);
//     lib::L2CValue::~L2CValue(&LStack240);
//     lib::L2CValue::L2CValue(&LStack96,FIGHTER_KINETIC_ENERGY_ID_STOP);
//     lib::L2CAgent::clear_lua_stack((L2CAgent *)this);
//     lib::L2CAgent::push_lua_stack((L2CAgent *)this,(L2CValue *)&LStack96);
//     fVar9 = (float)app::sv_kinetic_energy::get_speed_y(this->luaStateAgent);
//     lib::L2CValue::L2CValue(&LStack240,fVar9);
//     lib::L2CValue::~L2CValue(&LStack96);
//     lib::L2CAgent::math_abs((L2CValue *)&LStack224,(L2CValue *)&LStack96);
//     uVar6 = lib::L2CValue::operator<(&LStack160,(L2CValue *)&LStack96);
//     lib::L2CValue::~L2CValue(&LStack96);
//     if ((uVar6 & 1) != 0) {
//       lib::L2CValue::L2CValue(&LStack96,0.0);
//       uVar6 = lib::L2CValue::operator<(&LStack96,(L2CValue *)&LStack224);
//       lib::L2CValue::~L2CValue(&LStack96);
//       if ((uVar6 & 1) == 0) {
//         lib::L2CValue::L2CValue(&LStack96,0.0);
//         uVar6 = lib::L2CValue::operator<(&LStack224,(L2CValue *)&LStack96);
//         lib::L2CValue::~L2CValue(&LStack96);
//         if ((uVar6 & 1) != 0) {
//           lib::L2CValue::operator-(&LStack160);
//           lib::L2CValue::operator=(&LStack224,(L2CValue *)&LStack96);
//           lib::L2CValue::~L2CValue(&LStack96);
//         }
//       }
//       else {
//         lib::L2CValue::operator=(&LStack224,(L2CValue *)&LStack160);
//       }
//     }
//     lib::L2CValue::L2CValue(&LStack96,FIGHTER_KINETIC_ENERGY_ID_STOP);
//     lib::L2CAgent::clear_lua_stack((L2CAgent *)this);
//     lib::L2CAgent::push_lua_stack((L2CAgent *)this,(L2CValue *)&LStack96);
//     lib::L2CAgent::push_lua_stack((L2CAgent *)this,(L2CValue *)&LStack224);
//     lib::L2CAgent::push_lua_stack((L2CAgent *)this,(L2CValue *)&LStack240);
//     app::sv_kinetic_energy::set_speed(this->luaStateAgent);
//     lib::L2CValue::~L2CValue(&LStack96);
//     lib::L2CValue::~L2CValue(&LStack240);
//     lib::L2CValue::~L2CValue(&LStack224);
//     lib::L2CValue::~L2CValue(&LStack160);
//     lib::L2CValue::~L2CValue(&LStack144);
//     lib::L2CValue::~L2CValue(&LStack128);
//     lVar1 = -0x60;
//   }
//   else {
//     lib::L2CValue::L2CValue(&LStack144,0x6e5ec7051);
//     lib::L2CValue::L2CValue(&LStack160,0x18c65c9c58);
//     uVar6 = lib::L2CValue::as_integer(&LStack144);
//     uVar7 = lib::L2CValue::as_integer(&LStack160);
//     iVar3 = app::lua_bind::WorkModule__get_param_int_impl(this->moduleAccessor,uVar6,uVar7);
//     lib::L2CValue::L2CValue(&LStack128,iVar3);
//     lib::L2CValue::L2CValue(&LStack96,0.0);
//     lib::L2CValue::operator+(&LStack128,(L2CValue *)&LStack96);
//     lib::L2CValue::~L2CValue(&LStack96);
//     lib::L2CValue::L2CValue(&LStack96,FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
//     fVar9 = (float)lib::L2CValue::as_number(&LStack112);
//     iVar3 = lib::L2CValue::as_integer(&LStack96);
//     app::lua_bind::WorkModule__set_float_impl(this->moduleAccessor,fVar9,iVar3);
//     lib::L2CValue::~L2CValue(&LStack96);
//     lib::L2CValue::~L2CValue(&LStack112);
//     lib::L2CValue::~L2CValue(&LStack128);
//     lib::L2CValue::~L2CValue(&LStack160);
//     lVar1 = -0x80;
//   }
//   lib::L2CValue::~L2CValue((L2CValue *)(&stack0xfffffffffffffff0 + lVar1));
//   pLVar8 = (L2CValue *)lib::L2CValue::operator[](pLVar8,0xb);
//   lib::L2CValue::L2CValue(&LStack96,FIGHTER_STATUS_KIND_LANDING);
//   uVar6 = lib::L2CValue::operator==(pLVar8,(L2CValue *)&LStack96);
//   lib::L2CValue::~L2CValue(&LStack96);
//   if ((uVar6 & 1) != 0) {
//     bVar2 = app::lua_bind::MotionModule__is_end_impl(this->moduleAccessor);
//     lib::L2CValue::L2CValue(&LStack112,(bool)(bVar2 & 1));
//     lib::L2CValue::L2CValue(&LStack96,false);
//     uVar6 = lib::L2CValue::operator==(&LStack112,(L2CValue *)&LStack96);
//     lib::L2CValue::~L2CValue(&LStack96);
//     lib::L2CValue::~L2CValue(&LStack112);
//     if ((uVar6 & 1) != 0) {
//       lib::L2CValue::L2CValue(&LStack96,_FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN);
//       iVar3 = lib::L2CValue::as_integer(&LStack96);
//       app::lua_bind::WorkModule__on_flag_impl(this->moduleAccessor,iVar3);
//       lib::L2CValue::~L2CValue(&LStack96);
//     }
//     lib::L2CValue::L2CValue(&LStack96,FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
//     iVar3 = lib::L2CValue::as_integer(&LStack96);
//     app::lua_bind::WorkModule__on_flag_impl(this->moduleAccessor,iVar3);
//     lib::L2CValue::~L2CValue(&LStack96);
//   }
// LAB_710015bbf0:
//   lib::L2CValue::L2CValue(in_x8,0);
//   return;
// }

pub fn install() {
    install_status_scripts!(
        status_EscapeAir_pre,
        status_EscapeAir,
        status_EscapeAirEnd
    );
    install_hooks!(
        sub_escape_air_common_strans_main,
        sub_escape_air_common_main
    );
}