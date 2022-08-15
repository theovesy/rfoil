// Definition of the airfoil class
use::plotters::prelude::*;

/// Represents an Airfoil
#[derive(Debug)]
pub struct Airfoil {

    pub name: String,
    /// `(x, yc)` for coordinate of the mean chord line
    pub x: Vec<f64>,
    pub yc: Vec<f64>,
    /// `yt` is the thickness distribution along the airfoil 
    pub yt: Vec<f64>,
    /// `(xu, yu)` for coordinates on the upper airfoil
    pub xu: Vec<f64>,
    pub yu: Vec<f64>,
    /// `(xl, yl)` for coordinates on the lower airfoil
    pub xl: Vec<f64>,
    pub yl: Vec<f64>,
}

impl Airfoil {
    /// Plots the airfoil's shape in a svg image
    pub fn plot_svg(&self) {
        let img_name = format!("{}.svg", self.name);
        let root = SVGBackend::new(&img_name, (1000, (0.4/1.4*1000.0) as u32))
            .into_drawing_area();
        
        let mut chart = ChartBuilder::on(&root)
            .caption(self.name.clone(), ("Arial", 40))
            .build_cartesian_2d(-0.2..1.2, -0.2..0.2)
            .unwrap();

        chart.draw_series(LineSeries::new(
            self.xu.iter().zip(self.yu.iter()).map(|(x, y)| (*x, *y)),
            &RED
        )).unwrap();

        chart.draw_series(LineSeries::new(
            self.xl.iter().zip(self.yl.iter()).map(|(x, y)| (*x, *y)),
            &RED
        )).unwrap();

        chart.draw_series(LineSeries::new(
            self.x.iter().zip(self.yc.iter()).map(|(x, y)| (*x, *y)),
            &BLUE
        )).unwrap();

    }

}
