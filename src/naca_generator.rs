// Generation of naca profiles
use crate::airfoil::Airfoil;

use::plotters::prelude::*;

/// generate a naca 4-digit airfoil
pub fn generate_naca_4(digit: u16, n: u32) -> Airfoil {

    // digit must have 4 digits 
    // (this may change later to generate any naca airfoil in one function) 
    if digit.to_string().len() != 4 {
        panic!("NACA 4-digit numbers must be 4-digits")
    }

    let m = 0.0;
    let p = 0.0;
    let t = 0.12;


    let x: Vec<f64> = chord_coord(n);
    let yc: Vec<f64> = x.iter()
                            .map(|&x| naca4_chord(&x, &p, &m))
                            .collect();

    let yt: Vec<f64> = x.iter()
                            .map(|&x| naca4_thickness_distribution(&x, &t))
                            .collect();
                        
    let theta = theta(&x, &yc);
    dbg!(&yc);

    let xu: Vec<f64> = x.iter()
                        .zip(yt.iter())
                        .zip(theta.iter())
                        .map(|((&x, &yt), &theta)| x - yt * theta.sin() )
                        .collect();

    let yu: Vec<f64> = yc.iter()
                        .zip(&yt)
                        .zip(theta.iter())
                        .map(|((&yc, &yt), &theta)| yc + yt * theta.cos())
                        .collect();

    let xl: Vec<f64> = x.iter()
                        .zip(&yt)
                        .zip(theta.iter())
                        .map(|((&x, &yt), &theta)| x + yt * theta.sin())
                        .collect();

    let yl: Vec<f64> = yc.iter()
                        .zip(&yt)
                        .zip(theta.iter())
                        .map(|((&yc, &yt), &theta)| yc - yt * theta.cos())
                        .collect();

    let root = BitMapBackend::new("0.png", (1000, (0.4/1.4*1000.0) as u32))
        .into_drawing_area();
    root.fill(&WHITE).unwrap(); 
    
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(-0.2..1.2, -0.2..0.2)
        .unwrap();

    chart.draw_series(LineSeries::new(
        xu.iter().zip(yu.iter()).map(|(x, y)| (*x, *y)),
        &RED
    )).unwrap();

    chart.draw_series(LineSeries::new(
        xl.iter().zip(yl.iter()).map(|(x, y)| (*x, *y)),
        &RED
    )).unwrap();

    let name = format!("NACA {}", digit);
    Airfoil{name}
}

fn chord_coord(n: u32) -> Vec<f64> {
    let mut x: Vec<f64> = Vec::new();
    for i in 0..n {
        x.push(i as f64/(n as f64));
    }
    x
}

fn theta(x: &Vec<f64>, yc: &Vec<f64>) -> Vec<f64> {
    let mut theta: Vec<f64> = Vec::new();

    for i in 0..(x.len()-1) {
        let dyc: f64 = (yc[i+1]-yc[i])/(x[i+1]-x[i]);
        theta.push(dyc.atan());
    }
    theta.push(0.0);

    theta
}

fn naca4_chord(x: &f64, 
    p: &f64, 
    m: &f64) -> f64 {
       if x < p {
        m/(p*p) * (2.0*p*x - x*x)
       } else {
        m/((1.0-p)*(1.0-p))*((1.0-2.0*p) + 2.0*p*x - x*x)
       } 
}

fn naca4_thickness_distribution(x: &f64, t: &f64) -> f64 {
    let a0 = 0.2969;
    let a1 = 0.1260;
    let a2 = 0.3516;
    let a3 = 0.2843;
    let a4 = 0.1015;

    t/0.2*(a0*x.sqrt() 
        - a1*x 
        - a2*x*x 
        + a3*x*x*x 
        - a4*x*x*x*x)
}


#[cfg(test)]
mod tests {

    use crate::airfoil::Airfoil;

    #[test]
    fn test_naca4_gen() {
        use std::panic;
        use crate::naca_generator::generate_naca_4;

        let actual: Airfoil = generate_naca_4(1234, 100);
        assert_eq!(actual.name, "NACA 1234");
        assert!(panic::catch_unwind(|| generate_naca_4(12345, 100)).is_err());
        assert!(panic::catch_unwind(|| generate_naca_4(123, 100)).is_err());
    }
}