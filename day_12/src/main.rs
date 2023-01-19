const ASCII_OFFSET: u32 = 96;

#[derive(Debug, Copy, Clone, PartialEq)]
enum TileType {
    Land(u32),
    Start,
    End
}

impl TileType {
    fn get_height(&self) -> u32 {
        match self {
            TileType::Land(height) => *height,
            TileType::Start => ('a' as u32) - ASCII_OFFSET,
            TileType::End => ('z' as u32) - ASCII_OFFSET,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Tile {
    tile_type: TileType,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    map: Vec<Vec<Tile>>,
}

impl Map {
    fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.map[y][x]
    }
}

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("./inputs/day12.txt")?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)?;

    let map = read_input(contents);

    let mut start_pos: Option<Tile> = None;
    let mut end_pos: Option<Tile> = None;

    for row in &map.map {
        for tile in row {
            match tile.tile_type {
                TileType::Start => start_pos = Some(tile.clone()),
                TileType::End => end_pos = Some(tile.clone()),
                _ => (),
            }
        }
    }

    assert!(start_pos.is_some());
    assert!(end_pos.is_some());

    let start_pos = start_pos.unwrap();
    let end_pos = end_pos.unwrap();

    let path = Vec::new();

    let result = rec(&map, &start_pos, &end_pos, path, usize::MAX);

    println!("Result: {}", result);

    Ok(())
}

fn rec(map: &Map, player_tile: &Tile, end_tile: &Tile, path: Vec<Tile>, shortest_dist_found: usize) -> usize {
    let neighbours_pre_filter = get_reachable_neighbours(map, player_tile);
    let neighbours = neighbours_pre_filter.into_iter().filter(|tile| !path.contains(tile)).collect::<Vec<Tile>>();

    if player_tile.x == 4 && player_tile.y == 4 {
        print!("");
    }

    if neighbours.is_empty() {
        return std::usize::MAX;
    }

    if neighbours.contains(end_tile) {
        let res = path.len() + 1;
        if res == 15 {
            print!("");
        } 
        return res;
    } else {
        let mut res = usize::MAX;
        for neighbour in neighbours {
            let mut new_path = path.clone();
            new_path.push(player_tile.clone());
            let new_res = rec(map, &neighbour, end_tile, new_path, shortest_dist_found);
            if new_res < res {
                res = new_res
            }
            
        }
        return res;
    }
}




fn read_input(content: String) -> Map {

    let height = content.lines().count();
    let width = (content.chars().count() / height) - 1; // -1 because of the newline at the end of each line

    let mut result = Map {
        width,
        height,
        map: Vec::new(),
    };

    for (y, line) in content.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let tiletype: TileType = match c {
                'S' => TileType::Start,
                'E' => TileType::End,
                _ => TileType::Land((c as u32) - ASCII_OFFSET),
            };
            row.push(Tile {
                tile_type: tiletype,
                x,
                y,
            });
        }
        result.map.push(row);
    }
    
    result
}

fn get_reachable_neighbours(map: &Map, player_tile: &Tile) -> Vec<Tile> {
    let mut result = Vec::new();

    if player_tile.x == 6 && player_tile.y == 2 {
        print!("");
    }

    let x = player_tile.x;
    let y = player_tile.y;

    if x > 0 {
        let potential_tile = map.get_tile(x - 1, y).clone();
        if is_tile_reachable(player_tile, &potential_tile) {
            result.push(potential_tile);
        }
    }
    if x < map.width - 1 {
        let potential_tile = map.get_tile(x + 1, y).clone();
        if is_tile_reachable(player_tile, &potential_tile) {
            result.push(potential_tile);
        }
    }
    if y > 0 {
        let potential_tile = map.get_tile(x, y - 1).clone();
        if is_tile_reachable(player_tile, &potential_tile) {
            result.push(potential_tile);
        }
    }
    if y < map.height - 1 {
        let potential_tile = map.get_tile(x, y + 1).clone();
        if is_tile_reachable(player_tile, &potential_tile) {
            result.push(potential_tile);
        }
    }

    result
}

fn is_tile_reachable(player_tile: &Tile, tile: &Tile) -> bool {
    let player_height = player_tile.tile_type.get_height();
    let tile_height = tile.tile_type.get_height();

    (player_height + 1) >= tile_height
}