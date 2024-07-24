use std::{
    io::{self, stdout, Stdout, Read},
    time::{Duration, Instant},
    fs::File
};


use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::Rect,
    style::{Color, Style},
    terminal::{Frame, Terminal},
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget},
};

use rand::Rng;

fn main() -> io::Result<()> {
    App::run()
}

struct BouncingText {
    text: String,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    color: Color,
}

impl BouncingText {
    fn new(text: String) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            text,
            x: rng.gen_range(0.0..1.0),
            y: rng.gen_range(0.0..1.0),
            vx: rng.gen_range(-0.02..0.02),
            vy: rng.gen_range(-0.01..0.01),
            color: Color::Rgb(
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255),
            ),
        }
    }

    fn update(&mut self) {
        if self.x <= 0.0 || self.x >= 1.0 {
            self.vx = -self.vx;
        }
        if self.y <= 0.0 || self.y >= 1.0 {
            self.vy = -self.vy;
        }

        self.x += self.vx;
        self.y += self.vy;

        self.x = self.x.clamp(0.0, 1.0);
        self.y = self.y.clamp(0.0, 1.0);
    }
}

struct App {
    texts: Vec<BouncingText>,
    tick_count: u64,
}

impl App {
    fn new() -> Self {
        Self {
            texts: vec![
                BouncingText::new("Detective".to_string()),
                BouncingText::new("Trapped".to_string()),
                BouncingText::new("Clues".to_string()),
                BouncingText::new("Murder".to_string()),
                BouncingText::new("Betrayal".to_string()),
                BouncingText::new("Gun".to_string()),
                BouncingText::new("Shower".to_string()),
                BouncingText::new("Minigames".to_string()),
                BouncingText::new("Leave".to_string()),
                BouncingText::new("Scared".to_string()),
            ],
            tick_count: 0,
        }
    }

    pub fn run() -> io::Result<()> {
        
        let mut terminal = init_terminal()?;
        let mut app = Self::new();
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(50);
        loop {
            let _ = terminal.draw(|frame| app.ui(frame));
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                app.on_tick();
                last_tick = Instant::now();
            }
        }
        restore_terminal()
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
        for text in &mut self.texts {
            text.update();
        }
    }

    fn ui(&self, frame: &mut Frame) {
        let size = frame.size();
        
        let box_area = Rect::new(
            1,
            1,
            size.width.saturating_sub(2),
            size.height.saturating_sub(2),
        );
        let ascii_art = r#"
.....................................................................................................................................................................................................
.                                                                   .            ________  __    __  ________        ________  __    __  _______                                                    .
.                                                                   .            ________  __    __  ________        ________  __    __  _______                                                    .
.                                                                   .           /        |/  |  /  |/        |      /        |/  \  /  |/       \                                                   .
.                                                                   .           $$$$$$$$/ $$ |  $$ |$$$$$$$$/       $$$$$$$$/ $$  \ $$ |$$$$$$$  |                                                  .
.                                                                   .              $$ |   $$ |__$$ |$$ |__          $$ |__    $$$  \$$ |$$ |  $$ |                                                  .
.                                                                   .              $$ |   $$    $$ |$$    |         $$    |   $$$$  $$ |$$ |  $$ |                                                  .
.                                                                   .              $$ |   $$$$$$$$ |$$$$$/          $$$$$/    $$ $$ $$ |$$ |  $$ |                                                  .
.                                                                   .              $$ |   $$ |  $$ |$$ |_____       $$ |_____ $$ |$$$$ |$$ |__$$ |                                                  .
.                                                                   .              $$ |   $$ |  $$ |$$       |      $$       |$$ | $$$ |$$    $$/                                                   .
.                                                                   .              $$/    $$/   $$/ $$$$$$$$/       $$$$$$$$/ $$/   $$/ $$$$$$$/                                                    .
.....................................................................................................................................................................................................                       
                        "#;
        let block = Block::default()
            .style(Style::default().fg(Color::White));
            

        let paragraph = Paragraph::new(ascii_art)
            .block(block)
            .wrap(ratatui::widgets::Wrap { trim: true }); 
            
        frame.render_widget(paragraph, box_area);

        for bouncing_text in &self.texts {
            let text = Text::styled(&bouncing_text.text, Style::default().fg(bouncing_text.color));
            let paragraph = Paragraph::new(text).style(Style::default());

            let max_x = (box_area.width.saturating_sub(bouncing_text.text.len() as u16) - 2) as f64;
            let max_y = (box_area.height.saturating_sub(1)-2) as f64;
            let x = (bouncing_text.x * max_x).round() as u16 + box_area.left();
            let y = (bouncing_text.y * max_y).round() as u16 + box_area.top();

            let text_area = Rect::new(
                x,
                y,
                bouncing_text.text.len() as u16,
                1,
            );

            frame.render_widget(paragraph, text_area);
        }
    }
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}