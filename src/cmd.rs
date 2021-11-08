//! Commands for the bot.
use paste::paste;

macro_rules! add_command {
    ($cmd:ident) => {
        mod $cmd;
        paste! {
            pub use $cmd::handler as [<handler__ $cmd>];
        }
    };
}

add_command!(card);
add_command!(curr_borders);
add_command!(curr_event);
add_command!(last_event);
