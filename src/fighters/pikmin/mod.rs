use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd::*;
use smashline::*;
use smash_script::*;

// #[fighter_frame( agent = FIGHTER_KIND_DEMON )]
// fn gardencaptain_frame(fighter: &mut L2CFighterCommon) {
//     unsafe {


//     }
// }

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



//global edits
#[acmd_script( agent = "pikmin", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "pikmin", script = "game_turndash", category = ACMD_GAME, low_priority)]
unsafe fn turndash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

//ground
#[acmd_script( agent = "pikmin", script = "game_attacks3", category = ACMD_GAME, low_priority)]
unsafe fn attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 1.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.2);
        }
    frame(lua_state, 15.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 97, 0, 35, 3.5, 0.0, 5.0, 13.5, Some(0.0), Some(5.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 97, 0, 35, 2.2, 0.0, 5.0, 11.0, Some(0.0), Some(5.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear(fighter.module_accessor, 0, false);
        }
    frame(lua_state, 18.);
        if macros::is_excute(fighter)
        {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

//specials
#[acmd_script( agent = "pikmin", script = "game_specialnstart", category = ACMD_GAME, low_priority)]
unsafe fn specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

        if macros::is_excute(fighter)
        {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, false, -1);
        }
    frame(lua_state, 3.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 0.66);
        }
    frame(lua_state, 6.);
        if macros::is_excute(fighter)
        {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
}

#[acmd_script( agent = "pikmin", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority)]
unsafe fn speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 2.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_SPECIAL_LW_FLAG_SORT);
            smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
    frame(lua_state, 9.);
        if macros::is_excute(fighter)
        {
            smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        }
}

//others
#[acmd_script( agent = "pikmin", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
unsafe fn escapeairslide(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        attacks3,
        specialn,
        speciallw,
        escapeairslide
    );
}