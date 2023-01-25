extern crate argparse;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use argparse::{ArgumentParser, Store};

use std::collections::HashSet;

const ONE_2_NINE: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

fn get_one_2_nine() -> HashSet<i32> {
    return HashSet::from(ONE_2_NINE);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_empty_board() -> [[i32; 9]; 9] {
    return [
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
}

fn read_board(filename: &str) -> [[i32; 9]; 9] {
    println!("filename: {}", filename);
    let mut board: [[i32; 9]; 9] = get_empty_board();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut l = 0;
        let zero: char = '0';
        let nine: char = '9';
        for line in lines {
            if let Ok(ip) = line {
                let b: u8 = ip.as_bytes()[0];
                let c: char = b as char; // if you need to get the character as a unicode code point
                if (c >= zero) && (c <= nine) {
                    let mut i = 0;
                    for n in ip.split(",") {
                        let v: i32 = n.parse().unwrap();
                        board[l][i] = if v < 0 { 0 } else { v };
                        i += 1;
                    }
                    assert!(i == 9, "found {} values in row {}", i, l);
                    l += 1;
                }
            }
        }
        assert!(l == 9, "found {} rows in board", l);
    }
    assert!(is_valid(&board, true), "board is invalid!");
    return board;
}

fn print_board(board: &[[i32; 9]; 9]) {
    for row in board {
        for n in row {
            print!("{}  ", n);
        }
        println!("");
    }
}

fn get_opts(board: &mut [[i32; 9]; 9], r: usize, c: usize) -> HashSet<i32> {
    let mut opts: HashSet<i32> = HashSet::from(ONE_2_NINE);
    for r in get_row(board, r) {
        opts.remove(&r);
    }
    for c in get_col(board, c) {
        opts.remove(&c);
    }
    for b in get_box(board, r / 3, c / 3) {
        opts.remove(&b);
    }
    return opts;
}

fn solve_one(board: &mut [[i32; 9]; 9]) -> bool {
    for r in 0..=8 {
        for c in 0..=8 {
            if board[r][c] < 1 {
                let opts: HashSet<i32> = get_opts(board, r, c);
                if opts.len() == 0 {
                    println!("no options found for {}, {}!", r, c);
                    return false;
                }
                if opts.len() == 1 {
                    for o in &opts {
                        println!("setting {}, {} to {}", r, c, o);
                        board[r][c] = *o;
                        return true;
                    }
                }
            }
        }
    }
    return false;
}

fn get_row(board: &[[i32; 9]; 9], r: usize) -> [i32; 9] {
    let mut row: [i32; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for c in 0..=8 {
        row[c] = board[r][c];
    }
    return row;
}

fn get_col(board: &[[i32; 9]; 9], c: usize) -> [i32; 9] {
    let mut col: [i32; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for r in 0..=8 {
        col[r] = board[r][c];
    }
    return col;
}

fn get_box(board: &[[i32; 9]; 9], rb: usize, cb: usize) -> [i32; 9] {
    let mut bx: [i32; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut i = 0;
    for r in (rb * 3)..=((rb * 3) + 2) {
        for c in (cb * 3)..=((cb * 3) + 2) {
            bx[i] = board[r][c];
            i += 1;
        }
    }
    return bx;
}

fn get_backup(board: &[[i32; 9]; 9]) -> [[i32; 9]; 9] {
    let mut backup = get_empty_board();
    for r in 0..=8 {
        for c in 0..=8 {
            backup[r][c] = board[r][c];
        }
    }
    return backup;
}

fn backtrack(board: &mut [[i32; 9]; 9]) -> bool {
    struct Space {
        row: usize,
        col: usize,
        opts: Vec<i32>,
    }
    let mut space: Vec<Space> = Vec::new();
    for r in 0..=8 {
        for c in 0..=8 {
            if board[r][c] < 1 {
                let opts: Vec<i32> = get_opts(board, r, c).into_iter().collect();
                space.push(Space {
                    row: r,
                    col: c,
                    opts: opts,
                });
            }
        }
    }
    space.sort_by(|a, b| a.opts.len().cmp(&b.opts.len()));
    assert!(space.len() > 0);
    let s = &space[0];
    if s.opts.len() == 0 {
        return false;
    }
    // make a backup
    let backup: [[i32; 9]; 9] = get_backup(board);
    for o in &s.opts {
        println!("backtracking on {}, {} = {}", s.row, s.col, o);
        // enforce the option
        board[s.row][s.col] = *o;
        // try to solve
        if solve(board) {
            return true;
        } else {
            println!(
                "backtracking on {}, {} = {} failed! reverting board",
                s.row, s.col, o
            );
            // revert
            for r in 0..=8 {
                for c in 0..=8 {
                    board[r][c] = backup[r][c];
                }
            }
        }
    }
    return false;
}

fn solve(board: &mut [[i32; 9]; 9]) -> bool {
    while (!done(board)) & is_valid(board, false) {
        if !solve_one(board) {
            return backtrack(board);
        }
    }
    return done(board) & is_valid(board, false);
}

fn done(board: &[[i32; 9]; 9]) -> bool {
    let o2n = get_one_2_nine();
    // check each row & col
    for i in 0..=8 {
        let row: HashSet<i32> = HashSet::from(get_row(board, i));
        if row != o2n {
            return false;
        }
        let col: HashSet<i32> = HashSet::from(get_col(board, i));
        if col != o2n {
            return false;
        }
    }
    // check each box
    for rb in 0..=2 {
        for cb in 0..=2 {
            let bx: HashSet<i32> = HashSet::from(get_box(board, rb, cb));
            if bx != o2n {
                return false;
            }
        }
    }
    return true;
}

fn is_valid(board: &[[i32; 9]; 9], verbose: bool) -> bool {
    for i in 0..=8 {
        if !is_valid_set(get_row(board, i)) {
            if verbose {
                println!("row {} is invalid", i);
            }
            return false;
        }
        if !is_valid_set(get_col(board, i)) {
            if verbose {
                println!("col {} is invalid", i);
            }
            return false;
        }
    }
    for rb in 0..=2 {
        for cb in 0..=2 {
            if !is_valid_set(get_box(board, rb, cb)) {
                if verbose {
                    println!("box {}, {} is invalid", rb, cb);
                }
                return false;
            }
        }
    }
    return true;
}

fn is_valid_set(set: [i32; 9]) -> bool {
    let mut counts = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for i in 0..=8 {
        // only check the non-zero
        if set[i] > 0 {
            let p: usize = (set[i] - 1).try_into().unwrap();
            counts[p] += 1;
        }
    }
    for i in 0..=8 {
        if counts[i] > 1 {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut filename = "puzzles/sudoku-com-evil-20221206T173500.txt".to_string();
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Sudoku Solver.");
        ap.refer(&mut filename)
            .add_option(&["-f", "--file"], Store, "Puzzle file to solve");
        ap.parse_args_or_exit();
    }

    let mut board = read_board(&filename);
    println!("Starting board:");
    print_board(&board);

    solve(&mut board);
    let mut out = 1;
    if done(&board) {
        println!("Solved board!");
        out = 0;
    } else {
        println!("Failed to solve board!");
    }
    print_board(&board);
    std::process::exit(out);
}
