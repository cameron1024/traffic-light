use std::{io::stdout, sync::atomic::{AtomicUsize, Ordering}};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    Result,
};

static COUNT: AtomicUsize = AtomicUsize::new(0);

use crate::TrafficLight;

pub fn draw_light(TrafficLight { red, orange, green }: &TrafficLight) -> Result<()> {
    let count = COUNT.fetch_add(1, Ordering::Relaxed);
    let red_mark = if *red { "X" } else { " " };
    let orange_mark = if *orange { "X" } else { " " };
    let green_mark = if *green { "X" } else { " " };
    println!("[step {count}]: red: [{red_mark}], orange: [{orange_mark}], green: [{green_mark}]");
    execute!(stdout(), Print("\n"),)?;
    execute!(stdout(), Print("/--\\\n"),)?;
    draw_row(*red, Color::Red)?;
    execute!(stdout(), Print("|--|\n"),)?;
    draw_row(*orange, Color::DarkYellow)?;
    execute!(stdout(), Print("|--|\n"),)?;
    draw_row(*green, Color::Green)?;
    execute!(stdout(), Print("\\--/\n"),)?;
    execute!(stdout(), Print("-------------------\n"),)?;
    Ok(())
}

fn draw_row(enabled: bool, color: Color) -> Result<()> {
    execute! {
        stdout(),
        Print("|"),
        SetForegroundColor(color),
        Print(if enabled { "XX" } else { "  " }),
        ResetColor,
        Print("|\n"),
    }
}
