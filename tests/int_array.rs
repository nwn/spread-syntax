use spread_syntax::array;

#[test]
fn int_array() {
    const A: [u32; 4] = [1, 2, 3, 4];
    const B: [u32; 2] = [6, 7];
    const C: [u32; A.len() + 1 + B.len()] = array![..A, 5, ..B];
    assert_eq!(C, [1, 2, 3, 4, 5, 6, 7]);
}
