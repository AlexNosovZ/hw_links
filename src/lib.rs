pub fn get_tuple_link(a: &mut (i32, i32), b: bool) -> &mut i32 {
    //     Принимает мутабельную ссылку на кортеж и bool значение.
    // Если false, возвращает мутабельную ссылку на первый элемент кортежа.
    // Если true, возвращает мутабельную ссылку на второй элемент кортежа.
    if !b {
        &mut a.0
    } else {
        &mut a.1
    }
}
pub fn get_mut_link_n_elem(slc: &mut [i32], n: usize) -> &mut i32 {
    //     Принимает мутабельную ссылку на слайс и число N.
    //     Возвращает мутабельную ссылку на N-ый элемент.
    //
    if (slc.len() >= n) & (n != 0) {
        &mut slc[n - 1]
    } else {
        panic!("n is out of slc");
    }
}

pub fn get_link_n_elem_from_end(slc: &[i32], n: usize) -> &i32 {
    // Принимает слайс и число N.
    // Возвращает ссылку на N-ый элемент слайса с конца.
    if (slc.len() >= n) & (n != 0) {
        &slc[slc.len() - n]
    } else {
        panic!("n is out of slc");
    }
}
pub fn get_splitted_slice_at_n(slc: &[i32], n: usize) -> (&[i32], &[i32]) {
    //     Принимает слайс и число N. Возвращает два слайса с элементами:
    // с нулевого по N-1;
    // с N-го по последний;
    if (slc.len() > n) & (n != 0) & (n != 1) {
        (&slc[0..n - 1], &slc[n - 1..])
    } else {
        panic!("n is out of slc or incorrest");
    }
}

pub fn get_4_slices(slc: &[i32]) -> (&[i32], &[i32], &[i32], &[i32]) {
    //     Принимает слайс и возвращает массив слайсов, содержащий четыре равные (насколько возможно) части исходного слайса.
    // Протестировать функции.
    if slc.len() >= 4 {
        (
            &slc[0..slc.len() / 4],
            &slc[slc.len() / 4..slc.len() / 4 * 2],
            &slc[slc.len() / 4 * 2..slc.len() / 4 * 3],
            &slc[slc.len() / 4 * 3..],
        )
    } else {
        panic!("not correct size of slice");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let mut tup = (1, 2);
        let x: &mut i32 = get_tuple_link(&mut tup, false);
        *x = *x + 5;
        assert_eq!(tup.0, 6);
        let x: &mut i32 = get_tuple_link(&mut tup, true);
        *x = *x + 5;
        assert_eq!(tup.1, 7);
    }
    #[test]
    fn it_works_2() {
        let mut a: [i32; 5] = [1, 2, 3, 4, 5];
        let b = get_mut_link_n_elem(&mut a, 3);
        assert_eq!(*b, 3);
    }
    #[test]
    fn it_works_3() {
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let b = get_link_n_elem_from_end(&a, 4);
        assert_eq!(*b, 2);
    }
    #[test]
    fn it_works_4() {
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let b = get_splitted_slice_at_n(&a, 3);
        assert_eq!(*b.0, [1, 2]);
        assert_eq!(*b.1, [3, 4, 5]);
    }
    #[test]
    fn it_works_5() {
        let a: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let b = get_4_slices(&a);
        assert_eq!(*b.0, [1, 2]);
        assert_eq!(*b.1, [3, 4]);
        assert_eq!(*b.2, [5, 6]);
        assert_eq!(*b.3, [7, 8, 9]);
    }
}
