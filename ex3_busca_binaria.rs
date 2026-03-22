pub fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    if lista.is_empty() {
        return None;
    }

    let mut esquerda: isize = 0;
    let mut direita: isize = lista.len() as isize - 1;

    while esquerda <= direita {
        let meio = (esquerda + direita) / 2;
        let idx = meio as usize;

        if lista[idx] == alvo {
            return Some(idx);
        } else if lista[idx] < alvo {
            esquerda = meio + 1; // descarta metade esquerda
        } else {
            direita = meio - 1; // descarta metade direita
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encontra_elemento_presente() {
        let lista = [1, 3, 5, 7, 9, 11];
        assert_eq!(busca_binaria(&lista, 7), Some(3));
    }

    #[test]
    fn elemento_ausente() {
        let lista = [1, 3, 5, 7, 9];
        assert_eq!(busca_binaria(&lista, 4), None);
    }

    #[test]
    fn lista_vazia() {
        assert_eq!(busca_binaria(&[], 1), None);
    }

    #[test]
    fn primeiro_e_ultimo() {
        let lista = [2, 4, 6, 8, 10];
        assert_eq!(busca_binaria(&lista, 2), Some(0));
        assert_eq!(busca_binaria(&lista, 10), Some(4));
    }
}
