type Map = Vec<Vec<bool>>;
type Route = Vec<Point>;

fn get_point_value(map: &Map, point: &Point) -> bool {
    return map[point.y as usize][point.x as usize];
}

pub enum Direction {
    Up,
    Left,
    Down,
    Right
}

const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Left, Direction::Down, Direction::Right];

#[derive(PartialEq, Eq, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn add_dir(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Point { x: self.x, y: self.y - 1},
            Direction::Down => Point { x: self.x, y: self.y + 1},
            Direction::Left => Point { x: self.x - 1, y: self.y},
            Direction::Right => Point { x: self.x + 1, y: self.y}
        }
    }

    pub fn has_negative_coordinates(&self) -> bool {
        if self.x < 0 || self.y < 0 { true } else { false }
    }
}

pub fn find_path(map: &mut Map, start: &Point, end: &Point) -> Option<Route> { 
    if start == end {
        return Some(vec![start.clone()]);
    }

    let mut best_route: Option<Route> = None;

    // set the current point as a wall temporarly
    map[start.y as usize][start.x as usize] = true;

    for direction in DIRECTIONS {
        let next_point = start.add_dir(direction);
        if next_point.has_negative_coordinates() || 
            next_point.x as usize >= map[0].len() ||
            next_point.y as usize >= map.len()
        {
            continue;
        }

        if get_point_value(&map, &next_point) == true {
            continue;
        }

        let found_route = find_path(map, &next_point, end);
        match found_route {
            None => continue,
            Some(mut route) => {
                route.insert(0, start.clone());
                if let Some(best_route_vec) = &best_route {
                    if best_route_vec.len() > route.len() { 
                        best_route = Some(route.clone());
                    }
                } else {
                    best_route = Some(route.clone());
                }
            }
        }
    }

    // unset the wall
    map[start.y as usize][start.x as usize] = false;

    best_route.map(|route| {
        let mut owned_route: Route = Vec::new();
        for point in route {
            owned_route.push(point.to_owned()); 
        }

        owned_route
    })
}

#[cfg(test)]
mod test_routing {
    use super::{find_path, Point};

    #[test]
    fn test_routes_1() {
        let mut exmaple_map = vec![
            vec![false, false, true, true, true, true],
            vec![false, false, false, false, true, true],
            vec![false, false, false, false, true, true],
            vec![true, true, false, false, false, true],
            vec![true, true, false, false, false, true],
        ];

        let start = Point::new(0, 0);
        let end = Point::new(3, 4);
        let best_route_option = find_path(&mut exmaple_map, &start, &end);

        assert!(best_route_option.is_some());

        let best_route = best_route_option.unwrap();

        for point in &best_route {
           println!("x: {}, y: {}", point.x, point.y);
        }

        assert_eq!(best_route.len(), 8);
    }
}
