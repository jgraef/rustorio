mod vanilla;

use rustorio_core::blueprint::types::SignalID;


pub fn get_vanilla_signals() -> &'static [SignalID] {
    &vanilla::VANILLA_SIGNALS
}

// TODO: Helper methods to load signals from prototypes.
