use std::{io::{Lines, BufReader}, fs::File};

#[derive(Debug)]
struct Gear {
    position: Point,
}

impl Gear {
    fn new(position: Point) -> Self {
        Self { position }
    }

    fn find_ships_nearby_by_one_point<'a>(&'a self, ships: &'a Vec<Ship>) -> Vec<&Ship> {
        let mut ships_nearby = Vec::new();
        let x = self.position.x;
        let y = self.position.y;

        for ship in ships {
            for point in &ship.position {
                let relative_position = (point.x - x, point.y - y);
                match relative_position {
                    (1, 1) | (1, -1) | (-1, 1) | (-1, -1) | (1, 0) | (-1, 0) | (0, 1) | (0, -1) => {
                        ships_nearby.push(ship);
                        break;
                    }
                    _ => {}
                }
            }
        }

        ships_nearby
    }
}

#[derive(Debug, Clone)]
struct Ship {
    position: Vec<Point>,
}

impl Ship {
    fn new() -> Self {
        Self {
            position: Vec::new(),
        }
    }

    fn add(&mut self, point: Point) {
        self.position.push(point);
    }

    fn get_value(&self) -> isize {
        let mut value_str = String::new();
        for point in &self.position {
            value_str.push(point.c);
        }

        value_str.parse::<isize>().unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    c: char,
}

impl Point {
    fn new(x: isize, y: isize, c: char) -> Self {
        Self { x, y, c }
    }
}

pub fn part2(lines: Lines<BufReader<File>>) {

    let mut in_ship = false;
    let mut ships = Vec::new();
    let mut gears = Vec::new();
    let mut ship = Ship::new();

    for (i, line) in lines.enumerate() {
        let line = line.unwrap();
        for (j, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                in_ship = true;
                let point = Point::new(i as isize, j as isize, c);
                ship.add(point);
            } else {
                if c == '*' {
                    gears.push(Gear::new(Point::new(i as isize, j as isize, c)));
                }
                if in_ship {
                    in_ship = false;
                    ships.push(ship.clone());
                    ship = Ship::new();
                }
            }
        }

        if in_ship {
            in_ship = false;
            ships.push(ship.clone());
            ship = Ship::new();
        }
    }

    let mut sum = 0;
    for gear in gears {
        let shippo = gear.find_ships_nearby_by_one_point(&ships);
        if shippo.len() == 2 {
            let summo = shippo[0].get_value() * shippo[1].get_value();
            sum += summo;
        }
    }

    println!("{}", sum);

}
