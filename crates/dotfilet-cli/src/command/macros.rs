/// Derives a [clap](https://docs.rs/clap) Command and applies common configuration to it.
///
/// Usage:
/// ```rust
/// dotfilet_command! {
///     /// Documentation for the command
///     pub struct MyCommand {
///         // fields...
///     }
/// }
/// ```
#[macro_export]
macro_rules! dotfilet_command {
    // Internal rule for applying common config
    (@apply_config $(#[$meta:meta])* $vis:vis struct $name:ident $($rest:tt)*) => {
        #[allow(unused_imports)]
        use $crate::command::dotfilet_command::DotfiletCommand as _;

        #[derive(clap::Parser)]
        #[command(setup_dotfilet_command = "no-op")]
        $(#[$meta])*
        $vis struct $name $($rest)*
    };

    // Variant for structs with fields
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($body:tt)*
        }
    ) => {
        dotfilet_command!(@apply_config $(#[$meta])* $vis struct $name {
            $($body)*
        });
    };

    // Variant for unit structs
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident;
    ) => {
        dotfilet_command!(@apply_config $(#[$meta])* $vis struct $name;);
    };
}
