use crossterm::{
    cursor::{position, MoveTo, MoveToColumn, MoveToNextLine, MoveToPreviousLine},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use std::{
    error::Error,
    io::{stdout, Write},
};

fn main() -> Result<(), std::io::Error> {
    let code_snippet = "This is code!!!!\nOne more line??????????";

    let mut stdout = stdout();

    enable_raw_mode()?;

    execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0),
        Print(code_snippet),
        MoveTo(0, 0)
    )?;

    let original_chars: Vec<char> = code_snippet.chars().collect();
    let original_lines: Vec<&str> = code_snippet.split('\n').collect();
    let mut snippet_pos = 0;
    let mut typo_mode = false;
    let mut typo_len: usize = 0;

    while snippet_pos < original_chars.len() {
        let event = event::read()?;
        if event == Event::Key(KeyCode::Enter.into()) {
            execute!(stdout, MoveToNextLine(1))?;
        } else if event == Event::Key(KeyCode::Backspace.into()) {
            let (col, row) = position()?;
            // println!("{}, {}", col, row);
            if col == 0 && row > 0 {
                execute!(stdout, MoveToPreviousLine(1))?;
            } else if col > 0 {
                execute!(
                    stdout,
                    MoveToColumn(col - 1),
                    Print(original_chars[snippet_pos]),
                    MoveToColumn(col - 1)
                )?;
            }
        } else if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') => break,
                _ => {
                    let cur_char = key.code.to_string();
                    let (col, row) = position()?;
                    if cur_char == original_chars[snippet_pos].to_string() {
                        execute!(stdout, Print(key.code))?;
                    } else {
                        if !typo_mode {
                            execute!(
                                stdout,
                                MoveToNextLine(1),
                                Clear(ClearType::CurrentLine),
                                Clear(ClearType::FromCursorDown),
                                MoveToNextLine(1)
                            )?;
                            for i in (row + 1) as usize..original_lines.len() {
                                execute!(stdout, Print(original_lines[i]), MoveToNextLine(1))?;
                            }
                            execute!(stdout, MoveTo(col, row + 1))?;
                            typo_mode = true;
                        }
                        execute!(
                            stdout,
                            Print(original_chars[snippet_pos]),
                            SetForegroundColor(Color::DarkRed)
                        )?;
                        typo_len += 1;
                    }
                    snippet_pos += 1;
                }
            }
        }
    }

    // while position < original_chars.len() {
    //     if let Event::Key(key_event) = event::read()? {
    //         if let KeyCode::Char(c) = key_event.code {
    //             if c == original_chars[position] {
    //                 current_text[position] = c;
    // execute!(stdout, MoveToColumn((position + 1) as u16), ResetColor, Print(c))?;
    //                 position += 1;
    //             } else {
    // execute!(stdout, MoveToColumn((position) as u16), SetForegroundColor(Color::DarkRed), Print(original_chars[position]), ResetColor)?;
    //             }
    //         }
    //     }
    // }

    disable_raw_mode()?;
    execute!(stdout, MoveTo(0, 2))?;
    Ok(())
}
