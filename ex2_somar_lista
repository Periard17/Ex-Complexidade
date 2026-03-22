pub fn somar_lista(lista: &[i32]) -> i32 {
    let mut total = 0;
    for &elemento in lista {
        total += elemento;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soma_normal() {
        assert_eq!(somar_lista(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn lista_vazia() {
        assert_eq!(somar_lista(&[]), 0);
    }

    #[test]
    fn versao_iter_equivalente() {
        let lista = &[10, 20, 30];
        assert_eq!(somar_lista(lista), somar_lista_iter(lista));
    }
}
