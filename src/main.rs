#![allow(clippy::needless_late_init)]

extern crate pancurses;

pub use pancurses::*;

// The fix they give doesn't work so

fn main() {
    let window = initscr();

    resize_term(25, 80);
    set_title("Keegan's Game");

    // 0: Start Game, 1: Info, 2: Exit, 20: Call Link
    let mut menuvar = 0;
    loop {
        loop {
            window.clear();
            window.mvaddstr(0, 0, "  Keegan's Game\n\n");
            if menuvar == 20 {
                window.mvaddstr(
                    22,
                    0,
                    "Info in link: https://github.com/TheKamboy/kgame-rust/blob/master/README.org",
                );
                menuvar = 1;
            }

            if menuvar == 0 {
                window.mvaddstr(2, 0, " > Start Game <\n");
                window.mvaddstr(3, 0, "      Info     \n");
                window.mvaddstr(4, 0, "      Exit     \n");
            } else if menuvar == 1 {
                window.mvaddstr(2, 0, "   Start Game  \n");
                window.mvaddstr(3, 0, " >    Info    <\n");
                window.mvaddstr(4, 0, "      Exit     \n");
            } else if menuvar == 2 {
                window.mvaddstr(2, 0, "   Start Game  \n");
                window.mvaddstr(3, 0, "      Info     \n");
                window.mvaddstr(4, 0, " >    Exit    <\n");
            }

            window.mvaddstr(24, 0, "W: Up, S: Down, E: Select");

            let ginput: char;
            match window.getch() {
                Some(Input::Character(c)) => {
                    ginput = c;
                }
                Some(_input) => ginput = ' ',
                None => ginput = ' ',
            }

            if ginput == 'w' {
                menuvar -= 1;

                if menuvar < 0 {
                    menuvar = 0;
                }
            } else if ginput == 's' {
                menuvar += 1;

                if menuvar > 2 {
                    menuvar = 2;
                }
            } else if ginput == 'e' {
                break;
            }
        }
        if menuvar == 0 {
            break;
        } else if menuvar == 1 {
            menuvar = 20;
        } else if menuvar == 2 {
            break;
        }
    }
    if menuvar == 0 {
        chapter_select(&window);
    } else if menuvar == 2 {
        endwin();
    }
}

fn chapter_select(window: &Window) {
    let mut menuvar = 0;
    let menumax = 1;
    loop {
        loop {
            window.clear();
            window.mvaddstr(0, 0, "    Chapter Select\n\n");

            if menuvar == 0 {
                window.mvaddstr(2, 0, " > Chapter 1 and 2 <\n");
                window.mvaddstr(3, 0, "      Tutorial      \n");
                //window.mvaddstr(4, 0, "     Exit     \n");
            } else if menuvar == 1 {
                window.mvaddstr(2, 0, "   Chapter 1 and 2  \n");
                window.mvaddstr(3, 0, " >    Tutorial     <\n");
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
                Some(Input::Character(c)) => {
                    ginput = c;
                }
                Some(_input) => ginput = ' ',
                None => ginput = ' ',
            }

            if ginput == 'w' {
                menuvar -= 1;

                if menuvar < 0 {
                    menuvar = 0;
                }
            } else if ginput == 's' {
                menuvar += 1;

                if menuvar > menumax {
                    menuvar = menumax;
                }
            } else if ginput == 'e' {
                break;
            }
        }

        if menuvar == 0 || menuvar == 1 {
            break;
        }
    }
    if menuvar == 0 {
        chapter_1_intro(window);
    } else if menuvar == 1 {
        tutorialch(window);
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
            window.mvaddstr(
                7,
                0,
                "Kameron: I made something I want to show you. Follow me!",
            );
        }
        if dialogue >= 6 {
            window.mvaddstr(8, 0, "Kameron runs off. Gotta catch up to him!");
        }
        if dialogue == 7 {
            break;
        }

        match window.getch() {
            // Lazy Moment
            Some(Input::Character(c)) => {
                _ginput = c;
            }
            Some(_input) => _ginput = ' ',
            None => _ginput = ' ',
        }

        dialogue += 1;
    }
    keegans_room_ch1(window, 39, 11, true);
}

fn keegans_room_ch1(window: &Window, x: i32, y: i32, showinfoonhud: bool) {
    let mut hudtext: String =
        "\"W A S D\" to move, \"E\" to examine when on E objects, Walk on \"D\" to exit room."
            .to_string();

    if !showinfoonhud {
        hudtext.clear();
        hudtext = "Keegan's Room".to_string();
    }

    let ksymbol: String = "K".to_string();
    let mut kx: i32 = x;
    let mut ky: i32 = y;
    let mut kbx: i32 = kx;
    let mut kby: i32 = ky;
    let debug = true;
    window.keypad(true);
    noecho();
    // Game Loop
    loop {
        window.clear();
        set_blink(false);
        if debug {
            window.mvaddstr(0, 10, format!("X: {}, Y: {}", kx, ky));
            window.mvaddstr(1, 10, format!("BX: {}, BY: {}", kbx, kby));
        }

        // Examine Points
        window.mvaddstr(11, 45, "E"); // Examine Point

        // Room (Middle Coords: 11, 39) (Corner Y: 13, 9)
        window.mvaddstr(11, 29, "D");
        window.mvaddstr(11, 49, "#");
        window.mvaddstr(10, 29, "#");
        window.mvaddstr(12, 29, "#");
        window.mvaddstr(13, 29, "#####################");
        window.mvaddstr(9, 29, "#####################");
        window.mvaddstr(13, 49, "#");
        window.mvaddstr(12, 49, "#");
        window.mvaddstr(10, 49, "#");
        window.mvaddstr(9, 49, "#");
        // Table
        window.mvaddstr(10, 46, "---");
        window.mvaddstr(11, 46, "|[]");
        window.mvaddstr(12, 46, "---");

        window.mvaddstr(ky, kx, ksymbol.as_str()); // Keegan
        window.mvaddstr(24, 0, hudtext.as_str()); // HUD
        window.refresh();

        let ginput: char;
        match window.getch() {
            Some(Input::Character(c)) => {
                ginput = c;
            }
            Some(Input::KeyDC) => break,
            Some(_input) => ginput = ' ',
            None => ginput = ' ',
        };

        hudtext = "Keegan's Room".to_string();

        // Set backup x and y values
        kbx = kx;
        kby = ky;

        // Movement
        if ginput == 'w' {
            ky -= 1;
        }
        if ginput == 'd' {
            kx += 1;
        }
        if ginput == 's' {
            ky += 1;
        }
        if ginput == 'a' {
            kx -= 1;
        }

        // Wall Barriers
        if kx == 29 && (9..=13).contains(&ky) && ky != 11 {
            kx = move_x_back(kbx);
            ky = move_y_back(kby);
        }
        if kx == 49 && (9..=13).contains(&ky) {
            kx = move_x_back(kbx);
            ky = move_y_back(kby)
        }

        // (X1: 30, X2: 48)
        if (30..=48).contains(&kx) && (ky == 13 || ky == 9) {
            kx = move_x_back(kbx);
            ky = move_y_back(kby);
        }

        // Table Barriers
        // X: 46-48, Y: 10-12
        if (46..=48).contains(&kx) && (10..=12).contains(&ky) {
            kx = move_x_back(kbx);
            ky = move_y_back(kby);
        }

        // Barrier
        if !(0..=79).contains(&kx) {
            kx = move_x_back(kbx);
        } else if !(0..=23).contains(&ky) {
            ky = move_y_back(kby);
        }

        // Detect Examine Point Position
        if at_point(kx, ky, 45, 11) {
            hudtext = "Press E to examine.".to_string();
        }

        // Detect Entering Door
        if at_point(kx, ky, 29, 11) {
            fb_hallway_ch1(window, 48, 11, 1);
        }

        // Examine Key
        if ginput == 'e' && at_point(kx, ky, 45, 11) {
            hudtext = "Keegan: This is my computer. It's only for work.".to_string();
        }
    }
    endwin();
}

fn fb_hallway_ch1(window: &Window, x: i32, y: i32, location: i32) {
    let mut hudtext: String = "Hallway".to_string();
    let ksymbol: String = "K".to_string();
    let mut kx: i32 = x; // 48
    let mut ky: i32 = y; // 11
    let mut kbx: i32 = kx;
    let mut kby: i32 = ky;
    let debug = true;
    window.keypad(true);
    noecho();
    // Game Loop
    loop {
        window.clear();
        set_blink(false);
        if debug {
            window.mvaddstr(0, 10, format!("X: {}, Y: {}", kx, ky));
            window.mvaddstr(1, 10, format!("BX: {}, BY: {}", kbx, kby));
        }

        // Hallway
        window.mvaddstr(13, 49, "#");
        window.mvaddstr(12, 49, "#");
        window.mvaddstr(10, 49, "#");
        window.mvaddstr(9, 49, "#");
        window.mvaddstr(8, 38, "#");
        window.mvaddstr(8, 40, "#");
        window.mvaddstr(13, 29, "####################");
        window.mvaddstr(9, 29, "########## #########");
        window.mvaddstr(13, 29, "#");
        window.mvaddstr(9, 29, "#");
        window.mvaddstr(10, 29, "#");
        window.mvaddstr(11, 29, "#");
        window.mvaddstr(12, 29, "#");

        // Hallway Doors (X: 29)
        window.mvaddstr(11, 49, "D");

        let mut i: i32 = 47;

        while i != 29 {
            if i != 39 {
                if location == 1 {
                    window.mvaddstr(9, i, "D");
                } else {
                    window.mvaddstr(9, i, "#");
                }
            }
            if location == 1 {
                window.mvaddstr(13, i, "D");
            } else {
                window.mvaddstr(13, i, "#");
            }

            i -= 2;
        }

        // Other Game Elements
        window.mvaddstr(ky, kx, ksymbol.as_str()); // Keegan
        window.mvaddstr(24, 0, hudtext.as_str()); // HUD
        window.refresh();

        let ginput: char;
        match window.getch() {
            Some(Input::Character(c)) => {
                ginput = c;
            }
            Some(Input::KeyDC) => break,
            Some(_input) => ginput = ' ',
            None => ginput = ' ',
        }

        hudtext = "Hallway".to_string();

        // Set backup x and y values
        kbx = kx;
        kby = ky;

        // Movement
        if ginput == 'w' {
            ky -= 1;
        }
        if ginput == 'd' {
            kx += 1;
        }
        if ginput == 's' {
            ky += 1;
        }
        if ginput == 'a' {
            kx -= 1;
        }

        // Wall Barriers
        if kx == 29 && (9..=13).contains(&ky) {
            kx = move_x_back(kbx);
            ky = move_y_back(kby);
        }
        if kx == 49 && (9..=13).contains(&ky) && ky != 11 {
            kx = move_x_back(kbx);
            ky = move_y_back(kby)
        }

        // (X1: 30, X2: 48)
        if (30..=48).contains(&kx) && kx != 39 && (ky == 13 || ky == 9) {
            kx = move_x_back(kbx);
            ky = move_y_back(kby);
        }

        // Barrier
        if !(0..=79).contains(&kx) {
            kx = move_x_back(kbx);
        } else if !(0..=23).contains(&ky) {
            ky = move_y_back(kby);
        }

        // Detect Entering Door
        if at_point(kx, ky, 49, 11) {
            if location == 1 {
                keegans_room_ch1(window, 30, 11, false);
            } else {
                ch1_cutscene(window);
            }
        }

        // Detect Entering Elevator
        if at_point(39, 8, kx, ky) {
            elevator_ch1(window, 1);
        }
    }
}

fn elevator_ch1(window: &Window, mut location: i32) {
    // Location: 1=Hallway near Keegan's Room, 2=Hallway near Tech Room
    let mut hudtext: String = "Hallway".to_string();
    let ksymbol: String = "K".to_string();
    let mut kx: i32 = 39;
    let mut ky: i32 = 11;
    let mut kbx: i32 = kx;
    let mut kby: i32 = ky;
    let debug = true;
    window.keypad(true);
    noecho();
    // Game Loop
    loop {
        window.clear();
        set_blink(false);
        if debug {
            window.mvaddstr(0, 10, format!("X: {}, Y: {}", kx, ky));
            window.mvaddstr(1, 10, format!("BX: {}, BY: {}", kbx, kby));
        }

        // Short Path Y: 11
        // Short Path X1: 40, X2: 38
        window.mvaddstr(11, 40, "#");
        window.mvaddstr(11, 38, "#");
        window.mvaddstr(12, 38, "#");
        window.mvaddstr(12, 40, "#");

        // Walls (Side Walls Y: 10-8)
        window.mvaddstr(11, 37, "#");
        window.mvaddstr(10, 37, "#");
        window.mvaddstr(9, 37, "#");
        window.mvaddstr(8, 37, "#");
        window.mvaddstr(11, 41, "#");
        window.mvaddstr(10, 41, "#");
        window.mvaddstr(9, 41, "#");
        window.mvaddstr(8, 41, "#");
        window.mvaddstr(8, 38, "###");

        // Examine Point
        window.mvaddstr(10, 38, "E"); // Is controls for elevator

        // Other Game Elements
        window.mvaddstr(ky, kx, ksymbol.as_str()); // Keegan
        window.mvaddstr(24, 0, hudtext.as_str()); // HUD
        window.refresh();

        let ginput: char;
        match window.getch() {
            Some(Input::Character(c)) => {
                ginput = c;
            }
            Some(Input::KeyDC) => break,
            Some(_input) => ginput = ' ',
            None => ginput = ' ',
        }

        hudtext = "Elevator".to_string();

        // Set backup x and y values
        kbx = kx;
        kby = ky;

        // Movement
        if ginput == 'w' {
            ky -= 1;
        }
        if ginput == 'd' {
            kx += 1;
        }
        if ginput == 's' {
            ky += 1;
        }
        if ginput == 'a' {
            kx -= 1;
        }

        // Wall Barriers
        if (kx == 38 || kx == 40) && ky == 12 {
            kx = move_x_back(kbx);
            ky = move_y_back(kby);
        }

        if (37..=41).contains(&kx) && kx != 39 && ky == 11 {
            kx = move_x_back(kbx);
            ky = move_y_back(kby);
        }

        // (X1: 37, X2: 41) (Y: 10-8)
        if (kx == 37 || kx == 41) && (8..=10).contains(&ky) {
            kx = move_x_back(kbx);
            ky = move_y_back(kby);
        }

        // X: 38-40
        if (38..=40).contains(&kx) && ky == 8 {
            kx = move_x_back(kbx);
            ky = move_y_back(kby);
        }

        // Barrier
        if !(0..=79).contains(&kx) {
            kx = move_x_back(kbx);
        } else if !(0..=23).contains(&ky) {
            ky = move_y_back(kby);
        }

        if at_point(38, 10, kx, ky) {
            hudtext = "Press E to examine.".to_string();
        }

        if ginput == 'e' && at_point(38, 10, kx, ky) {
            let mut emenunum: i32 = 0;
            loop {
                window.clear();
                window.mvaddstr(0, 0, "     Elevator Menu\n\n");

                if emenunum == 0 {
                    if location == 1 {
                        window.mvaddstr(2, 0, " >    Lower Floor*    <\n");
                        window.mvaddstr(3, 0, "      Upper Floor      \n");
                    } else {
                        window.mvaddstr(2, 0, " >    Lower Floor     <\n");
                        window.mvaddstr(3, 0, "      Upper Floor*     \n");
                    }

                    //window.mvaddstr(4, 0, "     Exit     \n");
                } else if emenunum == 1 {
                    if location == 1 {
                        window.mvaddstr(2, 0, "      Lower Floor*     \n");
                        window.mvaddstr(3, 0, " >    Upper Floor     <\n");
                    } else {
                        window.mvaddstr(2, 0, "      Lower Floor      \n");
                        window.mvaddstr(3, 0, " >    Upper Floor*    <\n");
                    }
                }

                window.mvaddstr(24, 0, "W: Up, S: Down, E: Select");

                let ginput: char;
                match window.getch() {
                    Some(Input::Character(c)) => {
                        ginput = c;
                    }
                    Some(_input) => ginput = ' ',
                    None => ginput = ' ',
                }

                if ginput == 'w' {
                    emenunum -= 1;

                    if emenunum < 0 {
                        emenunum = 0;
                    }
                } else if ginput == 's' {
                    emenunum += 1;

                    if emenunum > 1 {
                        emenunum = 1;
                    }
                } else if ginput == 'e' {
                    break;
                }
            }
            if emenunum + 1 != location {
                location = emenunum + 1;
            }
        }

        // Detect Exiting Elevator
        if at_point(39, 12, kx, ky) {
            fb_hallway_ch1(window, 39, 9, location)
        }
    }
}

fn ch1_cutscene(window: &Window) {
    let mut dialogue = 0;
    let mut _ginput: char;
    loop {
        window.clear();
        window.mvaddstr(0, 0, "Keegan enters the Tech Room to find his brother.");

        window.mvaddstr(24, 0, "Press any key to continue...");

        if dialogue >= 1 {
            window.mvaddstr(2, 0, "Keegan: You should've waited for me!");
        }
        if dialogue >= 2 {
            window.mvaddstr(3, 0, "Kameron: Sorry, I was in a rush.");
        }
        if dialogue >= 3 {
            window.mvaddstr(4, 0, "Keegan notices a machine covered with cloth.");
        }
        if dialogue >= 4 {
            window.mvaddstr(5, 0, "Keegan: What is that?");
        }
        if dialogue >= 5 {
            window.mvaddstr(6, 0, "Kameron: This is what I made! Look!");
        }
        if dialogue >= 6 {
            window.mvaddstr(7, 0, "Kameron unwraps the machine.");
        }
        if dialogue >= 7 {
            window.mvaddstr(
                8,
                0,
                "It's a box with a touch screen, with a big capsule, presumably one you can",
            );
            window.mvaddstr(9, 0, "enter.");
        }
        if dialogue >= 8 {
            window.mvaddstr(10, 0, "Keegan: ...What is it?");
        }
        if dialogue >= 9 {
            window.mvaddstr(
                11,
                0,
                "Kameron: It's a special time traveling machine! It teleports you to the",
            );
            window.mvaddstr(12, 0, "time period you want.");
        }
        if dialogue >= 10 {
            window.mvaddstr(13, 0, "Keegan: Nice. Can I test it?");
        }
        if dialogue >= 11 {
            window.mvaddstr(14, 0, "Kameron: Yeah, that is what I called you over for.");
            window.mvaddstr(15, 0, "What time period do you want to go to?");
        }
        if dialogue >= 12 {
            window.mvaddstr(16, 0, "Keegan: Normandy, June 6, 1944.");
        }
        if dialogue >= 13 {
            window.mvaddstr(
                17,
                0,
                "Kameron: Isn't that D-Day? Whatever, just don't ruin the timeline.",
            );
        }
        if dialogue >= 14 {
            window.mvaddstr(18, 0, "Keegan: Don't worry, I won't.");
        }
        if dialogue >= 15 {
            window.mvaddstr(19, 0, "Kameron turns on the machine, and sets it up.");
        }
        if dialogue >= 16 {
            window.mvaddstr(20, 0, "Keegan gets in, and Kameron presses a red button.");
        }
        if dialogue >= 17 {
            window.mvaddstr(21, 0, "Everything turns white for Keegan.");
        }
        if dialogue >= 18 {
            break;
        }

        match window.getch() {
            // Lazy Moment
            Some(Input::Character(c)) => {
                _ginput = c;
            }
            Some(_input) => _ginput = ' ',
            None => _ginput = ' ',
        }

        dialogue += 1;
    }
    chapter_2_intro(window);
}

fn chapter_2_intro(window: &Window) {
    let mut dialogue = 0;
    let mut _ginput: char;
    loop {
        window.clear();
        window.mvaddstr(0, 0, "Chapter 2: A Quick Disaster");

        window.mvaddstr(24, 0, "Press any key to continue...");

        if dialogue >= 1 {
            window.mvaddstr(
                2,
                0,
                "Keegan wakes up to gunfire, behind a wall, on the beach.",
            );
        }
        if dialogue >= 2 {
            window.mvaddstr(3, 0, "Keegan: ...Ughhh, that was quick.");
        }
        if dialogue >= 3 {
            window.mvaddstr(4, 0, "Keegan spots a box next to him.");
        }
        if dialogue >= 4 {
            window.mvaddstr(5, 0, "He opens it to find an earpiece.");
        }
        if dialogue >= 5 {
            window.mvaddstr(
                6,
                0,
                "Keegan puts on the earpiece and tries to speak into it.",
            );
        }
        if dialogue >= 6 {
            window.mvaddstr(7, 0, "Keegan: Hello?");
        }
        if dialogue >= 7 {
            window.mvaddstr(
                8,
                0,
                "Kameron (On Earpiece): Hey! Took you long enough to wake up.",
            );
        }
        if dialogue >= 8 {
            window.mvaddstr(9, 0, "Keegan: Hey! Glad to here your voice again.");
        }
        // Bruh my brother does not want to help me with the dialogue. Guess I will continue trying
        // to make his dialogue
        if dialogue >= 9 {
            window.mvaddstr(
                10,
                0,
                "Kameron: Yeah yeah, Whatever. Anyways, why did you want to go here?",
            );
        }
        if dialogue >= 10 {
            window.mvaddstr(
                11,
                0,
                "Keegan: Well, I thought we should investigate to see what Hitler",
            );
            window.mvaddstr(12, 0, "was up to at this time period.");
        }
        if dialogue >= 11 {
            window.mvaddstr(13, 0, "Kameron: Alright, I guess. Just don't get spotted!");
        }
        if dialogue >= 12 {
            window.mvaddstr(14, 0, "Keegan: Don't worry, I won't.");
        }
        if dialogue >= 13 {
            break;
        }

        match window.getch() {
            // Lazy Moment
            Some(Input::Character(c)) => {
                _ginput = c;
            }
            Some(_input) => _ginput = ' ',
            None => _ginput = ' ',
        }

        dialogue += 1;
    }
    sneak_past_guard_tutorial_ch2(window, 49, 11);
}

fn sneak_past_guard_tutorial_ch2(window: &Window, x: i32, y: i32) {
    let mut hudtext: String = "You can press any other key to pass the time!".to_string();

    let ksymbol: &str = "K";
    let mut kx: i32 = x;
    let mut ky: i32 = y;
    let mut kbx: i32 = kx;
    let mut kby: i32 = ky;

    let mut guardsym = "G";
    let mut guardtime = 3;

    let debug = true;

    window.keypad(true);
    noecho();
    // Game Loop
    loop {
        window.clear();
        set_blink(false);
        if debug {
            window.mvaddstr(0, 10, format!("X: {}, Y: {}", kx, ky));
            window.mvaddstr(1, 10, format!("BX: {}, BY: {}", kbx, kby));
        }

        // Guard
        let timermsg = format!("{}", guardtime);
        window.mvaddstr(7, 40, timermsg);
        window.mvaddstr(8, 40, "|"); // Part of timer
        window.mvaddstr(9, 40, guardsym);

        window.mvaddstr(ky, kx, ksymbol); // Keegan
        window.mvaddstr(24, 0, hudtext.as_str()); // HUD
        window.refresh();

        let ginput: char;
        match window.getch() {
            Some(Input::Character(c)) => {
                ginput = c;
            }

            Some(Input::KeyDC) => break,
            Some(_input) => ginput = ' ',
            None => ginput = ' ',
        }

        hudtext = "You can press any other key to pass the time!".to_string();

        // Set backup x and y values
        kbx = kx;
        kby = ky;

        // Movement
        if ginput == 'w' {
            ky -= 1;
        }
        if ginput == 'd' {
            kx += 1;
        }
        if ginput == 's' {
            ky += 1;
        }
        if ginput == 'a' {
            kx -= 1;
        }

        // Barrier
        if !(0..=79).contains(&kx) {
            kx = move_x_back(kbx);
        } else if !(0..=23).contains(&ky) {
            ky = move_y_back(kby);
        }

        if guardtime >= 0 {
            guardtime -= 1;
        } else {
            guardtime = 3;

            if guardsym == "G" {
                guardsym = "g";
            } else {
                guardsym = "G";
            }
        }
    }
}

fn at_point(x: i32, y: i32, x2: i32, y2: i32) -> bool {
    if x == x2 && y == y2 {
        return true;
    }
    false
}

fn move_x_back(kbx: i32) -> i32 {
    kbx
}

fn move_y_back(kby: i32) -> i32 {
    kby
}

//
fn tutorialch(window: &Window) {
    // Keegan Variables
    let mut kx: i32 = 39;
    let mut ky: i32 = 11;
    let mut kbx: i32 = kx;
    let mut kby: i32 = ky;

    let hudtext: [&str; 6] = [
        "Welcome to the Tutorial! Let's get one thing out of the way first.",
        "This is you, the letter \"K\".",
        "Anyways, with that out of the way, try moving with WASD!",
        "Good job! Now try moving to that \"E\".",
        "Now press \"E\" on your keyboard to examine.",
        "That's basically it, press \"0\" when you are ready to quit.",
    ];
    let debug = true;
    let mut tutorialnum = 0;
    window.keypad(true);
    noecho();
    // Game Loop
    // TODO: Set up tutorial
    // TODO: Make tutorial map
    loop {
        window.clear();
        set_blink(false);
        if debug {
            window.mvaddstr(0, 10, format!("X: {}, Y: {}", kx, ky));
            window.mvaddstr(1, 10, format!("BX: {}, BY: {}", kbx, kby));
        }
        window.mvaddstr(11, 50, "E");
        window.mvaddstr(ky, kx, "K");

        if tutorialnum <= 1 {
            window.mvaddstr(24, 0, "Press any key to continue...");
            window.mvaddstr(23, 0, hudtext[tutorialnum]);
        } else {
            window.mvaddstr(24, 0, hudtext[tutorialnum]);
        }

        window.refresh();
        let ginput: char;
        match window.getch() {
            Some(Input::Character(c)) => ginput = c,
            Some(Input::KeyDC) => break,
            Some(_input) => ginput = ' ',
            None => ginput = ' ',
        }

        // Set backup x and y values
        kby = ky;
        kbx = kx;

        // Check tutorialnum
        if tutorialnum <= 2 {
            tutorialnum += 1;
        }

        if tutorialnum > 1 {
            // Movement
            if ginput == 'w' {
                ky -= 1;
                if tutorialnum == 2 {
                    tutorialnum += 1;
                }
            } else if ginput == 's' {
                ky += 1;
                if tutorialnum == 2 {
                    tutorialnum += 1;
                }
            } else if ginput == 'a' {
                kx -= 1;
                if tutorialnum == 2 {
                    tutorialnum += 1;
                }
            } else if ginput == 'd' {
                kx += 1;
                if tutorialnum == 2 {
                    tutorialnum += 1;
                }
            }
        }
        if ginput == '0' {
            chapter_select(window);
        }

        if ginput == 'e' && at_point(kx, ky, 50, 11) && tutorialnum == 4 {
            tutorialnum += 1;
        }

        if at_point(kx, ky, 50, 11) && tutorialnum == 3 {
            tutorialnum += 1;
        }

        // Borders
        if !(0..=79).contains(&kx) {
            kx = move_x_back(kbx);
        } else if !(0..=23).contains(&ky) {
            ky = move_y_back(kby);
        }
    }
    chapter_select(window);
}
