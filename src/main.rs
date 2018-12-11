use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone, Hash, Eq, Copy)]
struct Point {
    x : i64,
    y : i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point{x, y}
    }

    fn dist(&self, that: &Point) -> i64 {
        i64::abs(self.x - that.x) + i64::abs(self.y - that.y)
    }

    fn bound_by(&self, that: &Point, mut bound: Bound) -> Bound {
        if self.x < that.x {
            bound.rig = true;
        }

        if self.x > that.x {
            bound.lef = true;
        }

        if self.y < that.y {
            bound.bot = true;
        }

        if self.y > that.y {
            bound.top = true;
        }
        bound
    }

}

fn read_points(file: BufReader<&File>) -> Vec<Point> {
    let mut res = Vec::new();

    for line in file.lines() {
        let l = line.unwrap();
        let split = l.split(", ").collect::<Vec<&str>>();
        let x = split[0].parse::<i64>().expect("Could not parse x");
        let y = split[1].parse::<i64>().expect("Could not parse y");
        res.push(Point::new(x, y));
    }
    res
}

#[derive(Debug, PartialEq)]
struct Bound {
    top : bool,
    bot : bool,
    lef : bool,
    rig : bool,
}

impl Bound {

    fn new() -> Bound {
        Bound{ top : false, bot : false, lef : false, rig : false}
    }

    fn bounded(&self) -> bool {
        self.top && self.bot && self.lef && self.rig
    }

}

fn find_enclosed_points(points: &Vec<Point>) -> (Vec<Point>, Vec<Point>) {
    let mut bounded = Vec::new();
    let mut unbounded = Vec::new();

    for p1 in points {
        let mut b = Bound::new();
        for p2 in points {
            if p1 != p2 {
                b = p1.bound_by(p2, b);
            }
        }

        if b.bounded() {
            bounded.push(p1.clone());
        } else {
            unbounded.push(p1.clone());
        }
    }
    (bounded, unbounded)
}

fn find_closest_point(cur: &Point, all: &Vec<Point>) -> Point {
    let mut min = std::i64::MAX;
    let mut res = Point::new(-1, -1);

    //println!("Testing {:?}", cur);

    if all.contains(cur) {
        return cur.clone();
    }

    for p in all {
        let dist = cur.dist(p);
        if dist < min {
            min = dist;
            res = p.clone();
        } else if dist == min {
            res = Point::new(-1, -1)
        }
    }
    res
}

fn expand(point: &Point, unbounded: &Vec<Point>, all: &Vec<Point>,
          map: &mut HashMap<Point, Point>) {

    let mut openlist: VecDeque<Point> = VecDeque::new();
    let exit = false;
    openlist.push_front(point.clone());

    while !exit || openlist.is_empty() {
        let current = openlist.pop_front();
        match current {
            Some(cur) => {
                if !map.contains_key(&cur) {
                    let closest = find_closest_point(&cur, &all);
                    if unbounded.contains(&closest) || closest == Point::new(-1, -1){
                        continue;
                    }
                    map.insert(cur.clone(), closest.clone());
                }
                for x in -1..2 {
                    for y in -1..2 {
                        let np = Point::new(cur.x + x, cur.y + y);
                        if !map.contains_key(&np) && !openlist.contains(&np) {
                            openlist.push_back(np);
                        }
                    }
                }
            },
            None => break
        }
    }
}

fn part01(points: &Vec<Point>) {
    let (bounded, unbounded) = find_enclosed_points(&points);
    let mut map: HashMap<Point, Point> = HashMap::new();
    let mut count_map = HashMap::new();

    for p in bounded.iter() {
        expand(&p, &unbounded, &points, &mut map);
    }

    for (_, val) in map.iter() {
        if bounded.contains(&val) {
            let count = count_map.entry(val.clone()).or_insert(0);
            *count += 1;
        }
    }

    let mut max = 0;
    for (_, v) in count_map.iter() {
        max = std::cmp::max(max, *v);
    }
    println!("Part 01 result = {}", max);
}

fn main() {
    let f = File::open("input.txt").expect("file not found");
    let file = BufReader::new(&f);
    let points = read_points(file);

    part01(&points);
}
