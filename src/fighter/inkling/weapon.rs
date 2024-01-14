use super::*;

unsafe extern "C" fn inkbulletfly(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    //let boma = sv_system::battle_object_module_accessor(lua_state);
    //let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    
        if macros::is_excute(weapon)
        {
            // if MotionModule::motion_kind(owner_fighter.module_accessor) == hash40("attack_air_b") {
            //     macros::SET_SPEED_EX(weapon, -5.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ID_MAIN);
            // }
            // if MotionModule::motion_kind(owner_fighter.module_accessor) == hash40("attack_air_lw") {
            //     macros::SET_SPEED_EX(weapon, -5.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ID_MAIN);
            // }
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.0, 38, 100, 0, 10, 3.0, 0.0, 0.0, 5.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(weapon.module_accessor, 0, 0.0);
        }
    wait(lua_state, 3.);
        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.0, 361, 100, 0, 10, 3.0, 0.0, 0.0, 5.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(weapon.module_accessor, 0, 0.0);
        }
}

pub fn install(agent: &mut smashline::Agent) {
    let inkbullet = &mut smashline::Agent::new("inkling_inkbullet");
    inkbullet.game_acmd("game_fly", inkbulletfly,);
}