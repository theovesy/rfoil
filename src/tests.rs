// Unit testing

#[cfg(test)]
mod tests {
    #[test]
    fn naca4_gen() {
        let actual = crate::naca_generator::generate_naca_4(1234);
        assert_eq!(actual.name, "NACA 1234");
    }
}
