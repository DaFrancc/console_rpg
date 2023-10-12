use std::time::Duration;
use std::process::Command;

use std::io::{Write, stdout};

use crossterm::event::{poll, read, Event};
use crossterm::{QueueableCommand, cursor};

fn main() {
    let mut player_input = String::from("");
    clear_console();
    print_pic();
    set_game_text(String::from("you are cringe lmao so you started this stupid ass game thinking that you       couldnt not be cringe but that was not the case you are stupid"));
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

fn clear_console() {
    let _ = Command::new("cmd")
        .args(&["/C", "cls"])
        .status();
}

fn set_game_text(str: String) {
    let mut stdout = stdout();

    if str.len() < 80 {
        print!("{}{}", " ".repeat(20), str);
    }
    else {
        println!("{}{}", " ".repeat(20), str[..80].to_string());
        print!("{}{}", " ".repeat(20), str[80..].to_string());
        stdout.flush().unwrap();
    }
}

fn set_player_text(str: &String) {
    let mut stdout = stdout();

    if str.len() < 80 {
        let position: (u16, u16) = ((20 + str.len()) as u16, 27);

        stdout.queue(cursor::MoveTo(position.0, position.1)).unwrap();

        print!("\x1b[2K\r{}>> {}", " ".repeat(17), str);
        //println!("{}", str.len());

        stdout.flush().unwrap();
    }
    else {
        let position: (u16, u16) = ((20 + str.len() - 80) as u16, 27);

        stdout.queue(cursor::MoveTo(position.0, position.1)).unwrap();

        println!("\x1b[2K\r{}>> {}", " ".repeat(17), str[..80].to_string());
        print!("\x1b[2K\r{}{}", " ".repeat(20), str[80..].to_string());
        //println!("{}", str.len());

        stdout.flush().unwrap();
    }
}

fn parse_input(str: &mut String) -> bool {
    clear_console();
    print_pic();
    if str.to_lowercase() == String::from("quit") {
        false
    }
    else {
        str.clear();
        set_game_text(String::from("you are cringe lmao so you started this stupid ass game thinking that you       couldnt not be cringe but that was not the case you are stupid"));
        set_player_text(str);
        true
    }
}

fn print_pic() {
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
{0}     .......................::::::::::::::::::::::::::::...........         \n", " ".repeat(20));
}