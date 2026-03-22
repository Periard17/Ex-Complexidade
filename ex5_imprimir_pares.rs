pub fn imprimir_pares_e_pares(lista: &[i32]) {
    // Bloco 1: O(n) — percorre cada elemento uma vez
    for &x in lista {
        println!("{}", x);
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contagem_correta() {
        let lista = [1, 2, 3];
        let (elementos, pares) = coletar_pares_e_pares(&lista);
        assert_eq!(elementos.len(), 3);   // n
        assert_eq!(pares.len(), 9);       // n²
    }
}
