pub fn ordenacao_bolha(lista: &mut [i32]) {
    let n = lista.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if lista[j] > lista[j + 1] {
                lista.swap(j, j + 1); // bolha: move o maior para a direita
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordena_vetor_desordenado() {
        let mut lista = vec![5, 3, 8, 1, 2];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn lista_ja_ordenada() {
        let mut lista = vec![1, 2, 3, 4, 5];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn lista_com_um_elemento() {
        let mut lista = vec![42];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![42]);
    }

    #[test]
    fn lista_vazia() {
        let mut lista: Vec<i32> = vec![];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![]);
    }
}
