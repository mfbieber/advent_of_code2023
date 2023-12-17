use std::ops::Deref;
use std::path::PathBuf;
use num::Integer;
use crate::read_lines;

#[derive(PartialEq, Debug, Copy, Clone)]
struct Tile {
    char: char,
    x: i32,
    y: i32,
    passed: bool,
    distance: i32,
    enclosed: bool
}

impl Tile {

    fn new(char: char, x: i32, y: i32) -> Tile {
        return Tile {
            char,
            x,
            y,
            passed: false,
            distance: -1,
            enclosed: false
        }
    }

    fn get_connected_pipes(&self, tiles: &Vec<Vec<Tile>>) -> (Tile, Tile) {
        let mut x1: i32 = -1;
        let mut y1: i32 = -1;
        let mut x2: i32 = -1;
        let mut y2: i32 = -1;
        if self.char == '|' {
            x1 = self.x;
            x2 = self.x;
            if self.y > 0 {
                y1 = self.y - 1;
            }
            y2 = self.y + 1;
        } else if self.char == '-' {
            y1 = self.y;
            y2 = self.y;
            if self.x > 0 {
                x1 = self.x - 1;
            }
            x2 = self.x + 1;
        } else if self.char == 'L' {
            x1 = self.x;
            if self.y > 0 {
                y1 = self.y - 1;
            }
            y2 = self.y;
            x2 = self.x + 1;
        } else if self.char == 'J' {
            x1 = self.x;
            if self.y > 0 {
                y1 = self.y - 1;
            }
            y2 = self.y;
            x2 = self.x - 1;
        } else if self.char == 'F' {
            x1 = self.x;
            y1 = self.y + 1;
            y2 = self.y;
            x2 = self.x + 1;
        } else if self.char == '7' {
            x1 = self.x;
            y1 = self.y + 1;
            y2 = self.y;
            if self.x > 0 {
                x2 = self.x - 1;
            }
        } else if self.char == 'S' {
            let mut neighbors: Vec<Tile> = vec![];
            //top not L - J
            if self.y > 0 && tiles[(self.y - 1) as usize][self.x as usize].char != '.'
                && tiles[(self.y - 1) as usize][self.x as usize].char != 'L'
                && tiles[(self.y - 1) as usize][self.x as usize].char != '-'
                && tiles[(self.y - 1) as usize][self.x as usize].char != 'J' {
                neighbors.push(tiles[(self.y - 1) as usize][self.x as usize]);
            }
            //down not F - 7
            if tiles[(self.y + 1) as usize][self.x as usize].char != '.'
                && tiles[(self.y + 1) as usize][self.x as usize].char != 'F'
                && tiles[(self.y + 1) as usize][self.x as usize].char != '-'
                && tiles[(self.y + 1) as usize][self.x as usize].char != '7' {
                neighbors.push(tiles[(self.y + 1) as usize][self.x as usize]);
            }
            //left not 7 | J
            if self.x > 0 && tiles[self.y as usize][(self.x - 1) as usize].char != '.'
                && tiles[self.y as usize][(self.x - 1) as usize].char != '7'
                && tiles[self.y as usize][(self.x - 1) as usize].char != '|'
                && tiles[self.y as usize][(self.x - 1) as usize].char != 'J' {
                neighbors.push(tiles[self.y as usize][(self.x - 1) as usize]);
            }
            //right not F | L
            if tiles[self.y as usize][(self.x + 1) as usize].char != '.'
                && tiles[self.y as usize][(self.x + 1) as usize].char != 'F'
                && tiles[self.y as usize][(self.x + 1) as usize].char != '|'
                && tiles[self.y as usize][(self.x + 1) as usize].char != 'L' {
                neighbors.push(tiles[self.y as usize][(self.x + 1) as usize]);
            }
            x1 = neighbors[0].x;
            y1 = neighbors[0].y;
            x2 = neighbors[1].x;
            y2 = neighbors[1].y;
        }
        let mut tile1: Tile = Tile::new('.', -1, -1);
        let mut tile2: Tile = Tile::new('.', -1, -1);
        if y1 < 0 || x1 < 0 {
            tile1.passed = true;
        } else {
            tile1 = tiles[y1 as usize][x1 as usize].clone();
        }
        if y2 < 0 || x2 < 0 {
            tile2.passed = true;
        } else {
            tile2 = tiles[y2 as usize][x2 as usize].clone();
        }
        return (tile1, tile2);
    }
}

fn read_tiles(path: &PathBuf) -> Vec<Vec<Tile>> {
    let mut tiles: Vec<Vec<Tile>> = vec![];
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            x = 0;
            let line_tiles: Vec<Tile> = line
                .as_ref()
                .unwrap()
                .chars()
                .map(|char| {
                    let tile: Tile = Tile::new(char, x, y);
                    x += 1;
                    return tile;
                })
                .collect::<Vec<Tile>>();
            y += 1;
            tiles.push(line_tiles);
        }
    }
    return tiles;
}

fn get_longest_path(tiles: &mut Vec<Vec<Tile>>, x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let mut next_tiles0: (Tile, Tile) = tiles[y1 as usize][x1 as usize].get_connected_pipes(tiles);
    let mut next_tiles1: (Tile, Tile) = tiles[y2 as usize][x2 as usize].get_connected_pipes(tiles);
    let mut next_tile0: Tile = tiles[y1 as usize][x1 as usize];
    let mut next_tile1: Tile = tiles[y2 as usize][x2 as usize];
    let mut distance: i32 = 0;
    let mut end: bool = false;
    while !end {
        tiles[next_tile0.y as usize][next_tile0.x as usize].passed = true;
        tiles[next_tile0.y as usize][next_tile0.x as usize].distance = distance + 1;
        tiles[next_tile1.y as usize][next_tile1.x as usize].passed = true;
        tiles[next_tile1.y as usize][next_tile1.x as usize].distance = distance + 1;
        if !next_tiles0.0.passed {
            next_tile0 = next_tiles0.0;
        } else if !next_tiles0.1.passed {
            next_tile0 = next_tiles0.1;
        } else {
            end = true;
            break;
        }
        if !next_tiles1.0.passed {
            next_tile1 = next_tiles1.0;
        } else if !next_tiles1.1.passed {
            next_tile1 = next_tiles1.1;
        } else {
            end = true;
            break;
        }
        next_tiles0 = tiles[next_tile0.y as usize][next_tile0.x as usize].get_connected_pipes(tiles);
        next_tiles1 = tiles[next_tile1.y as usize][next_tile1.x as usize].get_connected_pipes(tiles);
        distance += 1;
    }
    return distance + 1;
}

fn find_start_tile(tiles: &Vec<Vec<Tile>>) -> Tile {
    let mut start_tile: Tile = Tile::new('S', 0, 0);
    tiles.iter().for_each(|line| {
        line.iter().for_each(|tile | {
            if tile.char == 'S' {
                start_tile = *tile;
            }
        })
    });
    return start_tile;
}

//https://www.eecs.umich.edu/courses/eecs380/HANDOUTS/PROJ2/InsidePoly.html
//Determining if a point lies on the interior of a polygon
fn find_enclosed_tiles(tiles: &mut Vec<Vec<Tile>>) -> i32 {
    let mut enclosed_tiles: i32 = 0;
    let mut y: usize = 0;
    let mut x: usize = 0;
    while y < tiles.len() {
        let mut traverse: i32 = 0;
        while x < tiles[y].len() {
            if tiles[y][x].passed
                && (tiles[y][x].char == '|' || tiles[y][x].char == 'L'
                || tiles[y][x].char == 'J' ){
                traverse += 1;
            } else if !tiles[y][x].passed && traverse != 0 && traverse.is_odd() {
                tiles[y][x].enclosed = true;
                enclosed_tiles += 1;
            }
            x += 1;
        }
        x = 0;
        traverse = 0;
        y += 1;
    }
    return enclosed_tiles;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day10::{find_enclosed_tiles, find_start_tile, get_longest_path, read_tiles, Tile};

    #[test]
    fn test_finds_longest_path_length_correctly_start_input2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day10/test/input2.txt");
        let mut tiles: Vec<Vec<Tile>> = read_tiles(&d);
        let mut start_tile: Tile = find_start_tile(&tiles);
        let connected = start_tile.get_connected_pipes(&tiles);
        tiles[start_tile.y as usize][start_tile.x as usize].distance = 0;
        tiles[start_tile.y as usize][start_tile.x as usize].passed = true;
        let path_length: i32 = get_longest_path(&mut tiles, connected.0.x, connected.0.y, connected.1.x, connected.1.y);
        assert_eq!(path_length, 8);
    }

    #[test]
    fn test_finds_longest_path_length_correctly_start_input() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day10/test/input1.txt");
        let mut tiles: Vec<Vec<Tile>> = read_tiles(&d);
        let mut start_tile: Tile = find_start_tile(&tiles);
        let connected = start_tile.get_connected_pipes(&tiles);
        tiles[start_tile.y as usize][start_tile.x as usize].distance = 0;
        tiles[start_tile.y as usize][start_tile.x as usize].passed = true;
        let path_length: i32 = get_longest_path(&mut tiles, connected.0.x, connected.0.y, connected.1.x, connected.1.y);
        assert_eq!(path_length, 4);
    }

    #[test]
    fn test_finds_longest_path_length_correctly() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day10/test/input.txt");
        let mut tiles: Vec<Vec<Tile>> = read_tiles(&d);
        let connected = tiles[1][1].get_connected_pipes(&tiles);
        tiles[1][1].distance = 0;
        tiles[1][1].passed = true;
        let path_length: i32 = get_longest_path(&mut tiles, connected.0.x, connected.0.y, connected.1.x, connected.1.y);
        assert_eq!(path_length, 4);
    }

    #[test]
    fn test_gets_neighboring_tiles_correctly() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day10/test/input.txt");
        let tiles: Vec<Vec<Tile>> = read_tiles(&d);
        let tile: Tile = Tile::new('F',1,1);
        let neighbor_tiles: (Tile, Tile) = tile.get_connected_pipes(&tiles);
        assert_eq!(neighbor_tiles.1, Tile::new('-',2,1));
        assert_eq!(neighbor_tiles.0, Tile::new('|',1,2));
    }

    #[test]
    fn test_reads_tiles_correctly() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day10/test/input.txt");
        let expected_tiles: Vec<Vec<Tile>> = vec![
            vec![Tile::new('.',0,0), Tile::new('.',1,0), Tile::new('.',2,0), Tile::new('.',3,0), Tile::new('.',4,0)],
            vec![Tile::new('.',0,1), Tile::new('F',1,1), Tile::new('-',2,1), Tile::new('7',3,1), Tile::new('.',4,1)],
            vec![Tile::new('.',0,2), Tile::new('|',1,2), Tile::new('.',2,2), Tile::new('|',3,2), Tile::new('.',4,2)],
            vec![Tile::new('.',0,3), Tile::new('L',1,3), Tile::new('-',2,3), Tile::new('J',3,3), Tile::new('.',4,3)],
            vec![Tile::new('.',0,4), Tile::new('.',1,4), Tile::new('.',2,4), Tile::new('.',3,4), Tile::new('.',4,4)],
        ];
        assert_eq!(read_tiles(&d), expected_tiles);
    }

    #[test]
    fn part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day10/input.txt");
        let mut tiles: Vec<Vec<Tile>> = read_tiles(&d);
        let mut start_tile: Tile = find_start_tile(&tiles);
        let connected = start_tile.get_connected_pipes(&tiles);
        tiles[start_tile.y as usize][start_tile.x as usize].distance = 0;
        tiles[start_tile.y as usize][start_tile.x as usize].passed = true;
        let path_length: i32 = get_longest_path(&mut tiles, connected.0.x, connected.0.y, connected.1.x, connected.1.y);
        assert_eq!(path_length, 7173);
    }

    #[test]
    fn test_part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day10/test/input3.txt");
        let mut tiles: Vec<Vec<Tile>> = read_tiles(&d);
        let mut start_tile: Tile = find_start_tile(&tiles);
        let connected = start_tile.get_connected_pipes(&tiles);
        tiles[start_tile.y as usize][start_tile.x as usize].distance = 0;
        tiles[start_tile.y as usize][start_tile.x as usize].passed = true;
        let path_length: i32 = get_longest_path(&mut tiles, connected.0.x, connected.0.y, connected.1.x, connected.1.y);
        let enclosed_tiles: i32 = find_enclosed_tiles(&mut tiles);
        assert_eq!(enclosed_tiles, 4);
    }

    #[test]
    fn test_part2_larger_example() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day10/test/input5.txt");
        let mut tiles: Vec<Vec<Tile>> = read_tiles(&d);
        let mut start_tile: Tile = find_start_tile(&tiles);
        let connected = start_tile.get_connected_pipes(&tiles);
        tiles[start_tile.y as usize][start_tile.x as usize].distance = 0;
        tiles[start_tile.y as usize][start_tile.x as usize].passed = true;
        let path_length: i32 = get_longest_path(&mut tiles, connected.0.x, connected.0.y, connected.1.x, connected.1.y);
        let enclosed_tiles: i32 = find_enclosed_tiles(&mut tiles);
        print_path(&mut tiles);
        assert_eq!(enclosed_tiles, 10);
    }

    #[test]
    fn test_part2_larger_example2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day10/test/input4.txt");
        let mut tiles: Vec<Vec<Tile>> = read_tiles(&d);
        let mut start_tile: Tile = find_start_tile(&tiles);
        let connected = start_tile.get_connected_pipes(&tiles);
        tiles[start_tile.y as usize][start_tile.x as usize].distance = 0;
        tiles[start_tile.y as usize][start_tile.x as usize].passed = true;
        let path_length: i32 = get_longest_path(&mut tiles, connected.0.x, connected.0.y, connected.1.x, connected.1.y);
        let enclosed_tiles: i32 = find_enclosed_tiles(&mut tiles);
        print_path(&mut tiles);
        assert_eq!(enclosed_tiles, 8);
    }

    fn print_path(tiles: &mut Vec<Vec<Tile>>) {
        tiles.iter().for_each(|line| {
            line.iter().for_each(|tile| {
                if tile.enclosed {
                    print!("{}", "I");
                } else if tile.passed {
                    if tile.char == '7' {
                        print!("{}", '┐');
                    } else if tile.char == 'L' {
                        print!("{}", '└');
                    } else if tile.char == 'J' {
                        print!("{}", '┘');
                    } else if tile.char == 'F' {
                        print!("{}", '┌');
                    } else if tile.char == '-' {
                        print!("{}", '-');
                    } else if tile.char == '|' {
                        print!("{}", '│');
                    } else if tile.char == 'S' {
                        print!("{}", 'S');
                    }
                } else {
                    print!("{}", tile.char)
                }
            });
            println!();
        });
    }

    #[test]
    fn part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day10/input.txt");
        let mut tiles: Vec<Vec<Tile>> = read_tiles(&d);
        let mut start_tile: Tile = find_start_tile(&tiles);
        let connected = start_tile.get_connected_pipes(&tiles);
        tiles[start_tile.y as usize][start_tile.x as usize].distance = 0;
        tiles[start_tile.y as usize][start_tile.x as usize].passed = true;
        let path_length: i32 = get_longest_path(&mut tiles, connected.0.x, connected.0.y, connected.1.x, connected.1.y);
        let enclosed_tiles: i32 = find_enclosed_tiles(&mut tiles);
        print_path(&mut tiles);
        assert_eq!(enclosed_tiles, 291);
    }

}