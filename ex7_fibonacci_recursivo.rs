pub fn fibonacci_recursivo(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casos_base() {
        assert_eq!(fibonacci_recursivo(0), 0);
        assert_eq!(fibonacci_recursivo(1), 1);
    }

    #[test]
    fn sequencia_conhecida() {
        // fib: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34
        let esperado = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        for (i, &val) in esperado.iter().enumerate() {
            assert_eq!(fibonacci_recursivo(i as u64), val);
        }
    }

    #[test]
    fn memo_coincide_com_recursivo() {
        for n in 0..20 {
            assert_eq!(fibonacci_recursivo(n), fibonacci_memo(n));
        }
    }
}
