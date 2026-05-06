mod model;
mod tui;
mod update;
mod view;

use model::Model;
pub fn run() -> anyhow::Result<()> {
    let mut terminal = tui::init_terminal()?;

    let mut model = Model::new();

    while model.running_state != model::RUNSTATE::END {
        terminal.draw(|f| view::draw(&mut model, f));
        let mut current_msg = tui::handle_event()?;
        while current_msg != update::MESSAGE::NONE {
            current_msg = update::update(&mut model, current_msg);
        }

    }
    tui::exit_terminal()?;

    Ok(())
}
