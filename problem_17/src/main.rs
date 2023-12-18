use std::{
    collections::{BinaryHeap, HashSet},
    env,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

type Grid = Vec<Vec<u64>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_move_values(&self) -> (i64, i64) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn get_90_deg_turn_directions(&self) -> Vec<Direction> {
        match self {
            Direction::Up => vec![Direction::Right, Direction::Left],
            Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Left => vec![Direction::Down, Direction::Up],
            Direction::Right => vec![Direction::Up, Direction::Down],
        }
    }
}

#[derive(Clone, Eq)]
struct Tile {
    heat: u64,
    row: i64,
    col: i64,
    direction: Direction,
    same_direction_count: u8,
}

impl Tile {
    fn new(heat: u64, row: i64, col: i64, direction: Direction, same_direction_count: u8) -> Tile {
        Tile {
            heat,
            row,
            col,
            direction,
            same_direction_count,
        }
    }

    fn get_next_pos(&self) -> (i64, i64) {
        let (row_change, col_change) = self.direction.get_move_values();
        (self.row + row_change, self.col + col_change)
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, ({}, {}), {:?}, {}]",
            self.heat, self.row, self.col, self.direction, self.same_direction_count
        )
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.row.eq(&other.row)
            && self.col.eq(&other.col)
            && self.direction.eq(&other.direction)
            && self.same_direction_count.eq(&other.same_direction_count)
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat.cmp(&other.heat).reverse() // Since rust heap is max, reverse this
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct TileWithoutHeat {
    row: i64,
    col: i64,
    direction: Direction,
    same_direction_count: u8,
}

impl TileWithoutHeat {
    fn from_tile(tile: Tile) -> TileWithoutHeat {
        TileWithoutHeat {
            row: tile.row,
            col: tile.col,
            direction: tile.direction,
            same_direction_count: tile.same_direction_count,
        }
    }
}

fn dijkstra_part1(grid: &Grid) -> u64 {
    let row_count = grid.len();
    let col_count = grid.get(0).unwrap().len();

    let mut visited: HashSet<TileWithoutHeat> = HashSet::new();
    let mut heap: BinaryHeap<Tile> = BinaryHeap::new();
    let first_right_tile = Tile::new(
        *grid.get(0).unwrap().get(1).unwrap(),
        0,
        1,
        Direction::Right,
        1,
    );
    let first_down_tile = Tile::new(
        *grid.get(1).unwrap().get(0).unwrap(),
        1,
        0,
        Direction::Down,
        1,
    );

    heap.push(first_right_tile);
    heap.push(first_down_tile);

    while !heap.is_empty() {
        let tile = heap.pop().unwrap();

        if tile.row + 1 == row_count as i64 && tile.col + 1 == col_count as i64 {
            return tile.heat;
        }

        let tile_without_heat = TileWithoutHeat::from_tile(tile.clone());
        if visited.contains(&tile_without_heat) {
            continue;
        }

        visited.insert(tile_without_heat);

        if tile.same_direction_count < 3 {
            let (next_row, next_col) = tile.get_next_pos();
            if 0 <= next_row
                && next_row < row_count as i64
                && 0 <= next_col
                && next_col < col_count as i64
            {
                heap.push(Tile::new(
                    tile.heat
                        + grid
                            .get(next_row as usize)
                            .unwrap()
                            .get(next_col as usize)
                            .unwrap(),
                    next_row,
                    next_col,
                    tile.direction,
                    tile.same_direction_count + 1,
                ));
            }
        }

        for direction in tile.direction.get_90_deg_turn_directions() {
            let (row_change, col_change) = direction.get_move_values();
            let (next_row, next_col) = (tile.row + row_change, tile.col + col_change);
            if 0 <= next_row
                && next_row < row_count as i64
                && 0 <= next_col
                && next_col < col_count as i64
            {
                heap.push(Tile::new(
                    tile.heat
                        + grid
                            .get(next_row as usize)
                            .unwrap()
                            .get(next_col as usize)
                            .unwrap(),
                    next_row,
                    next_col,
                    direction,
                    1,
                ));
            }
        }
    }

    0
}

fn dijkstra_part2(grid: &Grid) -> u64 {
    let row_count = grid.len();
    let col_count = grid.get(0).unwrap().len();

    let mut visited: HashSet<TileWithoutHeat> = HashSet::new();
    let mut heap: BinaryHeap<Tile> = BinaryHeap::new();
    let first_right_tile = Tile::new(
        *grid.get(0).unwrap().get(1).unwrap(),
        0,
        1,
        Direction::Right,
        1,
    );
    let first_down_tile = Tile::new(
        *grid.get(1).unwrap().get(0).unwrap(),
        1,
        0,
        Direction::Down,
        1,
    );

    heap.push(first_right_tile);
    heap.push(first_down_tile);

    while !heap.is_empty() {
        let tile = heap.pop().unwrap();

        if tile.row + 1 == row_count as i64 && tile.col + 1 == col_count as i64 {
            return tile.heat;
        }

        let tile_without_heat = TileWithoutHeat::from_tile(tile.clone());
        if visited.contains(&tile_without_heat) {
            continue;
        }

        visited.insert(tile_without_heat);

        if tile.same_direction_count < 10 {
            let (next_row, next_col) = tile.get_next_pos();
            if 0 <= next_row
                && next_row < row_count as i64
                && 0 <= next_col
                && next_col < col_count as i64
            {
                heap.push(Tile::new(
                    tile.heat
                        + grid
                            .get(next_row as usize)
                            .unwrap()
                            .get(next_col as usize)
                            .unwrap(),
                    next_row,
                    next_col,
                    tile.direction,
                    tile.same_direction_count + 1,
                ));
            }
        }

        if tile.same_direction_count > 3 {
            for direction in tile.direction.get_90_deg_turn_directions() {
                let (row_change, col_change) = direction.get_move_values();
                let (next_row, next_col) = (tile.row + row_change, tile.col + col_change);
                if 0 <= next_row
                    && next_row < row_count as i64
                    && 0 <= next_col
                    && next_col < col_count as i64
                {
                    heap.push(Tile::new(
                        tile.heat
                            + grid
                                .get(next_row as usize)
                                .unwrap()
                                .get(next_col as usize)
                                .unwrap(),
                        next_row,
                        next_col,
                        direction,
                        1,
                    ));
                }
            }
        }
    }

    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let grid: Grid = BufReader::new(File::open(args.get(1).unwrap()).unwrap())
        .lines()
        .map(|line_result| {
            line_result
                .unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    let part1 = dijkstra_part1(&grid);
    let part2 = dijkstra_part2(&grid);

    println!("Result part1: {}", part1);
    println!("Result part2: {}", part2);
}
