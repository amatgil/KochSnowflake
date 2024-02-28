use std::f64::consts::TAU;

use ppmitzador::*;

const WIDTH: usize     = 2000;
const HEIGHT: usize    = WIDTH;

fn lerp_c(a: Coord, b: Coord, t: f64) -> Coord {
    let x = ((a.x as f64)*(1.0 - t) + (b.x as f64)*t).round() as usize;
    let y = ((a.y as f64)*(1.0 - t) + (b.y as f64)*t).round() as usize;

    Coord { x, y }
}

fn save_flake(n: usize, anti: bool) {
    println!("[INFO]: Initializing...");
    let mut data = ImagePBM::new(WIDTH, HEIGHT, false);
    let mut points = vec![
        Coord { x: 1*WIDTH/4, y: 1*HEIGHT / 4 },
        Coord { x: 2*WIDTH/4, y: 3*HEIGHT / 4 },
        Coord { x: 3*WIDTH/4, y: 1*HEIGHT / 4 },
        Coord { x: 1*WIDTH/4, y: 1*HEIGHT / 4 }, // repetida pel cicle
    ];

    println!("[INFO]: Starting computations...");
    for _ in 0..n {
        let mut new_points = vec![];
        
        for pair in points.windows(2) {
            //        d
            //       / \            <- Koch pattern
            // a -- c   e -- b
            let a = pair[0];
            let b = pair[1];

            let c = lerp_c(a, b, 1.0/3.0);
            let e = lerp_c(a, b, 2.0/3.0);

            // rotate e around c by 60º
            let d = { 
                let x1 = c.x as f64;
                let x2 = e.x as f64;
                let y1 = c.y as f64;
                let y2 = e.y as f64;

                let dx = x2 - x1;
                let dy = y2 - y1;

                let theta = TAU/6.0 * if anti { 1.0 } else { -1.0 }; // 60ª

                let rx = x1 +    theta.cos()*dx + theta.sin()*dy;
                let ry = y1 + (-theta).sin()*dx + theta.cos()*dy;
                
                Coord { x: rx as usize, y: ry as usize}
            };
            new_points.extend([a, c, d, e, b]);
        }
        points = new_points
    }
    println!("[INFO]: Drawing lines...");

    for pair in points.windows(2) { data.draw_line_with_thickness(pair[0], pair[1], true, 2); }


    println!("[INFO]: Saving to file...");
    let filename = format!("koch-n{:0>2}-anti{anti}.ppm", n);
    data.save_to_file(&filename).unwrap();

    println!("When n = {n}, there were {} points", points.len());
    println!("[INFO]: Saved '{}'", filename);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let n: usize = args.get(1).expect("You forgor n")
        .parse().expect("n must be positive integer");

    let anti: bool = {
        let s = args.get(2).expect("You forgor to ask for anti or not (0 for normal, 1 for anti)");
        if &*s == "0" { false }
        else if &*s == "1" { true }
        else { panic!("anti must be either 0 (normal) or 1 (anti) ")}
    };

    save_flake(n, anti);
}
