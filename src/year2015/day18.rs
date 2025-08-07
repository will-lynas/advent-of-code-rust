use crate::utils::grid::Grid;

type Input = Grid<CellState>;

#[derive(Clone, Default, PartialEq, Copy)]
pub enum CellState {
    On,
    #[default]
    Off,
}

pub fn parse(input: &str) -> Input {
    let lines: Vec<_> = input.lines().collect();
    let n = lines.len();
    let mut grid: Grid<CellState> = Grid::new(n as i32, n as i32);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[(x, y)] = match c {
                '#' => CellState::On,
                '.' => CellState::Off,
                _ => unreachable!(),
            };
        }
    }
    grid
}

pub fn part1(grid: &Input) -> usize {
    let mut grid = grid.clone();
    for _ in 0..100 {
        let mut new_grid = grid.clone();
        for (point, cell) in &grid {
            let count = grid
                .neighbors(point)
                .iter()
                .map(|&point| grid[point])
                .filter(|&cell| cell == CellState::On)
                .count();
            let new_cell = match cell {
                CellState::On => {
                    if count == 2 || count == 3 {
                        CellState::On
                    } else {
                        CellState::Off
                    }
                }
                CellState::Off => {
                    if count == 3 {
                        CellState::On
                    } else {
                        CellState::Off
                    }
                }
            };
            new_grid[point] = new_cell;
        }
        grid = new_grid;
    }

    grid.iter()
        .filter(|&(_, &cell)| cell == CellState::On)
        .count()
}

pub fn part2(_grid: &Input) -> usize {
    0
}
