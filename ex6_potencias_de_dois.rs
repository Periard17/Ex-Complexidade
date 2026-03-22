pub fn potencias_de_dois(n: u64) {
    let mut i: u64 = 1;
    while i < n {
        println!("{}", i);
        i *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn potencias_ate_32() {
        assert_eq!(potencias_de_dois_vec(32), vec![1, 2, 4, 8, 16]);
    }

    #[test]
    fn n_igual_a_1_retorna_vazio() {
        assert!(potencias_de_dois_vec(1).is_empty());
    }

    #[test]
    fn contagem_logaritmica() {
        // Para n = 2^10 = 1024, esperamos exatamente 10 valores
        assert_eq!(potencias_de_dois_vec(1024).len(), 10);
    }
}
