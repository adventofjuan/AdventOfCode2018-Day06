use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::cmp::max;

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

fn find_max(points: &Vec<Point>) -> (i64, i64) {
    let mut max_x = 0;
    let mut max_y = 0;

    for p in points {
        max_x = max(max_x, p.x);
        max_y = max(max_y, p.y);
    }

    (max_x, max_y)
}

fn find_closest_point(pos: Point, points: &Vec<Point>) -> Point {
    let mut min_dist = std::i64::MAX;
    let mut min_point = Point::new(-1, -1);
    for p in points {
        let dist = pos.dist(&p);
        if dist < min_dist {
            min_dist = dist;
            min_point = *p;
        } else if dist == min_dist {
            min_point = Point::new(-1, -1);
        }
    }
    min_point
}

fn part01(points: &Vec<Point>) {
    let mut grid = HashMap::new();
    let mut regions = HashMap::new();
    let mut maximum = 0;

    let (max_x, max_y) = find_max(points);

    for i in 0..=max_x {
        for j in 0..=max_y {
            let cur = Point::new(i, j);
            let closest = find_closest_point(cur , points);
            grid.insert(cur, closest);

            let total = regions.entry(closest).or_insert(0);
            *total += 1;
        }
    }

    for x in 0..=max_x {
        let bad = grid[&Point::new(x, 0)];
        regions.remove_entry(&bad);
        let bad = grid[&Point::new(x, max_y)];
        regions.remove_entry(&bad);
    }

    for y in 0..=max_y {
        let bad = grid[&Point::new(0, y)];
        regions.remove_entry(&bad);
        let bad = grid[&Point::new(max_x, y)];
        regions.remove_entry(&bad);
    }

    for (_, v) in regions {
        maximum = max(maximum, v)
    }

    println!("Part 01 result = {}", maximum);
}

fn main() {
    let f = File::open("input.txt").expect("file not found");
    let file = BufReader::new(&f);
    let points = read_points(file);

    part01(&points);
}
