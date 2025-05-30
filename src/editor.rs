use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    text::{Span, Spans, Text}, // ✅ works in tui v0.19
    style::{Style, Color},
};
use std::io::{stdout, Write, Result};
use crossterm::event::KeyEvent;

pub fn start_editor_with_content(initial: &str) -> Result<String> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input_lines: Vec<String> = initial.lines().map(String::from).collect();
    if input_lines.is_empty() {
        input_lines.push(String::new());
    }

    let mut cursor_y = input_lines.len() - 1;
    let mut cursor_x = input_lines[cursor_y].chars().count();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(3)].as_ref())
                .split(size);

            let mut text = Text::default();
            for line in &input_lines {
                text.lines.push(parse_spans(line));
            }

            let paragraph = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL).title("✏️ Editing Note"))
                .scroll(((cursor_y as u16).saturating_sub((chunks[0].height - 3) as u16), 0))
                .style(Style::default());

            f.render_widget(paragraph, chunks[0]);

            let x = cursor_x as u16 + 1;
            let y = (cursor_y as u16 + 1)
                .saturating_sub((cursor_y as u16).saturating_sub(chunks[0].height - 3));
            f.set_cursor(x, y);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char(c) => {
                        let line = &mut input_lines[cursor_y];
                        let byte_idx = line.char_indices().nth(cursor_x).map(|(i, _)| i).unwrap_or(line.len());
                        line.insert(byte_idx, c);
                        cursor_x += 1;
                    }
                    KeyCode::Enter => {
                        let new_line = input_lines[cursor_y].split_off(cursor_x);
                        input_lines.insert(cursor_y + 1, new_line);
                        cursor_y += 1;
                        cursor_x = 0;
                    }
                    KeyCode::Backspace => {
                        if cursor_x > 0 {
                            let line = &mut input_lines[cursor_y];
                            let byte_idx = line.char_indices().nth(cursor_x - 1).map(|(i, _)| i).unwrap_or(0);
                            line.remove(byte_idx);
                            cursor_x -= 1;
                        } else if cursor_y > 0 {
                            let removed = input_lines.remove(cursor_y);
                            cursor_y -= 1;
                            cursor_x = input_lines[cursor_y].chars().count();
                            input_lines[cursor_y].push_str(&removed);
                        }
                    }
                    KeyCode::Up => {
                        if cursor_y > 0 {
                            cursor_y -= 1;
                            cursor_x = cursor_x.min(input_lines[cursor_y].chars().count());
                        }
                    }
                    KeyCode::Down => {
                        if cursor_y + 1 < input_lines.len() {
                            cursor_y += 1;
                            cursor_x = cursor_x.min(input_lines[cursor_y].chars().count());
                        }
                    }
                    KeyCode::Left => {
                        if cursor_x > 0 {
                            cursor_x -= 1;
                        } else if cursor_y > 0 {
                            cursor_y -= 1;
                            cursor_x = input_lines[cursor_y].chars().count();
                        }
                    }
                    KeyCode::Right => {
                        if cursor_x < input_lines[cursor_y].chars().count() {
                            cursor_x += 1;
                        } else if cursor_y + 1 < input_lines.len() {
                            cursor_y += 1;
                            cursor_x = 0;
                        }
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(input_lines.join("\n"))
}

pub fn parse_spans(line: &str) -> Spans<'_> {
    let known_cmds = ["/alldone", "/reset", "/bold", "/italic", "/hr"];
    let mut spans = vec![];
    let mut buf = String::new();
    let mut in_command = false;

    for ch in line.chars() {
        if ch == '/' {
            if !buf.is_empty() {
                spans.push(Span::raw(buf.clone()));
                buf.clear();
            }
            buf.push(ch);
            in_command = true;
        } else if ch.is_whitespace() {
            buf.push(ch);
            if in_command && known_cmds.iter().any(|cmd| cmd.starts_with(buf.trim_end())) {
                spans.push(Span::styled(buf.clone(), Style::default().fg(Color::Yellow)));
            } else {
                spans.push(Span::raw(buf.clone()));
            }
            buf.clear();
            in_command = false;
        } else {
            buf.push(ch);
        }
    }

    if !buf.is_empty() {
        if in_command && known_cmds.iter().any(|cmd| cmd.starts_with(&buf)) {
            spans.push(Span::styled(buf.clone(), Style::default().fg(Color::Yellow)));
        } else {
            spans.push(Span::raw(buf.clone()));
        }
    }

    Spans::from(spans)
}

