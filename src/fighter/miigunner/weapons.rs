use super::*;

//weapons
unsafe extern "C" fn stealthbombmove(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;

        if macros::is_excute(weapon)
        {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 7.5, 55, 94, 0, 48, 6.0, 0.0,0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true,true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"),*ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
    frame(lua_state, 3.);
        if macros::is_excute(weapon)
        {
            AttackModule::clear_all(weapon.module_accessor);
        }
    frame(lua_state, 4.);
        if macros::is_excute(weapon)
        {
            macros::AREA_WIND_2ND_RAD(weapon, 0, 1, 0.02, 1000, 1, 0, 0, 12);
        }
    frame(lua_state, 15.);
        if macros::is_excute(weapon)
        {
            AreaModule::erase_wind(weapon.module_accessor, 0);
        }
}

pub fn install() {
    let sBomb = &mut smashline::Agent::new("miigunner_stealthbomb_s");

    sBomb.game_acmd("game_move", stealthbombmove,);
}