use super::*;

//attacks
#[acmd_script( agent = "bayonetta", script = "game_shootingarml_atkon_attack11", category = ACMD_GAME, low_priority)]
unsafe fn shootingarml_atkon_attack11(fighter: &mut L2CAgentBase) {
    let _boma = fighter.module_accessor; //the smash script line needs this or else it doesn't compile. immutable borrow stuff

    if macros::is_excute(fighter)
    {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
            macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 0, Hash40::new("top"), 1.0, 361, 80, 0, 10, 2.5, 0.0, 10.5, 9.0, Some(0.0), Some(10.5), Some(69.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        }
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 10.5, 9.0, Some(0.0), Some(10.5), Some(69.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        smash_script::notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, WorkModule::get_int64(_boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 8, 4);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_shootingarml_atkoff_attack11", category = ACMD_GAME, low_priority)]
unsafe fn shootingarml_atkoff_attack11(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
            AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00) as i32, false);
        }
        AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01) as i32, false);
    }
}

//air
#[acmd_script( agent = "bayonetta", script = "game_shootinglegr_atkon_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn game_shootinglegr_atkon_attackairb(fighter: &mut L2CAgentBase) {
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
#[acmd_script( agent = "bayonetta", script = "game_shootinglegr_atkoff_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn game_shootinglegr_atkoff_attackairb(fighter: &mut L2CAgentBase) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_02, false);
        }
    }
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_03, false);
    }
}
#[acmd_script( agent = "bayonetta", script = "game_shootingarmr_atkon_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn game_shootingarmr_atkon_attackairb(fighter: &mut L2CAgentBase) {
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
#[acmd_script( agent = "bayonetta", script = "game_shootingarmr_atkoff_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn game_shootingarmr_atkoff_attackairb(fighter: &mut L2CAgentBase) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00, false);
        }
    }
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, false);
    }
}
#[acmd_script( agent = "bayonetta", script = "game_shootingarml_atkon_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn game_shootingarml_atkon_attackairb(fighter: &mut L2CAgentBase) {
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
#[acmd_script( agent = "bayonetta", script = "game_shootingarml_atkoff_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn game_shootingarml_atkoff_attackairb(fighter: &mut L2CAgentBase) {
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
#[acmd_script( agent = "bayonetta", scripts = ["game_shootingarml_atkon_specialhi", "game_shootingarml_atkon_specialairhi"], category = ACMD_GAME, low_priority)]
unsafe fn shooting_on_specialhi(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        VarModule::on_flag(fighter.battle_object,bayonetta::status::flag::SPECIAL_HI_SHOOT);
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 1, Hash40::new("top"), 0.5, 361, 0, 0, 0, 11.0, 0.0, 11.0, 0.0, Some(0.0), Some(11.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
    }
}
#[acmd_script( agent = "bayonetta", scripts = ["game_shootingarml_atkoff_specialhi", "game_shootingarml_atkoff_specialairhi"], category = ACMD_GAME, low_priority)]
unsafe fn shooting_off_specialhi(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01) as i32, false);
    }
}

//other
#[acmd_script( agent = "bayonetta", script = "game_shootinglegl_atkon_escapeb", category = ACMD_GAME, low_priority)]
unsafe fn shootinglegl_on_escapeb(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor;

    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 0, Hash40::new("footl"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(68.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        smash_script::notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 8, 4);
    }
}
#[acmd_script( agent = "bayonetta", script = "game_shootinglegl_atkoff_escapeb", category = ACMD_GAME, low_priority)]
unsafe fn shootinglegl_off_escapeb(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00) as i32, false);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_shootinglegr_atkon_escapeb", category = ACMD_GAME, low_priority)]
unsafe fn shootinglegr_on_escapeb(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor; //the smash script line needs this or else it doesn't compile. immutable borrow stuff

    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 1, Hash40::new("footr"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(68.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        smash_script::notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 8, 4);
    }
}
#[acmd_script( agent = "bayonetta", script = "game_shootinglegr_atkoff_escapeb", category = ACMD_GAME, low_priority)]
unsafe fn shootinglegr_off_escapeb(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00) as i32, false);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_shootingarml_atkon_escapef", category = ACMD_GAME, low_priority)]
unsafe fn shootingarml_on_escapef(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor; //the smash script line needs this or else it doesn't compile. immutable borrow stuff

    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 1, Hash40::new("footl"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(68.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        smash_script::notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00), 16, 4);
    }
}
#[acmd_script( agent = "bayonetta", script = "game_shootingarml_atkoff_escapef", category = ACMD_GAME, low_priority)]
unsafe fn shootingarml_off_escapef(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00) as i32, false);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_shootingarmr_atkon_escapef", category = ACMD_GAME, low_priority)]
unsafe fn shootingarmr_on_escapef(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor; //the smash script line needs this or else it doesn't compile. immutable borrow stuff

    if macros::is_excute(fighter)
    {
        macros::ATTACK(fighter, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 1, Hash40::new("footr"), 0.5, 361, 0, 0, 0, 2.5, 0.0, 2.5, 8.0, Some(0.0), Some(2.5), Some(68.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        smash_script::notify_event_msc_cmd!(fighter, 0x36db1a34c9 as u64, WorkModule::get_int64(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01), 8, 4);
    }
}
#[acmd_script( agent = "bayonetta", script = "game_shootingarmr_atkoff_escapef", category = ACMD_GAME, low_priority)]
unsafe fn shootingarmr_off_escapef(fighter: &mut L2CAgentBase) {

    if macros::is_excute(fighter)
    {
        AttackModule::clear(fighter.module_accessor, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00) as i32, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        shootingarml_atkon_attack11, shootingarml_atkoff_attack11,

        game_shootinglegr_atkon_attackairb, game_shootinglegr_atkoff_attackairb,
        game_shootingarmr_atkon_attackairb, game_shootingarmr_atkoff_attackairb,
        game_shootingarml_atkon_attackairb, game_shootingarml_atkoff_attackairb,

        shooting_on_specialhi, shooting_off_specialhi,

        shootinglegl_on_escapeb, shootinglegl_off_escapeb,
        shootinglegr_on_escapeb, shootinglegr_off_escapeb,

        shootingarml_on_escapef, shootingarml_off_escapef,
        shootingarmr_on_escapef, shootingarmr_off_escapef,
    );
}