use vulkano_graphics::add;

#[test]
fn it_adds_two() {
    let result = add(2u64, 2u64);
    assert_eq!(result, 4);
}