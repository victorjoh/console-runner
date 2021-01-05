use std::{thread, time};
use termion;
use termion::{clear, color, cursor, style};

mod year2020 {
    pub mod day1 {
        use std::{thread, time};
        pub fn solve() {
            println!("\tStarting work");
            println!("\tProcessing...");
            let one_second = time::Duration::from_secs(1);
            thread::sleep(one_second);
            println!("\tFinished!");
        }
    }
}

fn run_something() {
    let lines = [
        "Starting",
        "on going",
        "what are we doing to ourselves?",
        "work work",
        "working hard or hardly working?",
        "are we there yet",
        "yeti",
        "something is happening",
        // "this is a very long line that should require wrapping. I am sure that it will since I am adding a lot of text to it. Now I am really making sure",
        "where were you?",
        "done",
    ];

    println!(
        "{}{}Solving{} 2020 day 1",
        color::Fg(color::Blue),
        style::Bold,
        style::Reset
    );

    for line in &lines[..4] {
        println!("  {}", line);
        thread::sleep(time::Duration::from_millis(500));
    }
    for (index, _) in lines[4..].iter().enumerate() {
        print!("{}{}", cursor::Up(4), clear::AfterCursor);
        for prev in &lines[index + 1..index + 5] {
            println!("  {}", prev);
        }
        thread::sleep(time::Duration::from_millis(500));
    }

    print!("{}{}", cursor::Up(5), clear::AfterCursor);
    println!(
        "{}{}Solved{} 2020 day 1",
        color::Fg(color::Green),
        style::Bold,
        style::Reset
    );
    println!("  Part 1:{} {}{}", style::Italic, 5, style::Reset);
    println!("  Part 2:{} {}{}", style::Italic, 1235, style::Reset);
}

pub fn run(day: Option<usize>, session: Option<String>) {
    run_something()
    // println!(
    //     "{}{}Solving{} 2020 day 1",
    //     color::Fg(color::Green),
    //     style::Bold,
    //     style::Reset
    // );
    // println!(
    //     "{red}more red than any comrade{reset}",
    //     red = color::Fg(color::Red),
    //     reset = color::Fg(color::Reset)
    // );
    // // Sleep for a short period of time.
    // thread::sleep(time::Duration::from_millis(300));
    // // Go back;
    // println!("\r");
    // // Clear the line and print some new stuff
    // print!(
    //     "{clear}{red}g{blue}a{green}y{red} space communism{reset}",
    //     clear = clear::CurrentLine,
    //     red = color::Fg(color::Red),
    //     blue = color::Fg(color::Blue),
    //     green = color::Fg(color::Green),
    //     reset = color::Fg(color::Reset)
    // );
    // thread::sleep(time::Duration::from_secs(1));
    // let line = "123 dwadwadaw dwadaw dwada dwaadw awdjdw ajoidwa oijeoiföjewaifoöjfiewoqjfwoieqjfoiwq joiefwqjfoiewöq jfioweqjoif wqjfoiwejqoifjwq oi jf";
    // let (width, height) = termion::terminal_size().unwrap();
    // let rows_covered = ceiling(line.len(), width as usize);
    // println!(
    //     "width={}, height={}, line length={}, covered rows={}",
    //     width,
    //     height,
    //     line.len(),
    //     rows_covered
    // );
    // println!("{}", line);
    // thread::sleep(time::Duration::from_secs(1));
    // println!(
    //     "{}{}456",
    //     cursor::Up(rows_covered as u16),
    //     clear::AfterCursor
    // );
    // thread::sleep(time::Duration::from_secs(1));
    // print!("456");
    // thread::sleep(time::Duration::from_secs(1));
    // // Clear the line and print some new stuff
    // println!("test4");
}

fn ceiling(dividend: usize, divisor: usize) -> usize {
    (dividend + divisor - 1) / divisor
}
