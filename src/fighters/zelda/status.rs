use super::*;

unsafe extern "C" fn status_specialhi2exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        VarModule::on_flag(fighter.module_accessor, zelda::instance::flag::SPECIAL_HI_CANCEL);
        fighter.change_status(FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3.into(), false.into());
    }
    return 0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Exec, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, status_specialhi2exec);
}

// #[status_script(agent="zelda", status = FIGHTER_STATUS_KIND_SPECIAL_HI_2, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn status_specialsmain(fighter: &mut L2CFighterCommon) -> L2CValue {
//     HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
//     VisibilityModule::set_whole(fighter.module_accessor, false);
//     notify_event_msc_cmd!(fighter, 0x1f20a9d549u64, false);
//     WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
//     GroundModule::set_passable_check(fighter.module_accessor, true);
//     let cliff_check = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_CLIFF_CHECK);
//     fighter.sub_fighter_cliff_check(cliff_check.into());
//     WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
//     if !StopModule::is_stop(fighter.module_accessor) {
//         zelda_special_hi_2_substatus(fighter, false.into());
//     }
//     fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(zelda_special_hi_2_substatus as *const () as _));
//     fighter.sub_shift_status_main(L2CValue::Ptr(zelda_specialhi2_main_loop as *const () as _))
// }

// unsafe extern "C" fn zelda_special_hi_2_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
//     if !param_1.get_bool() {
//         let specialhi_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
//         if WorkModule::get_param_int(fighter.module_accessor, 0x1086bc4a93, hash40("move_xlu")) == specialhi_frame {
//             GroundModule::set_passable_check(fighter.module_accessor, false);
//         }
//         if WorkModule::get_param_int(fighter.module_accessor, 0x1086bc4a93, hash40("move_cliff_check")) == specialhi_frame {
//             fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
//         }
//     }
//     else {
//         WorkModule::inc_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
//         if WorkModule::get_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME) >= 2 {
//             WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CHECK_GROUND);
//         }
//     }
//     return 0.into()
// }

// unsafe extern "C" fn zelda_specialhi2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {  
//     if fighter.sub_transition_group_check_air_cliff().get_bool() {
//         return 1.into();
//     }
//     let specialhi_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
//     if WorkModule::get_param_int(fighter.module_accessor, 0x1086bc4a93, hash40("move_time")) <= specialhi_frame {
//         if StatusModule::is_changing(fighter.module_accessor) {
//             if fighter.global_table[SITUATION_KIND].get_int() != *SITUATION_KIND_GROUND {
//                 GroundModule::correct(fighter.module_accessor, *GROUND_CORRECT_KIND_AIR);
//             }
//             else {
//                 GroundModule::correct(fighter.module_accessor, *GROUND_CORRECT_KIND_GROUND);
//             }
//         }
//         zelda_specialhi2_check_ground_touch(fighter.lua_state_agent);
//     }
//     else {
//         fighter.change_status(FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3.into(), false.into());
//     }
//     return 0.into();
// }

//   lua2cpp::L2CFighterCommon::sub_transition_group_check_air_cliff(this,&LStack80);
//   bVar1 = lib::L2CValue::operator.cast.to.bool((L2CValue *)&LStack80);
//   lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
//   if ((bVar1 & 1U) != 0) {
//     iVar3 = 1;
//     goto LAB_710000e3a8;
//   }
//   lib::L2CValue::L2CValue((L2CValue *)&LStack96,_FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
//   iVar3 = lib::L2CValue::as_integer((L2CValue *)&LStack96);
//   iVar3 = app::lua_bind::WorkModule__get_int_impl(this->moduleAccessor,iVar3);
//   lib::L2CValue::L2CValue((L2CValue *)&LStack80,iVar3);
//   lib::L2CValue::L2CValue(aLStack128,0x1086bc4a93);
//   lib::L2CValue::L2CValue(aLStack144,0x961fca4f1);
//   uVar5 = lib::L2CValue::as_integer(aLStack128);
//   uVar6 = lib::L2CValue::as_integer(aLStack144);
//   iVar3 = app::lua_bind::WorkModule__get_param_int_impl(this->moduleAccessor,uVar5,uVar6);
//   lib::L2CValue::L2CValue(aLStack112,iVar3);
//   uVar5 = lib::L2CValue::operator<=(aLStack112,(L2CValue *)&LStack80);
//   lib::L2CValue::~L2CValue(aLStack112);
//   lib::L2CValue::~L2CValue(aLStack144);
//   lib::L2CValue::~L2CValue(aLStack128);
//   lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
//   lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
//   if ((uVar5 & 1) == 0) {
//     bVar2 = app::lua_bind::StatusModule__is_changing_impl(this->moduleAccessor);
//     lib::L2CValue::L2CValue((L2CValue *)&LStack96,(bool)(bVar2 & 1));
//     bVar1 = lib::L2CValue::operator.cast.to.bool((L2CValue *)&LStack96);
//     if ((bVar1 & 1U) == 0) {
//       pLVar8 = &this->globalTable;
//       pLVar7 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar8,0x17);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack80,_SITUATION_KIND_GROUND);
//       uVar5 = lib::L2CValue::operator==(pLVar7,(L2CValue *)&LStack80);
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
//       if ((uVar5 & 1) != 0) {
//         pLVar7 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar8,0x16);
//         lib::L2CValue::L2CValue((L2CValue *)&LStack80,SITUATION_KIND_AIR);
//         uVar5 = lib::L2CValue::operator==(pLVar7,(L2CValue *)&LStack80);
//         lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
//         if ((uVar5 & 1) != 0) {
//           lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
//           goto LAB_710000e254;
//         }
//       }
//       pLVar7 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar8,0x17);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack80,_SITUATION_KIND_GROUND);
//       uVar5 = lib::L2CValue::operator==(pLVar7,(L2CValue *)&LStack80);
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
//       if ((uVar5 & 1) != 0) {
//         pLVar8 = &LStack96;
//         goto LAB_710000e394;
//       }
//       pLVar7 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar8,0x16);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack80,_SITUATION_KIND_GROUND);
//       uVar5 = lib::L2CValue::operator==(pLVar7,(L2CValue *)&LStack80);
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
//       if ((uVar5 & 1) != 0) goto LAB_710000e254;
//     }
//     else {
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
// LAB_710000e254:
//       pLVar7 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)&this->globalTable,0x16);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack80,_SITUATION_KIND_GROUND);
//       uVar5 = lib::L2CValue::operator==(pLVar7,(L2CValue *)&LStack80);
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
//       if ((uVar5 & 1) == 0) {
//         lib::L2CValue::L2CValue((L2CValue *)&LStack80,GROUND_CORRECT_KIND_AIR);
//         GVar4 = lib::L2CValue::as_integer((L2CValue *)&LStack80);
//         app::lua_bind::GroundModule__correct_impl(this->moduleAccessor,GVar4);
//       }
//       else {
//         lib::L2CValue::L2CValue((L2CValue *)&LStack80,GROUND_CORRECT_KIND_GROUND);
//         GVar4 = lib::L2CValue::as_integer((L2CValue *)&LStack80);
//         app::lua_bind::GroundModule__correct_impl(this->moduleAccessor,GVar4);
//       }
//       pLVar8 = &LStack80;
// LAB_710000e394:
//       lib::L2CValue::~L2CValue((L2CValue *)pLVar8);
//     }
//     FUN_710000e4c0(this);
//   }
//   else {
//     lib::L2CValue::L2CValue((L2CValue *)&LStack80,_FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3);
//     lib::L2CValue::L2CValue((L2CValue *)&LStack96,false);
//     lua2cpp::L2CFighterBase::change_status(this,(L2CValue)0xb0,(L2CValue)0xa0);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack96);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
//   }
//   iVar3 = 0;
// LAB_710000e3a8:
//   lib::L2CValue::L2CValue((L2CValue *)return_value,iVar3);
//   return;
// }

// unsafe extern "C" fn zelda_specialhi2_check_ground_touch(agent: &mut L2CAgentBase) {
//     if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CHECK_GROUND) {
//         if !GroundModule::is_attach_cliff(agent.module_accessor) {
//             let ground_touch_flag : u32 = 0;
//             let ground_touch_id : u32 = 0;
//             if GroundModule::is_touch(agent.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
//                 ground_touch_flag = *GROUND_TOUCH_FLAG_RIGHT as u32;
//                 ground_touch_id = *GROUND_TOUCH_ID_RIGHT as u32;
//             }
//             else if GroundModule::is_touch(agent.module_accessor, *GROUND_TOUCH_FLAG_LEFT) {
//                 ground_touch_flag = *GROUND_TOUCH_FLAG_LEFT as u32;
//                 ground_touch_id = *GROUND_TOUCH_ID_LEFT as u32;
//             }
//             else if GroundModule::is_touch(agent.module_accessor, *GROUND_TOUCH_FLAG_UP as u32 as u32) {
//                 ground_touch_flag = *GROUND_TOUCH_FLAG_UP as u32;
//                 ground_touch_id = *GROUND_TOUCH_ID_UP as u32;
//             }
//             else if GroundModule::is_touch(agent.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
//                 ground_touch_flag = *GROUND_TOUCH_FLAG_DOWN as u32;
//                 ground_touch_id = *GROUND_TOUCH_ID_DOWN as u32;
//             }
//             if ground_touch_flag == *GROUND_TOUCH_FLAG_NONE as u32 {
//                 return
//             }
//             else {
//                 // agent.clear_lua_stack();
//                 // smash_script::lua_args!(agent, *FIGHTER_KINETIC_ENERGY_ID_STOP);
//                 // let speed = smash::app::sv_kinetic_energy::get_speed3f(agent.lua_state_agent);
//                 let kinetic_energy_stop = std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP));
//                 //let speed = KineticEnergy::get_speed_x(kinetic_energy_stop);
//                 let speed = KineticEnergy::get_speed3f(kinetic_energy_stop);
//                 let vec_length = smash::app::sv_math::vec3_length(speed.x, speed.y, speed.z);
//                 if speed.y < 0 {
//                     let touch_normal = GroundModule::get_touch_normal(fighter.module_accessor, ground_touch_flag);
//                     let vec3_touch = Vector3f{x: touch_normal.x, y: touch_normal.y, z: 0.0};
//                     let vec3 = Vector3f{x: 0.0, y: 0.0, z: 1.0};
//                     let vec3_cross = smash::app::sv_math::vec3_cross(vec3_touch.x, vec3_touch.y, vec3_touch.z, vec3.x, vec3.y, vec3.z);
//                     //the rest of the script is cursed, abandon ship, never come back
//                 }
//             }
//         }
//     }
// }
// unsafe extern "C" fn zelda_specialhi2_check_ground_touch_subfn(fighter: &mut L2CFighterCommon, speed: Vector3f) {
//     this is just creating a vec3 from the let speed = KineticEnergy::get_speed3f(kinetic_energy_stop) line, safely ignore
// }

//   lib::L2CValue::L2CValue(aLStack160,_FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CHECK_GROUND);
//   iVar3 = lib::L2CValue::as_integer(aLStack160);
//   bVar1 = app::lua_bind::WorkModule__is_flag_impl(param_1->moduleAccessor,iVar3);
//   lib::L2CValue::L2CValue((L2CValue *)&LStack144,(bool)(bVar1 & 1));
//   bVar2 = lib::L2CValue::operator.cast.to.bool((L2CValue *)&LStack144);
//   lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//   lib::L2CValue::~L2CValue(aLStack160);
//   if ((bVar2 & 1U) == 0) {
//     return;
//   }
//   bVar1 = app::lua_bind::GroundModule__is_attach_cliff_impl(param_1->moduleAccessor);
//   lib::L2CValue::L2CValue((L2CValue *)&LStack144,(bool)(bVar1 & 1));
//   bVar2 = lib::L2CValue::operator.cast.to.bool((L2CValue *)&LStack144);
//   lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//   if ((bVar2 & 1U) != 0) {
//     return;
//   }
//   lib::L2CValue::L2CValue(aLStack176,_GROUND_TOUCH_ID_NONE);
//   lib::L2CValue::L2CValue(aLStack192,_GROUND_TOUCH_FLAG_NONE);
//   lib::L2CValue::L2CValue(aLStack160,GROUND_TOUCH_FLAG_RIGHT);
//   uVar4 = lib::L2CValue::as_integer(aLStack160);
//   bVar1 = app::lua_bind::GroundModule__is_touch_impl(param_1->moduleAccessor,uVar4);
//   lib::L2CValue::L2CValue((L2CValue *)&LStack144,(bool)(bVar1 & 1));
//   bVar2 = lib::L2CValue::operator.cast.to.bool((L2CValue *)&LStack144);
//   lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//   lib::L2CValue::~L2CValue(aLStack160);
//   if ((bVar2 & 1U) == 0) {
//     lib::L2CValue::L2CValue(aLStack160,_GROUND_TOUCH_FLAG_LEFT);
//     uVar4 = lib::L2CValue::as_integer(aLStack160);
//     bVar1 = app::lua_bind::GroundModule__is_touch_impl(param_1->moduleAccessor,uVar4);
//     lib::L2CValue::L2CValue((L2CValue *)&LStack144,(bool)(bVar1 & 1));
//     bVar2 = lib::L2CValue::operator.cast.to.bool((L2CValue *)&LStack144);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//     lib::L2CValue::~L2CValue(aLStack160);
//     if ((bVar2 & 1U) != 0) {
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,_GROUND_TOUCH_FLAG_LEFT);
//       lib::L2CValue::operator=(aLStack192,(L2CValue *)&LStack144);
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,_GROUND_TOUCH_ID_LEFT);
//       lib::L2CValue::operator=(aLStack176,(L2CValue *)&LStack144);
//       goto LAB_710000e7ac;
//     }
//     lib::L2CValue::L2CValue(aLStack160,_GROUND_TOUCH_FLAG_UP);
//     uVar4 = lib::L2CValue::as_integer(aLStack160);
//     bVar1 = app::lua_bind::GroundModule__is_touch_impl(param_1->moduleAccessor,uVar4);
//     lib::L2CValue::L2CValue((L2CValue *)&LStack144,(bool)(bVar1 & 1));
//     bVar2 = lib::L2CValue::operator.cast.to.bool((L2CValue *)&LStack144);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//     lib::L2CValue::~L2CValue(aLStack160);
//     if ((bVar2 & 1U) != 0) {
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,_GROUND_TOUCH_FLAG_UP);
//       lib::L2CValue::operator=(aLStack192,(L2CValue *)&LStack144);
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,_GROUND_TOUCH_ID_UP);
//       lib::L2CValue::operator=(aLStack176,(L2CValue *)&LStack144);
//       goto LAB_710000e7ac;
//     }
//     lib::L2CValue::L2CValue(aLStack160,GROUND_TOUCH_FLAG_DOWN);
//     uVar4 = lib::L2CValue::as_integer(aLStack160);
//     bVar1 = app::lua_bind::GroundModule__is_touch_impl(param_1->moduleAccessor,uVar4);
//     lib::L2CValue::L2CValue((L2CValue *)&LStack144,(bool)(bVar1 & 1));
//     bVar2 = lib::L2CValue::operator.cast.to.bool((L2CValue *)&LStack144);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//     lib::L2CValue::~L2CValue(aLStack160);
//     if ((bVar2 & 1U) != 0) {
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,GROUND_TOUCH_FLAG_DOWN);
//       lib::L2CValue::operator=(aLStack192,(L2CValue *)&LStack144);
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,_GROUND_TOUCH_ID_DOWN);
//       lib::L2CValue::operator=(aLStack176,(L2CValue *)&LStack144);
//       goto LAB_710000e7ac;
//     }
//   }
//   else {
//     lib::L2CValue::L2CValue((L2CValue *)&LStack144,GROUND_TOUCH_FLAG_RIGHT);
//     lib::L2CValue::operator=(aLStack192,(L2CValue *)&LStack144);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//     lib::L2CValue::L2CValue((L2CValue *)&LStack144,_GROUND_TOUCH_ID_RIGHT);
//     lib::L2CValue::operator=(aLStack176,(L2CValue *)&LStack144);
// LAB_710000e7ac:
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//   }
//   lib::L2CValue::L2CValue((L2CValue *)&LStack144,_GROUND_TOUCH_FLAG_NONE);
//   uVar6 = lib::L2CValue::operator==(aLStack192,(L2CValue *)&LStack144);
//   lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//   if ((uVar6 & 1) != 0) goto LAB_710000f40c;
//   lib::L2CValue::L2CValue((L2CValue *)&LStack144,_FIGHTER_KINETIC_ENERGY_ID_STOP);
//   lib::L2CAgent::clear_lua_stack(param_1);
//   lib::L2CAgent::push_lua_stack(param_1,(L2CValue *)&LStack144);
//   uVar21 = app::sv_kinetic_energy::get_speed3f(param_1->luaStateAgent);
//   lib::L2CValue::L2CValue((L2CValue *)&LStack256,(float)uVar21,return_value);
//   lib::L2CValue::L2CValue(aLStack240,(float)((ulong)uVar21 >> 0x20),return_value_00);
//   lib::L2CValue::L2CValue(aLStack224,in_register_00005008,&LStack256);
//   FUN_710000bf50(aLStack208,param_1,&LStack256);
//   lib::L2CValue::~L2CValue(aLStack224);
//   lib::L2CValue::~L2CValue(aLStack240);
//   lib::L2CValue::~L2CValue((L2CValue *)&LStack256);
//   lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//   pLVar7 = (L2CValue *)lib::L2CValue::operator[](aLStack208,0x18cdc1683);
//   pLVar8 = (L2CValue *)lib::L2CValue::operator[](aLStack208,0x1fbdb2615);
//   pLVar9 = (L2CValue *)lib::L2CValue::operator[](aLStack208,0x162d277af);
//   fVar15 = (float)lib::L2CValue::as_number(pLVar7);
//   fVar16 = (float)lib::L2CValue::as_number(pLVar8);
//   fVar17 = (float)lib::L2CValue::as_number(pLVar9);
//   fVar15 = (float)app::sv_math::vec3_length(fVar15,fVar16,fVar17);
//   lib::L2CValue::L2CValue(aLStack272,fVar15,return_value_01);
//   lib::L2CValue::L2CValue((L2CValue *)&LStack144,0.0,return_value_02);
//   uVar6 = lib::L2CValue::operator<((L2CValue *)&LStack144,aLStack272);
//   lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//   if ((uVar6 & 1) != 0) {
//     uVar4 = lib::L2CValue::as_integer(aLStack192);
//     uVar21 = app::lua_bind::GroundModule__get_touch_normal_impl(param_1->moduleAccessor,uVar4);
//     lib::L2CValue::L2CValue(aLStack320,(float)uVar21,return_value_03);
//     lib::L2CValue::L2CValue(aLStack304,(float)((ulong)uVar21 >> 0x20),return_value_04);
//     lib::L2CValue::L2CValue((L2CValue *)&LStack144,aLStack320);
//     lib::L2CValue::L2CValue(aLStack160,aLStack304);
//     lua2cpp::L2CFighterBase::Vector2__create(param_1,(L2CValue *)&LStack144,aLStack160,&LStack288);
//     lib::L2CValue::~L2CValue(aLStack160);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//     lib::L2CValue::~L2CValue(aLStack304);
//     lib::L2CValue::~L2CValue(aLStack320);
//     pLVar7 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)&LStack288,0x18cdc1683);
//     lib::L2CValue::L2CValue(aLStack352,pLVar7);
//     pLVar7 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)&LStack288,0x1fbdb2615);
//     lib::L2CValue::L2CValue(aLStack368,pLVar7);
//     lib::L2CValue::L2CValue(aLStack384,0.0,return_value_05);
//     lua2cpp::L2CFighterBase::Vector3__create(param_1,aLStack352,aLStack368,aLStack384,&LStack336);
//     lib::L2CValue::L2CValue(aLStack416,0.0,return_value_06);
//     lib::L2CValue::L2CValue(aLStack432,0.0,return_value_07);
//     lib::L2CValue::L2CValue((L2CValue *)asStack448,1.0,return_value_08);
//     siglen = asStack448;
//     lua2cpp::L2CFighterBase::Vector3__create
//               (param_1,aLStack416,aLStack432,(L2CValue *)siglen,&LStack400);
// //     pLVar14 = &LStack400;
//     lua2cpp::L2CFighterBase::Vector3__cross(param_1,(L2CValue *)&LStack336,(L2CValue *)pLVar14);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack400);
//     lib::L2CValue::~L2CValue((L2CValue *)asStack448);
//     lib::L2CValue::~L2CValue(aLStack432);
//     lib::L2CValue::~L2CValue(aLStack416);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack336);
//     lib::L2CValue::~L2CValue(aLStack384);
//     lib::L2CValue::~L2CValue(aLStack368);
//     lib::L2CValue::~L2CValue(aLStack352);
//     lib::L2CValue::L2CValue((L2CValue *)&LStack144,1.0,return_value_09);
//     lib::L2CValue::operator/((L2CValue *)&LStack144,aLStack272,&LStack464);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//     lib::L2CValue::operator*(aLStack208,(L2CValue *)&LStack464,&LStack144);
//     lib::L2CValue::operator=(aLStack208,(L2CValue *)&LStack144);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//     lib::L2CValue::L2CValue((L2CValue *)(auStack496 + 0x10),0.0,return_value_10);
//     lib::L2CValue::L2CValue((L2CValue *)&LStack144,GROUND_TOUCH_FLAG_DOWN);
//     uVar6 = lib::L2CValue::operator==(aLStack192,(L2CValue *)&LStack144);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//     if ((uVar6 & 1) == 0) {
//       pLVar7 = (L2CValue *)lib::L2CValue::operator[](aLStack208,0x1fbdb2615);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,0.0,return_value_11);
//       uVar6 = lib::L2CValue::operator<((L2CValue *)&LStack144,pLVar7);
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//       if ((uVar6 & 1) == 0) goto LAB_710000ed58;
//       pLVar7 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x1fbdb2615);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,0.0,return_value_12);
//       uVar6 = lib::L2CValue::operator<(pLVar7,(L2CValue *)&LStack144);
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//       if ((uVar6 & 1) != 0) {
//         lib::L2CValue::L2CValue((L2CValue *)&LStack144,-1.0,return_value_13);
//         lib::L2CValue::operator=((L2CValue *)(auStack496 + 0x10),(L2CValue *)&LStack144);
//         lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//       }
//       pLVar10 = (L2CAgent *)lib::L2CValue::operator[]((L2CValue *)&LStack288,0x18cdc1683);
//       pLVar7 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)&LStack288,0x1fbdb2615);
//       lib::L2CAgent::math_atan(pLVar10,pLVar7,(L2CValue *)pLVar14);
//       lib::L2CAgent::math_deg((L2CAgent *)auStack496,pLVar7);
//       lib::L2CAgent::math_abs((L2CAgent *)&LStack144,pLVar7);
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,180.0,(L2CValue *)0x7100124000);
//       pLVar7 = (L2CValue *)(auStack544 + 0x20);
//       lib::L2CValue::operator-((L2CValue *)&LStack144,pLVar7,(L2CValue *)auStack544);
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//       lib::L2CAgent::math_abs((L2CAgent *)auStack544,pLVar7);
//       lib::L2CValue::~L2CValue((L2CValue *)auStack544);
//       lib::L2CValue::L2CValue((L2CValue *)auStack544,0x6e5ec7051);
//       lib::L2CValue::L2CValue(aLStack560,0x158bb5418d);
//       uVar6 = lib::L2CValue::as_integer((L2CValue *)auStack544);
//       sig = (uchar *)lib::L2CValue::as_integer(aLStack560);
//       fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
//                                 (param_1->moduleAccessor,uVar6,(ulong)sig);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,fVar15,return_value_14);
//       lib::L2CValue::~L2CValue(aLStack560);
//       lib::L2CValue::~L2CValue((L2CValue *)auStack544);
//       uVar6 = lib::L2CValue::operator<=((L2CValue *)(auStack544 + 0x10),(L2CValue *)&LStack144);
//       if ((uVar6 & 1) != 0) {
//         pLVar7 = (L2CValue *)0x18cdc1683;
//         pLVar10 = (L2CAgent *)lib::L2CValue::operator[](aLStack208,0x18cdc1683);
//         lib::L2CAgent::math_abs(pLVar10,pLVar7);
//         lib::L2CValue::operator=(aLStack272,(L2CValue *)auStack544);
//         lib::L2CValue::~L2CValue((L2CValue *)auStack544);
//         pLVar7 = (L2CValue *)lib::L2CValue::operator[](aLStack208,0x18cdc1683);
//         lib::L2CValue::L2CValue(aLStack576,pLVar7);
//         lua2cpp::L2CFighterBase::sign(param_1,(EVP_PKEY_CTX *)aLStack576,sig,siglen,in_x4,in_x5);
//         pLVar7 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x18cdc1683);
//         lib::L2CValue::operator=(pLVar7,(L2CValue *)auStack544);
//         lib::L2CValue::~L2CValue((L2CValue *)auStack544);
//         lib::L2CValue::~L2CValue(aLStack576);
//       }
// LAB_710000f0d4:
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//       lib::L2CValue::~L2CValue((L2CValue *)(auStack544 + 0x10));
//       lib::L2CValue::~L2CValue((L2CValue *)(auStack544 + 0x20));
//       lib::L2CValue::~L2CValue((L2CValue *)auStack496);
//       pLVar14 = extraout_x8_01;
//     }
//     else {
// LAB_710000ed58:
//       pLVar7 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x18cdc1683);
//       pLVar8 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x1fbdb2615);
//       pLVar9 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x162d277af);
//       pLVar11 = (L2CValue *)lib::L2CValue::operator[](aLStack208,0x18cdc1683);
//       this = (L2CValue *)lib::L2CValue::operator[](aLStack208,0x1fbdb2615);
//       this_00 = (L2CValue *)lib::L2CValue::operator[](aLStack208,0x162d277af);
//       fVar15 = (float)lib::L2CValue::as_number(pLVar7);
//       fVar16 = (float)lib::L2CValue::as_number(pLVar8);
//       fVar17 = (float)lib::L2CValue::as_number(pLVar9);
//       fVar18 = (float)lib::L2CValue::as_number(pLVar11);
//       fVar19 = (float)lib::L2CValue::as_number(this);
//       fVar20 = (float)lib::L2CValue::as_number(this_00);
//       fVar15 = (float)app::sv_math::vec3_dot(fVar15,fVar16,fVar17,fVar18,fVar19,fVar20);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,fVar15,return_value_15);
//       lib::L2CValue::operator=((L2CValue *)(auStack496 + 0x10),(L2CValue *)&LStack144);
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,-1e-05,(L2CValue *)0x7100124000);
//       uVar6 = lib::L2CValue::operator<=((L2CValue *)&LStack144,(L2CValue *)(auStack496 + 0x10));
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//       pLVar14 = extraout_x8;
//       if ((uVar6 & 1) != 0) {
//         lib::L2CValue::L2CValue((L2CValue *)&LStack144,1e-05,(L2CValue *)0x7100124000);
//         uVar6 = lib::L2CValue::operator<=((L2CValue *)(auStack496 + 0x10),(L2CValue *)&LStack144);
//         lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//         pLVar14 = extraout_x8_00;
//         if ((uVar6 & 1) != 0) {
//           lib::L2CValue::L2CValue((L2CValue *)&LStack144,GROUND_TOUCH_FLAG_RIGHT);
//           uVar6 = lib::L2CValue::operator==(aLStack192,(L2CValue *)&LStack144);
//           lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//           if ((uVar6 & 1) == 0) {
//             lib::L2CValue::L2CValue((L2CValue *)&LStack144,_GROUND_TOUCH_FLAG_LEFT);
//             uVar6 = lib::L2CValue::operator==(aLStack192,(L2CValue *)&LStack144);
//             lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//             if ((uVar6 & 1) == 0) {
//               pLVar7 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x18cdc1683);
//               pLVar8 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x1fbdb2615);
//               pLVar9 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x162d277af);
//               fVar15 = (float)app::lua_bind::PostureModule__lr_impl(param_1->moduleAccessor);
//               lib::L2CValue::L2CValue((L2CValue *)auStack496,fVar15,return_value_20);
//               lib::L2CValue::L2CValue((L2CValue *)(auStack544 + 0x20),0.0,return_value_21);
//               lib::L2CValue::L2CValue((L2CValue *)(auStack544 + 0x10),0.0,return_value_22);
//               fVar15 = (float)lib::L2CValue::as_number(pLVar7);
//               fVar16 = (float)lib::L2CValue::as_number(pLVar8);
//               fVar17 = (float)lib::L2CValue::as_number(pLVar9);
//               fVar18 = (float)lib::L2CValue::as_number((L2CValue *)auStack496);
//               fVar19 = (float)lib::L2CValue::as_number((L2CValue *)(auStack544 + 0x20));
//               fVar20 = (float)lib::L2CValue::as_number((L2CValue *)(auStack544 + 0x10));
//               fVar15 = (float)app::sv_math::vec3_dot(fVar15,fVar16,fVar17,fVar18,fVar19,fVar20);
//               lib::L2CValue::L2CValue((L2CValue *)&LStack144,fVar15,return_value_23);
//               lib::L2CValue::operator=((L2CValue *)(auStack496 + 0x10),(L2CValue *)&LStack144);
//               goto LAB_710000f0d4;
//             }
//           }
//           pLVar7 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x18cdc1683);
//           pLVar8 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x1fbdb2615);
//           pLVar9 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x162d277af);
//           lib::L2CValue::L2CValue((L2CValue *)auStack496,0.0,return_value_16);
//           lib::L2CValue::L2CValue((L2CValue *)(auStack544 + 0x20),1.0,return_value_17);
//           lib::L2CValue::L2CValue((L2CValue *)(auStack544 + 0x10),0.0,return_value_18);
//           fVar15 = (float)lib::L2CValue::as_number(pLVar7);
//           fVar16 = (float)lib::L2CValue::as_number(pLVar8);
//           fVar17 = (float)lib::L2CValue::as_number(pLVar9);
//           fVar18 = (float)lib::L2CValue::as_number((L2CValue *)auStack496);
//           fVar19 = (float)lib::L2CValue::as_number((L2CValue *)(auStack544 + 0x20));
//           fVar20 = (float)lib::L2CValue::as_number((L2CValue *)(auStack544 + 0x10));
//           fVar15 = (float)app::sv_math::vec3_dot(fVar15,fVar16,fVar17,fVar18,fVar19,fVar20);
//           lib::L2CValue::L2CValue((L2CValue *)&LStack144,fVar15,return_value_19);
//           lib::L2CValue::operator=((L2CValue *)(auStack496 + 0x10),(L2CValue *)&LStack144);
//           goto LAB_710000f0d4;
//         }
//       }
//     }
//     lib::L2CValue::L2CValue((L2CValue *)&LStack144,0.0,pLVar14);
//     uVar6 = lib::L2CValue::operator<((L2CValue *)(auStack496 + 0x10),(L2CValue *)&LStack144);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//     if ((uVar6 & 1) != 0) {
//       pLVar7 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x18cdc1683);
//       pLVar8 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x1fbdb2615);
//       pLVar9 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x162d277af);
//       pLVar11 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x18cdc1683);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,-1.0,return_value_24);
//       lib::L2CValue::operator*(pLVar11,(L2CValue *)&LStack144,(L2CValue *)auStack496);
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//       pLVar11 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x1fbdb2615);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,-1.0,return_value_25);
//       lib::L2CValue::operator*(pLVar11,(L2CValue *)&LStack144,(L2CValue *)(auStack544 + 0x20));
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//       pLVar11 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x162d277af);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,-1.0,return_value_26);
//       lib::L2CValue::operator*(pLVar11,(L2CValue *)&LStack144,(L2CValue *)(auStack544 + 0x10));
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//       lib::L2CValue::operator=(pLVar7,(L2CValue *)auStack496);
//       lib::L2CValue::operator=(pLVar8,(L2CValue *)(auStack544 + 0x20));
//       lib::L2CValue::operator=(pLVar9,(L2CValue *)(auStack544 + 0x10));
//       lib::L2CValue::~L2CValue((L2CValue *)(auStack544 + 0x10));
//       lib::L2CValue::~L2CValue((L2CValue *)(auStack544 + 0x20));
//       lib::L2CValue::~L2CValue((L2CValue *)auStack496);
//     }
//     lib::L2CValue::L2CValue((L2CValue *)&LStack144,_FIGHTER_KINETIC_ENERGY_ID_STOP);
//     pLVar7 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x18cdc1683);
//     lib::L2CValue::operator*(pLVar7,aLStack272,(L2CValue *)auStack496);
//     pLVar7 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x1fbdb2615);
//     lib::L2CValue::operator*(pLVar7,aLStack272,(L2CValue *)(auStack544 + 0x20));
//     pLVar7 = (L2CValue *)lib::L2CValue::operator[](aLStack160,0x162d277af);
//     lib::L2CValue::operator*(pLVar7,aLStack272,(L2CValue *)(auStack544 + 0x10));
//     lib::L2CAgent::clear_lua_stack(param_1);
//     lib::L2CAgent::push_lua_stack(param_1,(L2CValue *)&LStack144);
//     lib::L2CAgent::push_lua_stack(param_1,(L2CValue *)auStack496);
//     lib::L2CAgent::push_lua_stack(param_1,(L2CValue *)(auStack544 + 0x20));
//     lib::L2CAgent::push_lua_stack(param_1,(L2CValue *)(auStack544 + 0x10));
//     app::sv_kinetic_energy::set_speed(param_1->luaStateAgent);
//     lib::L2CValue::~L2CValue((L2CValue *)(auStack544 + 0x10));
//     lib::L2CValue::~L2CValue((L2CValue *)(auStack544 + 0x20));
//     lib::L2CValue::~L2CValue((L2CValue *)auStack496);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//     iVar3 = app::lua_bind::StatusModule__situation_kind_impl(param_1->moduleAccessor);
//     lib::L2CValue::L2CValue((L2CValue *)auStack496,iVar3);
//     lib::L2CValue::L2CValue((L2CValue *)&LStack144,_SITUATION_KIND_GROUND);
//     uVar6 = lib::L2CValue::operator==((L2CValue *)auStack496,(L2CValue *)&LStack144);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//     lib::L2CValue::~L2CValue((L2CValue *)auStack496);
//     if ((uVar6 & 1) != 0) {
//       GVar5 = lib::L2CValue::as_integer(aLStack176);
//       pvVar12 = (void *)app::lua_bind::GroundModule__get_touch_line_raw_impl
//                                   (param_1->moduleAccessor,GVar5);
//       if (pvVar12 == (void *)0x0) {
//         lib::L2CValue::L2CValue((L2CValue *)auStack496,(L2CValue *)&LUA_SCRIPT_LINE_STATUS_SHIFT);
//       }
//       else {
//         lib::L2CValue::L2CValue((L2CValue *)auStack496,pvVar12);
//       }
//       pGVar13 = (GroundCollisionLine *)lib::L2CValue::as_pointer((L2CValue *)auStack496);
//       bVar1 = app::sv_ground_collision_line::is_floor(pGVar13);
//       lib::L2CValue::L2CValue((L2CValue *)&LStack144,(bool)(bVar1 & 1));
//       bVar1 = lib::L2CValue::as_bool((L2CValue *)&LStack144);
//       app::lua_bind::GroundModule__set_attach_ground_impl(param_1->moduleAccessor,(bool)(bVar1 & 1))
//       ;
//       lib::L2CValue::~L2CValue((L2CValue *)&LStack144);
//       lib::L2CValue::~L2CValue((L2CValue *)auStack496);
//     }
//     lib::L2CValue::~L2CValue((L2CValue *)(auStack496 + 0x10));
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack464);
//     lib::L2CValue::~L2CValue(aLStack160);
//     lib::L2CValue::~L2CValue((L2CValue *)&LStack288);
//   }
//   lib::L2CValue::~L2CValue(aLStack272);
//   lib::L2CValue::~L2CValue(aLStack208);
// LAB_710000f40c:
//   lib::L2CValue::~L2CValue(aLStack192);
//   lib::L2CValue::~L2CValue(aLStack176);
//   return;
// }