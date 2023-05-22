use super::*;

// #[status_script(agent="samusd", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn samusd_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
//         GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
//         KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GLIDE_LANDING_STOP); //don't stop :)
//         if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART) {
//             MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(0xb16d2b473),-1.0,1.0,0.0,false,false);
//         }
//         else {
//             MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0xb16d2b473), 0.0, 1.0, false, 0.0, false, false);
//             WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART);
//         }
//     }
//     else {
//         GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
//         KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL); //don't stop :)
//         if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART) {
//             MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(0xfe140b144), -1.0, 1.0, 0.0, false, false);
//         }
//         else {
//             MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0xfe140b144), 0.0, 1.0, false, 0.0, false, false);
//             WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART);
//         }
//     }
//     ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
//     fighter.sub_shift_status_main(L2CValue::Ptr(specialn_loop as *const () as _));
//     return 0.into();
// }

// unsafe fn specialn_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
//         if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
//             GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
//             KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL); //don't stop :)
//             MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(0xfe140b144),-1.0,1.0,0.0,false,false);
//         }
//     }
//     else {
//         if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
//             GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
//             KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GLIDE_LANDING_STOP); //don't stop :)
//             MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(0xb16d2b473),-1.0,1.0,0.0,false,false);
//         }
//     }
//     if MotionModule::is_end(fighter.module_accessor) {
//         fighter.fastshift(L2CValue::Ptr(special_n_undefined_fn as *const () as _))
//     }
//     else {
//         return 0.into();
//     }
// }

//unsafe fn special_n_undefined_fn(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
//         if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
//             GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
//             KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL); //don't stop :)
//             MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(0xfe140b144),-1.0,1.0,0.0,false,false);
//         }
//     }
//     else {
//         if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
//             GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
//             KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GLIDE_LANDING_STOP); //don't stop :)
//             MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(0xb16d2b473),-1.0,1.0,0.0,false,false);
//         }
//     }
//     the actual fn
//     let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT) as f32;
//     let charge_param = WorkModule::get_param_float(fighter.module_accessor, 0xf899192aa, hash40("cshot_charge_frame"));
//     if count <= charge_param {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART);
//         fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F.into(), false.into());
//     }
//     else {
//         fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H.into(), false.into());
//     }
//     return 1.into();
// }

//manages speed, and the shoot flag for some reason
#[status_script(agent="samusd", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn samusd_specialnf_exec(fighter: &mut L2CFighterCommon) -> L2CValue {

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT) {
        //let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f32;
        //let lr = PostureModule::lr(fighter.module_accessor);
        //let min_spd = WorkModule::get_param_float(fighter.module_accessor, 0xf899192aa, hash40("cshot_shot_spd_min"));
        //let max_spd = WorkModule::get_param_float(fighter.module_accessor, 0xf899192aa, hash40("cshot_shot_spd_max"));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT);
    }
    return 0.into();
}

//   lib::L2CValue::L2CValue(&LStack_60,0);
//   lib::L2CValue::L2CValue(&LStack_70,0);
//   pLVar4 = &this->globalTable;
//   this_00 = (L2CValue *)lib::L2CValue::operator[](pLVar4,0x17);
//   lib::L2CValue::L2CValue(&LStack_50,_SITUATION_KIND_GROUND);
//   bVar1 = lib::L2CValue::operator==(this_00,(L2CValue *)&LStack_50);
//   lib::L2CValue::~L2CValue(&LStack_50);
//   if (bVar1) {
//     pLVar4 = (L2CValue *)lib::L2CValue::operator[](pLVar4,0x16);
//     lib::L2CValue::L2CValue(&LStack_50,SITUATION_KIND_AIR);
//     bVar1 = lib::L2CValue::operator==(pLVar4,(L2CValue *)&LStack_50);
//     lib::L2CValue::~L2CValue(&LStack_50);
//     if (!bVar1) goto LAB_7100024620;
//     lib::L2CValue::L2CValue(&LStack_50,GROUND_CORRECT_KIND_AIR);
//     GVar2 = lib::L2CValue::as_integer(&LStack_50);
//     app::lua_bind::GroundModule__correct_impl(this->moduleAccessor,GVar2);
//     lib::L2CValue::~L2CValue(&LStack_50);
//     lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_KINETIC_TYPE_AIR_STOP);
//     iVar3 = lib::L2CValue::as_integer(&LStack_50);
//     app::lua_bind::KineticModule__change_kinetic_impl(this->moduleAccessor,iVar3);
//     lib::L2CValue::~L2CValue(&LStack_50);
//     lib::L2CValue::L2CValue(&LStack_50,0xfe140b144);
//     HVar5 = lib::L2CValue::as_hash(&LStack_50);
//     app::lua_bind::MotionModule__change_motion_inherit_frame_impl
//               (this->moduleAccessor,HVar5,-1.0,1.0,0.0,false,false);
//   }
//   else {
//     pLVar4 = (L2CValue *)lib::L2CValue::operator[](pLVar4,0x16);
//     lib::L2CValue::L2CValue(&LStack_50,_SITUATION_KIND_GROUND);
//     bVar1 = lib::L2CValue::operator==(pLVar4,(L2CValue *)&LStack_50);
//     lib::L2CValue::~L2CValue(&LStack_50);
//     if (!bVar1) goto LAB_7100024620;
//     lib::L2CValue::L2CValue(&LStack_50,_GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
//     GVar2 = lib::L2CValue::as_integer(&LStack_50);
//     app::lua_bind::GroundModule__correct_impl(this->moduleAccessor,GVar2);
//     lib::L2CValue::~L2CValue(&LStack_50);
//     lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_KINETIC_TYPE_GROUND_STOP);
//     iVar3 = lib::L2CValue::as_integer(&LStack_50);
//     app::lua_bind::KineticModule__change_kinetic_impl(this->moduleAccessor,iVar3);
//     lib::L2CValue::~L2CValue(&LStack_50);
//     lib::L2CValue::L2CValue(&LStack_50,0xb16d2b473);
//     HVar5 = lib::L2CValue::as_hash(&LStack_50);
//     app::lua_bind::MotionModule__change_motion_inherit_frame_impl
//               (this->moduleAccessor,HVar5,-1.0,1.0,0.0,false,false);
//   }
//   lib::L2CValue::~L2CValue(&LStack_50);
// LAB_7100024620:
//   lib::L2CValue::L2CValue(&LStack_80,_FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
//   iVar3 = lib::L2CValue::as_integer(&LStack_80);
//   iVar3 = app::lua_bind::WorkModule__get_int_impl(this->moduleAccessor,iVar3);
//   lib::L2CValue::L2CValue(&LStack_50,iVar3);
//   lib::L2CValue::operator=(&LStack_60,(L2CValue *)&LStack_50);
//   lib::L2CValue::~L2CValue(&LStack_50);
//   lib::L2CValue::~L2CValue(&LStack_80);
//   FUN_7100005770(&LStack_50,this);
//   lib::L2CValue::operator=(&LStack_70,(L2CValue *)&LStack_50);
//   lib::L2CValue::~L2CValue(&LStack_50);
//   uVar6 = lib::L2CValue::operator<=(&LStack_70,(L2CValue *)&LStack_60);
//   if ((uVar6 & 1) == 0) {
//     lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H);
//     lib::L2CValue::L2CValue(&LStack_80,false);
//     lua2cpp::L2CFighterBase::change_status(this,(L2CValue *)&LStack_50,(L2CValue *)&LStack_80);
//     lib::L2CValue::~L2CValue(&LStack_80);
//     lib::L2CValue::~L2CValue(&LStack_50);
//     lib::L2CValue::L2CValue(in_x8,1);
//   }
//   else {
//     lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART);
//     iVar3 = lib::L2CValue::as_integer(&LStack_50);
//     app::lua_bind::WorkModule__on_flag_impl(this->moduleAccessor,iVar3);
//     lib::L2CValue::~L2CValue(&LStack_50);
//     lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F);
//     lib::L2CValue::L2CValue(&LStack_80,false);
//     lua2cpp::L2CFighterBase::change_status(this,(L2CValue *)&LStack_50,(L2CValue *)&LStack_80);
//     lib::L2CValue::~L2CValue(&LStack_80);
//     lib::L2CValue::~L2CValue(&LStack_50);
//     lib::L2CValue::L2CValue(in_x8,1);
//   }
//   lib::L2CValue::~L2CValue(&LStack_70);
//   lib::L2CValue::~L2CValue(&LStack_60);
//   return;
// }

pub fn install() {
    install_status_scripts!(
        //samusd_specialn_main,
        samusd_specialnf_exec
    );
}

//   pLVar5 = (L2CValue *)lib::L2CValue::operator[](&this->globalTable,0x17);
//   lib::L2CValue::L2CValue(&LStack_50,_SITUATION_KIND_GROUND);
//   bVar1 = lib::L2CValue::operator==(pLVar5,(L2CValue *)&LStack_50);
//   lib::L2CValue::~L2CValue(&LStack_50);
//   pLVar5 = (L2CValue *)lib::L2CValue::operator[](&this->globalTable,0x16);
//   if (bVar1) {
//     lib::L2CValue::L2CValue(&LStack_50,SITUATION_KIND_AIR);
//     bVar1 = lib::L2CValue::operator==(pLVar5,(L2CValue *)&LStack_50);
//     lib::L2CValue::~L2CValue(&LStack_50);
//     if (!bVar1) goto LAB_7100024358;
//     lib::L2CValue::L2CValue(&LStack_50,GROUND_CORRECT_KIND_AIR);
//     GVar3 = lib::L2CValue::as_integer(&LStack_50);
//     app::lua_bind::GroundModule__correct_impl(this->moduleAccessor,GVar3);
//     lib::L2CValue::~L2CValue(&LStack_50);
//     lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_KINETIC_TYPE_AIR_STOP);
//     iVar4 = lib::L2CValue::as_integer(&LStack_50);
//     app::lua_bind::KineticModule__change_kinetic_impl(this->moduleAccessor,iVar4);
//     lib::L2CValue::~L2CValue(&LStack_50);
//     lib::L2CValue::L2CValue(&LStack_50,0xfe140b144);
//     HVar6 = lib::L2CValue::as_hash(&LStack_50);
//     app::lua_bind::MotionModule__change_motion_inherit_frame_impl
//               (this->moduleAccessor,HVar6,-1.0,1.0,0.0,false,false);
//   }
//   else {
//     lib::L2CValue::L2CValue(&LStack_50,_SITUATION_KIND_GROUND);
//     bVar1 = lib::L2CValue::operator==(pLVar5,(L2CValue *)&LStack_50);
//     lib::L2CValue::~L2CValue(&LStack_50);
//     if (!bVar1) goto LAB_7100024358;
//     lib::L2CValue::L2CValue(&LStack_50,_GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
//     GVar3 = lib::L2CValue::as_integer(&LStack_50);
//     app::lua_bind::GroundModule__correct_impl(this->moduleAccessor,GVar3);
//     lib::L2CValue::~L2CValue(&LStack_50);
//     lib::L2CValue::L2CValue(&LStack_50,_FIGHTER_KINETIC_TYPE_GROUND_STOP);
//     iVar4 = lib::L2CValue::as_integer(&LStack_50);
//     app::lua_bind::KineticModule__change_kinetic_impl(this->moduleAccessor,iVar4);
//     lib::L2CValue::~L2CValue(&LStack_50);
//     lib::L2CValue::L2CValue(&LStack_50,0xb16d2b473);
//     HVar6 = lib::L2CValue::as_hash(&LStack_50);
//     app::lua_bind::MotionModule__change_motion_inherit_frame_impl
//               (this->moduleAccessor,HVar6,-1.0,1.0,0.0,false,false);
//   }
//   lib::L2CValue::~L2CValue(&LStack_50);
// LAB_7100024358:
//   bVar2 = app::lua_bind::MotionModule__is_end_impl(this->moduleAccessor);
//   lib::L2CValue::L2CValue(&LStack_50,(bool)(bVar2 & 1));
//   bVar1 = lib::L2CValue::operator.cast.to.bool(&LStack_50);
//   lib::L2CValue::~L2CValue(&LStack_50);
//   if (bVar1) {
//     lib::L2CValue::L2CValue(&LStack_50,&DAT_7100024410);
//     lua2cpp::L2CFighterBase::fastshift(this,SUB81(&LStack_50,0),return_value);
//     lib::L2CValue::~L2CValue(&LStack_50);
//   }
//   else {
//     lib::L2CValue::L2CValue(return_value,0);
//   }
//   return;
// }


//   this_00 = (L2CValue *)lib::L2CValue::operator[](&this->globalTable,0x16);
//   lib::L2CValue::L2CValue(&LStack_60,_SITUATION_KIND_GROUND);
//   bVar1 = lib::L2CValue::operator==(this_00,(L2CValue *)&LStack_60);
//   lib::L2CValue::~L2CValue(&LStack_60);
//   if (bVar1) {
//     lib::L2CValue::L2CValue(&LStack_60,_GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
//     GVar3 = lib::L2CValue::as_integer(&LStack_60);
//     app::lua_bind::GroundModule__correct_impl(this->moduleAccessor,GVar3);
//     lib::L2CValue::~L2CValue(&LStack_60);
//     lib::L2CValue::L2CValue(&LStack_60,_FIGHTER_KINETIC_TYPE_GROUND_STOP);
//     iVar4 = lib::L2CValue::as_integer(&LStack_60);
//     app::lua_bind::KineticModule__change_kinetic_impl(this->moduleAccessor,iVar4);
//     lib::L2CValue::~L2CValue(&LStack_60);
//     lib::L2CValue::L2CValue(&LStack_80,_FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART);
//     iVar4 = lib::L2CValue::as_integer(&LStack_80);
//     bVar1 = app::lua_bind::WorkModule__is_flag_impl(this->moduleAccessor,iVar4);
//     lib::L2CValue::L2CValue(&LStack_70,bVar1);
//     lib::L2CValue::L2CValue(&LStack_60,false);
//     bVar1 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_60);
//     lib::L2CValue::~L2CValue(&LStack_60);
//     lib::L2CValue::~L2CValue(&LStack_70);
//     lib::L2CValue::~L2CValue(&LStack_80);
//     if (bVar1) {
//       lib::L2CValue::L2CValue(&LStack_60,0xb16d2b473);
//       HVar6 = lib::L2CValue::as_hash(&LStack_60);
//       app::lua_bind::MotionModule__change_motion_inherit_frame_impl
//                 (this->moduleAccessor,HVar6,-1.0,1.0,0.0,false,false);
//     }
    // else {
    //   lib::L2CValue::L2CValue(&LStack_60,0xb16d2b473);
    //   lib::L2CValue::L2CValue(&LStack_70,0.0);
    //   lib::L2CValue::L2CValue(&LStack_80,1.0);
    //   lib::L2CValue::L2CValue(&LStack_90,false);
    //   HVar6 = lib::L2CValue::as_hash(&LStack_60);
    //   fVar7 = (float)lib::L2CValue::as_number(&LStack_70);
    //   fVar8 = (float)lib::L2CValue::as_number(&LStack_80);
    //   bVar2 = lib::L2CValue::as_bool(&LStack_90);
    //   app::lua_bind::MotionModule__change_motion_impl
    //             (this->moduleAccessor,HVar6,fVar7,fVar8,(bool)(bVar2 & 1),0.0,false,false);
    //   lib::L2CValue::~L2CValue(&LStack_90);
    //   lib::L2CValue::~L2CValue(&LStack_80);
    //   lib::L2CValue::~L2CValue(&LStack_70);
    //   lib::L2CValue::~L2CValue(&LStack_60);
    //   lib::L2CValue::L2CValue(&LStack_60,_FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART);
    //   iVar4 = lib::L2CValue::as_integer(&LStack_60);
    //   app::lua_bind::WorkModule__off_flag_impl(this->moduleAccessor,iVar4);
    // }
//   }
//   else {
//     lib::L2CValue::L2CValue(&LStack_60,GROUND_CORRECT_KIND_AIR);
//     GVar3 = lib::L2CValue::as_integer(&LStack_60);
//     app::lua_bind::GroundModule__correct_impl(this->moduleAccessor,GVar3);
//     lib::L2CValue::~L2CValue(&LStack_60);
//     lib::L2CValue::L2CValue(&LStack_60,_FIGHTER_KINETIC_TYPE_AIR_STOP);
//     iVar4 = lib::L2CValue::as_integer(&LStack_60);
//     app::lua_bind::KineticModule__change_kinetic_impl(this->moduleAccessor,iVar4);
//     lib::L2CValue::~L2CValue(&LStack_60);
//     lib::L2CValue::L2CValue(&LStack_80,_FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART);
//     iVar4 = lib::L2CValue::as_integer(&LStack_80);
//     bVar1 = app::lua_bind::WorkModule__is_flag_impl(this->moduleAccessor,iVar4);
//     lib::L2CValue::L2CValue(&LStack_70,bVar1);
//     lib::L2CValue::L2CValue(&LStack_60,false);
//     bVar1 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_60);
//     lib::L2CValue::~L2CValue(&LStack_60);
//     lib::L2CValue::~L2CValue(&LStack_70);
//     lib::L2CValue::~L2CValue(&LStack_80);
//     if (bVar1) {
//       lib::L2CValue::L2CValue(&LStack_60,0xfe140b144);
//       HVar6 = lib::L2CValue::as_hash(&LStack_60);
//       app::lua_bind::MotionModule__change_motion_inherit_frame_impl
//                 (this->moduleAccessor,HVar6,-1.0,1.0,0.0,false,false);
//     }
//     else {
//       lib::L2CValue::L2CValue(&LStack_60,0xfe140b144);
//       lib::L2CValue::L2CValue(&LStack_70,0.0);
//       lib::L2CValue::L2CValue(&LStack_80,1.0);
//       lib::L2CValue::L2CValue(&LStack_90,false);
//       HVar6 = lib::L2CValue::as_hash(&LStack_60);
//       fVar7 = (float)lib::L2CValue::as_number(&LStack_70);
//       fVar8 = (float)lib::L2CValue::as_number(&LStack_80);
//       bVar2 = lib::L2CValue::as_bool(&LStack_90);
//       app::lua_bind::MotionModule__change_motion_impl
//                 (this->moduleAccessor,HVar6,fVar7,fVar8,(bool)(bVar2 & 1),0.0,false,false);
//       lib::L2CValue::~L2CValue(&LStack_90);
//       lib::L2CValue::~L2CValue(&LStack_80);
//       lib::L2CValue::~L2CValue(&LStack_70);
//       lib::L2CValue::~L2CValue(&LStack_60);
//       lib::L2CValue::L2CValue(&LStack_60,_FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART);
//       iVar4 = lib::L2CValue::as_integer(&LStack_60);
//       app::lua_bind::WorkModule__off_flag_impl(this->moduleAccessor,iVar4);
//     }
//   }
//   lib::L2CValue::~L2CValue(&LStack_60);
//   lib::L2CValue::L2CValue(&LStack_60,8);
//   sVar5 = lib::L2CValue::as_integer(&LStack_60);
//   app::lua_bind::ControlModule__set_add_jump_mini_button_life_impl(this->moduleAccessor,sVar5);
//   lib::L2CValue::~L2CValue(&LStack_60);
//   lib::L2CValue::L2CValue(&LStack_60,SpecialN::loop);
//   lua2cpp::L2CFighterCommon::sub_shift_status_main(this,SUB81(&LStack_60,0),return_value);
//   lib::L2CValue::~L2CValue(&LStack_60);
//   return;
// }