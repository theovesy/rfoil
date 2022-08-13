// Generation of naca profiles

use crate::airfoil::Airfoil;

/// generate a naca 4-digit airfoil
pub fn generate_naca_4(digit: u16) -> Airfoil {

    let name = format!("NACA {}", digit);
    Airfoil{name}
}