use clap::{Parser, Subcommand};

/// mouse_actions allows to execute some commands from mouse events such as
/// clicks on the side / corners of the screen, or drawing shapes.
/// It's a mix between Easystroke and Compiz edge commands.
/// https://github.com/jersou/mouse-actions
#[derive(Parser, Debug, Clone, PartialEq)]
#[clap(author, about, verbatim_doc_comment)]
pub struct Args {
    /// don't run the listen thread (autodetected on Wayland), the edge bindings might not work
    #[clap(short, long)]
    pub no_listen: bool,

    /// config path, default : ~/.config/mouse-actions.json
    #[clap(short, long)]
    pub config_path: Option<String>,

    #[clap(subcommand)]
    pub command: Option<MouseActionsCommands>,

    /// print version
    #[clap(short, long)]
    pub version: bool,

    /// log level : error, warn, info, debug, trace. [default=info]
    #[clap(short, long)]
    pub log_level: Option<String>,
}

#[derive(Debug, Subcommand, PartialEq, Clone)]
pub enum MouseActionsCommands {
    /// Default command with mouse-actions-gui, show Mouse Actions Config Editor
    #[clap()]
    ShowGui,

    /// Default command with mouse-actions, Start mouse_actions bindings
    #[clap()]
    Start,

    /// Trace events
    #[clap()]
    Trace,

    /// Start record mode to add some mouse bindings
    #[clap()]
    Record,

    /// List the current config bindings
    #[clap()]
    ListBindings,

    /// Grab one event, print it and exit
    #[clap()]
    GrabOneEvent,

    /// Stop mouse action
    #[clap()]
    Stop,

    /// Get mouse action status : exit 0 if running
    #[clap()]
    Status,

    /// print the json config
    #[clap()]
    ShowConfig,

    /// set the json config from stdin
    #[clap()]
    SetConfig,
}

pub fn parse() -> Args {
    Args::parse()
}
