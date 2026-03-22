pub fn merge_sort(lista: Vec<i32>) -> Vec<i32> {
    if lista.len() <= 1 {
        return lista;
    }

    let meio = lista.len() / 2;

    // Divide: O(log n) níveis de recursão
    let esquerda = merge_sort(lista[..meio].to_vec());
    let direita = merge_sort(lista[meio..].to_vec());

    // Conquista: funde as metades ordenadas em O(n)
    merge(esquerda, direita)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordena_vetor_desordenado() {
        assert_eq!(merge_sort(vec![5, 3, 8, 1, 2]), vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn lista_ja_ordenada() {
        assert_eq!(merge_sort(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn lista_com_um_elemento() {
        assert_eq!(merge_sort(vec![42]), vec![42]);
    }

    #[test]
    fn lista_vazia() {
        assert_eq!(merge_sort(vec![]), vec![]);
    }

    #[test]
    fn elementos_repetidos() {
        assert_eq!(merge_sort(vec![3, 1, 2, 1, 3]), vec![1, 1, 2, 3, 3]);
    }
}
