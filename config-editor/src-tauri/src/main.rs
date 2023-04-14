use mouse_actions::args::MouseActionsCommands;

pub mod config_editor;

fn main() {
    let args = mouse_actions::args::parse();
    if let Some(MouseActionsCommands::Start) = args.command {
        mouse_actions::process_args::main();
    } else {
        config_editor::open_config_editor();
    }
}
