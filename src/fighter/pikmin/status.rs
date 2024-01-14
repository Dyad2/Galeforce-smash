
//status stuff, pikmin goes through plats

// #[status_script(agent = "pikmin_pikmin", status = WEAPON_PIKMIN_PIKMIN_STATUS_KIND_PULL_OUT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn pikminpikmin_pullout_main(fighter: &mut L2CFighterCommon) -> L2CValue {

//     let ivar: i32 = 0;
//     let fvar: f32 = 0.0;

//     ivar = GroundModule::get_correct(fighter.module_accessor);

//     if ivar == *GROUND_CORRECT_KIND_NONE {
//         GroundModule::correct(fighter.module_accessor,smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR_TRANS));
//     }
//     if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
//         fighter.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_PULL_OUT_LANDING.into(), false.into());
//     }

//     fvar = PostureModule::pos_x(fighter.module_accessor);

//     if fvar == WorkModule::get_float(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_PULL_OUT_WORK_FLOAT_OWNER_X) {
        
//     }
//   fVar7 = (float)app::lua_bind::PostureModule__pos_x_impl(this->moduleAccessor);
//   lib::L2CValue::L2CValue((L2CValue *)local_50,fVar7,return_value_14);
//   lib::L2CValue::operator=(aLStack224,(L2CValue *)local_50);
//   lib::L2CValue::~L2CValue((L2CValue *)local_50);
//   lib::L2CValue::L2CValue
//             ((L2CValue *)local_60,_WEAPON_PIKMIN_PIKMIN_STATUS_PULL_OUT_WORK_FLOAT_OWNER_X,
//              (L2CValue *)0x84c8);
//   iVar2 = lib::L2CValue::as_integer((L2CValue *)local_60);
//   fVar7 = (float)app::lua_bind::WorkModule__get_float_impl(this->moduleAccessor,iVar2);
//   lib::L2CValue::L2CValue((L2CValue *)local_50,fVar7,return_value_15);
//   lib::L2CValue::operator=(aLStack208,(L2CValue *)local_50);
//   lib::L2CValue::~L2CValue((L2CValue *)local_50);
//   lib::L2CValue::~L2CValue((L2CValue *)local_60);
//   fVar7 = (float)app::lua_bind::PostureModule__lr_impl(this->moduleAccessor);
//   lib::L2CValue::L2CValue((L2CValue *)local_50,fVar7,return_value_16);
//   lib::L2CValue::operator=(aLStack160,(L2CValue *)local_50);
//   lib::L2CValue::~L2CValue((L2CValue *)local_50);
//   lib::L2CValue::L2CValue((L2CValue *)local_50,0,return_value_17);
//   uVar4 = lib::L2CValue::operator<(aLStack160,(L2CValue *)local_50);
//   lib::L2CValue::~L2CValue((L2CValue *)local_50);
//   pLVar6 = extraout_x8;
//   if (((uVar4 & 1) == 0) ||
//      (uVar4 = lib::L2CValue::operator<=(aLStack208,aLStack224), pLVar6 = extraout_x8_00,
//      (uVar4 & 1) == 0)) {
//     lib::L2CValue::L2CValue((L2CValue *)local_50,0,pLVar6);
//     uVar4 = lib::L2CValue::operator<=((L2CValue *)local_50,aLStack160);
//     lib::L2CValue::~L2CValue((L2CValue *)local_50);
//     pLVar6 = extraout_x8_01;
//     if (((uVar4 & 1) != 0) &&
//        (uVar4 = lib::L2CValue::operator<=(aLStack224,aLStack208), pLVar6 = extraout_x8_02,
//        (uVar4 & 1) != 0)) goto LAB_710004df3c;
//   }
//   else {
// LAB_710004df3c:
//     fVar7 = (float)app::lua_bind::PostureModule__pos_y_impl(this->moduleAccessor);
//     lib::L2CValue::L2CValue((L2CValue *)local_50,fVar7,return_value_18);
//     lib::L2CValue::operator=(aLStack176,(L2CValue *)local_50);
//     lib::L2CValue::~L2CValue((L2CValue *)local_50);
//     lib::L2CValue::L2CValue((L2CValue *)local_50,0.0,return_value_19);
//     lib::L2CValue::operator=(aLStack144,(L2CValue *)local_50);
//     lib::L2CValue::~L2CValue((L2CValue *)local_50);
//     lib::L2CValue::L2CValue
//               (aLStack256,_WEAPON_PIKMIN_PIKMIN_STATUS_PULL_OUT_WORK_FLOAT_GROUND_Y,
//                (L2CValue *)0x84c4);
//     iVar2 = lib::L2CValue::as_integer(aLStack256);
//     fVar7 = (float)app::lua_bind::WorkModule__get_float_impl(this->moduleAccessor,iVar2);
//     lib::L2CValue::L2CValue(aLStack240,fVar7,return_value_20);
//     lib::L2CValue::operator-(aLStack176,aLStack240,(L2CValue *)local_60);
//     lib::L2CValue::operator-((L2CValue *)local_60,(L2CValue *)local_50);
//     lib::L2CValue::operator=(aLStack128,(L2CValue *)local_50);
//     lib::L2CValue::~L2CValue((L2CValue *)local_50);
//     lib::L2CValue::~L2CValue((L2CValue *)local_60);
//     lib::L2CValue::~L2CValue(aLStack240);
//     lib::L2CValue::~L2CValue(aLStack256);
//     lib::L2CValue::L2CValue((L2CValue *)local_50,20.0,return_value_21);
//     lib::L2CValue::operator-(aLStack128,(L2CValue *)local_50,(L2CValue *)local_60);
//     lib::L2CValue::~L2CValue((L2CValue *)local_50);
//     lib::L2CValue::operator=(aLStack128,(L2CValue *)local_60);
//     lib::L2CValue::~L2CValue((L2CValue *)local_60);
//     lib::L2CValue::L2CValue(aLStack256,true);
//     uVar8 = lib::L2CValue::as_number(aLStack224);
//     uVar9 = lib::L2CValue::as_number(aLStack176);
//     local_50._0_8_ = CONCAT44(uVar9,uVar8);
//     local_50._8_8_ = 0;
//     uVar8 = lib::L2CValue::as_number(aLStack144);
//     uVar9 = lib::L2CValue::as_number(aLStack128);
//     local_60._0_8_ = CONCAT44(uVar9,uVar8);
//     local_60._8_8_ = 0;
//     bVar1 = lib::L2CValue::as_bool(aLStack256);
//     bVar1 = app::lua_bind::GroundModule__ray_check_impl
//                       (this->moduleAccessor,(Vector2f *)local_50,(Vector2f *)local_60,
//                        (bool)(bVar1 & 1));
//     lib::L2CValue::L2CValue(aLStack240,(bool)(bVar1 & 1));
//     lib::L2CValue::L2CValue((L2CValue *)local_50,false);
//     uVar4 = lib::L2CValue::operator==((L2CValue *)local_50,aLStack240);
//     lib::L2CValue::~L2CValue((L2CValue *)local_50);
//     lib::L2CValue::~L2CValue(aLStack240);
//     lib::L2CValue::~L2CValue(aLStack256);
//     pLVar6 = return_value_22;
//     if ((uVar4 & 1) != 0) {
//       lib::L2CValue::L2CValue
//                 ((L2CValue *)local_60,_KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN,return_value_22);
//       iVar2 = lib::L2CValue::as_integer((L2CValue *)local_60);
//       fVar7 = (float)app::lua_bind::KineticModule__get_sum_speed_x_impl(this->moduleAccessor,iVar2);
//       lib::L2CValue::L2CValue((L2CValue *)local_50,fVar7,return_value_23);
//       lib::L2CValue::operator=(aLStack192,(L2CValue *)local_50);
//       lib::L2CValue::~L2CValue((L2CValue *)local_50);
//       lib::L2CValue::~L2CValue((L2CValue *)local_60);
//       lib::L2CValue::operator-(aLStack224,aLStack192,(L2CValue *)local_50);
//       lib::L2CValue::operator=(aLStack224,(L2CValue *)local_50);
//       lib::L2CValue::~L2CValue((L2CValue *)local_50);
//       fVar7 = (float)app::lua_bind::PostureModule__pos_z_impl(this->moduleAccessor);
//       lib::L2CValue::L2CValue((L2CValue *)local_50,fVar7,return_value_24);
//       lib::L2CValue::operator=(aLStack112,(L2CValue *)local_50);
//       lib::L2CValue::~L2CValue((L2CValue *)local_50);
//       uVar8 = lib::L2CValue::as_number(aLStack224);
//       uVar9 = lib::L2CValue::as_number(aLStack176);
//       uVar10 = lib::L2CValue::as_number(aLStack112);
//       local_50._0_8_ = CONCAT44(uVar9,uVar8);
//       local_50._8_8_ = (ulong)uVar10;
//       app::lua_bind::PostureModule__set_pos_impl(this->moduleAccessor,(Vector3f *)local_50);
//       lib::L2CValue::L2CValue
//                 ((L2CValue *)local_50,_WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,return_value_25);
//       iVar2 = lib::L2CValue::as_integer((L2CValue *)local_50);
//       app::lua_bind::KineticModule__unable_energy_impl(this->moduleAccessor,iVar2);
//       lib::L2CValue::~L2CValue((L2CValue *)local_50);
//       pLVar6 = extraout_x8_03;
//     }
//   }
//   lib::L2CValue::L2CValue((L2CValue *)return_value,0,pLVar6);
// LAB_710004e1f4:
//   lib::L2CValue::~L2CValue(aLStack224);
//   lib::L2CValue::~L2CValue(aLStack208);
//   lib::L2CValue::~L2CValue(aLStack192);
//   lib::L2CValue::~L2CValue(aLStack176);
//   lib::L2CValue::~L2CValue(aLStack160);
//   lib::L2CValue::~L2CValue(aLStack144);
//   lib::L2CValue::~L2CValue(aLStack128);
//   lib::L2CValue::~L2CValue(aLStack112);
//   return;
// }
