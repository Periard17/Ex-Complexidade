pub fn pares_com_soma(lista: &[i32], alvo: i32) {
    let n = lista.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if lista[i] + lista[j] == alvo {
                println!("{} + {} = {}", lista[i], lista[j], alvo);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encontra_pares() {
        let lista = [1, 2, 3, 4, 5];
        let pares = pares_com_soma_vec(&lista, 6);
        assert_eq!(pares, vec![(1, 5), (2, 4)]);
    }

    #[test]
    fn nenhum_par() {
        let lista = [1, 2, 3];
        assert!(pares_com_soma_vec(&lista, 10).is_empty());
    }
}
