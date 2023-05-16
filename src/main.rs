extern crate pancurses;

use pancurses::*;
use std::cmp::Ordering;

// Who you play as lol
struct Player {
    pub x: i32,
    pub y: i32,
    pub bx: i32,
    pub by: i32,
    pub symbol: String,
    pub hudtext: String,
}

fn new(x: i32, y: i32, symbol: String, hudtext: String) -> Player {
    Player { x: (x), y: (y), symbol: (symbol), bx: (x), by: (y), hudtext: hudtext.to_string() }
}

fn main() {
    let window = initscr();
    set_title("Keegan's Game");

    // 0: Start Game, 1: Info, 2: Exit, 20: Call Link
    let mut menuvar = 0;
    loop {
        loop {
            window.clear();
            window.mvaddstr(0, 0, " Keegan's Game\n\n");
            if menuvar == 20 {
                window.mvaddstr(22, 0, "Info in link: https://github.com/TheKamboy/kgame-rust/blob/master/README.org");
                menuvar = 1;
            }
    
            if menuvar == 0 {
                window.mvaddstr(2, 0, "> Start Game <\n");
                window.mvaddstr(3, 0, "     Info     \n");
                window.mvaddstr(4, 0, "     Exit     \n");
            }
            else if menuvar == 1 {
                window.mvaddstr(2, 0, "  Start Game  \n");
                window.mvaddstr(3, 0, ">    Info    <\n");
                window.mvaddstr(4, 0, "     Exit     \n");
            }
            else if menuvar == 2 {
                window.mvaddstr(2, 0, "  Start Game  \n");
                window.mvaddstr(3, 0, "     Info     \n");
                window.mvaddstr(4, 0, ">    Exit    <\n");
            }
    
            window.mvaddstr(24, 0, "W: Up, S: Down, E: Select");
    
            let ginput: char;
            match window.getch() {
                Some(Input::Character(c)) => { ginput = c; },
                Some(_input) => {ginput = ' '},
                None => {ginput = ' '}
            }
    
            if ginput == 'w' {
                menuvar -= 1;
    
                if menuvar < 0 {
                    menuvar = 0;
                }
            }
            else if ginput == 's' {
                menuvar += 1;
    
                if menuvar > 2 {
                    menuvar = 2;
                }
            }
            else if ginput == 'e' {
                break;
            }
        }
        if menuvar == 0 {
            break;
        }
        else if menuvar == 1 {
            menuvar = 20;
        }
        else if menuvar == 2 {
            break;
        }
    }
    if menuvar == 0 {
        chapter_select(&window);
    }
    else if menuvar == 2 {
        endwin();
    }
}

fn chapter_select(window: &Window) {
    let mut menuvar = 0;
    let menumax = 1;
    loop {
        loop {
            window.clear();
            window.mvaddstr(0, 0, "   Chapter Select\n\n");
    
            if menuvar == 0 {
                window.mvaddstr(2, 0, "> Chapter 1 and 2 <\n");
                window.mvaddstr(3, 0, "  Testing Grounds  \n");
                //window.mvaddstr(4, 0, "     Exit     \n");
            }
            else if menuvar == 1 {
                window.mvaddstr(2, 0, "  Chapter 1 and 2  \n");
                window.mvaddstr(3, 0, "> Testing Grounds <\n");
                //window.mvaddstr(4, 0, "     Exit     \n");
            }
            // else if menuvar == 2 {
            //     window.mvaddstr(2, 0, "  Start Game  \n");
            //     window.mvaddstr(3, 0, "     Info     \n");
            //     window.mvaddstr(4, 0, ">    Exit    <\n");
            // }
    
            window.mvaddstr(24, 0, "W: Up, S: Down, E: Select");
    
            let ginput: char;
            match window.getch() {
                Some(Input::Character(c)) => { ginput = c; },
                Some(_input) => {ginput = ' '},
                None => {ginput = ' '}
            }
    
            if ginput == 'w' {
                menuvar -= 1;
    
                if menuvar < 0 {
                    menuvar = 0;
                }
            }
            else if ginput == 's' {
                menuvar += 1;
    
                if menuvar > menumax {
                    menuvar = menumax;
                }
            }
            else if ginput == 'e' {
                break;
            }
        }

        if menuvar == 0 {
            break;
        }
        else if menuvar == 1 {
            break;
        }
    }
    if menuvar == 0 {
        chapter_1_intro(&window);
    }
    else if menuvar == 1 {
        test(&window);
    }
}

fn chapter_1_intro(window: &Window) {
    let mut dialogue = 0;
    let mut _ginput: char;
    loop {
        window.clear();
        window.mvaddstr(0, 0, "Chapter 1: Humble Beginnings");

        window.mvaddstr(24, 0, "Press any key to continue...");

        if dialogue >= 1 {
            window.mvaddstr(2, 0, "You will be playing through the story of Keegan Miller, a millitary commander of a military organization called \"The Ghosts\".");
        }
        if dialogue >= 2 {
            window.mvaddstr(4, 0, "???: Keegan! Wake up!");
        }
        if dialogue >= 3 {
            window.mvaddstr(5, 0, "Keegan wakes up to see his brother, Kameron Miller.");
        }
        if dialogue >= 4 {
            window.mvaddstr(6, 0, "Keegan: ...yes?");
        }
        if dialogue >= 5 {
            window.mvaddstr(7, 0, "Kameron: I made something I want to show you. Follow me!");
        }
        if dialogue >= 6 {
            window.mvaddstr(8, 0, "Kameron runs off. Gotta catch up to him!");
        }
        if dialogue == 7 {
            break;
        }

        match window.getch() {
            // Lazy Moment
            Some(Input::Character(c)) => { _ginput = c; },
            Some(_input) => {_ginput = ' '},
            None => {_ginput = ' '}
        }

        dialogue += 1;
    }
    keegans_room_ch1(window);
}

fn keegans_room_ch1(window: &Window) {
    // I hate rust but i have to use it
    let mut k: Player = new(0, 0, "K".to_string(), "\"W A S D\" to move, \"E\" to examine when on E objects".to_string());
    let mut hudtext: String = k.hudtext;
    let debug = true;
    window.keypad(true);
    noecho();
    // Game Loop
    loop {
        window.clear();
        set_blink(false);
        if debug {
            window.mvaddstr(0, 10, format!("X: {}, Y: {}", k.x, k.y));
            window.mvaddstr(1, 10, format!("BX: {}, BY: {}", k.bx, k.by));
        }

        // Examine Points
        window.mvaddstr(15, 65, "E");

        window.mvaddstr(k.y, k.x, k.symbol.as_str());
        window.mvaddstr(24, 0, hudtext.as_str());
        window.refresh();

        let ginput: char;
        match window.getch() {
            Some(Input::Character(c)) => { ginput = c; },
            Some(Input::KeyDC) => break,
            Some(_input) => {ginput = ' '},
            None => {ginput = ' '}
        }
        
        hudtext.clear();

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
            k.x = k.x + 1;
        }
        else if ginput == 'd' {
            k.x += 1;
        }

        // Borders
        match k.x.cmp(&0) {
            Ordering::Less => { k.x = 0; }
            _ => {}
        } 
        // if k.x < 0 || k.x > 79 {
        //     k = move_player_back(k);
        // }
        // else if k.y < 0 || k.y > 23 {
        //     k = move_player_back(k);
        // }

        // Detect Examine Point Position
        if player_at_point(k, 65, 15) {
            hudtext = "Press E to examine.".to_string();
        }
        
        if ginput == 'e' {
            if player_at_point(k, 65, 15) {
                
            }
        }
    }
    endwin();
}

fn player_at_point(player: Player, x: i32, y: i32) -> bool {
    let mut boolean = false;
    
    if player.x == x && player.y == y {
        boolean = true;
    }

    return boolean;
}

fn test(window: &Window) {
    let mut k: Player = new(0, 0, "K".to_string(), "unused".to_string());
    let mut debug = true;
    window.keypad(true);
    noecho();
    // Game Loop
    loop {
        window.clear();
        set_blink(false);
        if debug {
            window.mvaddstr(0, 10, format!("X: {}, Y: {}", k.x, k.y));
            window.mvaddstr(1, 10, format!("BX: {}, BY: {}", k.bx, k.by));
        }
        window.mvaddstr(k.y, k.x, k.symbol.as_str());
        window.mvaddstr(24, 0, "W: Up, A: Left, S: Down, D: Right, 9: Enable/Disable Debug");
        window.refresh();
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
        else if ginput == '9' {
            if debug {
                debug = false;
            }
            else {
                debug = true;
            }
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
