use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::env;

extern crate ncurses;
use ncurses::*;

//extern crate getopts;
//use getopts::Options;

fn main() {
    let mut buffer = vec![];
    let mut cursorpos:usize = 0;
    const SPALTEN:usize = 16;
    let mut screenheight;

    initscr(); //start ncursesw
    screenheight = to_usize(getmaxy(stdscr));
    cbreak();  //ctrl+z and fg works with this
    noecho();
    start_color();
    init_pair(1, COLOR_GREEN, COLOR_BLACK);

    printw("Welcome to Hexdino.\nPlease find the any-key on your keyboard and press it.\n");

    let args: Vec<_> = env::args().collect();
    /*
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("f", "", "set file name", "NAME");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        let brief = format!("Usage: {} FILE [options]", program);
            print!("{}", opts.usage(&brief));
        return;
    }
    let path = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        let brief = format!("Usage: {} FILE [options]", program);
            print!("{}", opts.usage(&brief));
        return;
    };*/

    let path = if args.len() > 1 {
        printw(&format!("Opening file {} (TODO)", args[1]));
        args[1].clone()
    } else {
        "foo.txt".into()
    };

    let path = Path::new(&path);
    let display = path.display();

    getch();
    clear();

    if !has_colors() {
        endwin();
        println!("Your terminal does not support color!\n");
        return;
    }

//TODO in this order: 1)disp files 2) hjkl movement 3) edit file by 'r' 4) save file 5) edit file by 'x, i' 6) search by '/'

    let mut file = match File::open(&path) {
//        Err(why) => printw("oh... file not opened..."),
        Err(why) => panic!("couldn't open {}: {}", display,
            Error::description(&why)),
        Ok(file) => file,
    };

//    file.read_to_end(&mut buffer).ok().expect("File could not be read.");
    file.read_to_end(&mut buffer).ok().expect("File could not be read.");
    draw(&buffer, cursorpos, SPALTEN, screenheight);

    let mut key;
    let mut mode = 0; // 0 Command mode, 1 Insert mode

//    key = getch();
//    printw(&format!("{:?}", key));

    let mut ragequitnow = 0;
    while ragequitnow == 0 {
        key = getch();
        if mode == 0 {
            match key {
                104 => if cursorpos != 0 {cursorpos-=1}, //left is "-1"
                106 => if cursorpos+SPALTEN < buffer.len() {cursorpos+=SPALTEN} //down is "+16"
                        else {cursorpos=buffer.len()-1}, //down if on last line is "to end"
                107 => if cursorpos >= SPALTEN {cursorpos-=SPALTEN}, //up is "-16"
                108 => if cursorpos != buffer.len()-1 {cursorpos+=1}, //right is "+1"
                 48 => cursorpos -= cursorpos%16, //start of line is "to start"
                 36 => if cursorpos-(cursorpos%16)+(SPALTEN-1) < buffer.len() {cursorpos = cursorpos-(cursorpos%16)+(SPALTEN-1)} //dollar is "to end"
                        else {cursorpos = buffer.len()-1},
                114 => mode = 1, //r replaces the char
                 27 => ragequitnow = 1, // TODO replace by KEY_...?
                 58 => (), //TODO doppelpunkt, befehlsmodus! screenheight;
//                 63 => printw("{:?}", asdf), //TODO: print available key helpfile
                _ => (),
            }
        } else {
            match key {
                c @ 32...126 => { buffer[cursorpos] = c as u8; mode = 0 },
                27 => mode = 0,
                _ => (),
            }
        }
 //       printw(&format!("{:?}", key));
        draw(&buffer, cursorpos, SPALTEN, screenheight);
    }

    refresh();
    endwin();
}

fn draw(buffer:&Vec<u8>, cursorpos:usize, spalten:usize, maxzeilen:usize) {
//    let zeilen = buffer.len() / spalten;
    erase();

    let mut zeilen = buffer.len() / spalten;
    if zeilen >= maxzeilen {
        zeilen = maxzeilen-1;
    }

    for z in 0 .. zeilen+1 {
        for s in 0 .. spalten {
            if z*spalten+s == cursorpos { attron(COLOR_PAIR(1)); }
            if z*spalten+s < buffer.len() { printw(&format!("{:02X} ", buffer[z*spalten+s]) ); }
                else {printw("-- "); }
            if z*spalten+s == cursorpos { attroff(COLOR_PAIR(1)); }
        }
        printw(" "); // Space between hex and ascii
        for s in 0 .. spalten {
            if z*spalten+s == cursorpos { attron(COLOR_PAIR(1)); }
                if z*spalten+s < buffer.len() {
                    if let c @ 32...126 = buffer[z*spalten+s] {printw(&format!("{}", c as char) );}
                    else {printw(&format!(".") );}
                }
            if z*spalten+s == cursorpos { attroff(COLOR_PAIR(1)); }
        }
        printw("\n");
    }
}
