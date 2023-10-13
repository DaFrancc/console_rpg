use std::time::Duration;
use std::process::Command;

use std::io::{Write, stdout};

use crossterm::event::{poll, read, Event};
use crossterm::{QueueableCommand, cursor};

fn main() {
    let mut player_input = String::from("");
    clear_console();
    print_pic();
    set_game_text(String::from("you are cringe lmao so you started this stupid ass game thinking \
    that you       couldn't not be cringe but that was not the case you are stupid"));
    set_player_text(&player_input);
    loop {
        // `poll()` waits for an `Event` for a given time period
        let thing: bool = match poll(Duration::from_millis(500)) {
            Ok(b) => b,
            Err(_) => false
        };
        if thing {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read() {
                Ok(Event::Key(event)) =>  {

                    if event.kind != crossterm::event::KeyEventKind::Press {
                        continue;
                    }

                    //clear_console();
                    
                    // print_pic();

                    match event.code {
                        crossterm::event::KeyCode::Char(c) => {
                            if player_input.len() < 160 {
                                player_input.push(c)
                            }
                        },
                        crossterm::event::KeyCode::Backspace => match player_input.pop() { _ => {} },
                        crossterm::event::KeyCode::Enter => match parse_input(&mut player_input) {
                            false => break,
                            _ => {}
                        },
                        _ => {}
                    }

                    set_player_text(&player_input);
                },
                _ => {}
            }
        }
    }
}

fn set_background_color() {
    print!("\x1b[48;2;0;0;0m");
    print!("\x1b[38;2;255;255;255m");
}

fn clear_console() {
    let _ = Command::new("clear")
        .status()
        .expect("Failed to clear the screen");
    set_background_color();
    flush_console();
}

fn flush_console() {
    let console_width = " ".repeat(130);
    let console_height = 28;
    for _ in 0..console_height {
        println!("{}", console_width);
    }
}

fn set_game_text(str: String) {
    let mut stdout = stdout();

    match stdout.queue(cursor::MoveTo(0, 23)) {
        Ok(_) => {}
        Err(_) => {}
    }

    if str.len() < 80 {
        print!("{}{}", " ".repeat(20), str);
    }
    else {
        println!("{}{}", " ".repeat(20), str[..80].to_string());
        print!("{}{}", " ".repeat(20), str[80..].to_string());
        stdout.flush().unwrap();
    }
}

fn draw_on_screen(str: &String, x: u16, y: u16) {
    let mut stdout = stdout();
    // const ANSI_CLEAR: &str = "\x1b[2K";

    let pic: Vec<&str> = str.split('\n').collect();

    match stdout.queue(cursor::MoveTo(x, y)) {
        Ok(_) => {}
        Err(_) => {}
    }

    let color = "\x1b[38;2;163;163;163m";

    print!("{}", color);

    for (line, i) in pic.iter().zip(1..=pic.len()) {
        print!("{}", line);

        match stdout.queue(cursor::MoveTo(x, y + i as u16)) {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    print!("\x1b[0m");
}

fn set_player_text(str: &String) {
    let mut stdout = stdout();
    const ANSI_CLEAR: &str = "\x1b[2K";

    if str.len() < 80 {
        let position: (u16, u16) = ((20 + str.len()) as u16, 27);

        stdout.queue(cursor::MoveTo(position.0, position.1)).unwrap();

        print!("{}\r{}>> {}", ANSI_CLEAR, " ".repeat(17), str);

        stdout.flush().unwrap();
    }
    else {
        let position: (u16, u16) = ((20 + str.len() - 80) as u16, 27);

        stdout.queue(cursor::MoveTo(position.0, position.1)).unwrap();

        println!("{}\r{}>> {}", ANSI_CLEAR, " ".repeat(17), str[..80].to_string());
        print!("{}\r{}{}", ANSI_CLEAR, " ".repeat(20), str[80..].to_string());
        //println!("{}", str.len());

        stdout.flush().unwrap();
    }
}

fn parse_input(str: &mut String) -> bool {
    clear_console();
    print_pic();
    if str.to_lowercase() == String::from("quit") {
        clear_console();
        false
    }
    else {
        if str.to_lowercase() == String::from("draw") {
            draw_on_screen(&String::from("      \n\u{2502} \u{25CB}_\u{0332}_\u{0332}  \n\u{253C}/|)*)\n / \\\u{203E} \n      "), 50, 10);
        }
        str.clear();
        set_game_text(String::from("you are cringe lmao so you started this stupid ass game thinking \
    that you       couldn't not be cringe but that was not the case you are stupid"));
        set_player_text(str);
        true
    }
}

fn print_pic() {
    let twenty_spaces = " ".repeat(20);
    println!("\n{0}   .............::::::::::^^^^^^^^^^^^::::::::............                
{0} ..........::::::::::::::^^^~~~~~!!!!!~~~~^^^::::::........                
{0}..........:::::::::::::^^^~~!!77???77777777!~~^::::............            
{0}............::::::::::^^^^~~!77??JJJJ?????JJ??7!~~^::..............          
{0} ..........:::::::::^^^~~~!7777!!!!7????7!~~!777!!~^:::...............       
{0}   ..........:::::::^^^^~~!!!!!~:.....:?J!^...:^~!7!!~^^::.......:.......      
{0}............:::::^^^^^^~~!!!!!~~:..   7BP!:....:^!777!~~^^::.::::........      
{0}...........::::::::^^^^~~!!!!!!!~~^^!JJ77?!^^:^^!!7777!!~~^^::::::........     
{0} ....  ....::::::::::^^^^^~~!!!777???7~:..:^^~~~~~!!!!!!!!~^^:::::::..........  
{0}.........::::::::::::::^^^^~~~!!!!77!~^::.:::^^^~~~~~~~~~~~^^::::::::::.........
{0} .........::::::::::::::^^^^~~~!!!!!!!!~^^^^^^^^^^^^^^^~~^^^^:::::::::..........
{0}..........:::::::::::::::^^^~~~!~~~~~~^:::::::::::^^^^^^^^^^^^:::::::...........
{0}   .......:::::::::::::::^^^~~~~~^^^^::..   ...:::^^^^^^^^^^^^:::::::::::.....  
{0} ...........:::::::::::::^^^~~~~~^^::...     ..::^^::^^^^^^^::::::^::::::.......
{0}  ..............::::::::::^^^^^^^^::....     ..::::::::::^^^::::::::::::...... .
{0}................::::::::::::^^^:::::...     ..::^^^::^^^^^^^:::::::::::.........
{0} .........  ....:::::::::::::::::::::...   .::^^^^^^^^^^^^^^::::::::::::.. .....
{0}................:::::::::::::::::::::::::::^^^^^^^^:::^^^^^::::::...............
{0}..................:.........:::::::::^^^^^~~~~^^::::::::::::::............... 
{0}  .....  ..................::::::::::^^^^^^^^^:::::::::::::..............    
{0}     .......................::::::::::::::::::::::::::::...........         \n", twenty_spaces);
}