use std::mem::align_of;

fn get_input() -> String {
    // Your code goes here
    include_str!("../../inputs/day8.txt").to_string()
}

struct Grid {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            data: vec![0; width * height],
        }
    }
}

// returns column y of the grid
fn column(grid: &Grid, x: usize) -> Vec<u8> {
    let mut col = Vec::with_capacity(grid.height);
    for y in 0..grid.height {
        col.push(grid.data[x + y * grid.width]);
    }
    col
}

fn row(grid: &Grid, y: usize) -> Vec<u8> {
    let mut row = Vec::with_capacity(grid.width);
    for x in 0..grid.width {
        row.push(grid.data[x + y * grid.width]);
    }
    row
}
fn print_pixels(pixels: &Vec<u8>, pos: usize, is_col: bool) {
    // print pixels
    println!("{} pixels: {}", if is_col { "col" } else { "row" }, pos);
    for c in pixels.iter() {
        print!("{}", if c == &1 { "#" } else { "." });
    }
    println!();
    println!();
}
// shift and rotate pixels in column down by shift
fn shift_and_rotate_pixels(grid: &mut Grid, is_col: bool, pos: usize, shift: i32) {
    let mut pixels;
    if is_col {
        pixels = column(grid, pos);
    } else {
        pixels = row(grid, pos);
    }

    let shift = if is_col {
        grid.height as i32 - shift % grid.height as i32
    } else {
        grid.width as i32 - shift % grid.width as i32
    };

    print_pixels(&pixels, pos, is_col);

    if shift < 0 {
        pixels.rotate_right(-shift as usize);
    } else {
        pixels.rotate_left(shift as usize);
    }
    print_pixels(&pixels, pos, is_col);
    for c in 0..(if is_col { grid.height } else { grid.width }) {
        let index = if is_col {
            pos + c * grid.width
        } else {
            c + pos * grid.width
        };
        grid.data[index] = pixels[c];
    }
}

// print grid where each on pixel is a #
fn print_grid(grid: &Grid) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.data[x + y * grid.width] == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

// place x by y rectangle of pixels in the top left corner of the grid
fn rect(grid: &mut Grid, x: usize, y: usize) {
    for y in 0..y {
        for x in 0..x {
            grid.data[x + y * grid.width] = 1;
        }
    }
}

fn count_pixels(grid: &Grid) -> usize {
    grid.data
        .iter()
        .filter(|&c| *c == 1)
        .collect::<Vec<&u8>>()
        .len()
}

fn parse_line_and_do_it(grid: &mut Grid, line: &str) {
    // the following is pretty much written by copilot! awesome!
    let split: Vec<&str> = line.split(" ").collect();
    if split[0] == "rect" {
        let mut split = split[1].split("x");
        let x = split.next().unwrap().parse::<usize>().unwrap();
        let y = split.next().unwrap().parse::<usize>().unwrap();
        rect(grid, x, y);
    } else if split[0] == "rotate" && split[1] == "column" {
        let pos = split[2]
            .split("=")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let shift = split[4].parse::<i32>().unwrap();
        shift_and_rotate_pixels(grid, true, pos, shift);
    } else if split[0] == "rotate" && split[1] == "row" {
        let pos = split[2]
            .split("=")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let shift = split[4].parse::<i32>().unwrap();
        shift_and_rotate_pixels(grid, false, pos, shift);
    } else {
        panic!("unknown command");
    }
}

pub(crate) fn main() {
    let mut grid = Grid::new(50, 6);
    get_input()
        .lines()
        .for_each(|line| parse_line_and_do_it(&mut grid, line));
    print_grid(&grid);
    println!("pixels: {}", count_pixels(&grid));
}
