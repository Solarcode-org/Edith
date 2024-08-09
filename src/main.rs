use clap::Parser;
use color_eyre::{eyre::bail, Result};
use crossterm::{
    cursor::{Hide, MoveDown, MoveTo, Show},
    event::{read, Event, KeyCode, KeyEventKind},
    style::{Print, PrintStyledContent, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand, QueueableCommand,
};
use std::fs::read_to_string;
use std::io::{stdout, Write};
use std::path::PathBuf;

/// Edith editor CLI for EDITing files Harmonically.
#[derive(Parser, Debug)]
struct Args {
    /// The path to the file to open.
    filename: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let mut stdout = stdout();

    stdout.queue(EnterAlternateScreen)?.queue(Hide)?;
    enable_raw_mode()?;

    stdout.flush()?;

    let mut scrolldown = 0;

    let contents = read_to_string(&args.filename)?;

    loop {
        stdout.execute(MoveTo(0, 0))?;
        stdout.execute(Clear(ClearType::All))?;

        let mut lines = contents.lines();

        for _ in 0..scrolldown {
            lines.next();
        }

        for _ in 0..match termsize::get() {
            Some(size) => size,
            None => bail!("Could not get terminal size"),
        }
        .rows
            - 2
        {
            let line = lines.next();

            stdout.queue(MoveDown(1))?;
            // stdout.queue(PrintStyledContent(format!("{lines:?}").red()))?;

            if let Some(line) = line {
                stdout.queue(Print(line))?.queue(Print("\r"))?;
            } else {
                stdout
                    .queue(PrintStyledContent("~".blue()))?
                    .queue(Print("\r"))?;
            }
        }

        stdout.flush()?;

        match read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                if event.code == KeyCode::Char('q') {
                    break;
                }

                if event.code == KeyCode::Down || event.code == KeyCode::Char('j') {
                    scrolldown += 1
                }

                if event.code == KeyCode::Up || event.code == KeyCode::Char('k') {
                    if scrolldown - 1 >= 0 {
                        scrolldown -= 1
                    }
                }
            }
            _ => {}
        }
    }

    disable_raw_mode()?;
    stdout.execute(Show)?.execute(LeaveAlternateScreen)?;

    Ok(())
}
