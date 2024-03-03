use ratatui::{
    layout::Rect, style::{Color, Stylize}, widgets::{Block, Borders, Paragraph}, Frame
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let b = Block::default()
        .title(format!("frame.size {:?}", frame.size()))
        .borders(Borders::ALL)
        .bg(Color::DarkGray);
    frame.render_widget(b, frame.size());

    let ball = Block::default()
        .style(Color::Red)
        .bg(Color::Red);
    let ball_size: Rect = Rect {
        x: app.ball_center.0,
        y: app.ball_center.1,
        width: app.ball_size.0*2,
        height: app.ball_size.1*2,
    };
    frame.render_widget(ball, ball_size);

    let p = Paragraph::new(format!(
        "Ball Center: {:?}\nBounds: {:?}\nBall Render: {:?}",
        app.ball_center, app.ball_bounds, ball_size
    )).style(Color::LightRed);
    let mut p_size = frame.size();
    p_size.y += 2;
    p_size.x += 2;
    frame.render_widget(p, p_size);
}
