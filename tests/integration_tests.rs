use vulkano_graphics::create_window;

#[test]
fn cloud_create_window() {
    assert!(create_window().is_err());
}

#[test]
fn client_create_window() {
    create_window().unwrap();
}