// Generation of naca profiles
use crate::airfoil::Airfoil;

use std::f64::consts::PI;

fn parse_naca4(digit: &String) -> (f64, f64, f64) {

    let m: f64 = digit[0..1].parse::<f64>().unwrap()/100.0;
    let p: f64 = digit[1..2].parse::<f64>().unwrap()/10.0;
    let t: f64 = digit[2..4].parse::<f64>().unwrap()/100.0;

    (m, p, t)
}

/// generate a naca 4-digit airfoil
pub fn generate_naca_4(digit: String, n: u32) -> Airfoil {

    // digit must have 4 digits 
    // (this may change later to generate any naca airfoil in one function) 
    if digit.len() != 4 {
        panic!("NACA 4-digit numbers must be 4-digits")
    }

    let (m, p, t) = parse_naca4(&digit);


    let x: Vec<f64> = chord_coord(n);
    let yc: Vec<f64> = x.iter()
                            .map(|&x| naca4_chord(&x, &p, &m))
                            .collect();

    let yt: Vec<f64> = x.iter()
                            .map(|&x| naca4_thickness_distribution(&x, &t))
                            .collect();
                        
    let theta = theta(&x, &yc);

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

    let name = format!("NACA_{}", digit);
    Airfoil{name, x, yc, yt, xu, yu, xl, yl}
}

/// Generate `x` coordinates along the chord
/// To have points near the leading and trailing edges
/// closer to each other, we compute $x$ with the formula
/// $$x = \frac{1-\cos(\Beta)}{2}$$
/// with $\Beta$ in $[0, \pi]$
fn chord_coord(n: u32) -> Vec<f64> {
    let mut x: Vec<f64> = Vec::new();
    for i in 0..n { 
        let b = PI*(i as f64)/(n as f64);
        x.push((1.0 - b.cos())/2.0)
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

        let actual: Airfoil = generate_naca_4("2412".to_string(), 100);
        assert_eq!(actual.name, "NACA_2412");
        assert!(panic::catch_unwind(|| generate_naca_4("12345".to_string(), 100)).is_err());
        assert!(panic::catch_unwind(|| generate_naca_4("123".to_string(), 100)).is_err());
        actual.plot_svg();
    } 

    #[test]
    fn test_parse_naca4() {
        use crate::naca_generator::parse_naca4;

        assert_eq!((0.0,0.0,0.12), parse_naca4(&String::from("0012")));
        assert_eq!((0.02,0.4,0.12), parse_naca4(&String::from("2412")));
    }

}