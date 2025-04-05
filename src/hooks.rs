
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(static_mut_refs)]

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