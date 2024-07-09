use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    thread,
    time::Duration,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Open the file
    let file = File::open("ascii_art.txt")?;
    let reader = BufReader::new(file);

    // Run the application
    let res = run_app(&mut terminal, reader);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut reader: BufReader<File>) -> io::Result<()> {
    let mut lines = Vec::new();
    let mut buf = String::new();
    let mut done_reading = false;

    loop {
        // Non-blocking read of a line from the file
        if !done_reading {
            if let Ok(n) = reader.read_line(&mut buf) {
                if n == 0 {
                    done_reading = true;
                } else {
                    lines.push(buf.trim_end().to_string());
                    buf.clear();
                }
            }
        }

        // Draw the updated lines
        terminal.draw(|f| {
            let size = f.size();

            let block = Block::default()
                .title("Ratatui ASCII Art")
                .borders(Borders::ALL);

            let text = Text::from(lines.clone().join("\n"));

            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(Color::White));

            f.render_widget(paragraph, size);
        })?;

        // Exit if 'q' is pressed
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }

        // Sleep for a short duration to simulate incremental loading
        thread::sleep(Duration::from_millis(100));
    }
}
