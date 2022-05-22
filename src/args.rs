use clap::{Parser, Subcommand};

/// https://github.com/jersou/mouse-actions
/// mouse_actions allows to execute some command from mouse events such as clicks on
/// the side / corners of the screen, or drawing shapes. It's a mix between
/// Easystroke and Compiz edge commands.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// don't run the listen thread (for Wayland), the edge bindings might not work
    #[clap(short, long)]
    pub no_listen: bool,

    #[clap(subcommand)]
    pub command: Option<MouseActionsCommands>,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum MouseActionsCommands {
    /// Default command, use mouse_actions bindings
    #[clap()]
    Start,

    /// Open the config file (xdg-open)
    #[clap()]
    OpenConfig,

    /// Trace events
    #[clap()]
    Trace,

    /// Start record mode to add some mouse bindings
    #[clap()]
    Record,
}
