use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd::*;
use smash::app::sv_system;
use smashline::*;
use smash_script::*;

#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
fn tink_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi") {
            if MotionModule::frame(fighter.module_accessor) < 45. {
                let toonlink_upb_speed = smash::phx::Vector3f { x: 0.1 * ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor), y: 0., z: 0.0 };
                let x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if (x_vel <= 0.5 && x_vel >= 0.0) || toonlink_upb_speed.x < 0.0 {
                    KineticModule::add_speed(fighter.module_accessor, &toonlink_upb_speed);
                }
                if (x_vel >= -0.5 && x_vel <= 0.0) || toonlink_upb_speed.x > 0.0 {
                    KineticModule::add_speed(fighter.module_accessor, &toonlink_upb_speed);
                }
            }
        }
        // if curr_motion_kind == hash40("special_air_hi") {
        //     if MotionModule::frame(boma) <= 3.0 && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
        //         MotionModule::set_rate(boma, 0.1);
        //         FIGHTER_GLOBALS[entry_id_int as usize].tink_specialhi_charge += 0.02;
        //         FIGHTER_GLOBALS[entry_id_int as usize].tink_specialhi_charging = true;
        //     }
        //     else {
        //         MotionModule::set_rate(boma, 1.0);
        //         FIGHTER_GLOBALS[entry_id_int as usize].tink_specialhi_charging = false;
        //     }
        // }
    }
}

//global edits
#[acmd_script( agent = "toonlink", script = "game_dash", category = ACMD_GAME, low_priority)]
unsafe fn dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

#[acmd_script( agent = "toonlink", script = "game_turndash", category = ACMD_GAME, low_priority)]
unsafe fn turndash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 4.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
        }
    frame(lua_state, 16.);
        if macros::is_excute(fighter)
        {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
}

//air
#[acmd_script( agent = "toonlink", script = "game_attackairlw", category = ACMD_GAME, low_priority)]
unsafe fn attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

        frame(lua_state, 0.);
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                macros::SET_SPEED_EX(fighter,0, 1.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                MotionModule::set_rate(boma, 0.85);
                WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            }
        //get_value_int(SO_VAR_INT_PREV_STATUS)
        //0x119130(0, FIGHTER_STATUS_KIND_DAMAGE_FLY)
            if macros::is_excute(fighter)
            {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
            }
            if macros::is_excute(fighter)
            {
                JostleModule::set_status(boma, false);
            }
        frame(lua_state, 17.);
            if macros::is_excute(fighter)
            {
                MotionModule::set_rate(boma, 2.25);
                JostleModule::set_status(boma, true);
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                macros::SET_SPEED_EX(fighter,0, -3.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                WorkModule::on_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK);
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 268, 80, 0, 40, 5.5, 1.0, -1.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
            }
        wait(lua_state, 9.);
            if macros::is_excute(fighter)
            {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 98, 0, 40, 5.0, 1.0, -1.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
            }
        frame(lua_state, 64.);
        if macros::is_excute(fighter)
            {
                MotionModule::set_rate(boma, 0.9);
                AttackModule::clear_all(boma);
                WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK);
                macros::SET_SPEED_EX(fighter,0, -1.9, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        frame(lua_state, 71.);
            if macros::is_excute(fighter)
            {
                KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                }
            }
}

//other
#[acmd_script( agent = "toonlink", script = "game_escapeairslide", category = ACMD_GAME, low_priority)]
unsafe fn escapeairslide(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 14.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
    frame(lua_state, 24.);
        if macros::is_excute(fighter)
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        }
}

pub fn install() {
    smashline::install_agent_frames!(
        tink_frame
    );
    smashline::install_acmd_scripts!(
        dash,
        turndash,
        attackairlw,
        escapeairslide
    );
}
