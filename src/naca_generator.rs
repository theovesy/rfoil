// Generation of naca profiles

use crate::airfoil::Airfoil;

/// generate a naca 4-digit airfoil
pub fn generate_naca_4(digit: u16) -> Airfoil {

    let name = format!("NACA {}", digit);
    Airfoil{name}
}

#[cfg(test)]
mod tests {
    #[test]
    fn naca4_gen() {
        let actual = crate::naca_generator::generate_naca_4(1234);
        assert_eq!(actual.name, "NACA 1234");
    }
}