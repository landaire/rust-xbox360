//! Draws an animated scene with the GPU renderer for ten seconds: a
//! gradient, overlapping textured layers in every blend mode, clipped quads,
//! many small quads and a render target pass, and prints the time per
//! frame. The target holds a half-transparent red rect and white modulated
//! to half alpha; it is shown at the top left, each texel 32 pixels wide,
//! on the clear color. xbox360-raster's
//! `target_pass_squares_source_alpha_and_samples_straight` gives its
//! pixels: (44, 12, 24) and (136, 136, 144).

#![no_main]

use core::num::NonZeroU32;
use core::time::Duration;

use xbox360::time::Timebase;
use xbox360::time::TimebaseFrequency;
use xbox360_draw::draw::AddressMode;
use xbox360_draw::draw::BlendMode;
use xbox360_draw::draw::ClipRect;
use xbox360_draw::draw::DrawList;
use xbox360_draw::draw::DrawState;
use xbox360_draw::draw::Filter;
use xbox360_draw::draw::Rgba8;
use xbox360_draw::draw::Sampler;
use xbox360_draw::draw::Vertex;
use xbox360_draw::image::Image;
use xbox360_draw::texture::TextureStore;
use xbox360_gpu::Gpu;
use xbox360_video::Display;

xbox360::sys::bridged!(#[unsafe(no_mangle)] pub _start, main);

fn image(size: u32, texel: impl Fn(u32, u32) -> Rgba8) -> Image {
    let pixels = (0..size * size).map(|i| texel(i % size, i / size)).collect();
    let edge = NonZeroU32::new(size).expect("nonzero size");
    Image::new(edge, edge, pixels).expect("square image")
}

fn quad(list: &mut DrawList, state: DrawState, rect: [f32; 4], uv: [f32; 4], colors: [Rgba8; 4]) {
    let [l, t, r, b] = rect;
    let [u0, v0, u1, v1] = uv;
    let vertices = [
        Vertex { position: [l, t], uv: [u0, v0], color: colors[0] },
        Vertex { position: [r, t], uv: [u1, v0], color: colors[1] },
        Vertex { position: [r, b], uv: [u1, v1], color: colors[2] },
        Vertex { position: [l, b], uv: [u0, v1], color: colors[3] },
    ];
    list.push(state, &vertices, &[[0, 1, 2], [0, 2, 3]]).expect("valid quad");
}

extern "C" fn main(_context: u32) -> u32 {
    run();
    xbox360::title::terminate()
}

fn run() {
    let mut display = match Display::new() {
        Ok(display) => display,
        Err(error) => return println!("gpu: {error}"),
    };
    let mut gpu = match Gpu::new() {
        Ok(gpu) => gpu,
        Err(error) => return println!("gpu: {error}"),
    };
    let size = display.size();
    let (w, h) = (size.width as f32, size.height as f32);
    // Red, green, blue and white quadrants, to check the channel order.
    let quadrants = gpu
        .create_texture(image(64, |x, y| match (x < 32, y < 32) {
            (true, true) => Rgba8::new(255, 0, 0, 255),
            (false, true) => Rgba8::new(0, 255, 0, 255),
            (true, false) => Rgba8::new(0, 0, 255, 255),
            (false, false) => Rgba8::new(255, 255, 255, 255),
        }))
        .expect("texture");
    let checker = gpu
        .create_texture(image(16, |x, y| if (x / 4 + y / 4) % 2 == 0 { Rgba8::new(0x60, 0x60, 0x60, 0x60) } else { Rgba8::TRANSPARENT }))
        .expect("texture");
    let target = gpu
        .create_target(NonZeroU32::new(4).expect("nonzero"), NonZeroU32::new(2).expect("nonzero"))
        .expect("render target");
    let background = Rgba8::new(0x10, 0x10, 0x20, 0xff);
    let frequency = TimebaseFrequency::query();
    let start = Timebase::now();
    let mut list = DrawList::new();
    let (mut frames, mut render_total, mut render_max, mut gpu_total) = (0u32, 0f32, 0f32, 0f32);
    let mut window = Timebase::now();
    loop {
        let t = Timebase::now().duration_since(start, frequency).as_secs_f32();
        if t > 10.0 {
            break;
        }
        list.clear();
        list.begin_target(target).expect("first pass");
        let opaque = [Rgba8::WHITE; 4];
        quad(&mut list, DrawState::default(), [0.0, 0.0, 2.0, 2.0], [0.0; 4], [Rgba8::new(128, 0, 0, 128); 4]);
        quad(&mut list, DrawState::default(), [2.0, 0.0, 4.0, 2.0], [0.0; 4], opaque);
        let modulate = DrawState { blend: BlendMode::Modulate, ..DrawState::default() };
        quad(&mut list, modulate, [2.0, 0.0, 4.0, 2.0], [0.0; 4], [Rgba8::new(255, 255, 255, 128); 4]);
        list.end_target();
        list.set_clear_color(Some(background));
        let (top, bottom) = (Rgba8::new(0x20, 0x30, 0x80, 0xff), Rgba8::new(0x80, 0x90, 0x20, 0xff));
        quad(&mut list, DrawState::default(), [0.0, 0.0, w, h / 2.0], [0.0; 4], [top, top, bottom, bottom]);
        let linear = Sampler { filter: Filter::Linear, ..Sampler::default() };
        let wrap = Sampler { address_u: AddressMode::Wrap, address_v: AddressMode::Wrap, filter: Filter::Nearest, ..Sampler::default() };
        let modes = [BlendMode::Normal, BlendMode::Additive, BlendMode::Modulate, BlendMode::Min, BlendMode::Max, BlendMode::ReverseSubtract];
        for (i, mode) in modes.into_iter().enumerate() {
            let x = 20.0 + i as f32 * 100.0;
            let state = DrawState { texture: Some(quadrants), sampler: linear, blend: mode, ..DrawState::default() };
            quad(&mut list, state, [x, 180.0, x + 90.0, 300.0], [0.0, 0.0, 1.0, 1.0], [Rgba8::new(255, 255, 255, 255); 4]);
        }
        let s = t * 0.2;
        quad(&mut list, DrawState { texture: Some(checker), sampler: wrap, ..DrawState::default() }, [0.0, 0.0, w, h], [s, 0.0, s + 8.0, 6.0], [Rgba8::WHITE; 4]);
        let clip = ClipRect { left: 100, top: 320, right: 300, bottom: 400 };
        quad(&mut list, DrawState { clip: Some(clip), ..DrawState::default() }, [50.0, 300.0, 350.0, 450.0], [0.0; 4], [Rgba8::new(255, 128, 0, 255); 4]);
        for i in 0..400 {
            let x = 380.0 + (i % 20) as f32 * 12.0 + (t * 3.0 + i as f32).sin() * 3.0;
            let y = 320.0 + (i / 20) as f32 * 7.0;
            quad(&mut list, DrawState { texture: Some(quadrants), ..DrawState::default() }, [x, y, x + 8.0, y + 5.0], [0.0, 0.0, 1.0, 1.0], [Rgba8::WHITE; 4]);
        }
        quad(&mut list, DrawState::default(), [40.0, 40.0, 168.0, 104.0], [0.0; 4], [background; 4]);
        let nearest = Sampler { filter: Filter::Nearest, ..Sampler::default() };
        quad(&mut list, DrawState { texture: Some(target), sampler: nearest, ..DrawState::default() }, [40.0, 40.0, 168.0, 104.0], [0.0, 0.0, 1.0, 1.0], opaque);
        let begin = Timebase::now();
        if let Err(error) = gpu.render(&mut display, &list) {
            return println!("gpu: {error}");
        }
        let submitted = Timebase::now();
        // The frame's draws and resolve have left the pipeline.
        while !display.ring().pipeline_drained() {}
        gpu_total += Timebase::now().duration_since(submitted, frequency).as_secs_f32() * 1000.0;
        let spent = submitted.duration_since(begin, frequency).as_secs_f32() * 1000.0;
        render_total += spent;
        render_max = render_max.max(spent);
        frames += 1;
        let elapsed = Timebase::now().duration_since(window, frequency);
        if elapsed >= Duration::from_secs(2) {
            println!(
                "gpu: {:.1} fps, render() avg {:.2} ms max {:.2} ms, GPU after submit {:.2} ms, {} triangles",
                frames as f32 / elapsed.as_secs_f32(),
                render_total / frames as f32,
                render_max,
                gpu_total / frames as f32,
                list.triangles().len()
            );
            (frames, render_total, render_max, gpu_total) = (0, 0.0, 0.0, 0.0);
            window = Timebase::now();
        }
    }
    println!("gpu: done, status {:08x}", xbox360::sys::mmio::GpuRegisters::get().register(0x5d0).read());
}
