//! Commands for the bot.
use paste::paste;

#[doc(hidden)]
macro_rules! add_command {
    ($cmd:ident) => {
        pub(crate) mod $cmd;
        paste! {
            pub(crate) use $cmd::handler as [<handler__ $cmd>];
        }
    };
}

add_command!(card);
add_command!(curr_borders);
add_command!(curr_event);
add_command!(last_event);
add_command!(list_characters);
add_command!(last_borders);
