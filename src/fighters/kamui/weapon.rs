use super::*;

unsafe fn ryusensya_min(weapon: &mut L2CAgentBase) {

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 4.0, 45, 90, 0, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.45, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 9.0, 45, 90, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            smash_script::attack!(weapon, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
            macros::ATK_LERP_RATIO(weapon, WorkModule::get_float(weapon.module_accessor, *WEAPON_KAMUI_RYUSENSYA_INSTANCE_WORK_ID_FLOAT_HOLD_RATE));
            AttackModule::enable_safe_pos(weapon.module_accessor);
        }
}

unsafe fn ryusensya_max(weapon: &mut L2CAgentBase) {

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 11.0, 55, 90, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
}

unsafe fn dragoncounter(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;

    frame(lua_state, 26.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 8.0, 80, 20, 0, 90, 12.0, 0.0, 6.0, 11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 8.0, 80, 20, 0, 90, 12.0, 0.0, 6.0, -11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
            AttackModule::set_force_reaction(weapon.module_accessor, 0, true, false);
            AttackModule::set_force_reaction(weapon.module_accessor, 1, true, false);
            if WorkModule::is_flag(weapon.module_accessor, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
                if macros::is_excute(weapon)
                {
                    AttackModule::set_optional_hit_sound(weapon.module_accessor, 0, Hash40::new("se_kamui_criticalhit"));
                    AttackModule::set_optional_hit_sound(weapon.module_accessor, 1, Hash40::new("se_kamui_criticalhit"));
                }
            }
        }
    wait(lua_state, 1.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 8.0, 90, 20, 0, 90, 8.0, 0.0, 21.0, 11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
            macros::ATTACK(weapon, 3, 0, Hash40::new("top"), 8.0, 90, 20, 0, 90, 8.0, 0.0, 21.0, -11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
            AttackModule::set_force_reaction(weapon.module_accessor, 2, true, false);
            AttackModule::set_force_reaction(weapon.module_accessor, 3, true, false);
            if WorkModule::is_flag(weapon.module_accessor, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
                if macros::is_excute(weapon)
                {
                    AttackModule::set_optional_hit_sound(weapon.module_accessor, 2, Hash40::new("se_kamui_criticalhit"));
                    AttackModule::set_optional_hit_sound(weapon.module_accessor, 3, Hash40::new("se_kamui_criticalhit"));
                }
            }
        }
    frame(lua_state, 31.);
        if macros::is_excute(weapon)
        {
            AttackModule::clear_all(weapon.module_accessor);
            MotionModule::set_rate(weapon.module_accessor, 1.2);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    let waterDragon = &mut smashline::Agent::new("kamui_waterdragon");
    let ryusensya = &mut smashline::Agent::new("kamui_ryusensya");

    waterDragon.game_acmd("game_speciallwhit", dragoncounter,);
    waterDragon.game_acmd("game_speciallwhitturn", dragoncounter,);
    waterDragon.game_acmd("game_specialairlwhit", dragoncounter,);
    waterDragon.game_acmd("game_specialairlwhitturn", dragoncounter,);

    waterDragon.game_acmd("game_regular", ryusensya_min,);
    waterDragon.game_acmd("game_shotmax", ryusensya_max,);
}