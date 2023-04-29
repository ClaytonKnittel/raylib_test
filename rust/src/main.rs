extern crate raylib;
use raylib::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct Opt {
  #[structopt(short = "w", long = "width", default_value = "800")]
  pub width: i32,
  #[structopt(short = "h", long = "height", default_value = "450")]
  pub height: i32,
  #[structopt(long = "fps", default_value = "60")]
  pub fps: u32,
}

impl Opt {
  pub fn new() -> Self {
    Opt::from_args()
  }
  pub fn open_window(&self, name: &str) -> (raylib::RaylibHandle, raylib::RaylibThread) {
    let (mut rl, thread) = raylib::init()
      .size(self.width, self.height)
      .title(name)
      .resizable()
      .build();
    // let logo = raylib::prelude::Image::load_image("static/logo.png").unwrap();
    // rl.set_window_icon(&logo);
    rl.set_target_fps(self.fps);
    (rl, thread)
  }
}

fn main() {
  let opt = Opt::from_args();
  let (mut rl, thread) = opt.open_window("Specs Example");

  let mut last_key = None;

  rl.set_exit_key(None);

  while !rl.window_should_close() {
    let pressed_key = rl.get_key_pressed();
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::WHITE);
    if let Some(pressed_key) = pressed_key {
      // Certain keyboards may have keys raylib does not expect. Uncomment this line if so.
      // let pressed_key: u32 = unsafe { std::mem::transmute(pressed_key) };
      println!("{:?}", pressed_key);
      last_key = Some(pressed_key);
    }

    if let Some(last_key) = last_key {
      d.draw_text(&format!("{:?}", last_key), 20, 12, 10, Color::BLACK);
    }
  }
}
