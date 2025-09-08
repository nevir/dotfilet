/// Derives a [clap](https://docs.rs/clap) command and applies common configuration to it.
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
        #[derive(clap::Parser)]
        #[command(styles = $crate::ui::style::STYLES)]
        #[command(help_template = $crate::ui::style::get_help_template())]
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

// Re-export the macro for use in parent modules
pub(crate) use dotfilet_command;
