use super::*;

unsafe extern "C" fn specials_setkind(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("game and watch!");

    if VarModule::is_flag(fighter.battle_object,commons::instance::flag::GALEFORCE_ATTACK_ON) {
        WorkModule::set_int(fighter.module_accessor, VarModule::get_int(fighter.battle_object, gamewatch::instance::int::JUDGE_STORED_KIND), *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND);
        //WorkModule::set_int64(
    }

    println!("gnw stored judge: {}", VarModule::get_int(fighter.battle_object, gamewatch::instance::int::JUDGE_STORED_KIND));
    println!("gnw special_s_kind: {}", WorkModule::get_int(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND));

    let ret = original!(fighter);

    return ret;
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Init, *FIGHTER_STATUS_KIND_SPECIAL_S, specials_setkind);
}

// // Rewrite this you NERD
// void __thiscall+
// L2CFighterGamewatch::status::init::SpecialS(L2CFighterGamewatch *this,L2CValue *return_value)

// {
//   Hash40Map *this_00;
//   bool bVar1;
//   byte bVar2;
//   int iVar3;
//   FighterEntryID FVar4;
//   int iVar5;
//   uint uVar6;
//   int iVar7;
//   Hash40 HVar8;
//   void *pvVar9;
//   FighterInformation *pFVar10;
//   L2CValue *pLVar11;
//   long lVar12;
//   long lVar13;
//   BattleObjectModuleAccessor *pBVar14;
//   L2CValue *pLVar15;
//   ulong uVar16;
//   ulong uVar17;
//   L2CValue *pLVar18;
//   L2CTable *pLVar19;
//   undefined8 *puVar20;
//   undefined8 *puVar21;
//   code *pcVar22;
//   BattleObjectModuleAccessor **ppBVar23;
//   float fVar24;
//   undefined8 uVar25;
//   L2CValue local_210 [2];
//   undefined ***local_1f0;
//   L2CValue LStack_1d8;
//   L2CValue LStack_1c8;
//   L2CValue LStack_1b8;
//   L2CValue LStack_1a8;
//   L2CValue LStack_198;
//   L2CValue LStack_188;
//   L2CValue LStack_178;
//   L2CValue LStack_168;
//   L2CValue LStack_158;
//   L2CValue LStack_148;
//   L2CValue LStack_138;
//   L2CValue LStack_128;
//   L2CValue LStack_118;
//   L2CValue LStack_108;
//   L2CValue LStack_f8;
//   L2CValue LStack_e8;
//   L2CValue LStack_d8;
//   L2CValue LStack_c8;
//   L2CValue LStack_b8;
//   L2CValue LStack_a8;
//   L2CValue LStack_98;
//   L2CValue LStack_88;
//   L2CValue LStack_78;
  
//   lib::L2CValue::L2CValue(local_210,_FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_PREV_KIND);
//   iVar3 = lib::L2CValue::as_integer(local_210);
//   ppBVar23 = &this->moduleAccessor;
//   iVar3 = app::lua_bind::WorkModule__get_int_impl(*ppBVar23,iVar3);
//   lib::L2CValue::L2CValue(&LStack_f8,iVar3);
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::L2CValue(local_210,_FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND);
//   iVar3 = lib::L2CValue::as_integer(local_210);
//   iVar3 = app::lua_bind::WorkModule__get_int_impl(*ppBVar23,iVar3);
//   lib::L2CValue::L2CValue(&LStack_108,iVar3);
//   lib::L2CValue::~L2CValue(local_210);
//   HVar8 = app::sv_global_parameter::melee_rule_mode();
//   lib::L2CValue::L2CValue(&LStack_118,HVar8);
//   lib::L2CValue::L2CValue(local_210,0x1dbcd0f8e6);
//   bVar1 = lib::L2CValue::operator==(&LStack_118,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   if (bVar1) {
//     lib::L2CValue::L2CValue(local_210,5);
//     lib::L2CValue::operator=(&LStack_108,(L2CValue *)local_210);
//     pLVar11 = local_210;
// LAB_7100005eb0:
//     lib::L2CValue::~L2CValue(pLVar11);
//   }
//   else {
//     lib::L2CValue::L2CValue(local_210,-1);
//     bVar1 = lib::L2CValue::operator==(&LStack_108,(L2CValue *)local_210);
//     lib::L2CValue::~L2CValue(local_210);
//     if (bVar1) {
//       lib::L2CValue::L2CValue(local_210,8);
//       bVar1 = lib::L2CValue::operator==(&LStack_f8,(L2CValue *)local_210);
//       lib::L2CValue::~L2CValue(local_210);
//       if (!bVar1) {
//         iVar3 = app::sv_information::get_remaining_time_as_frame();
//         lib::L2CValue::L2CValue(&LStack_78,iVar3);
//         lib::L2CValue::L2CValue(&LStack_a8,0xfea97fe73);
//         lib::L2CValue::L2CValue(&LStack_b8,0xfd91dd935);
//         uVar16 = lib::L2CValue::as_integer(&LStack_a8);
//         uVar17 = lib::L2CValue::as_integer(&LStack_b8);
//         iVar3 = app::lua_bind::WorkModule__get_param_int_impl(*ppBVar23,uVar16,uVar17);
//         lib::L2CValue::L2CValue(&LStack_98,iVar3);
//         lib::L2CValue::L2CValue(local_210,0x3c);
//         lib::L2CValue::operator*((L2CValue *)&LStack_98,(L2CValue *)local_210,&LStack_88);
//         lib::L2CValue::~L2CValue(local_210);
//         lib::L2CValue::~L2CValue(&LStack_98);
//         lib::L2CValue::~L2CValue(&LStack_b8);
//         lib::L2CValue::~L2CValue(&LStack_a8);
//         lib::L2CValue::L2CValue(local_210,0);
//         uVar16 = lib::L2CValue::operator<(local_210,(L2CValue *)&LStack_78);
//         lib::L2CValue::~L2CValue(local_210);
//         if (((uVar16 & 1) != 0) &&
//            (uVar16 = lib::L2CValue::operator<=(&LStack_78,(L2CValue *)&LStack_88), (uVar16 & 1) != 0
//            )) {
//           lib::L2CValue::L2CValue(local_210,_FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
//           iVar3 = lib::L2CValue::as_integer(local_210);
//           iVar3 = app::lua_bind::WorkModule__get_int_impl(*ppBVar23,iVar3);
//           lib::L2CValue::L2CValue(&LStack_98,iVar3);
//           lib::L2CValue::~L2CValue(local_210);
//           FVar4 = lib::L2CValue::as_integer(&LStack_98);
//           pvVar9 = (void *)app::lua_bind::FighterManager__get_fighter_information_impl
//                                      (LUA_SCRIPT_LINE_MAP_CORRECTION,FVar4);
//           lib::L2CValue::L2CValue(&LStack_a8,pvVar9);
//           lib::L2CValue::L2CValue(local_210,0);
//           iVar3 = lib::L2CValue::as_integer(local_210);
//           iVar3 = app::lua_bind::FighterManager__get_top_rank_player_impl
//                             (LUA_SCRIPT_LINE_MAP_CORRECTION,iVar3);
//           lib::L2CValue::L2CValue(&LStack_b8,iVar3);
//           lib::L2CValue::~L2CValue(local_210);
//           FVar4 = lib::L2CValue::as_integer(&LStack_b8);
//           pvVar9 = (void *)app::lua_bind::FighterManager__get_fighter_information_impl
//                                      (LUA_SCRIPT_LINE_MAP_CORRECTION,FVar4);
//           lib::L2CValue::L2CValue(&LStack_c8,pvVar9);
//           HVar8 = app::sv_global_parameter::melee_rule_type();
//           lib::L2CValue::L2CValue(&LStack_d8,HVar8);
//           lib::L2CValue::L2CValue(&LStack_e8,false);
//           lib::L2CValue::L2CValue(local_210,0x16586570f3);
//           bVar1 = lib::L2CValue::operator==(&LStack_d8,(L2CValue *)local_210);
//           lib::L2CValue::~L2CValue(local_210);
//           if (bVar1) {
//             bVar1 = lib::L2CValue::operator==(&LStack_98,(L2CValue *)&LStack_b8);
//             if (bVar1) {
//               lib::L2CValue::L2CValue(local_210,false);
//               lib::L2CValue::operator=(&LStack_e8,(L2CValue *)local_210);
// LAB_7100005480:
//               pLVar11 = local_210;
//             }
//             else {
//               pFVar10 = (FighterInformation *)lib::L2CValue::as_pointer(&LStack_a8);
//               iVar3 = app::lua_bind::FighterInformation__total_beat_count_impl(pFVar10,-2);
//               lib::L2CValue::L2CValue(local_210,iVar3);
//               pFVar10 = (FighterInformation *)lib::L2CValue::as_pointer(&LStack_a8);
//               uVar6 = app::lua_bind::FighterInformation__suicide_count_impl(pFVar10,-2);
//               lib::L2CValue::L2CValue(&LStack_138,uVar6 & 0xffff);
//               lib::L2CValue::operator-((L2CValue *)local_210,(L2CValue *)&LStack_138,&LStack_128);
//               lib::L2CValue::~L2CValue(&LStack_138);
//               lib::L2CValue::~L2CValue(local_210);
//               pFVar10 = (FighterInformation *)lib::L2CValue::as_pointer(&LStack_c8);
//               iVar3 = app::lua_bind::FighterInformation__total_beat_count_impl(pFVar10,-2);
//               lib::L2CValue::L2CValue(local_210,iVar3);
//               pFVar10 = (FighterInformation *)lib::L2CValue::as_pointer(&LStack_c8);
//               uVar6 = app::lua_bind::FighterInformation__suicide_count_impl(pFVar10,-2);
//               lib::L2CValue::L2CValue(&LStack_148,uVar6 & 0xffff);
//               lib::L2CValue::operator-((L2CValue *)local_210,(L2CValue *)&LStack_148,&LStack_138);
//               lib::L2CValue::~L2CValue(&LStack_148);
//               lib::L2CValue::~L2CValue(local_210);
//               lib::L2CValue::L2CValue(local_210,0);
//               bVar1 = lib::L2CValue::operator==(&LStack_128,(L2CValue *)local_210);
//               lib::L2CValue::~L2CValue(local_210);
//               if (!bVar1) {
//                 lib::L2CValue::L2CValue(local_210,0);
//                 uVar16 = lib::L2CValue::operator<=(&LStack_128,(L2CValue *)local_210);
//                 lib::L2CValue::~L2CValue(local_210);
//                 if ((uVar16 & 1) == 0) {
//                   lib::L2CValue::L2CValue(local_210,2);
//                   lib::L2CValue::operator*
//                             ((L2CValue *)local_210,(L2CValue *)&LStack_128,&LStack_148);
//                   lib::L2CValue::~L2CValue(local_210);
//                   bVar2 = lib::L2CValue::operator<=(&LStack_148,(L2CValue *)&LStack_138);
//                   lib::L2CValue::L2CValue(local_210,(bool)(bVar2 & 1));
//                   lib::L2CValue::operator=(&LStack_e8,(L2CValue *)local_210);
//                 }
//                 else {
//                   lib::L2CValue::L2CValue(local_210,0.5);
//                   lib::L2CValue::operator*
//                             ((L2CValue *)local_210,(L2CValue *)&LStack_128,&LStack_148);
//                   lib::L2CValue::~L2CValue(local_210);
//                   bVar2 = lib::L2CValue::operator<=(&LStack_148,(L2CValue *)&LStack_138);
//                   lib::L2CValue::L2CValue(local_210,(bool)(bVar2 & 1));
//                   lib::L2CValue::operator=(&LStack_e8,(L2CValue *)local_210);
//                 }
//                 goto LAB_7100005950;
//               }
//               lib::L2CValue::L2CValue(local_210,0);
//               bVar2 = lib::L2CValue::operator<(local_210,(L2CValue *)&LStack_138);
//               lib::L2CValue::~L2CValue(local_210);
//               lib::L2CValue::L2CValue(local_210,(bool)(bVar2 & 1));
//               lib::L2CValue::operator=(&LStack_e8,(L2CValue *)local_210);
//               pLVar11 = local_210;
// LAB_710000595c:
//               lib::L2CValue::~L2CValue(pLVar11);
//               lib::L2CValue::~L2CValue(&LStack_138);
//               pLVar11 = &LStack_128;
//             }
//             lib::L2CValue::~L2CValue(pLVar11);
//           }
//           else {
//             lib::L2CValue::L2CValue(local_210,0x176903b131);
//             bVar1 = lib::L2CValue::operator==(&LStack_d8,(L2CValue *)local_210);
//             lib::L2CValue::~L2CValue(local_210);
//             if (bVar1) {
//               bVar1 = lib::L2CValue::operator==(&LStack_98,(L2CValue *)&LStack_b8);
//               if (bVar1) {
//                 lib::L2CValue::L2CValue(local_210,false);
//                 lib::L2CValue::operator=(&LStack_e8,(L2CValue *)local_210);
//                 goto LAB_7100005480;
//               }
//               pFVar10 = (FighterInformation *)lib::L2CValue::as_pointer(&LStack_a8);
//               iVar3 = app::lua_bind::FighterInformation__stock_count_impl(pFVar10);
//               lib::L2CValue::L2CValue(&LStack_128,iVar3);
//               pFVar10 = (FighterInformation *)lib::L2CValue::as_pointer(&LStack_c8);
//               iVar3 = app::lua_bind::FighterInformation__stock_count_impl(pFVar10);
//               lib::L2CValue::L2CValue(&LStack_138,iVar3);
//               lib::L2CValue::L2CValue(local_210,2);
//               lib::L2CValue::operator*((L2CValue *)local_210,(L2CValue *)&LStack_128,&LStack_148);
//               lib::L2CValue::~L2CValue(local_210);
//               bVar2 = lib::L2CValue::operator<=(&LStack_148,(L2CValue *)&LStack_138);
//               lib::L2CValue::L2CValue(local_210,(bool)(bVar2 & 1));
//               lib::L2CValue::operator=(&LStack_e8,(L2CValue *)local_210);
// LAB_7100005950:
//               pLVar11 = local_210;
// LAB_7100005954:
//               lib::L2CValue::~L2CValue(pLVar11);
//               pLVar11 = &LStack_148;
//               goto LAB_710000595c;
//             }
//             lib::L2CValue::L2CValue(local_210,0x14b677f115);
//             bVar1 = lib::L2CValue::operator==(&LStack_d8,(L2CValue *)local_210);
//             lib::L2CValue::~L2CValue(local_210);
//             if (bVar1) {
//               pFVar10 = (FighterInformation *)lib::L2CValue::as_pointer(&LStack_a8);
//               iVar3 = app::lua_bind::FighterInformation__stock_count_impl(pFVar10);
//               lib::L2CValue::L2CValue(&LStack_128,iVar3);
//               pFVar10 = (FighterInformation *)lib::L2CValue::as_pointer(&LStack_c8);
//               iVar3 = app::lua_bind::FighterInformation__stock_count_impl(pFVar10);
//               lib::L2CValue::L2CValue(&LStack_138,iVar3);
//               lib::L2CValue::L2CValue(local_210,1);
//               uVar16 = lib::L2CValue::operator<(local_210,(L2CValue *)&LStack_138);
//               lib::L2CValue::~L2CValue(local_210);
//               if ((uVar16 & 1) != 0) {
//                 lib::L2CValue::L2CValue(local_210,2);
//                 lib::L2CValue::operator*((L2CValue *)local_210,(L2CValue *)&LStack_128,&LStack_148);
//                 lib::L2CValue::~L2CValue(local_210);
//                 bVar2 = lib::L2CValue::operator<=(&LStack_148,(L2CValue *)&LStack_138);
//                 lib::L2CValue::L2CValue(local_210,(bool)(bVar2 & 1));
//                 lib::L2CValue::operator=(&LStack_e8,(L2CValue *)local_210);
//                 goto LAB_7100005950;
//               }
//               pFVar10 = (FighterInformation *)lib::L2CValue::as_pointer(&LStack_a8);
//               fVar24 = (float)app::lua_bind::FighterInformation__hit_point_impl(pFVar10);
//               lib::L2CValue::L2CValue(&LStack_148,fVar24);
//               pFVar10 = (FighterInformation *)lib::L2CValue::as_pointer(&LStack_c8);
//               fVar24 = (float)app::lua_bind::FighterInformation__hit_point_impl(pFVar10);
//               lib::L2CValue::L2CValue(&LStack_158,fVar24);
//               bVar1 = lib::L2CValue::operator==(&LStack_98,(L2CValue *)&LStack_b8);
//               if (bVar1) {
//                 uVar6 = app::lua_bind::FighterManager__get_top_rank_player_num_impl
//                                   (LUA_SCRIPT_LINE_MAP_CORRECTION);
//                 lib::L2CValue::L2CValue(&LStack_168,uVar6 & 0xff);
//                 lib::L2CValue::L2CValue(local_210,1);
//                 lib::L2CValue::operator-((L2CValue *)&LStack_168,(L2CValue *)local_210,&LStack_178);
//                 lib::L2CValue::~L2CValue(local_210);
//                 iVar3 = lib::L2CValue::as_integer(&LStack_178);
//                 lib::L2CValue::~L2CValue(&LStack_178);
//                 if (0 < iVar3) {
//                   iVar7 = 0;
//                   do {
//                     lib::L2CValue::L2CValue(&LStack_178,iVar7 + 1);
//                     iVar5 = lib::L2CValue::as_integer(&LStack_178);
//                     iVar5 = app::lua_bind::FighterManager__get_top_rank_player_impl
//                                       (LUA_SCRIPT_LINE_MAP_CORRECTION,iVar5);
//                     lib::L2CValue::L2CValue(local_210,iVar5);
//                     lib::L2CValue::~L2CValue(&LStack_178);
//                     FVar4 = lib::L2CValue::as_integer(local_210);
//                     pvVar9 = (void *)app::lua_bind::FighterManager__get_fighter_information_impl
//                                                (LUA_SCRIPT_LINE_MAP_CORRECTION,FVar4);
//                     lib::L2CValue::L2CValue(&LStack_178,pvVar9);
//                     pFVar10 = (FighterInformation *)lib::L2CValue::as_pointer(&LStack_178);
//                     fVar24 = (float)app::lua_bind::FighterInformation__hit_point_impl(pFVar10);
//                     lib::L2CValue::L2CValue(&LStack_188,fVar24);
//                     uVar16 = lib::L2CValue::operator<(&LStack_158,(L2CValue *)&LStack_188);
//                     if ((uVar16 & 1) != 0) {
//                       lib::L2CValue::operator=(&LStack_158,(L2CValue *)&LStack_188);
//                     }
//                     lib::L2CValue::~L2CValue(&LStack_188);
//                     lib::L2CValue::~L2CValue(&LStack_178);
//                     lib::L2CValue::~L2CValue(local_210);
//                     iVar7 = iVar7 + 1;
//                   } while (iVar7 < iVar3);
//                 }
//                 lib::L2CValue::~L2CValue(&LStack_168);
//               }
//               lib::L2CValue::L2CValue(local_210,2);
//               lib::L2CValue::operator*((L2CValue *)local_210,(L2CValue *)&LStack_148,&LStack_168);
//               lib::L2CValue::~L2CValue(local_210);
//               bVar2 = lib::L2CValue::operator<=(&LStack_168,(L2CValue *)&LStack_158);
//               lib::L2CValue::L2CValue(local_210,(bool)(bVar2 & 1));
//               lib::L2CValue::operator=(&LStack_e8,(L2CValue *)local_210);
//               lib::L2CValue::~L2CValue(local_210);
//               lib::L2CValue::~L2CValue(&LStack_168);
//               pLVar11 = &LStack_158;
//               goto LAB_7100005954;
//             }
//           }
//           bVar1 = lib::L2CValue::operator.cast.to.bool(&LStack_e8);
//           if (bVar1) {
//             lib::L2CValue::L2CValue(local_210,0xfea97fe73);
//             lib::L2CValue::L2CValue(&LStack_138,0xa6fcf1cd0);
//             uVar16 = lib::L2CValue::as_integer(local_210);
//             uVar17 = lib::L2CValue::as_integer(&LStack_138);
//             iVar3 = app::lua_bind::WorkModule__get_param_int_impl(*ppBVar23,uVar16,uVar17);
//             lib::L2CValue::L2CValue(&LStack_128,iVar3);
//             lib::L2CValue::~L2CValue(&LStack_138);
//             lib::L2CValue::~L2CValue(local_210);
//             lib::L2CValue::L2CValue(local_210,0x77a08c3fc);
//             lib::L2CValue::L2CValue(&LStack_148,100);
//             lib::L2CValue::as_hash(local_210);
//             lib::L2CValue::as_integer(&LStack_148);
//             uVar6 = app::sv_math::rand();
//             lib::L2CValue::L2CValue(&LStack_138,uVar6);
//             lib::L2CValue::~L2CValue(&LStack_148);
//             lib::L2CValue::~L2CValue(local_210);
//             uVar16 = lib::L2CValue::operator<(&LStack_138,(L2CValue *)&LStack_128);
//             if ((uVar16 & 1) != 0) {
//               lib::L2CValue::L2CValue(local_210,8);
//               lib::L2CValue::operator=(&LStack_108,(L2CValue *)local_210);
//               lib::L2CValue::~L2CValue(local_210);
//             }
//             lib::L2CValue::~L2CValue(&LStack_138);
//             lib::L2CValue::~L2CValue(&LStack_128);
//           }
//           lib::L2CValue::~L2CValue(&LStack_e8);
//           lib::L2CValue::~L2CValue(&LStack_d8);
//           lib::L2CValue::~L2CValue(&LStack_c8);
//           lib::L2CValue::~L2CValue(&LStack_b8);
//           lib::L2CValue::~L2CValue(&LStack_a8);
//           lib::L2CValue::~L2CValue(&LStack_98);
//         }
//         lib::L2CValue::~L2CValue(&LStack_88);
//         lib::L2CValue::~L2CValue(&LStack_78);
//       }
//     }
//     lib::L2CValue::L2CValue(local_210,-1);
//     bVar1 = lib::L2CValue::operator==(&LStack_108,(L2CValue *)local_210);
//     lib::L2CValue::~L2CValue(local_210);
//     if (bVar1) {
//       if (((DAT_7100159a98 & 1) == 0) && (iVar3 = __cxa_guard_acquire(&DAT_7100159a98), iVar3 != 0))
//       {
//         pLVar19 = (L2CTable *)operator.new(0x48);
//         lib::L2CTable::L2CTable(pLVar19,9);
//         lib::L2CValue::L2CValue((L2CValue *)&DAT_7100159a38,pLVar19);
//         lib::L2CValue::L2CValue(local_210,0x7b5666e52);
//         lib::L2CValue::L2CValue(&LStack_78,0x72c6f3fe8);
//         lib::L2CValue::L2CValue(&LStack_88,0x75b680f7e);
//         lib::L2CValue::L2CValue(&LStack_98,0x7c50c9add);
//         lib::L2CValue::L2CValue(&LStack_a8,0x7b20baa4b);
//         lib::L2CValue::L2CValue(&LStack_b8,0x72b02fbf1);
//         lib::L2CValue::L2CValue(&LStack_c8,0x75c05cb67);
//         lib::L2CValue::L2CValue(&LStack_d8,0x7ccbad6f6);
//         lib::L2CValue::L2CValue(&LStack_e8,0x7bbbde660);
//         FUN_7100008560(&DAT_7100159a38,local_210,&LStack_78,&LStack_88,&LStack_98,&LStack_a8,
//                        &LStack_b8,&LStack_c8,&LStack_d8,&LStack_e8);
//         lib::L2CValue::~L2CValue(&LStack_e8);
//         lib::L2CValue::~L2CValue(&LStack_d8);
//         lib::L2CValue::~L2CValue(&LStack_c8);
//         lib::L2CValue::~L2CValue(&LStack_b8);
//         lib::L2CValue::~L2CValue(&LStack_a8);
//         lib::L2CValue::~L2CValue(&LStack_98);
//         lib::L2CValue::~L2CValue(&LStack_88);
//         lib::L2CValue::~L2CValue(&LStack_78);
//         lib::L2CValue::~L2CValue(local_210);
//         FUN_7100000300(lib::L2CValue::~L2CValue,&DAT_7100159a38,&PTR_LOOP_7100157000);
//         __cxa_guard_release(&DAT_7100159a98);
//       }
//       pLVar19 = (L2CTable *)operator.new(0x48);
//       lib::L2CTable::L2CTable(pLVar19,0);
//       lib::L2CValue::L2CValue(&LStack_78,pLVar19);
//       pLVar19 = (L2CTable *)operator.new(0x48);
//       lib::L2CTable::L2CTable(pLVar19,0);
//       lib::L2CValue::L2CValue(&LStack_88,pLVar19);
//       lib::L2CValue::L2CValue(&LStack_98,0);
//       lib::L2CValue::L2CValue(&LStack_a8,0);
//       uVar6 = 0;
//       do {
//         lib::L2CValue::L2CValue(&LStack_b8,0);
//         lib::L2CValue::L2CValue(local_210,uVar6);
//         bVar1 = lib::L2CValue::operator==(local_210,(L2CValue *)&LStack_f8);
//         lib::L2CValue::~L2CValue(local_210);
//         if (!bVar1) {
//           lib::L2CValue::L2CValue(&LStack_c8,0xfea97fe73);
//           pLVar11 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)&DAT_7100159a38,uVar6 + 1);
//           uVar16 = lib::L2CValue::as_integer(&LStack_c8);
//           uVar17 = lib::L2CValue::as_integer(pLVar11);
//           iVar3 = app::lua_bind::WorkModule__get_param_int_impl(*ppBVar23,uVar16,uVar17);
//           lib::L2CValue::L2CValue(local_210,iVar3);
//           lib::L2CValue::operator=(&LStack_b8,(L2CValue *)local_210);
//           lib::L2CValue::~L2CValue(local_210);
//           lib::L2CValue::~L2CValue(&LStack_c8);
//         }
//         if (uVar6 == 6) {
//           lib::L2CValue::L2CValue
//                     (&LStack_d8,
//                      _FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_PANEL_7_INVALID_FRAME);
//           iVar3 = lib::L2CValue::as_integer(&LStack_d8);
//           iVar3 = app::lua_bind::WorkModule__get_int_impl(*ppBVar23,iVar3);
//           lib::L2CValue::L2CValue(&LStack_c8,iVar3);
//           lib::L2CValue::L2CValue(local_210,0);
//           uVar16 = lib::L2CValue::operator<(local_210,(L2CValue *)&LStack_c8);
//           lib::L2CValue::~L2CValue(local_210);
//           lib::L2CValue::~L2CValue(&LStack_c8);
//           lib::L2CValue::~L2CValue(&LStack_d8);
//           if ((uVar16 & 1) != 0) {
//             lib::L2CValue::L2CValue(local_210,0);
//             lib::L2CValue::operator=(&LStack_b8,(L2CValue *)local_210);
//             lib::L2CValue::~L2CValue(local_210);
//           }
//         }
//         lib::L2CValue::L2CValue(local_210,0);
//         uVar16 = lib::L2CValue::operator<(local_210,(L2CValue *)&LStack_b8);
//         lib::L2CValue::~L2CValue(local_210);
//         if ((uVar16 & 1) != 0) {
//           lib::L2CValue::operator+(&LStack_98,(L2CValue *)&LStack_b8);
//           lib::L2CValue::operator=(&LStack_98,(L2CValue *)local_210);
//           lib::L2CValue::~L2CValue(local_210);
//           lib::L2CValue::L2CValue(local_210,1);
//           lib::L2CValue::operator+(&LStack_a8,(L2CValue *)local_210);
//           lib::L2CValue::~L2CValue(local_210);
//           pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,(L2CValue *)&LStack_c8);
//           lib::L2CValue::L2CValue(local_210,uVar6);
//           lib::L2CValue::operator=(pLVar11,(L2CValue *)local_210);
//           lib::L2CValue::~L2CValue(local_210);
//           lib::L2CValue::~L2CValue(&LStack_c8);
//           lib::L2CValue::L2CValue(local_210,1);
//           lib::L2CValue::operator+(&LStack_a8,(L2CValue *)local_210);
//           lib::L2CValue::~L2CValue(local_210);
//           pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_88,(L2CValue *)&LStack_c8);
//           lib::L2CValue::operator=(pLVar11,(L2CValue *)&LStack_98);
//           lib::L2CValue::~L2CValue(&LStack_c8);
//           lib::L2CValue::L2CValue(local_210,1);
//           lib::L2CValue::operator+(&LStack_a8,(L2CValue *)local_210);
//           lib::L2CValue::~L2CValue(local_210);
//           lib::L2CValue::operator=(&LStack_a8,(L2CValue *)&LStack_c8);
//           lib::L2CValue::~L2CValue(&LStack_c8);
//         }
//         lib::L2CValue::~L2CValue(&LStack_b8);
//         uVar6 = uVar6 + 1;
//       } while (uVar6 < 9);
//       lib::L2CValue::L2CValue(local_210,0x77a08c3fc);
//       lib::L2CValue::as_hash(local_210);
//       lib::L2CValue::as_integer(&LStack_98);
//       uVar6 = app::sv_math::rand();
//       lib::L2CValue::L2CValue(&LStack_b8,uVar6);
//       lib::L2CValue::~L2CValue(local_210);
//       lib::L2CValue::L2CValue(local_210,1);
//       lib::L2CValue::operator-((L2CValue *)&LStack_a8,(L2CValue *)local_210,&LStack_c8);
//       lib::L2CValue::~L2CValue(local_210);
//       iVar7 = lib::L2CValue::as_integer(&LStack_c8);
//       lib::L2CValue::~L2CValue(&LStack_c8);
//       iVar3 = 0;
//       do {
//         if (iVar7 < iVar3) goto LAB_7100005e8c;
//         iVar3 = iVar3 + 1;
//         pLVar15 = (L2CValue *)lib::L2CValue::operator[](&LStack_88,iVar3);
//         uVar16 = lib::L2CValue::operator<(&LStack_b8,pLVar15);
//       } while ((uVar16 & 1) == 0);
//       pLVar15 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,iVar3);
//       lib::L2CValue::operator=(&LStack_108,pLVar15);
// LAB_7100005e8c:
//       lib::L2CValue::~L2CValue(&LStack_b8);
//       lib::L2CValue::~L2CValue(&LStack_a8);
//       lib::L2CValue::~L2CValue(&LStack_98);
//       lib::L2CValue::~L2CValue(&LStack_88);
//       pLVar11 = &LStack_78;
//       goto LAB_7100005eb0;
//     }
//   }
//   if (((DAT_7100159aa0 & 1) == 0) && (iVar3 = __cxa_guard_acquire(&DAT_7100159aa0), iVar3 != 0)) {
//     pLVar19 = (L2CTable *)operator.new(0x48);
//     lib::L2CTable::L2CTable(pLVar19,9);
//     lib::L2CValue::L2CValue((L2CValue *)&DAT_7100159a48,pLVar19);
//     lib::L2CValue::L2CValue(local_210,0xb9afea4ec);
//     lib::L2CValue::L2CValue(&LStack_78,0xb03f7f556);
//     lib::L2CValue::L2CValue(&LStack_88,0xb74f0c5c0);
//     lib::L2CValue::L2CValue(&LStack_98,0xbea945063);
//     lib::L2CValue::L2CValue(&LStack_a8,0xb9d9360f5);
//     lib::L2CValue::L2CValue(&LStack_b8,0xb049a314f);
//     lib::L2CValue::L2CValue(&LStack_c8,0xb739d01d9);
//     lib::L2CValue::L2CValue(&LStack_d8,0xbe3221c48);
//     lib::L2CValue::L2CValue(&LStack_e8,0xb94252cde);
//     FUN_7100008560(&DAT_7100159a48,local_210,&LStack_78,&LStack_88,&LStack_98,&LStack_a8,&LStack_b8,
//                    &LStack_c8,&LStack_d8,&LStack_e8);
//     lib::L2CValue::~L2CValue(&LStack_e8);
//     lib::L2CValue::~L2CValue(&LStack_d8);
//     lib::L2CValue::~L2CValue(&LStack_c8);
//     lib::L2CValue::~L2CValue(&LStack_b8);
//     lib::L2CValue::~L2CValue(&LStack_a8);
//     lib::L2CValue::~L2CValue(&LStack_98);
//     lib::L2CValue::~L2CValue(&LStack_88);
//     lib::L2CValue::~L2CValue(&LStack_78);
//     lib::L2CValue::~L2CValue(local_210);
//     FUN_7100000300(lib::L2CValue::~L2CValue,&DAT_7100159a48,&PTR_LOOP_7100157000);
//     __cxa_guard_release(&DAT_7100159aa0);
//   }
//   if (((DAT_7100159aa8 & 1) == 0) && (iVar3 = __cxa_guard_acquire(&DAT_7100159aa8), iVar3 != 0)) {
//     pLVar19 = (L2CTable *)operator.new(0x48);
//     lib::L2CTable::L2CTable(pLVar19,9);
//     lib::L2CValue::L2CValue((L2CValue *)&DAT_7100159a58,pLVar19);
//     lib::L2CValue::L2CValue(local_210,0xf6d6ca1db);
//     lib::L2CValue::L2CValue(&LStack_78,0xff465f061);
//     lib::L2CValue::L2CValue(&LStack_88,0xf8362c0f7);
//     lib::L2CValue::L2CValue(&LStack_98,0xf1d065554);
//     lib::L2CValue::L2CValue(&LStack_a8,0xf6a0165c2);
//     lib::L2CValue::L2CValue(&LStack_b8,0xff3083478);
//     lib::L2CValue::L2CValue(&LStack_c8,0xf840f04ee);
//     lib::L2CValue::L2CValue(&LStack_d8,0xf14b0197f);
//     lib::L2CValue::L2CValue(&LStack_e8,0xf63b729e9);
//     FUN_7100008560(&DAT_7100159a58,local_210,&LStack_78,&LStack_88,&LStack_98,&LStack_a8,&LStack_b8,
//                    &LStack_c8,&LStack_d8,&LStack_e8);
//     lib::L2CValue::~L2CValue(&LStack_e8);
//     lib::L2CValue::~L2CValue(&LStack_d8);
//     lib::L2CValue::~L2CValue(&LStack_c8);
//     lib::L2CValue::~L2CValue(&LStack_b8);
//     lib::L2CValue::~L2CValue(&LStack_a8);
//     lib::L2CValue::~L2CValue(&LStack_98);
//     lib::L2CValue::~L2CValue(&LStack_88);
//     lib::L2CValue::~L2CValue(&LStack_78);
//     lib::L2CValue::~L2CValue(local_210);
//     FUN_7100000300(lib::L2CValue::~L2CValue,&DAT_7100159a58,&PTR_LOOP_7100157000);
//     __cxa_guard_release(&DAT_7100159aa8);
//   }
//   if (((DAT_7100159ab0 & 1) == 0) && (iVar3 = __cxa_guard_acquire(&DAT_7100159ab0), iVar3 != 0)) {
//     pLVar19 = (L2CTable *)operator.new(0x48);
//     lib::L2CTable::L2CTable(pLVar19,9);
//     lib::L2CValue::L2CValue((L2CValue *)&DAT_7100159a68,pLVar19);
//     lib::L2CValue::L2CValue(local_210,0xb9afea4ec);
//     lib::L2CValue::L2CValue(&LStack_78,0xb03f7f556);
//     lib::L2CValue::L2CValue(&LStack_88,0xb74f0c5c0);
//     lib::L2CValue::L2CValue(&LStack_98,0xbea945063);
//     lib::L2CValue::L2CValue(&LStack_a8,0xb9d9360f5);
//     lib::L2CValue::L2CValue(&LStack_b8,0xb049a314f);
//     lib::L2CValue::L2CValue(&LStack_c8,0xb739d01d9);
//     lib::L2CValue::L2CValue(&LStack_d8,0xbe3221c48);
//     lib::L2CValue::L2CValue(&LStack_e8,0xb94252cde);
//     FUN_7100008560(&DAT_7100159a68,local_210,&LStack_78,&LStack_88,&LStack_98,&LStack_a8,&LStack_b8,
//                    &LStack_c8,&LStack_d8,&LStack_e8);
//     lib::L2CValue::~L2CValue(&LStack_e8);
//     lib::L2CValue::~L2CValue(&LStack_d8);
//     lib::L2CValue::~L2CValue(&LStack_c8);
//     lib::L2CValue::~L2CValue(&LStack_b8);
//     lib::L2CValue::~L2CValue(&LStack_a8);
//     lib::L2CValue::~L2CValue(&LStack_98);
//     lib::L2CValue::~L2CValue(&LStack_88);
//     lib::L2CValue::~L2CValue(&LStack_78);
//     lib::L2CValue::~L2CValue(local_210);
//     FUN_7100000300(lib::L2CValue::~L2CValue,&DAT_7100159a68,&PTR_LOOP_7100157000);
//     __cxa_guard_release(&DAT_7100159ab0);
//   }
//   if (((DAT_7100159ab8 & 1) == 0) && (iVar3 = __cxa_guard_acquire(&DAT_7100159ab8), iVar3 != 0)) {
//     pLVar19 = (L2CTable *)operator.new(0x48);
//     lib::L2CTable::L2CTable(pLVar19,9);
//     lib::L2CValue::L2CValue((L2CValue *)&DAT_7100159a78,pLVar19);
//     lib::L2CValue::L2CValue(local_210,0xf6d6ca1db);
//     lib::L2CValue::L2CValue(&LStack_78,0xff465f061);
//     lib::L2CValue::L2CValue(&LStack_88,0xf8362c0f7);
//     lib::L2CValue::L2CValue(&LStack_98,0xf1d065554);
//     lib::L2CValue::L2CValue(&LStack_a8,0xf6a0165c2);
//     lib::L2CValue::L2CValue(&LStack_b8,0xff3083478);
//     lib::L2CValue::L2CValue(&LStack_c8,0xf840f04ee);
//     lib::L2CValue::L2CValue(&LStack_d8,0xf14b0197f);
//     lib::L2CValue::L2CValue(&LStack_e8,0xf63b729e9);
//     FUN_7100008560(&DAT_7100159a78,local_210,&LStack_78,&LStack_88,&LStack_98,&LStack_a8,&LStack_b8,
//                    &LStack_c8,&LStack_d8,&LStack_e8);
//     lib::L2CValue::~L2CValue(&LStack_e8);
//     lib::L2CValue::~L2CValue(&LStack_d8);
//     lib::L2CValue::~L2CValue(&LStack_c8);
//     lib::L2CValue::~L2CValue(&LStack_b8);
//     lib::L2CValue::~L2CValue(&LStack_a8);
//     lib::L2CValue::~L2CValue(&LStack_98);
//     lib::L2CValue::~L2CValue(&LStack_88);
//     lib::L2CValue::~L2CValue(&LStack_78);
//     lib::L2CValue::~L2CValue(local_210);
//     FUN_7100000300(lib::L2CValue::~L2CValue,&DAT_7100159a78,&PTR_LOOP_7100157000);
//     __cxa_guard_release(&DAT_7100159ab8);
//   }
//   lib::L2CValue::L2CValue(local_210,_FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND);
//   iVar3 = lib::L2CValue::as_integer(&LStack_108);
//   iVar7 = lib::L2CValue::as_integer(local_210);
//   app::lua_bind::WorkModule__set_int_impl(*ppBVar23,iVar3,iVar7);
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::L2CValue(local_210,_FIGHTER_INSTANCE_WORK_ID_INT_TRICK_SUB);
//   iVar3 = lib::L2CValue::as_integer(&LStack_108);
//   iVar7 = lib::L2CValue::as_integer(local_210);
//   app::lua_bind::WorkModule__set_int_impl(*ppBVar23,iVar3,iVar7);
//   lib::L2CValue::~L2CValue(local_210);
//   fVar24 = app::lua_bind::PostureModule__lr_impl(*ppBVar23);
//   lib::L2CValue::L2CValue(&LStack_78,fVar24);
//   lib::L2CValue::L2CValue(local_210,-1.0);
//   bVar1 = lib::L2CValue::operator==(&LStack_78,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::~L2CValue(&LStack_78);
//   if (bVar1) {
//     lib::L2CValue::L2CValue(local_210,1);
//     lib::L2CValue::operator+(&LStack_108,(L2CValue *)local_210);
//     lib::L2CValue::~L2CValue(local_210);
//     pLVar11 = (L2CValue *)
//               lib::L2CValue::operator[]((L2CValue *)&DAT_7100159a68,(L2CValue *)&LStack_78);
//     lib::L2CValue::L2CValue(local_210,_FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
//     lVar12 = lib::L2CValue::as_integer(pLVar11);
//     iVar3 = lib::L2CValue::as_integer(local_210);
//     app::lua_bind::WorkModule__set_int64_impl(*ppBVar23,lVar12,iVar3);
//     lib::L2CValue::~L2CValue(local_210);
//     lib::L2CValue::~L2CValue(&LStack_78);
//     lib::L2CValue::L2CValue(local_210,1);
//     lib::L2CValue::operator+(&LStack_108,(L2CValue *)local_210);
//     lib::L2CValue::~L2CValue(local_210);
//     pLVar11 = (L2CValue *)
//               lib::L2CValue::operator[]((L2CValue *)&DAT_7100159a78,(L2CValue *)&LStack_78);
//     lib::L2CValue::L2CValue(local_210,_FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
//     lVar12 = lib::L2CValue::as_integer(pLVar11);
//     iVar3 = lib::L2CValue::as_integer(local_210);
//     app::lua_bind::WorkModule__set_int64_impl(*ppBVar23,lVar12,iVar3);
//   }
//   else {
//     lib::L2CValue::L2CValue(local_210,1);
//     lib::L2CValue::operator+(&LStack_108,(L2CValue *)local_210);
//     lib::L2CValue::~L2CValue(local_210);
//     pLVar11 = (L2CValue *)
//               lib::L2CValue::operator[]((L2CValue *)&DAT_7100159a48,(L2CValue *)&LStack_78);
//     lib::L2CValue::L2CValue(local_210,_FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
//     lVar12 = lib::L2CValue::as_integer(pLVar11);
//     iVar3 = lib::L2CValue::as_integer(local_210);
//     app::lua_bind::WorkModule__set_int64_impl(*ppBVar23,lVar12,iVar3);
//     lib::L2CValue::~L2CValue(local_210);
//     lib::L2CValue::~L2CValue(&LStack_78);
//     lib::L2CValue::L2CValue(local_210,1);
//     lib::L2CValue::operator+(&LStack_108,(L2CValue *)local_210);
//     lib::L2CValue::~L2CValue(local_210);
//     pLVar11 = (L2CValue *)
//               lib::L2CValue::operator[]((L2CValue *)&DAT_7100159a58,(L2CValue *)&LStack_78);
//     lib::L2CValue::L2CValue(local_210,_FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
//     lVar12 = lib::L2CValue::as_integer(pLVar11);
//     iVar3 = lib::L2CValue::as_integer(local_210);
//     app::lua_bind::WorkModule__set_int64_impl(*ppBVar23,lVar12,iVar3);
//   }
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::~L2CValue(&LStack_78);
//   lib::L2CValue::L2CValue(local_210,1);
//   lib::L2CValue::operator+(&LStack_108,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   pLVar11 = (L2CValue *)
//             lib::L2CValue::operator[]((L2CValue *)&DAT_7100159a48,(L2CValue *)&LStack_78);
//   lib::L2CValue::L2CValue(local_210,0xb739d01d9);
//   bVar1 = lib::L2CValue::operator==(pLVar11,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   if (bVar1) {
//     lib::L2CValue::~L2CValue(&LStack_78);
// LAB_7100006278:
//     lib::L2CValue::L2CValue
//               (local_210,_FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HIT_TABEMONO);
//     iVar3 = lib::L2CValue::as_integer(local_210);
//     app::lua_bind::WorkModule__on_flag_impl(*ppBVar23,iVar3);
//   }
//   else {
//     lib::L2CValue::L2CValue(local_210,1);
//     lib::L2CValue::operator+(&LStack_108,(L2CValue *)local_210);
//     lib::L2CValue::~L2CValue(local_210);
//     pLVar11 = (L2CValue *)
//               lib::L2CValue::operator[]((L2CValue *)&DAT_7100159a68,(L2CValue *)&LStack_88);
//     lib::L2CValue::L2CValue(local_210,0xd1f03434a);
//     bVar1 = lib::L2CValue::operator==(pLVar11,(L2CValue *)local_210);
//     lib::L2CValue::~L2CValue(local_210);
//     lib::L2CValue::~L2CValue(&LStack_88);
//     lib::L2CValue::~L2CValue(&LStack_78);
//     if (bVar1) goto LAB_7100006278;
//     lib::L2CValue::L2CValue
//               (local_210,_FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HIT_TABEMONO);
//     iVar3 = lib::L2CValue::as_integer(local_210);
//     app::lua_bind::WorkModule__off_flag_impl(*ppBVar23,iVar3);
//   }
//   lib::L2CValue::~L2CValue(local_210);
//   if (((DAT_7100159ac0 & 1) == 0) && (iVar3 = __cxa_guard_acquire(&DAT_7100159ac0), iVar3 != 0)) {
//     pLVar19 = (L2CTable *)operator.new(0x48);
//     lib::L2CTable::L2CTable(pLVar19,9);
//     lib::L2CValue::L2CValue((L2CValue *)&DAT_7100159a88,pLVar19);
//     lib::L2CValue::L2CValue(local_210,0x4d5cb9290);
//     lib::L2CValue::L2CValue(&LStack_78,0x44cc2c32a);
//     lib::L2CValue::L2CValue(&LStack_88,0x43bc5f3bc);
//     lib::L2CValue::L2CValue(&LStack_98,0x4a5a1661f);
//     lib::L2CValue::L2CValue(&LStack_a8,0x4d2a65689);
//     lib::L2CValue::L2CValue(&LStack_b8,0x44baf0733);
//     lib::L2CValue::L2CValue(&LStack_c8,0x43ca837a5);
//     lib::L2CValue::L2CValue(&LStack_d8,0x4ac172a34);
//     lib::L2CValue::L2CValue(&LStack_e8,0x4db101aa2);
//     FUN_7100008560(&DAT_7100159a88,local_210,&LStack_78,&LStack_88,&LStack_98,&LStack_a8,&LStack_b8,
//                    &LStack_c8,&LStack_d8,&LStack_e8);
//     lib::L2CValue::~L2CValue(&LStack_e8);
//     lib::L2CValue::~L2CValue(&LStack_d8);
//     lib::L2CValue::~L2CValue(&LStack_c8);
//     lib::L2CValue::~L2CValue(&LStack_b8);
//     lib::L2CValue::~L2CValue(&LStack_a8);
//     lib::L2CValue::~L2CValue(&LStack_98);
//     lib::L2CValue::~L2CValue(&LStack_88);
//     lib::L2CValue::~L2CValue(&LStack_78);
//     lib::L2CValue::~L2CValue(local_210);
//     FUN_7100000300(lib::L2CValue::~L2CValue,&DAT_7100159a88,&PTR_LOOP_7100157000);
//     __cxa_guard_release(&DAT_7100159ac0);
//   }
//   lib::L2CValue::L2CValue(&LStack_78,0x5a2add30f);
//   lib::L2CValue::L2CValue(local_210,1);
//   lib::L2CValue::operator+(&LStack_108,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   pLVar11 = (L2CValue *)
//             lib::L2CValue::operator[]((L2CValue *)&DAT_7100159a88,(L2CValue *)&LStack_88);
//   lVar12 = lib::L2CValue::as_integer(&LStack_78);
//   lVar13 = lib::L2CValue::as_integer(pLVar11);
//   app::lua_bind::VisibilityModule__set_status_default_int64_impl(*ppBVar23,lVar12,lVar13);
//   lib::L2CValue::~L2CValue(&LStack_88);
//   lib::L2CValue::~L2CValue(&LStack_78);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&this->globalTable,5);
//   pBVar14 = (BattleObjectModuleAccessor *)lib::L2CValue::as_pointer(pLVar11);
//   app::FighterSpecializer_GameWatch::init_panel(pBVar14);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&this->globalTable,0x16);
//   lib::L2CValue::L2CValue(local_210,_SITUATION_KIND_GROUND);
//   bVar1 = lib::L2CValue::operator==(pLVar11,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   if (bVar1) {
//     lib::L2CValue::L2CValue(local_210,_FIGHTER_KINETIC_TYPE_GROUND_STOP);
//     iVar3 = lib::L2CValue::as_integer(local_210);
//     app::lua_bind::KineticModule__change_kinetic_impl(*ppBVar23,iVar3);
//   }
//   else {
//     lib::L2CValue::L2CValue(local_210,_FIGHTER_KINETIC_TYPE_GAMEWATCH_SPECIAL_AIR_S);
//     iVar3 = lib::L2CValue::as_integer(local_210);
//     app::lua_bind::KineticModule__change_kinetic_impl(*ppBVar23,iVar3);
//   }
//   lib::L2CValue::~L2CValue(local_210);
//   iVar3 = app::lua_bind::StatusModule__situation_kind_impl(*ppBVar23);
//   lib::L2CValue::L2CValue(&LStack_78,iVar3);
//   lib::L2CValue::L2CValue(local_210,SITUATION_KIND_AIR);
//   bVar1 = lib::L2CValue::operator==(&LStack_78,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::~L2CValue(&LStack_78);
//   if (bVar1) {
//     lib::L2CValue::L2CValue(&LStack_98,_KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
//     iVar3 = lib::L2CValue::as_integer(&LStack_98);
//     uVar25 = app::lua_bind::KineticModule__get_sum_speed_impl(*ppBVar23,iVar3);
//     lib::L2CValue::L2CValue(&LStack_1a8,(float)uVar25);
//     lib::L2CValue::L2CValue(&LStack_198,(float)((ulong)uVar25 >> 0x20));
//     lib::L2CValue::L2CValue(local_210,(L2CValue *)&LStack_1a8);
//     lib::L2CValue::L2CValue(&LStack_78,(L2CValue *)&LStack_198);
//     lua2cpp::L2CFighterBase::Vector2__create
//               (this,SUB81(local_210,0),(L2CValue)((char)&stack0xfffffffffffffff0 + -0x68));
//     lib::L2CValue::~L2CValue(&LStack_78);
//     lib::L2CValue::~L2CValue(local_210);
//     lib::L2CValue::~L2CValue(&LStack_198);
//     lib::L2CValue::~L2CValue(&LStack_1a8);
//     lib::L2CValue::~L2CValue(&LStack_98);
//     lib::L2CValue::L2CValue(&LStack_a8,_FIGHTER_KINETIC_ENERGY_ID_STOP);
//     lib::L2CAgent::clear_lua_stack((L2CAgent *)this);
//     lib::L2CAgent::push_lua_stack((L2CAgent *)this,(L2CValue *)&LStack_a8);
//     uVar25 = app::sv_kinetic_energy::get_speed(this->luaStateAgent);
//     lib::L2CValue::L2CValue(&LStack_1c8,(float)uVar25);
//     lib::L2CValue::L2CValue(&LStack_1b8,(float)((ulong)uVar25 >> 0x20));
//     lib::L2CValue::L2CValue(local_210,(L2CValue *)&LStack_1c8);
//     lib::L2CValue::L2CValue(&LStack_78,(L2CValue *)&LStack_1b8);
//     lua2cpp::L2CFighterBase::Vector2__create
//               (this,SUB81(local_210,0),(L2CValue)((char)&stack0xfffffffffffffff0 + -0x68));
//     lib::L2CValue::~L2CValue(&LStack_78);
//     lib::L2CValue::~L2CValue(local_210);
//     lib::L2CValue::~L2CValue(&LStack_1b8);
//     lib::L2CValue::~L2CValue(&LStack_1c8);
//     lib::L2CValue::~L2CValue(&LStack_a8);
//     pLVar15 = (L2CValue *)lib::L2CValue::operator[](&LStack_88,0x18cdc1683);
//     lib::L2CValue::L2CValue(&LStack_a8,0xfea97fe73);
//     lib::L2CValue::L2CValue(&LStack_b8,0x157d08ac58);
//     uVar16 = lib::L2CValue::as_integer(&LStack_a8);
//     uVar17 = lib::L2CValue::as_integer(&LStack_b8);
//     fVar24 = (float)app::lua_bind::WorkModule__get_param_float_impl(*ppBVar23,uVar16,uVar17);
//     lib::L2CValue::L2CValue(&LStack_78,fVar24);
//     lib::L2CValue::operator*(pLVar15,(L2CValue *)&LStack_78,local_210);
//     pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_88,0x18cdc1683);
//     lib::L2CValue::operator=(pLVar11,(L2CValue *)local_210);
//     lib::L2CValue::~L2CValue(local_210);
//     lib::L2CValue::~L2CValue(&LStack_78);
//     lib::L2CValue::~L2CValue(&LStack_b8);
//     lib::L2CValue::~L2CValue(&LStack_a8);
//     pLVar15 = (L2CValue *)lib::L2CValue::operator[](&LStack_98,0x1fbdb2615);
//     pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_88,0x1fbdb2615);
//     lib::L2CValue::operator=(pLVar11,pLVar15);
//     lib::L2CValue::L2CValue(local_210,_FIGHTER_KINETIC_ENERGY_ID_STOP);
//     pLVar15 = (L2CValue *)lib::L2CValue::operator[](&LStack_88,0x18cdc1683);
//     pLVar18 = (L2CValue *)lib::L2CValue::operator[](&LStack_88,0x1fbdb2615);
//     lib::L2CAgent::clear_lua_stack((L2CAgent *)this);
//     lib::L2CAgent::push_lua_stack((L2CAgent *)this,(L2CValue *)local_210);
//     lib::L2CAgent::push_lua_stack((L2CAgent *)this,pLVar15);
//     lib::L2CAgent::push_lua_stack((L2CAgent *)this,pLVar18);
//     app::sv_kinetic_energy::set_speed(this->luaStateAgent);
//     lib::L2CValue::~L2CValue(local_210);
//     lib::L2CValue::~L2CValue(&LStack_98);
//     lib::L2CValue::~L2CValue(&LStack_88);
//   }
//   pLVar19 = (L2CTable *)operator.new(0x48);
//   lib::L2CTable::L2CTable(pLVar19,0);
//   lib::L2CValue::L2CValue(&LStack_78,pLVar19);
//   lib::L2CValue::L2CValue(&LStack_1d8,1);
//   FUN_71000082d0(local_210,&LStack_1d8);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,1);
//   lib::L2CValue::operator=(pLVar11,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::~L2CValue(&LStack_1d8);
//   lib::L2CValue::L2CValue(&LStack_1d8,2);
//   FUN_71000082d0(local_210,&LStack_1d8);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,2);
//   lib::L2CValue::operator=(pLVar11,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::~L2CValue(&LStack_1d8);
//   lib::L2CValue::L2CValue(&LStack_1d8,3);
//   FUN_71000082d0(local_210,&LStack_1d8);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,3);
//   lib::L2CValue::operator=(pLVar11,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::~L2CValue(&LStack_1d8);
//   lib::L2CValue::L2CValue(&LStack_1d8,4);
//   FUN_71000082d0(local_210,&LStack_1d8);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,4);
//   lib::L2CValue::operator=(pLVar11,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::~L2CValue(&LStack_1d8);
//   lib::L2CValue::L2CValue(&LStack_1d8,5);
//   FUN_71000082d0(local_210,&LStack_1d8);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,5);
//   lib::L2CValue::operator=(pLVar11,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::~L2CValue(&LStack_1d8);
//   lib::L2CValue::L2CValue(&LStack_1d8,6);
//   FUN_71000082d0(local_210,&LStack_1d8);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,6);
//   lib::L2CValue::operator=(pLVar11,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::~L2CValue(&LStack_1d8);
//   lib::L2CValue::L2CValue(&LStack_1d8,7);
//   FUN_71000082d0(local_210,&LStack_1d8);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,7);
//   lib::L2CValue::operator=(pLVar11,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::~L2CValue(&LStack_1d8);
//   lib::L2CValue::L2CValue(&LStack_1d8,8);
//   FUN_71000082d0(local_210,&LStack_1d8);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,8);
//   lib::L2CValue::operator=(pLVar11,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::~L2CValue(&LStack_1d8);
//   lib::L2CValue::L2CValue(&LStack_1d8,9);
//   FUN_71000082d0(local_210,&LStack_1d8);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,9);
//   lib::L2CValue::operator=(pLVar11,(L2CValue *)local_210);
//   local_1f0 = (undefined ***)local_210;
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::~L2CValue(&LStack_1d8);
//   local_210[0]._0_8_ = &PTR_LAB_71001570a0;
//   puVar20 = (undefined8 *)operator.new(0x40);
//   puVar21 = puVar20 + 2;
//   *puVar21 = &PTR_LAB_71001570a0;
//   *puVar20 = puVar21;
//   *(undefined4 *)(puVar20 + 1) = 0;
//   puVar20[6] = puVar21;
//   lib::L2CValue::L2CValue(&LStack_88,(L2CInnerFunctionBase *)puVar20);
//   lib::L2CAgent::sort((L2CAgent *)this,(L2CValue *)&LStack_78,(L2CValue *)&LStack_88);
//   lib::L2CValue::~L2CValue(&LStack_88);
//   if (local_210 == (L2CValue *)local_1f0) {
//     pcVar22 = (code *)(*local_1f0)[4];
//   }
//   else {
//     if (local_1f0 == (undefined ***)0x0) goto LAB_71000069b0;
//     pcVar22 = (code *)(*local_1f0)[5];
//   }
//   (*pcVar22)();
// LAB_71000069b0:
//   this_00 = &this[1].functions;
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)this_00,0xb50b57417);
//   lib::L2CValue::L2CValue(local_210,1);
//   lib::L2CValue::operator=(pLVar11,(L2CValue *)local_210);
//   lib::L2CValue::~L2CValue(local_210);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,1);
//   pLVar15 = (L2CValue *)lib::L2CValue::operator[](pLVar11,0x43bc4bcd9);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)this_00,0xb34fc72d1);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](pLVar11,1);
//   lib::L2CValue::operator=(pLVar11,pLVar15);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,2);
//   pLVar15 = (L2CValue *)lib::L2CValue::operator[](pLVar11,0x43bc4bcd9);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)this_00,0xb34fc72d1);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](pLVar11,2);
//   lib::L2CValue::operator=(pLVar11,pLVar15);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,3);
//   pLVar15 = (L2CValue *)lib::L2CValue::operator[](pLVar11,0x43bc4bcd9);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)this_00,0xb34fc72d1);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](pLVar11,3);
//   lib::L2CValue::operator=(pLVar11,pLVar15);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,4);
//   pLVar15 = (L2CValue *)lib::L2CValue::operator[](pLVar11,0x43bc4bcd9);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)this_00,0xb34fc72d1);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](pLVar11,4);
//   lib::L2CValue::operator=(pLVar11,pLVar15);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,5);
//   pLVar15 = (L2CValue *)lib::L2CValue::operator[](pLVar11,0x43bc4bcd9);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)this_00,0xb34fc72d1);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](pLVar11,5);
//   lib::L2CValue::operator=(pLVar11,pLVar15);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,6);
//   pLVar15 = (L2CValue *)lib::L2CValue::operator[](pLVar11,0x43bc4bcd9);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)this_00,0xb34fc72d1);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](pLVar11,6);
//   lib::L2CValue::operator=(pLVar11,pLVar15);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,7);
//   pLVar15 = (L2CValue *)lib::L2CValue::operator[](pLVar11,0x43bc4bcd9);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)this_00,0xb34fc72d1);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](pLVar11,7);
//   lib::L2CValue::operator=(pLVar11,pLVar15);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,8);
//   pLVar15 = (L2CValue *)lib::L2CValue::operator[](pLVar11,0x43bc4bcd9);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)this_00,0xb34fc72d1);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](pLVar11,8);
//   lib::L2CValue::operator=(pLVar11,pLVar15);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](&LStack_78,9);
//   pLVar15 = (L2CValue *)lib::L2CValue::operator[](pLVar11,0x43bc4bcd9);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)this_00,0xb34fc72d1);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[](pLVar11,9);
//   lib::L2CValue::operator=(pLVar11,pLVar15);
//   lib::L2CValue::L2CValue(local_210,0x5a2add30f);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)this_00,0xb34fc72d1);
//   pLVar15 = (L2CValue *)lib::L2CValue::operator[](pLVar11,1);
//   pLVar11 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)&DAT_7100159a88,pLVar15);
//   lVar12 = lib::L2CValue::as_integer(local_210);
//   lVar13 = lib::L2CValue::as_integer(pLVar11);
//   app::lua_bind::VisibilityModule__set_int64_impl(*ppBVar23,lVar12,lVar13);
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::L2CValue(local_210,_FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL);
//   iVar3 = lib::L2CValue::as_integer(local_210);
//   app::lua_bind::WorkModule__off_flag_impl(*ppBVar23,iVar3);
//   lib::L2CValue::~L2CValue(local_210);
//   lib::L2CValue::L2CValue(return_value,0);
//   lib::L2CValue::~L2CValue(&LStack_78);
//   lib::L2CValue::~L2CValue(&LStack_118);
//   lib::L2CValue::~L2CValue(&LStack_108);
//   lib::L2CValue::~L2CValue(&LStack_f8);
//   return;
// }