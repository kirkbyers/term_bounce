use ratatui::{
    layout::Rect, style::{Color, Stylize}, widgets::{Block, Borders, Paragraph}, Frame
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let b = Block::default()
        .bg(Color::DarkGray);
    frame.render_widget(b, frame.size());

    let ball = Block::default()
        .bg(Color::Red);
    let ball_size: Rect = Rect {
        x: app.ball_pos.0,
        y: app.ball_pos.1,
        width: app.ball_size.0,
        height: app.ball_size.1,
    };
    frame.render_widget(ball, ball_size);

    let p = Paragraph::new(format!("
        Ball Center: {:?}\n
        Bounds: {:?}\n
        Ball Render: {:?}\n
        Ball Size: {:?}\n
    ",
        app.ball_pos, app.ball_bounds, ball_size, app.ball_size
    ))
    .fg(Color::LightRed);
    let mut p_size = frame.size();
    p_size.y += 2;
    p_size.x += 2;
    // frame.render_widget(p, p_size);
}
