#![allow(non_upper_case_globals)]

use {
    std::sync::Once,
    smash::{
        //phx::Vector3f,
        app::*,
    }
};

/*
WuBoy's VarModule Constant Overhaul!
The way our variable constants are labelled is changing.
Variables now have two categories:
INSTANCE, which persists until manually changed. Represented by 0x0XXX.
STATUS, which is automatically reset when the status changes. Represented by 0x1XXX.
In addition, there are two sub-categories.
Common, which is shared by every fighter. Represented by 0xX0XX.
Agent, which is specific to a certain fighter/agent. Represented by 0xX1XX.
This means for each combination, you have access to 256 common variables and 256 agent variables.
That's a LOT of space, and I don't anticipate it all gets used up with proper variable application.
*/

// System
pub static mut INT_OFFSET : usize = 0x4E19D0;
// pub static mut INT64_OFFSET : usize = 0x4E19F0;
pub static mut FLOAT_OFFSET : usize = 0x4E19D0;
pub static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;
pub static mut DEFINE_LUA_CONSTANT_OFFSET : usize = 0x3727390; //13.0.1
pub static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];
pub static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];
// pub static INT64_SEARCH_CODE: &[u8] = &[
//     0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x15, 0x40, 0xf9,
// ];
pub static NOTIFY_LOG_EVENT_COLLISION_HIT_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1,
    0xe8, 0x2b, 0x00, 0xfd,
    0xfc, 0x6f, 0x06, 0xa9,
    0xfa, 0x67, 0x07, 0xa9,
    0xf8, 0x5f, 0x08, 0xa9,
    0xf6, 0x57, 0x09, 0xa9,
    0xf4, 0x4f, 0x0a, 0xa9,
    0xfd, 0x7b, 0x0b, 0xa9,
    0xfd, 0xc3, 0x02, 0x91,
    0xfb, 0x03, 0x00, 0xaa
];

// pub mod archetype {
//     pub mod aggressive {
//         pub const All_ROUNDER   : i32 = 0x0000;
//         pub const RUSHDOWN      : i32 = 0x0001;
//         pub const GRAPPLER      : i32 = 0x0002;
//         pub const LEDGE_TRAP    : i32 = 0x0003;
//     }
//     pub mod defensive {
//         pub const ZONER         : i32 = 0x0000;
//         pub const SETPLAY       : i32 = 0x0001;
//         pub const BAIT_PUNISH   : i32 = 0x0002;
//     }
//     pub mod traits {
//         pub const HEAVY         : i32 = 0x0000;
//         pub const LIGHTWEIGHT   : i32 = 0x0001;
//     }
// }

pub mod commons {
    pub mod instance {
        pub mod flag {
            pub const DISABLE_SPECIAL_N : i32 = 0x0000;
            pub const DISABLE_SPECIAL_S : i32 = 0x0001;
            pub const DISABLE_SPECIAL_HI : i32 = 0x0002;
            pub const DISABLE_SPECIAL_LW : i32 = 0x0003;
            pub const DISABLE_SPECIAL_ALL : i32 = 0x0004;
            pub const AIR_TURN_STICK_RELEASED : i32 = 0x0005;
            pub const AIR_TURN_INVALID : i32 = 0x0006;
            pub const GALEFORCE_ATTACK_ON : i32 = 0x0007;
            pub const GALEFORCE_ATTACK_CONFIRM : i32 = 0x0008;
            pub const DO_ONCE : i32 = 0x0009;
            pub const PLATFORM_FALL_STUN : i32 = 0x000A;
            pub const ALLOW_REVERSE_ATTACK_LW3 : i32 = 0x000B;
            pub const WAVEDASH : i32 = 0x000C;
            pub const IS_VICTIM_GANON_GA : i32 = 0x000D;
            pub const PURIN_MARK : i32 = 0x000E;
            pub const ALLOW_PERFECT_PIVOT : i32 = 0x000F;
            pub const SMASH_TURN : i32 = 0x0010;
            pub const MEWTWO_PRESSURED : i32 = 0x0011;
            pub const HIT_CANCEL: i32 = 0x0012;
        }
        pub mod int {
            pub const FRAME_COUNTER : i32 = 0x0000; //make this sure is used for only one thing per character or it'll break
            pub const AIR_TURN_INITIATE_METHOD : i32 = 0x0001;
            pub const AIR_TURN_INPUT_FRAME : i32 = 0x0002;
            pub const AIR_TURN_COUNT : i32 = 0x0003;
            pub const SUBSTICK_AIR_ATTACK : i32 = 0x0004;
            pub const PURIN_MARK_DURATION : i32 = 0x0005;
            pub const KAMUI_DRAGON_HEX_DURATION : i32 = 0x0006;
            pub const HIT_CANCEL_FRAME_COUNTER : i32 = 0x0007; 
        }
        pub mod float {
            pub const STICK_X : i32 = 0x0000;
            pub const STICK_Y : i32 = 0x0001;
            pub const SUBSTICK_X : i32 = 0x0002;
            pub const SUBSTICK_Y : i32 = 0x0003;
            pub const ECB_OFFSET_Y : i32 = 0x0004;
        }
    }
     pub mod status {
         pub mod flag {
            pub const DISABLE_BACKDASH: i32 = 0x1000;
            //pub const SHIELD_BREAK_ONCE: i32 = 0x1001;
            //pub const WAVELAND : i32 = 0x1002;
            //pub const ESCAPE_AIR_IS_SLIDE : i32 = 0x1003;
            pub const JUMP_SQUAT_TO_ESCAPE_AIR : i32 = 0x1001;
            // pub const DASH_CANCEL : i32 = 0x1002;
            // pub const SPECIAL_CANCEL : i32 = 0x1003;
         }
         //pub mod int {
         //    pub const ENABLED_AERIALS : i32 = 0x1000;
         //}
         //pub mod float {
         //    pub const HIT_FRAME : i32 = 0x1000;
         //}
    }
}

pub mod bayonetta {
    pub mod instance {
        pub mod flag {
            pub const DODGE_OFFSET : i32 = 0x0100;
            pub const DODGE_OFFSET_SECOND : i32 = 0x0101;
            //pub const ENABLE_CANCEL_INTO_SPECIAL_N : i32 = 0x0102;
        }
        pub mod int {
            pub const DODGE_OFFSET_NUM : i32 = 0x0101;
        }
    }
    pub mod status {
        pub mod flag {
            pub const SPECIAL_HI_SHOOT : i32 = 0x1100;
            pub const DODGE_OFFSET_FORBID : i32 = 0x1101;
        }
    }
}
pub mod captain {
    pub mod status {
        pub mod flag {
            pub const ALLOW_SPECIAL_N_CRITICAL : i32 = 0x1100;
        }
    }
}
pub mod donkey {
    pub mod instance {
        pub mod flag {
            pub const GET_BARREL : i32 = 0x0100;
        }
    }
}
pub mod dolly { //terry
    pub mod instance {
        pub mod float {
            pub const SPECIAL_N_CHARGE : i32 = 0x0100;
        }
    }
}
pub mod duckhunt {
    pub mod instance {
        pub mod flag {
            pub const GUNMAN_REACTIVATE : i32 = 0x0100;
        }
    }
}
pub mod edge { //sephiroth
    pub mod instance {
        pub mod flag {
            pub const FLARE_EXISTS : i32 = 0x0100;
        }
    }
}
pub mod falco {
    pub mod instance {
        pub mod flag {
            pub const DIRECTIONAL_AIR_ESCAPE_FAF : i32 = 0x0100;
            pub const AIRDASH : i32 = 0x0101;
        }
        pub mod float {
            pub const STICK_Y : i32 = 0x0100;
            pub const STICK_X : i32 = 0x0101;
        }
    }
}
pub mod gaogaen { //incineroar
    pub mod instance {
        pub mod flag {
            pub const REVENGE_REDUCE_ONCE : i32 = 0x100;
        }
        pub mod float {
            pub const REVENGE_BONUS_PRESERVE : i32 = 0x0100;
        }
    }
}
pub mod gamewatch {
    pub mod instance {
        pub mod int {
            pub const JUDGE_STORED_KIND : i32 = 0x0100;
        }
    }
}
pub mod gekkouga { //greninja
    pub mod instance {
        pub mod float {
            pub const SHURICHARGE : i32 = 0x0100;
        }
    }
}
pub mod kamui { //corrin
    pub mod instance {
        pub mod int {
            pub const GA_DURATION : i32 = 0x0100;
        }
    }
}
pub mod kirby {
    pub mod instance {
        pub mod int {
            pub const LAST_HAT : i32 = 0x0100;
        }
    }
}
pub mod lucario { 
    pub mod instance {
        pub mod int {
            pub const MAX_AURA_TIMER : i32 = 0x0100;
        }
        pub mod float {
            pub const AURA_SCALE : i32 = 0x0100;
        }
        pub mod flag {
            pub const ATTACK_AIR_LW_CHARGED : i32 = 0x0100;
        }
    }
}
pub mod link {
    pub mod instance {
        pub mod float {
            pub const DAMAGE_STORAGE : i32 = 0x0100;
        }
        pub mod flag {
            pub const RESTORE_DAMAGE : i32 = 0x0100;
        }
    }
}
pub mod luigi {
    pub mod instance {
        pub mod int {
            pub const ELEC_CHARGE : i32 = 0x0100;
        }
    }
    pub mod status {
        pub mod flag {
            pub const SPECIAL_S_SPECIAL_BUTTON_RELEASED : i32 = 0x1100;
            pub const SPECIAL_S_CHARGE_USED : i32 = 0x1101;
        }
    }
}
pub mod mariod { 
    pub mod instance {
        pub mod int {
            pub const GA_MEDECINE_TIMER : i32 = 0x0100;
            pub const ATTACK_AIR_N_SOUND_LEVEL : i32 = 0x0101;
        }
        pub mod float {
            pub const ATTACK_AIR_N_DAMAGE : i32 = 0x0100;
        }
    }
}
pub mod marcina { //marth and lucina
    pub mod instance {
        pub mod flag {
            pub const LUCINA_SPECIAL_HI_LANDING : i32 = 0x0100;
        }
    }
}
pub mod master { //byleth
    pub mod instance {
        pub mod flag {
            pub const FAILNAUGHT_TO_AIRN : i32 = 0x0100;
        }
    }
}
pub mod metaknight {
    pub mod instance {
        pub mod flag {
            pub const MACH_TORNADO_HIT : i32 = 0x0100;
        }
    }
}
pub mod peach {
    pub mod instance {
        pub mod flag {
            pub const ALLOW_FLOAT : i32 = 0x0100;
        }
    }
}
pub mod pickel {
    pub mod instance {
        pub mod flag {
            pub const ALLOW_SPECIAL_N : i32 = 0x0100;
        }
    }
}
pub mod reflet {
    pub mod instance {
        pub mod flag {
            pub const GALEFORCE_ATTACK_ATTACK_OCCUR : i32 = 0x0100;
        }
        pub mod float {
            pub const GALEFORCE_ATTACK_INPUT_WINDOW : i32 = 0x0100;
        }
    }
    pub mod status {
        pub mod flag {
            pub const ATTACK_BUTTON_RELEASED : i32 = 0x1100;
        }
    }
}
pub mod rosetta { //rosalina
    pub mod instance {
        pub mod flag {
            pub const TICO_RECALL : i32 = 0x0100;
        }
    }
}
// pub mod rockman { //megaman
//     pub mod status {
//         pub mod flag {
//             pub const ATTACK_HI4_LANDING : i32 = 0x1100;
//         }
//     }
// }
pub mod roy {
    pub mod instance {
        pub mod float {
            pub const GALEFORCE_ATTACK_TIMER : i32 = 0x0100;
        }
    }
}
pub mod sheik {
    pub mod instance {
        pub mod flag {
            pub const ATTACK_AIR_LW_W : i32 = 0x0100;
        }
    }
}
pub mod zelda {
    pub mod instance {
        pub mod flag {
            pub const SPECIAL_HI_CANCEL : i32 = 0x0100;
        }
    }
}

pub mod singletons {
    // All credit for this to blujay, macros are very cool
    use super::*;
    use skyline::nn::ro::LookupSymbol;
    
    static INIT : Once = Once::new();
    
    macro_rules! expose_singleton {
        ($($public:ident, $private:ident)*) => {
            $(
                #[inline(always)]
                #[allow(non_snake_case)]
                pub fn $public() -> *mut $public {
                    unsafe {
                        *$private
                    }
                }
            )*
        }
    }

    macro_rules! assign_symbol {
        ($id:ident, $e:expr) => {{
            unsafe {
                let mut sym = 0usize;
                LookupSymbol(&mut sym as *mut usize, $e.as_ptr() as _);
                assert!(sym != 0, "Failed to find symbol {}", $e);
                $id = std::mem::transmute(sym)
            }
        }}
    }

    pub static mut FIGHTER_MANAGER : *const *mut smash::app::FighterManager = 0 as _;
    pub static mut FIGHTER_CUTIN_MANAGER : *const *mut smash::app::FighterCutInManager = 0 as _;
    pub static mut BATTLE_OBJECT_WORLD : *const *mut smash::app::BattleObjectWorld = 0 as _;
    
    expose_singleton!(
        FighterManager,              FIGHTER_MANAGER
        FighterCutInManager,         FIGHTER_CUTIN_MANAGER
        BattleObjectWorld,           BATTLE_OBJECT_WORLD
    );

    pub fn init() {
        INIT.call_once(|| {
            assign_symbol!(
                FIGHTER_MANAGER,
                "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\0"
            );
            assign_symbol!(
                FIGHTER_CUTIN_MANAGER,
                "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\0"
            );
            assign_symbol!(
                BATTLE_OBJECT_WORLD,
                "_ZN3lib9SingletonIN3app17BattleObjectWorldEE9instance_E\0"
            );
        });
    }
}

pub fn install() {
    singletons::init();
}