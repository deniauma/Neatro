use win32::Window;

fn main() {
    let win = Window::new(800, 600);
    while !win.message_loop() {}
}