use std::fs;
use std::f64::consts::PI;

pub fn run_sim() {
    let path = String::from("../airfoils/naca0012.dat");
    let mut naca0012 = Airfoil::from_file(&path);
    naca0012.define_panels(40);
}

#[derive(Debug)]
struct Airfoil {
    name: String,
    points: Vec<Point>,
    panels: Vec<Panel>,
    n_panels: u32,
}

impl Airfoil {
    pub fn from_file(path: &String) -> Self {
        // from .dat file in the "Selig format"
        let file_content = fs::read_to_string(path).expect("Unable to open .dat file");
        let mut points: Vec<Point> = Vec::new(); 
        let mut header = true;
        let mut name = String::from("Profile");
        for c in file_content.lines() {
            let mut num = c.split_whitespace();
            if header {
                name = num.next().unwrap().to_string(); 
                name += num.next().unwrap();
                header = false;
            } else {
                let x: f64 = num.next().unwrap().parse().unwrap();
                let y: f64 = num.next().unwrap().parse().unwrap();
                points.push(Point::new(x, y));
            }
        }
        let panels: Vec<Panel> = Vec::new();
        Airfoil {name, points, panels, n_panels: 0}
    }

    pub fn chord(&self) -> f64 {
        let mut min = self.points[0].x;
        let mut max = self.points[0].x;
        for point in self.points.iter() {
            if point.x < min { min = point.x; }
            if point.x > max { max = point.x; }
        }
        ((max-min)*(max-min)).sqrt()
    }

    pub fn chord_center_x(&self) -> f64 {
        let mut min = self.points[0].x;
        let mut max = self.points[0].x;
        for point in self.points.iter() {
            if point.x < min { min = point.x; }
            if point.x > max { max = point.x; }
        }
        (max + min) / 2.0
    }

    pub fn define_panels(&mut self, n_panels: usize) {
        // the circle
        let r = self.chord() / 2.0;
        let x_center = self.chord_center_x();
        let mut p_pan: Vec<Point> = Vec::new();
        for i in 0..=n_panels {
            let w = (i as f64)/(n_panels as f64) * 2.0 * PI;
            p_pan.push(Point::new(x_center + r * w.cos(), 0.0));
        }

        let mut points = self.points.clone();
        points.push(points[0]);

        let mut j = 0;
        for i in 0..n_panels {
            while j < points.len()-2 {
                // which point is directly before or after the panel point
                if (points[j].x <= p_pan[i].x && p_pan[i].x <= points[j+1].x) || 
                    (points[j].x >= p_pan[i].x && p_pan[i].x >= points[j+1].x)  {
                        break;
                } else { j+=1; }
            }           
            let a = (points[j+1].y - points[j].y) / (points[j+1].x - points[j].x);
            //println!("{}, {}", points[j+1].x, points[j].x);
            //println!("{}, {}", points[j+1].y, points[j].y);
            let b = points[j+1].y - a * points[j+1].x;
            p_pan[i].y = a * p_pan[i].x + b;
        }
        p_pan[n_panels].y = p_pan[n_panels].y;
        
        for i in 0..n_panels {
            self.panels.push(
                Panel::new(p_pan[i], p_pan[i+1])
            );
        }
        self.n_panels = n_panels as u32;
    }
}

#[test]
fn chord_calculation() {
    let airfoil = Airfoil{
        name: String::from("Test"),
        points: vec![
            Point::new(3.0,4.0),
            Point::new(5.0,0.0),
            Point::new(1.0,-2.0),
            Point::new(8.0,-6.0),
            Point::new(4.0,2.0),
            ],
        panels: Vec::new(),
        n_panels: 0,
    };
    assert_eq!(7.0, airfoil.chord());
}

#[test]
fn chord_center_test() {
    let airfoil = Airfoil{
        name: String::from("Test"),
        points: vec![
            Point::new(3.0,4.0),
            Point::new(5.0,0.0),
            Point::new(1.0,-2.0),
            Point::new(8.0,-6.0),
            Point::new(4.0,2.0),
            ],
        panels: Vec::new(),
        n_panels: 0,
    };
    assert_eq!(4.5, airfoil.chord_center_x());
}

#[test]
fn dat_parsing() {
    let path = String::from("airfoils/testairfoil.dat");
    let testfoil = Airfoil::from_file(&path);
    let pts = vec![
        Point::new(0.123456, 1.123456),
        Point::new(1.123456, 0.123456),
        Point::new(-1.123456, -0.123456),
        Point::new(1.234567, -1.234567),
        Point::new(-0.123456, 1.234567),
    ];
    assert_eq!(testfoil.name, String::from("TESTFOIL"));
    for (p, pt) in testfoil.points.iter().zip(pts.iter()) {
        assert_eq!(p, pt);
    }
}

#[derive(Debug)]
struct Panel {
    // geometry
    point_a: Point,
    point_b: Point,
    center: Point,
    beta: f64,
    length: f64,
    upper: bool,
    // calculated results
    sigma: f64,
    vt: f64, 
    cp: f64,
}

impl Panel {
    fn new(a: Point, b: Point) -> Self {
        let center = Point::new((a.x+b.x)/2.0, (a.y+b.y)/2.0);
        let length = ((b.x-a.x)*(b.x-a.x) + (b.x-a.x)*(b.x-a.x)).sqrt();
        // panel orientation
        let beta: f64;
        if b.x - a.x <= 0.0 {
            beta = ((b.y-a.y)/length).acos();
        } else {
            beta = PI + (-(b.y-a.y)/length).acos();  
        }
        // panel location
        let upper: bool;
        if beta <= PI { upper = true; } 
        else { upper = false; }

        Panel { point_a: a, point_b: b, center, beta, length, upper, sigma: 0.0, vt: 0.0, cp: 0.0}
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point {x, y}
    }
}