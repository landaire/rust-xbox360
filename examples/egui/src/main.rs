//! An egui window on the console, rendered by the GPU and driven by the
//! controller: the directions and shoulders move focus, A presses. Runs for
//! sixty seconds and prints the frame time.

#![no_main]

use core::time::Duration;

use xbox360::time::Timebase;
use xbox360::time::TimebaseFrequency;
use xbox360_draw::draw::DrawList;
use xbox360_draw::draw::Rgba8;
use xbox360_egui::Painter;
use xbox360_gpu::Gpu;
use xbox360_video::Display;

const RUN_TIME: Duration = Duration::from_secs(60);
const REPORT_INTERVAL: Duration = Duration::from_secs(2);
const PIXELS_PER_POINT: f32 = 1.0;
const BACKGROUND: Rgba8 = Rgba8::new(0x18, 0x18, 0x20, 0xff);

xbox360::sys::bridged!(#[unsafe(no_mangle)] pub _start, main);

/// egui's layout recursion needs more stack than the title's main thread has.
const UI_STACK_BYTES: usize = 1 << 20;

extern "C" fn main(_context: u32) -> u32 {
    match std::thread::Builder::new().stack_size(UI_STACK_BYTES).spawn(run).map(std::thread::JoinHandle::join) {
        Ok(Ok(())) => {}
        Ok(Err(_)) => println!("egui: thread panicked"),
        Err(error) => println!("egui: thread not started: {error}"),
    }
    xbox360::title::terminate()
}

fn run() {
    let mut display = match Display::new() {
        Ok(display) => display,
        Err(error) => return println!("egui: {error}"),
    };
    let mut gpu = match Gpu::new() {
        Ok(gpu) => gpu,
        Err(error) => return println!("egui: {error}"),
    };
    let size = display.size();
    let context = egui::Context::default();
    let mut painter = Painter::new();
    let mut list = DrawList::new();
    let frequency = TimebaseFrequency::query();
    let start = Timebase::now();
    let mut window = start;
    let (mut frames, mut cpu_total) = (0u32, 0f32);
    let (mut presses, mut volume, mut enabled, mut name) = (0u32, 50.0f32, true, String::from("Xbox 360"));
    loop {
        let now = Timebase::now();
        let elapsed = now.duration_since(start, frequency);
        if elapsed > RUN_TIME {
            break;
        }
        let mut input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(size.width as f32, size.height as f32))),
            time: Some(elapsed.as_secs_f64()),
            ..egui::RawInput::default()
        };
        while let Some(keystroke) = xbox360::xam::input::next_keystroke() {
            xbox360_egui::input::keystroke_events(&keystroke, &mut input.events);
        }
        let output = context.run(input, |context| {
            egui::Window::new("openblades egui").default_pos([80.0, 60.0]).show(context, |ui| {
                ui.label("egui on the Xbox 360 GPU");
                if ui.button("Press me").clicked() {
                    presses += 1;
                }
                ui.label(format!("pressed {presses} times"));
                ui.add(egui::Slider::new(&mut volume, 0.0..=100.0).text("volume"));
                ui.checkbox(&mut enabled, "enabled");
                ui.text_edit_singleline(&mut name);
                ui.label(format!("time {:.1} s", elapsed.as_secs_f32()));
            });
        });
        let primitives = context.tessellate(output.shapes, output.pixels_per_point);
        list.clear();
        list.set_clear_color(Some(BACKGROUND));
        if let Err(error) = painter.paint(&mut gpu, &mut list, &output.textures_delta, &primitives, PIXELS_PER_POINT) {
            return println!("egui: {error}");
        }
        cpu_total += Timebase::now().duration_since(now, frequency).as_secs_f32() * 1000.0;
        if let Err(error) = gpu.render(&mut display, &list) {
            return println!("egui: {error}");
        }
        frames += 1;
        let since = Timebase::now().duration_since(window, frequency);
        if since >= REPORT_INTERVAL {
            println!("egui: {:.1} fps, egui frame + tessellate + paint {:.2} ms, {} triangles", frames as f32 / since.as_secs_f32(), cpu_total / frames as f32, list.triangles().len());
            (frames, cpu_total) = (0, 0.0);
            window = Timebase::now();
        }
    }
    println!("egui: done");
}
