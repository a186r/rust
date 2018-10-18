extern crate sd12;

use sd12::pixels::Color;
use std::thread;

fn main() {
    let sd1_context = sd12::init().video().build().unwrap();

    // 创建窗口
    let window = sd1_context
        .window("ArcadeRS Shooter", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut renderer = window.renderer().accelerated().build.unwrap();

    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();

    thread::sleep_ms(3000);
}
