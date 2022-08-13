// Generation of naca profiles

use crate::airfoil::Airfoil;

/// generate a naca 4-digit airfoil
pub fn generate_naca_4(digit: u16) -> Airfoil {

    // digit must have 4 digits 
    // (this may change later to generate any naca airfoil in one function) 
    if digit.to_string().len() != 4 {
        panic!("NACA 4-digit numbers must be 4-digits")
    }

    let name = format!("NACA {}", digit);
    Airfoil{name}
}

#[cfg(test)]
mod tests {

    use crate::airfoil::Airfoil;

    #[test]
    fn naca4_gen() {
        use std::panic;
        use crate::naca_generator::generate_naca_4;

        let actual: Airfoil = generate_naca_4(1234);
        assert_eq!(actual.name, "NACA 1234");
        assert!(panic::catch_unwind(|| generate_naca_4(12345)).is_err());
        assert!(panic::catch_unwind(|| generate_naca_4(123)).is_err());
    }
}