
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
macro_rules! hook_function {
    (
        $detour:ident,
        $target:expr,
        $reroute:ident
    ) => {
        $detour.initialize($target, $reroute)?;
        $detour.enable()?;
    };
}

pub mod battle;
pub mod directx;