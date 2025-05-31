
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(static_mut_refs)]

macro_rules! subscribe_function {
    (
        $detour:ident,
        $target:expr,
        $reroute:ident
    ) => {
        $detour.initialize(std::mem::transmute($target), $reroute)?;
        $detour.enable()?;
    };
}

// Fix later
macro_rules! safe_call {
    ($body:expr) => {{
        #[allow(unused_must_use)]
        let _ = match microseh::try_seh(|| std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<()> { $body }))) {
            Ok(Ok(val)) => Ok(val),
            Ok(Err(panic)) => {
                let backtrace = std::backtrace::Backtrace::capture();
                let msg = panic
                    .downcast_ref::<&str>()
                    .map(|s| format!("Panic message: {}", s))
                    .or_else(|| panic.downcast_ref::<String>().map(|s| format!("Panic message: {}", s)))
                    .unwrap_or_else(|| format!("Unknown panic: {:#?}", panic));

                let message = format!(
                    "{} panicked {}\nBacktrace:\n{}",
                    function_name!(),
                    msg,
                    backtrace
                );

                log::error!("{}", message);
                Err(anyhow::anyhow!(message).context("Panic occurred"))
            }
            Err(seh) => {
                let backtrace = std::backtrace::Backtrace::capture();
                let message = format!(
                    "{} triggered SEH exception: {:?}\nBacktrace:\n{}",
                    function_name!(),
                    seh,
                    backtrace
                );

                log::error!("{}", message);
                Err(anyhow::anyhow!(message).context("SEH occurred"))
            }
        };
    }};
}

pub mod battle;