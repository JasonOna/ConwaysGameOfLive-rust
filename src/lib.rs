use std::process::Command;
use std::{thread, time};

#[derive(Debug, Clone, Copy)]
pub struct Cell {
  pub x: i8,
  pub y: i8,
  pub alive: bool,
}

impl Cell {
  pub fn new(x: i8, y: i8, state: bool) -> Cell {
    Cell {
      x: x,
      y: y,
      alive: state,
    }
  }

  pub fn set_alive(&mut self, state: bool) {
    self.alive = state;
  }
}

#[derive(Debug)]
pub struct Board {
  pub previous_cells: Vec<Cell>,
  pub current_cells: Vec<Cell>,
}

impl Board {
  pub fn create(string: &str) -> Board {
    let mut cells: Vec<Cell> = vec![];

    for (x, row) in string.split("\n").enumerate() {
      for (y, item) in row.split(",").enumerate() {
        if item == "1" {
          cells.push(Cell::new(x as i8, y as i8, true));
        } else {
          cells.push(Cell::new(x as i8, y as i8, false));
        }
      }
    }

    Board {
      current_cells: cells,
      previous_cells: vec![],
    }
  }

  pub fn print(&self) {
    let alive = "\u{1F7E9}";
    let dead = "\u{2B1B}";

    let ten_millis = time::Duration::from_millis(500);

    thread::sleep(ten_millis);

    let output = Command::new("clear")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    println!("{}", String::from_utf8_lossy(&output.stdout));

    for row in 0..10 {
      for col in 0..10 {
        if let Some(cell) = self.current_cells.iter().find(|cell| cell.x == row && cell.y == col) {
          if cell.alive {
            print!("{}", alive);
          } else {
            print!("{}", dead);
          }
        } else {}
      }
      println!("");
    }
  }

  fn live_neighbors(cells: &Vec<Cell>, x: i8, y: i8) -> i8 {
    cells.iter().filter(|c|
      x - 1 == c.x && y - 1 == c.y && c.alive ||
      x - 0 == c.x && y - 1 == c.y && c.alive ||
      x + 1 == c.x && y - 1 == c.y && c.alive ||
      x - 1 == c.x && y - 0 == c.y && c.alive ||
      x + 1 == c.x && y - 0 == c.y && c.alive ||
      x - 1 == c.x && y + 1 == c.y && c.alive ||
      x - 0 == c.x && y + 1 == c.y && c.alive ||
      x + 1 == c.x && y + 1 == c.y && c.alive
    ).count() as i8
  }

  pub fn calculate_next_board(&mut self) {
    self.previous_cells = vec![];

    for cell in self.current_cells.iter() {
      self.previous_cells.push(cell.clone());
    }

    for cell in self.current_cells.iter_mut() {
      let neighbors = Board::live_neighbors(&self.previous_cells, cell.x, cell.y);
      if cell.alive {
        if neighbors < 2 || neighbors > 3 {
          cell.set_alive(false);
        } else {
          cell.set_alive(true);
        }
      } else {
        cell.set_alive(neighbors == 3);
      }
    }
  }
}