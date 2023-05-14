extern crate pancurses;

use pancurses::*;

// Who you play as lol
struct Player {
    pub x: i32,
    pub y: i32,
    pub bx: i32,
    pub by: i32,
    pub symbol: String,
    pub hudtext: String,
}

fn new(x: i32, y: i32, symbol: String) -> Player {
    Player { x: (x), y: (y), symbol: (symbol), bx: (x), by: (y), hudtext: "\"W A S D\" to move.".to_string() }
}

fn main() {
    let mut k: Player = new(0, 0, "K".to_string());
    let window = initscr();
    window.keypad(true);
    noecho();
    // Game Loop
    loop {
        window.clear();
        set_blink(false);
        window.mvaddstr(0, 10, format!("X: {}, Y: {}", k.x, k.y));
        window.mvaddstr(1, 10, format!("BX: {}, BY: {}", k.bx, k.by));
        window.mvaddstr(k.y, k.x, k.symbol.as_str());
        let ginput: char;
        match window.getch() {
            Some(Input::Character(c)) => { ginput = c; },
            Some(Input::KeyDC) => break,
            Some(_input) => {ginput = ' '},
            None => {ginput = ' '}
        }
        
        // Set backup x and y values
        k.by = k.y;
        k.bx = k.x;

        // Movement
        if ginput == 'w' {
            k.y -= 1;
        }
        else if ginput == 's' {
            k.y += 1;
        }
        else if ginput == 'a' {
            k.x -= 1;
        }
        else if ginput == 'd' {
            k.x += 1;
        }

        // Borders
        if k.x < 0 || k.x > 79 {
            k = move_player_back(k);
        }
        else if k.y < 0 || k.y > 23 {
            k = move_player_back(k);
        }
    }
    endwin();
}

fn move_player_back(player: Player) -> Player {
    //println!("\nMove back sent!\n\nX: {}, Y: {}\nBX: {}, BY: {}", player.x, player.y, player.bx, player.by);
    Player { x: (player.bx), y: (player.by), symbol: (player.symbol), bx: (player.bx), by: (player.by), hudtext: (player.hudtext) }
}

// fn text_in_a_box(text: &str, window: &Window) {
//     let len = text.len();

//     window.attron(A_OVERLINE | A_UNDERLINE | A_LEFTLINE);
//     if len == 1 {
//         window.attron(A_RIGHTLINE);
//     }

//     window.addnstr(text, 1);
//     if len > 1 {
//         window.attroff(A_LEFTLINE);
//         if len > 2 {
//             window.addnstr(&text[1..], len - 2);
//         }
//         window.attron(A_RIGHTLINE);
//         window.addnstr(&text[len - 1..], 1);
//     }

//     window.attroff(A_OVERLINE | A_UNDERLINE | A_LEFTLINE | A_RIGHTLINE);
// }
