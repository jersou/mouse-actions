pub mod args;
pub mod binding;
pub mod cmd_str_spliter;
pub mod compare_angles;
pub mod config;
pub mod event;
pub mod grab;
pub mod listen;
pub mod points_to_angles;
pub mod process_args;
pub mod process_event;
pub mod record;
pub mod single_instance;
pub mod trace_svg;

#[tokio::main]
async fn main() {
    process_args::main()
}
