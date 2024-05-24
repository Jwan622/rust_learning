use hello_world_project::hello;

#[test]
fn hello_world() {
    assert_eq!("Hello, World!", hello());
}