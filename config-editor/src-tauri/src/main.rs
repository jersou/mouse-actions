use mouse_actions::args::MouseActionsCommands::ShowGui;

pub mod config_editor;

fn main() {
    let args = mouse_actions::args::parse();
    println!("{args:#?}");
    if args.command == None || args.command == Some(ShowGui) {
        config_editor::open_config_editor();
    } else {
        mouse_actions::process_args::main();
    }
}
