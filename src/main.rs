#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Write, Stdout};

const GRID_SIZE_X: usize = 10;
const GRID_SIZE_Y: usize = 10;

fn main() {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    //clear screen and print
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), Print(r#"q to exit""#)).unwrap();
    //create grid and show
    let mut grid:[[i32; GRID_SIZE_X]; GRID_SIZE_Y] = create_grid();
    let mut pos:(usize,usize) = (5,5);
    let mut pos_old:(usize,usize) = (0,0);
    // show_grid(&mut grid);
    update_grid(&mut grid,&mut pos,&mut pos_old);
    show_grid(&mut grid);
    loop {
        //matching the key
        match read().unwrap() {
            //quit
            Event::Key(KeyEvent { 
                code: KeyCode::Char('q'),
                modifiers: _,
                kind: KeyEventKind::Press,
                state: _,
            }) => break,
            Event::Key(KeyEvent { 
                code: KeyCode::Char('w'),
                modifiers: _,
                kind: KeyEventKind::Press,
                state: _,
            }) => {if pos.0!=0{pos_old=pos;pos.0-=1;update_grid(&mut grid,&mut pos,&mut pos_old);show_grid(&mut grid)}},
            Event::Key(KeyEvent { 
                code: KeyCode::Char('a'),
                modifiers: _,
                kind: KeyEventKind::Press,
                state: _,
            }) => {if pos.1!=0{pos_old=pos;pos.1-=1;update_grid(&mut grid,&mut pos,&mut pos_old);show_grid(&mut grid)}},
            Event::Key(KeyEvent { 
                code: KeyCode::Char('s'),
                modifiers: _,
                kind: KeyEventKind::Press,
                state: _,
            }) => {pos_old=pos;pos.0+=1;update_grid(&mut grid,&mut pos,&mut pos_old);show_grid(&mut grid)},
            Event::Key(KeyEvent { 
                code: KeyCode::Char('d'),
                modifiers: _,
                kind: KeyEventKind::Press,
                state: _,
            }) => {pos_old=pos;pos.1+=1;update_grid(&mut grid,&mut pos,&mut pos_old);show_grid(&mut grid)},
            _ => (),
        }
    }
    disable_raw_mode().unwrap();
}

// creates a 10x10 grid of 0's
fn create_grid() -> [[i32; GRID_SIZE_X]; GRID_SIZE_Y] {
    [[0; GRID_SIZE_X]; GRID_SIZE_Y]
}

// updates and prints the grid
fn update_grid(grid: &mut [[i32;GRID_SIZE_X];GRID_SIZE_Y],pos: &mut (usize,usize),pos_old: &mut (usize,usize)) {
    execute!(stdout(),cursor::MoveTo(0,11)).unwrap();
    write!(stdout(),"({},{})",pos.0,pos.1).unwrap();
    if pos.0 < 10 && pos.1 < 10 {
        grid[pos.0][pos.1] = 1;
        grid[pos_old.0][pos_old.1] = 0;
    }else {
        pos.0 = pos_old.0;
        pos.1 = pos_old.1;
    }
    // execute!(stdout(), cursor::MoveTo()) 
}

// prints the whole grid
fn show_grid(grid: &mut [[i32;GRID_SIZE_X];GRID_SIZE_Y]) {
    execute!(stdout(), cursor::MoveTo(0,1)).unwrap();
    for (i,el) in grid.iter().enumerate() {
        write!(stdout(),"{}: ",i).unwrap();
        for j in el.iter() {
            write!(stdout(),"{:?} ",j).unwrap();
        }
        let x:u16 = i as u16;
        execute!(stdout(), cursor::MoveTo(0,x+2)).unwrap();
    }
    println!("");
}