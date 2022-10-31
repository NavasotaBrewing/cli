use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Widget, ListItem, List, StatefulWidget},
    Frame, Terminal, style::{Style, Modifier, Color}, buffer::Buffer,
};

use brewdrivers::model::RTU;

struct DrawableRTU(RTU);

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
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

impl StatefulWidget for DrawableRTU {
    type State = RTU;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut content = String::new();
        // content.push_str(&format!("{} ({})\n\n", self.0.name, self.0.id));
        // for dev in &self.0.devices {
        //     content.push_str(&format!("{}: {}\t\t{:?}\n", dev.id, dev.name, dev.state));
        // }
        let items = [ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
        List::new(items)
            .block(Block::default().title("List").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");
        buf.set_string(area.left(), area.top(), content, Style::default());
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
            if let KeyCode::Char('f') = key.code {
                
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(f.size());

    let rtu = DrawableRTU(
        RTU::generate(None).unwrap()
    );


    let block = Block::default().title("Devices").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default().title("Options").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
    f.render_widget(rtu, chunks[2]);
}
