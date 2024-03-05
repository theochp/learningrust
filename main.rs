use beryllium::{
    events::{Event, SDLK_DOWN, SDLK_RIGHT},
    init::InitFlags,
    video::{CreateWinArgs, RendererFlags},
    Sdl,
};

fn main() {
    let sdl = Sdl::init(InitFlags::VIDEO);
    let win = sdl
        .create_renderer_window(
            CreateWinArgs {
                title: "My first window",
                ..Default::default()
            },
            RendererFlags::ACCELERATED,
        )
        .expect("Cannot create renderer");

    let mut pos_x = 10;
    let mut pos_y = 10;
    'main: loop {
        while let Some((event, timestamp)) = sdl.poll_events() {
            match event {
                Event::Quit => {
                    println!("{timestamp}");
                    break 'main;
                }
                Event::Key { keycode, .. } => match keycode {
                    SDLK_RIGHT => pos_x += 1,
                    SDLK_DOWN => pos_y += 1,
                    _ => (),
                },
                _ => (),
            }
        }

        win.set_draw_color(u8::MAX, u8::MAX, u8::MAX, u8::MAX)
            .unwrap();
        win.clear().unwrap();
        win.set_draw_color(255, 0, 0, 1).unwrap();
        win.draw_rects(&[[pos_x, pos_y, 5, 5]]).unwrap();

        win.present();
    }
}
