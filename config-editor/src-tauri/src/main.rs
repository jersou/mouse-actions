use mouse_actions::args::MouseActionsCommands;

pub mod config_editor;

fn main() {
    let args = mouse_actions::args::parse();
    println!("{args:#?}");
    if let None = args.command {
        config_editor::open_config_editor();
    } else {
        mouse_actions::process_args::main();
    }
}
