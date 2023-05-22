use super::*;

#[smashline::fighter_frame_callback]
fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        if smash::app::utility::get_category(&mut *fighter.module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            let status_kind = StatusModule::status_kind(fighter.module_accessor);
            let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
            let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
            TOTAL_FIGHTER = FighterManager::total_fighter_num(singletons::FighterManager());

            //resetting vars. in theory this should be done individually for each var, not here in opff
            if status_kind == *FIGHTER_STATUS_KIND_DEAD || KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_RESET_NORMAL {
                CustomVarManager::reset_var_module(fighter.battle_object);
            }

            edge_cancels::run(fighter, status_kind, situation_kind);
            ecb_shifts::run(fighter);
            controls::run(fighter);
            galeforce::run(fighter);

            //Custom Jostling.
            //  Jostling is enabled depending on fighter status. some fighters always have jostling enabled.
            // it works because fighters who do not fulfill the requirement are all in the same jostling "team"
            if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, 
                *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, 
                *FIGHTER_STATUS_KIND_FURAFURA,*FIGHTER_STATUS_KIND_FURAFURA_STAND, 
                *FIGHTER_STATUS_KIND_SLEEP,*FIGHTER_STATUS_KIND_BIND, 
                *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, 
                *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE, 
                *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_TURN, 
                *FIGHTER_STATUS_KIND_SHIELD_BREAK_DOWN, *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY, 
                *FIGHTER_STATUS_KIND_SLIP_STAND_ATTACK, 
                *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_DASH, 
                *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, 
                *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, 
                *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_START, 
                *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind)
            || [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_GANON, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_SNAKE].contains(&fighter_kind) {
                JostleModule::set_team(fighter.module_accessor, WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) + 1);
            }
            else {
                JostleModule::set_team(fighter.module_accessor, 0);
            }

            // if VarModule::is_flag(fighter.battle_object, commons::instance::flag::MEWTWO_PRESSURED) {
            //     DamageModule::set_damage_mul(fighter.module_accessor, 1.5);
            // }

            // if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) == 0 { //run only once, not per fighter
            //     for fighter1 in 0 .. TOTAL_FIGHTER {
            //         let mut jostle_on = false;
                    
            //         let fighter1_boma = &mut *sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(fighter1));
            //         let status_kind1 = StatusModule::status_kind(fighter1_boma);
            //         let curr_motion_kind1 = MotionModule::motion_kind(fighter1_boma);
            //         let fighter_kind1 = get_kind(fighter1_boma);
            //         for fighter2 in 0 .. TOTAL_FIGHTER {
            //             let fighter2_boma = &mut *sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(fighter2));
            //             let status_kind2 = StatusModule::status_kind(fighter2_boma);
            //             let curr_motion_kind2 = MotionModule::motion_kind(fighter2_boma);
            //             let fighter_kind2 = get_kind(fighter2_boma);

            //             if fighter2 > fighter1 { //if fighter 2 is smaller, the loop has already ran for that player pair. skip.
            //                 if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_FURAFURA_STAND, *FIGHTER_STATUS_KIND_SLEEP, *FIGHTER_STATUS_KIND_BIND, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_TURN].contains(&status_kind1) 
            //                   || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_FURAFURA_STAND, *FIGHTER_STATUS_KIND_SLEEP, *FIGHTER_STATUS_KIND_BIND, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_TURN].contains(&status_kind2)
            //                   || [hash40("attack11"), hash40("attack_100_start"), hash40("attack100"), hash40("attack_100_end"), hash40("attack12"), hash40("attack13"), hash40("attacks3"), hash40("attacks3hi"), hash40("attacks3lw"), hash40("attacks32"), hash40("attacks33"), hash40("attackdash")].contains(&curr_motion_kind1)
            //                   || [hash40("attack11"), hash40("attack_100_start"), hash40("attack100"), hash40("attack_100_end"), hash40("attack12"), hash40("attack13"), hash40("attacks3"), hash40("attacks3hi"), hash40("attacks3lw"), hash40("attacks32"), hash40("attacks33"), hash40("attackdash")].contains(&curr_motion_kind2)
            //                   || [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_GANON, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_SNAKE].contains(&fighter_kind1) 
            //                   || [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_GANON, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_SNAKE].contains(&fighter_kind2) {
            //                     jostle_on = true;
            //                 }
            //                 if jostle_on {
            //                     if (PostureModule::pos_x(fighter1_boma) - PostureModule::pos_x(fighter2_boma)).abs() < 14.0 && (PostureModule::pos_y(fighter1_boma) - PostureModule::pos_y(fighter2_boma)).abs() < 20.0 {
            //                         JostleModule::set_team(fighter1_boma,  fighter1 + 1);
            //                         JostleModule::set_team(fighter2_boma,  fighter2 + 1);
            //                     }
            //                 }
            //                 else {
            //                     JostleModule::set_team(fighter1_boma, 0);
            //                     JostleModule::set_team(fighter2_boma, 0);
            //                 }
            //             }
            //         }
            //     }
            // }
        }
    }
}

#[weapon_frame_callback]
fn global_weapon_frame(fighter_base : &mut L2CFighterBase) {
    unsafe {
        if smash::app::utility::get_category(&mut *fighter_base.module_accessor) == *BATTLE_OBJECT_CATEGORY_WEAPON {
            let weapon_kind = get_kind(&mut *fighter_base.module_accessor);
            let status_kind = StatusModule::status_kind(fighter_base.module_accessor);
            let situation_kind = StatusModule::situation_kind(fighter_base.module_accessor);

            let weapon =  mem::transmute::<&mut BattleObject, &mut smash::app::Weapon>(smash::app::sv_system::battle_object(fighter_base.lua_state_agent));
            let owner_id = smash::app::lua_bind::Weapon::get_founder_id(weapon) as u32;

            //weapons using owner_fighter.module_accessor stuff
            if !smash::app::sv_battle_object::is_null(owner_id) && smash::app::sv_battle_object::is_active(owner_id) {
                let owner_boma = &mut *sv_battle_object::module_accessor(owner_id);
                let owner_status = StatusModule::status_kind(owner_boma);

                let owner_object_id = owner_boma.battle_object_id;
                let owner_object = get_battle_object_from_id(owner_object_id);

                //rosalina
                if weapon_kind == *WEAPON_KIND_ROSETTA_TICO {
                    //return if rosa cancels special n charge
                    if VarModule::is_flag(owner_object, rosetta::instance::flag::TICO_RECALL) {
                        WorkModule::on_flag(owner_boma, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_RETURN);
                        VarModule::off_flag(owner_object, rosetta::instance::flag::TICO_RECALL);
                    }
                    //recover from helplessness
                    if status_kind == *WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FALL {
                        if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&owner_status) {
                            WorkModule::on_flag(fighter_base.module_accessor, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_RETURN);
                        }
                    }
                    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&owner_status) || WorkModule::is_flag(owner_boma, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
                        HitModule::set_whole(fighter_base.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                    }
                }

                //zelda
                if weapon_kind == *WEAPON_KIND_ZELDA_DEIN_S {
                    if AttackModule::is_infliction(fighter_base.module_accessor, *COLLISION_KIND_MASK_HIT) {
                        VarModule::on_flag(owner_object, commons::instance::flag::GALEFORCE_ATTACK_ON);
                    }
                }
                if weapon_kind == *WEAPON_KIND_ZELDA_PHANTOM {
                    //attempt at restoring 9.0.1 behavior. base offset was changed to detect ground when the phantom is higher up, and then it is set back down here
                    if situation_kind == *SITUATION_KIND_GROUND {
                        VarModule::set_float(fighter_base.battle_object, commons::instance::float::ECB_OFFSET_Y, 7.0);
                        GroundModule::set_rhombus_offset(fighter_base.module_accessor, &Vector2f{x : 0.0, y : VarModule::get_float(fighter_base.battle_object, commons::instance::float::ECB_OFFSET_Y)});
                    }
                    //ecb_shifts(fighter, status_kind, situation_kind, weapon_kind);
                    if !ReflectModule::is_reflect(fighter_base.module_accessor) {
                        if ![*WEAPON_ZELDA_PHANTOM_STATUS_KIND_ATTACK, *WEAPON_ZELDA_PHANTOM_STATUS_KIND_DISAPPEAR].contains(&status_kind)
                           && WorkModule::is_flag(fighter_base.module_accessor, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_FLAG_CANCEL) 
                           && !is_status_damage(owner_boma)
                           && WorkModule::get_float(fighter_base.module_accessor, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_FLOAT_HP) > 0.0 {
                            StatusModule::change_status_request(fighter_base.module_accessor, *WEAPON_ZELDA_PHANTOM_STATUS_KIND_BUILD, true);
                        }
                        if is_status_damage(owner_boma) || KineticModule::get_kinetic_type(fighter_base.module_accessor) == *WEAPON_KINETIC_TYPE_RESET {
                            WorkModule::set_float(fighter_base.module_accessor, 0.0, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_FLOAT_HP);
                            WorkModule::on_flag(fighter_base.module_accessor, *WEAPON_ZELDA_PHANTOM_INSTANCE_WORK_ID_FLAG_CANCEL);
                        }
                    }
                }
            }
        }
    }
}

pub fn install() {
    
    smashline::install_agent_frame_callback!(
        global_fighter_frame
    );
    smashline::install_agent_frame_callback!(
        global_weapon_frame
    );
}