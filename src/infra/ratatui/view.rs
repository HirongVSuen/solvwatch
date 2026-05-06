use super::model::{Model, MODELSTATE};
use ratatui::{Frame, prelude::*, symbols::*, widgets::*};

fn main_border_ui(frame: &mut Frame) {
    let instructions = Line::from(vec![
        " 首页: ".into(),
        "<H>".blue().bold(),
        " 设置: ".into(),
        "<S>".blue().bold(),
        " 退出: ".into(),
        "<Q> ".blue().bold(),
    ]);
    let block = Block::bordered()
        .title_top(Line::from("验证节点观察器").bold().centered())
        .title_bottom(instructions.centered());
    frame.render_widget(
        block,
        frame.area(),
    );
}

pub fn draw(model: &mut Model, frame: &mut Frame) {
    main_border_ui(frame);
    match model.model_state {
        MODELSTATE::HOME => {home_ui(frame);}
        MODELSTATE::SETTING => {setting_ui(frame);}
    }

}

fn setting_ui(frame: &mut Frame) {
    let block = Block::bordered().title("xxxxxxxxxxxxxx");
    let inner_area = block.inner(frame.area());
    frame.render_widget(block, inner_area);
}

fn home_ui(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Percentage(50),
            Constraint::Percentage(50),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let top_inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Percentage(40),
            Constraint::Percentage(60),
            Constraint::Length(1),
        ])
        .split(layout[1]);

    let bottom_inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(layout[2]);

    frame.render_widget(Block::bordered().title("节点列表"), top_inner_layout[1]);

    frame.render_widget(Block::bordered().title(Line::from("最近 5 epoch积分变化").right_aligned()), top_inner_layout[2]);

    frame.render_widget(Block::bordered().title("节点信息"), bottom_inner_layout[1]);
}
