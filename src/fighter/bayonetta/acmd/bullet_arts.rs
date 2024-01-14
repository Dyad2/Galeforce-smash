use super::*;

//attacks
unsafe extern "C" fn shootingarml_atkon_attack11(fighter: &mut L2CAgentBase) {
    let _boma = fighter.module_accessor; //the smash script line needs this or else it doesn't compile. immutable borrow stuff

    if macros::is_excute(fighter)
    {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
            macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 0, Hash40::new("top"), 1.0, 361, 80, 0, 10, 2.5, 0.0, 10.5, 9.0, Some(0.0), Some(10.5), Some(69.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        }
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 10.5, 9.0, Some(0.0), Some(10.5), Some(69.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, WorkModule::get_int64(_boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 8, 4);
    }
}

unsafe extern "C" fn shootingarml_atkoff_attack11(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
            AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00) as i32, false);
        }
        AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01) as i32, false);
    }
}

//air
unsafe extern "C" fn shootinglegr_atkon_attackairb(fighter: &mut L2CAgentBase) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_02), 0, Hash40::new("top"), 0.5, 250, 100, 0, 10, 2.5, 0.0, 2.0, -3.0, Some(0.0), Some(-1.8), Some(-4.4), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_03), 0, Hash40::new("top"), 0.5, 250, 0, 0, 0, 2.5, 0.0, 2.0, -3.0, Some(0.0), Some(-54.4), Some(-23.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x36db1a34c9), FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_03, 10, 4.05);
    }
}

unsafe extern "C" fn shootinglegr_atkoff_attackairb(fighter: &mut L2CAgentBase) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_02, false);
        }
    }
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_03, false);
    }
}

unsafe extern "C" fn shootingarmr_atkon_attackairb(fighter: &mut L2CAgentBase) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 0, Hash40::new("top"), 0.5, 12, 100, 0, 10, 2.5, 0.0, 14.0, -13.0, Some(0.0), Some(14.8), Some(-16.9), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 0, Hash40::new("top"), 0.5, 12, 0, 0, 0, 2.5, 0.0, 14.0, -13.0, Some(0.0), Some(26.5), Some(-71.7), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x36db1a34c9), FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, 10, 4);
    }
}

unsafe extern "C" fn shootingarmr_atkoff_attackairb(fighter: &mut L2CAgentBase) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00, false);
        }
    }
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, false);
    }
}

unsafe extern "C" fn shootingarml_atkon_attackairb(fighter: &mut L2CAgentBase) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 0, Hash40::new("top"), 0.5, 12, 100, 0, 10, 2.5, 0.0, 14.0, -13.0, Some(0.0), Some(14.8), Some(-16.9), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 0, Hash40::new("top"), 0.5, 12, 0, 0, 0, 2.5, 0.0, 14.0, -13.0, Some(0.0), Some(26.5), Some(-71.7), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x36db1a34c9), FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, 18, 3.981);
    }
}

unsafe extern "C" fn shootingarml_atkoff_attackairb(fighter: &mut L2CAgentBase) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00, false);
        }
    }
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, false);
    }
}

//specials
unsafe extern "C" fn shooting_on_specialhi(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        VarModule::on_flag(fighter.module_accessor,bayonetta::status::flag::SPECIAL_HI_SHOOT);
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 1, Hash40::new("top"), 0.5, 361, 0, 0, 0, 11.0, 0.0, 11.0, 0.0, Some(0.0), Some(11.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn shooting_off_specialhi(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01) as i32, false);
    }
}

//other
unsafe extern "C" fn shootinglegl_on_escapeb(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor;

    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 0, Hash40::new("footl"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(68.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 8, 4);
    }
}

unsafe extern "C" fn shootinglegl_off_escapeb(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00) as i32, false);
    }
}

unsafe extern "C" fn shootinglegr_on_escapeb(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor; //the smash script line needs this or else it doesn't compile. immutable borrow stuff

    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 1, Hash40::new("footr"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(68.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 8, 4);
    }
}

unsafe extern "C" fn shootinglegr_off_escapeb(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter)
    {
        AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00) as i32, false);
    }
}

unsafe extern "C" fn shootingarml_on_escapef(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor; //the smash script line needs this or else it doesn't compile. immutable borrow stuff

    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 1, Hash40::new("footl"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(68.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 16, 4);
    }
}

unsafe extern "C" fn shootingarml_off_escapef(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00) as i32, false);
    }
}

unsafe extern "C" fn shootingarmr_on_escapef(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor; //the smash script line needs this or else it doesn't compile. immutable borrow stuff

    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 1, Hash40::new("footr"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(68.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 8, 4);
    }
}

unsafe extern "C" fn shootingarmr_off_escapef(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00) as i32, false);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_shootingarml_atkon_attack11", shootingarml_atkon_attack11)
    .game_acmd("game_shootingarml_atkoff_attack11", shootingarml_atkoff_attack11)
    
    .game_acmd("game_shootinglegr_atkon_attackairb", shootinglegr_atkon_attackairb)
    .game_acmd("game_shootinglegr_atkoff_attackairb", shootinglegr_atkoff_attackairb)

    .game_acmd("game_shootingarmr_atkon_attackairb", shootingarmr_atkon_attackairb)
    .game_acmd("game_shootingarmr_atkoff_attackairb", shootingarmr_atkoff_attackairb)

    .game_acmd("game_shootingarml_atkon_attackairb", shootingarml_atkon_attackairb)
    .game_acmd("game_shootingarml_atkoff_attackairb", shootingarml_atkoff_attackairb)

    .game_acmd("game_shootingarml_atkon_specialhi", shooting_on_specialhi)
    .game_acmd("game_shootingarml_atkon_specialairhi", shooting_on_specialhi)
    .game_acmd("game_shootingarml_atkoff_specialhi", shooting_off_specialhi)
    .game_acmd("game_shootingarml_atkoff_specialairhi", shooting_off_specialhi)

    .game_acmd("game_shootinglegl_atkon_escapeb", shootinglegl_on_escapeb)
    .game_acmd("game_shootinglegl_atkoff_escapeb", shootinglegl_off_escapeb)

    .game_acmd("game_shootinglegr_atkon_escapeb", shootinglegr_on_escapeb)
    .game_acmd("game_shootinglegr_atkoff_escapeb", shootinglegr_off_escapeb)

    .game_acmd("game_shootingarml_atkon_escapef", shootingarml_on_escapef)
    .game_acmd("game_shootingarml_atkoff_escapef", shootingarml_off_escapef)

    .game_acmd("game_shootingarmr_atkon_escapef", shootingarmr_on_escapef)
    .game_acmd("game_shootingarmr_atkoff_escapef", shootingarmr_off_escapef)
    .install();
}