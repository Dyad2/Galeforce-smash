use super::*;

//weapons
unsafe extern "C" fn water_specialhi(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let weapon_struct =  mem::transmute::<&mut BattleObject, &mut smash::app::Weapon>(smash::app::sv_system::battle_object(lua_state));
    let owner_id = smash::app::lua_bind::Weapon::get_founder_id(weapon_struct) as u32;

    //weapons using owner_fighter.module_accessor stuff
    if !smash::app::sv_battle_object::is_null(owner_id) && smash::app::sv_battle_object::is_active(owner_id) {
        let owner_boma = &mut *sv_battle_object::module_accessor(owner_id);

        if VarModule::is_flag(owner_boma, commons::instance::flag::GALEFORCE_ATTACK_ON) {
            frame(lua_state, 1.);
                if macros::is_excute(weapon)
                {
                    macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 2.0, 58, 80, 0, 60, 4.7, 0.0, 0.0, 1.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
                }
        }
    }
    else {
        frame(lua_state, 1.);
            if macros::is_excute(weapon)
            {
                macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 2.0, 58, 80, 0, 60, 4.7, 0.0, 0.0, 1.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0.0, 0.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_NONE);
            }
        }
}

pub fn install(agent: &mut smashline::Agent) {
    let water = &mut smashline::Agent::new("gekkouga_water");

    water.game_acmd("game_specialhil", water_specialhi,);
    water.game_acmd("game_specialhir", water_specialhi,);
}