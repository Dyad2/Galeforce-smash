use super::*;

#[smashline::fighter_frame_callback(main)]
fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let attacker_number = get_attacker_number(&mut *fighter.module_accessor);
        TOTAL_FIGHTER = FighterManager::total_fighter_num(singletons::FighterManager());
        
        //resetting vars. in theory this should be done individually for each var, not here in opff
        if status_kind == *FIGHTER_STATUS_KIND_DEAD || KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_RESET {
            CustomVarManager::reset_var_module(fighter.battle_object);
        }

        //autocancel overlay. should be in status, opff makes it a frame late.
        //  can't find a valid status for it
        if is_training_mode() {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) && status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
                let cmb_vec1 = Vector4f{x: 0.35, y: 0.35, z: 0.66, w: 1.0};
                let cmb_vec2 = Vector4f{x: 0.35, y: 0.35, z: 0.66, w: 1.0};
                ColorBlendModule::set_main_color(fighter.module_accessor, &cmb_vec1, &cmb_vec2, 1.0, 0.33, 0, true);
            }
            else {
                ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
            }
        }

        edge_cancels::run(fighter, status_kind, situation_kind);
        ecb_shifts::run(fighter);
        controls::run(fighter);
        
        //galeforce attack opff stuff
        //Corrin
        if status_kind == *FIGHTER_STATUS_KIND_KAMUI_PIERCE && attacker_number < 8 {
            let attacker_object_id =  (*get_boma(attacker_number as i32)).battle_object_id;
            let attacker_object = get_battle_object_from_id(attacker_object_id);
            if smash::app::utility::get_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KIND_KAMUI /*&& StatusModule::status_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL*/ {
                VarModule::on_flag(attacker_object, commons::instance::flag::GALEFORCE_ATTACK_ON); //gives corrin their buff
                VarModule::set_int(fighter.battle_object, commons::instance::int::KAMUI_DRAGON_HEX_DURATION, 600); //hold timer while the opponent is pinned
            }
        }
        if VarModule::get_int(fighter.battle_object, commons::instance::int::KAMUI_DRAGON_HEX_DURATION) > 0 {
            AttackModule::set_power_mul(fighter.module_accessor, 0.75);
            VarModule::add_int(fighter.battle_object, commons::instance::int::KAMUI_DRAGON_HEX_DURATION, -1);
            galeforce::kamui_debuff_effect(fighter);
        }
        else {
            AttackModule::set_power_mul(fighter.module_accessor, 1.0);
        }
        //Puff (purin)
            // when rest is used vs an opponent without the mark, reduces endlag on rest and heals
            //gives opponent the mark when hit by up special
        if StopModule::is_hit(fighter.module_accessor) && attacker_number < 8 {
            let puff_curr_motion = MotionModule::motion_kind(&mut *get_boma(attacker_number as i32));
            if smash::app::utility::get_kind(&mut *get_boma(attacker_number as i32)) == *FIGHTER_KIND_PURIN &&
             [hash40("special_hi_r"), hash40("special_hi_l"), hash40("special_air_hi_r"), hash40("special_air_hi_l")].contains(&puff_curr_motion) &&
             !VarModule::is_flag(fighter.battle_object, commons::instance::flag::PURIN_MARK) {
                VarModule::on_flag(fighter.battle_object, commons::instance::flag::PURIN_MARK);
                VarModule::set_int(fighter.battle_object, commons::instance::int::PURIN_MARK_DURATION, 420);
            }
        }
        //mark decay and cleanup
        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::PURIN_MARK) {
            galeforce::zelda_buff_effect(fighter);
            VarModule::sub_int(fighter.battle_object, commons::instance::int::PURIN_MARK_DURATION, 1);
            if VarModule::get_int(fighter.battle_object, commons::instance::int::PURIN_MARK_DURATION) <= 0 {
                VarModule::off_flag(fighter.battle_object, commons::instance::flag::PURIN_MARK);
            }
        }

        //jostle stuff
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) == 0 { //run only once
            for fighter1 in 0 .. TOTAL_FIGHTER {
                let mut jostle_on = false;

                let fighter1_boma = get_boma(fighter1);
                let status_kind1 = StatusModule::status_kind(fighter1_boma);
                let curr_motion_kind1 = MotionModule::motion_kind(fighter1_boma);
                let fighter_kind1 = get_kind(&mut *sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(fighter1)));
                for fighter2 in 0 .. TOTAL_FIGHTER {
                    let fighter2_boma = get_boma(fighter2);
                    let status_kind2 = StatusModule::status_kind(fighter2_boma);
                    let curr_motion_kind2 = MotionModule::motion_kind(fighter2_boma);
                    let fighter_kind2 = get_kind(&mut *sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(fighter2)));

                    if fighter2 > fighter1 { //if fighter 2 is smaller, the loop has already ran for that player pair. skip.
                        if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_FURAFURA_STAND, *FIGHTER_STATUS_KIND_SLEEP, *FIGHTER_STATUS_KIND_BIND, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_TURN].contains(&status_kind1) 
                          || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_FURAFURA_STAND, *FIGHTER_STATUS_KIND_SLEEP, *FIGHTER_STATUS_KIND_BIND, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_TURN].contains(&status_kind2)
                          || [hash40("attack11"), hash40("attack_100_start"), hash40("attack100"), hash40("attack_100_end"), hash40("attack12"), hash40("attack13"), hash40("attacks3"), hash40("attacks3hi"), hash40("attacks3lw"), hash40("attacks32"), hash40("attacks33"), hash40("attackdash")].contains(&curr_motion_kind1)
                          || [hash40("attack11"), hash40("attack_100_start"), hash40("attack100"), hash40("attack_100_end"), hash40("attack12"), hash40("attack13"), hash40("attacks3"), hash40("attacks3hi"), hash40("attacks3lw"), hash40("attacks32"), hash40("attacks33"), hash40("attackdash")].contains(&curr_motion_kind2)
                          || [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_GANON, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_SNAKE].contains(&fighter_kind1) 
                          || [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_GANON, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_SNAKE].contains(&fighter_kind2) {
                            jostle_on = true;
                        }
                        if jostle_on {
                            if (PostureModule::pos_x(fighter1_boma) - PostureModule::pos_x(fighter2_boma)).abs() < 14.0 && (PostureModule::pos_y(fighter1_boma) - PostureModule::pos_y(fighter2_boma)).abs() < 20.0 {
                                JostleModule::set_team(fighter1_boma,  fighter1 + 1);
                                JostleModule::set_team(fighter2_boma,  fighter2 + 1);
                            }
                        }
                        else {
                            JostleModule::set_team(fighter1_boma, 0);
                            JostleModule::set_team(fighter2_boma, 0);
                        }
                    }
                }
            }
        }
    }
}

#[weapon_frame_callback]
fn global_weapon_frame(fighter_base : &mut L2CFighterBase) {
    unsafe {
        let weapon_kind = get_kind(&mut *fighter_base.module_accessor);
        let status_kind = StatusModule::status_kind(fighter_base.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter_base.module_accessor);

        let weapon =  mem::transmute::<&mut BattleObject, &mut smash::app::Weapon>(smash::app::sv_system::battle_object(fighter_base.lua_state_agent));
        let owner_id = smash::app::lua_bind::Weapon::get_founder_id(weapon) as u32;

        //weapons using owner_fighter.module_accessor stuff
        //let owner_id = smash::app::lua_bind::WorkModule::get_int(module_accessor, WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
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
            //if !is_operation_cpu( owner_boma) {
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
            //}
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