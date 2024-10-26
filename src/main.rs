mod ui;
mod app;

use std::cell::RefCell;

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

    let app = RefCell::new(app::App::new(args.file));
    let mut ui = ui::Ui::new(terminal, &app);

    while app.borrow().alive {
        ui.draw_app();
        let _ = app.borrow_mut().handle_events();
    }

    ratatui::restore();
}
