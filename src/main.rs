mod ui;
mod app;

use clap::Parser;


#[derive(Parser)]
#[command(name = "Text editor")]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let terminal = ratatui::init();

    app::App::new(terminal, args.file).run();

    ratatui::restore();
}
