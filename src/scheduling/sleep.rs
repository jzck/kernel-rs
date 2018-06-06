//! sleeping processing are stored in a delta queue
//! separate from other scheduling structures: this
//! way the scheduling algorithms don't have to worry about
//! managing these
//!
//! inspired from https://wiki.osdev.org/Blocking_Process
