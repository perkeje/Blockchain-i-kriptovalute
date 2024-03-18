use sha256::digest;

// Poziv funkcije je namjerno odvojen u drugi modul zbog preglednosti zadataka
pub fn sha256(input: &str) -> String {
    digest(input)
}

#[cfg(test)]
mod sha256_tests {

    #[test]
    fn test_sha256() {
        assert_eq!(
            super::sha256("FERIT"),
            "0ec299f96b70e36bc823d0a546b725fccc97c00e00dba67f7322abc9ab6ce0eb"
        );
    }
}
