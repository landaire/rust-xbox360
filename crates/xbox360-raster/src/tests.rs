use alloc::vec;
use alloc::vec::Vec;
use core::num::NonZeroU32;

use super::*;
use crate::draw::AddressMode;
use crate::draw::BlendMode;
use crate::draw::DrawState;
use crate::draw::Filter;
use crate::draw::Rgba8;
use crate::draw::Sampler;
use crate::draw::TextureMapping;
use crate::draw::Vertex;
use crate::error::TargetError;
use crate::error::TextureError;
use crate::image::Image;

const ONE_UNIT: Rgba8 = Rgba8::new(1, 1, 1, 1);

fn vertex(x: f32, y: f32, u: f32, v: f32, color: Rgba8) -> Vertex {
    Vertex {
        position: [x, y],
        uv: [u, v],
        color,
    }
}

/// Two triangles split along the top-left to bottom-right diagonal.
fn quad(
    list: &mut DrawList,
    state: DrawState,
    corners: [[f32; 2]; 4],
    uvs: [[f32; 2]; 4],
    color: Rgba8,
) {
    let vertices: Vec<Vertex> = corners
        .iter()
        .zip(uvs)
        .map(|(position, uv)| vertex(position[0], position[1], uv[0], uv[1], color))
        .collect();
    list.push(state, &vertices, &[[0, 1, 2], [0, 2, 3]])
        .unwrap();
}

/// Corners in order top left, top right, bottom right, bottom left.
fn rect(left: f32, top: f32, right: f32, bottom: f32) -> [[f32; 2]; 4] {
    [[left, top], [right, top], [right, bottom], [left, bottom]]
}

const FULL_UV: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

fn render(renderer: &mut SoftwareRenderer, list: &DrawList, width: u32, height: u32) -> Vec<Rgba8> {
    let mut pixels = vec![Rgba8::TRANSPARENT; (width * height) as usize];
    let mut target = RenderTarget::new(&mut pixels, width, height, width).unwrap();
    renderer.render(list, &mut target);

    pixels
}

fn additive() -> DrawState {
    DrawState {
        blend: BlendMode::Additive,
        ..DrawState::default()
    }
}

/// Coverage counts as `0` or `1`, one row per string.
fn coverage(pixels: &[Rgba8], width: u32) -> Vec<Vec<u8>> {
    pixels
        .chunks(width as usize)
        .map(|row| row.iter().map(|pixel| pixel.alpha()).collect())
        .collect()
}

fn gray(value: u8) -> Rgba8 {
    Rgba8::new(value, value, value, 255)
}

fn image(width: u32, height: u32, pixels: &[Rgba8]) -> Image {
    Image::new(
        NonZeroU32::new(width).unwrap(),
        NonZeroU32::new(height).unwrap(),
        pixels.to_vec(),
    )
    .unwrap()
}

fn texture(
    renderer: &mut SoftwareRenderer,
    width: u32,
    height: u32,
    pixels: &[Rgba8],
) -> TextureId {
    renderer
        .create_texture(image(width, height, pixels))
        .unwrap()
}

#[test]
fn grid_with_edges_through_integer_coordinates_covers_each_pixel_once() {
    let mut list = DrawList::new();

    // Unit cells whose corners lie exactly on integer target coordinates,
    // alternating diagonal direction so every edge orientation meets a tie.
    for row in 0..4 {
        for column in 0..4 {
            let (left, top) = (column as f32, row as f32);
            let corners = rect(left, top, left + 1.0, top + 1.0);
            let vertices: Vec<Vertex> = corners
                .iter()
                .map(|c| vertex(c[0], c[1], 0.0, 0.0, ONE_UNIT))
                .collect();
            let triangles = if (row + column) % 2 == 0 {
                [[0, 1, 2], [0, 2, 3]]
            } else {
                [[0, 1, 3], [1, 2, 3]]
            };
            list.push(additive(), &vertices, &triangles).unwrap();
        }
    }

    let pixels = render(&mut SoftwareRenderer::new(), &list, 6, 6);

    // Integer coordinates on the top and left edges of the union are
    // covered, while those on the bottom and right edges are not.
    let expected: Vec<Vec<u8>> = (0..6)
        .map(|y| (0..6).map(|x| u8::from(x < 4 && y < 4)).collect())
        .collect();
    assert_eq!(coverage(&pixels, 6), expected);
}

#[test]
fn triangle_fan_covers_each_pixel_once() {
    let center = [7.3, 6.9];
    let rim: [[f32; 2]; 7] = [
        [1.1, 2.7],
        [6.4, 0.6],
        [12.9, 1.3],
        [14.4, 8.8],
        [9.5, 13.6],
        [3.3, 12.4],
        [0.7, 7.1],
    ];
    let mut vertices = vec![vertex(center[0], center[1], 0.0, 0.0, ONE_UNIT)];
    vertices.extend(
        rim.iter()
            .map(|point| vertex(point[0], point[1], 0.0, 0.0, ONE_UNIT)),
    );
    let triangles: Vec<[u32; 3]> = (0..7)
        .map(|index| [0, index + 1, (index + 1) % 7 + 1])
        .collect();

    let mut list = DrawList::new();
    list.push(additive(), &vertices, &triangles).unwrap();
    let pixels = render(&mut SoftwareRenderer::new(), &list, 16, 16);

    // Coverage is the set of integer target coordinates strictly inside
    // the convex rim, given that no coordinate lies within 0.01 pixel of
    // the rim, so snapping to 1/256 pixel cannot move an edge across one.
    for (index, pixel) in pixels.iter().enumerate() {
        let (x, y) = ((index % 16) as f64, (index / 16) as f64);
        let distances: [f64; 7] = core::array::from_fn(|edge| {
            let a = rim[edge].map(f64::from);
            let b = rim[(edge + 1) % 7].map(f64::from);
            let length = ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt();
            ((b[0] - a[0]) * (y - a[1]) - (b[1] - a[1]) * (x - a[0])) / length
        });
        let near_rim = distances.iter().all(|distance| *distance > -0.01)
            && distances.iter().any(|distance| distance.abs() <= 0.01);
        assert!(!near_rim, "fixture rim passes near ({x}, {y})");

        let inside = distances.iter().all(|distance| *distance > 0.0);
        assert_eq!(pixel.alpha(), u8::from(inside), "pixel ({x}, {y})");
    }
}

#[test]
fn guard_band_clipped_quad_covers_each_pixel_once() {
    let mut list = DrawList::new();
    // The diagonal passes through every integer coordinate with x == y.
    quad(
        &mut list,
        additive(),
        rect(-1.0e6, -1.0e6, 1.0e6, 1.0e6),
        FULL_UV,
        ONE_UNIT,
    );

    let pixels = render(&mut SoftwareRenderer::new(), &list, 8, 8);

    assert!(pixels.iter().all(|pixel| *pixel == ONE_UNIT));
}

#[test]
fn clip_rect_limits_drawing() {
    let mut list = DrawList::new();
    let state = DrawState {
        clip: Some(ClipRect {
            left: 2,
            top: 1,
            right: 5,
            bottom: 3,
        }),
        ..DrawState::default()
    };
    quad(
        &mut list,
        state,
        rect(0.0, 0.0, 8.0, 4.0),
        FULL_UV,
        Rgba8::WHITE,
    );

    let pixels = render(&mut SoftwareRenderer::new(), &list, 8, 4);

    let expected: Vec<Vec<u8>> = (0..4)
        .map(|y| {
            (0..8)
                .map(|x| {
                    if (2..5).contains(&x) && (1..3).contains(&y) {
                        255
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect();
    assert_eq!(coverage(&pixels, 8), expected);
}

#[test]
fn clear_fills_target_and_ignores_clip() {
    let mut list = DrawList::new();
    list.set_clear_color(Some(gray(9)));
    let state = DrawState {
        clip: Some(ClipRect {
            left: 1,
            top: 0,
            right: 2,
            bottom: 1,
        }),
        ..DrawState::default()
    };
    quad(
        &mut list,
        state,
        rect(0.0, 0.0, 3.0, 2.0),
        FULL_UV,
        Rgba8::WHITE,
    );

    let pixels = render(&mut SoftwareRenderer::new(), &list, 3, 2);

    assert_eq!(
        pixels,
        [gray(9), Rgba8::WHITE, gray(9), gray(9), gray(9), gray(9)]
    );
}

#[test]
fn blend_modes_combine_with_target() {
    let target = Rgba8::new(100, 50, 200, 255);
    let dark_target = Rgba8::new(20, 40, 0, 100);
    let premultiplied = Rgba8::new(64, 32, 0, 128);
    // The texel above unpremultiplied; straight modes take this vertex color.
    let straight = Rgba8::new(128, 64, 0, 128);
    let cases = [
        (BlendMode::Normal, target, Rgba8::new(114, 57, 100, 255)),
        (BlendMode::Additive, target, Rgba8::new(164, 82, 200, 255)),
        (BlendMode::Modulate, target, Rgba8::new(50, 13, 0, 128)),
        (BlendMode::Min, target, Rgba8::new(100, 50, 0, 128)),
        (BlendMode::Max, target, Rgba8::new(128, 64, 200, 255)),
        (
            BlendMode::ReverseSubtract,
            dark_target,
            Rgba8::new(44, 0, 0, 28),
        ),
    ];

    for (blend, target, expected) in cases {
        let state = DrawState {
            blend,
            ..DrawState::default()
        };
        let vertex_color = if blend.uses_straight_source() {
            straight
        } else {
            premultiplied
        };

        let mut solid = DrawList::new();
        solid.set_clear_color(Some(target));
        quad(
            &mut solid,
            state,
            rect(0.0, 0.0, 2.0, 2.0),
            FULL_UV,
            vertex_color,
        );
        let pixels = render(&mut SoftwareRenderer::new(), &solid, 2, 2);
        assert!(
            pixels.iter().all(|pixel| *pixel == expected),
            "{blend:?} solid: {pixels:?}"
        );

        // A premultiplied texel modulated by white blends the same.
        let mut renderer = SoftwareRenderer::new();
        let id = texture(&mut renderer, 1, 1, &[premultiplied]);
        let mut textured = DrawList::new();
        textured.set_clear_color(Some(target));
        quad(
            &mut textured,
            DrawState {
                texture: Some(id),
                ..state
            },
            rect(0.0, 0.0, 2.0, 2.0),
            FULL_UV,
            Rgba8::WHITE,
        );
        let pixels = render(&mut renderer, &textured, 2, 2);
        assert!(
            pixels.iter().all(|pixel| *pixel == expected),
            "{blend:?} textured: {pixels:?}"
        );
    }
}

/// Straight modes keep the vertex color at zero opacity, so a fully
/// transparent modulating element still darkens the target.
#[test]
fn modulate_with_zero_opacity_keeps_color() {
    let mut list = DrawList::new();
    list.set_clear_color(Some(gray(200)));
    let state = DrawState {
        blend: BlendMode::Modulate,
        ..DrawState::default()
    };
    quad(
        &mut list,
        state,
        rect(0.0, 0.0, 1.0, 1.0),
        FULL_UV,
        Rgba8::new(128, 255, 0, 0),
    );

    assert_eq!(
        render(&mut SoftwareRenderer::new(), &list, 1, 1),
        [Rgba8::new(100, 200, 0, 0)]
    );
}

#[test]
fn bilinear_at_texel_centers_reproduces_texels() {
    let texels: Vec<Rgba8> = (0..16)
        .map(|index| Rgba8::new(index * 16, 255 - index * 16, index, 255))
        .collect();
    let mut renderer = SoftwareRenderer::new();
    let id = texture(&mut renderer, 4, 4, &texels);
    let state = DrawState {
        texture: Some(id),
        ..DrawState::default()
    };

    let texel_centers = [
        [0.125, 0.125],
        [1.125, 0.125],
        [1.125, 1.125],
        [0.125, 1.125],
    ];
    let mut list = DrawList::new();
    quad(
        &mut list,
        state,
        rect(0.0, 0.0, 4.0, 4.0),
        texel_centers,
        Rgba8::WHITE,
    );
    assert_eq!(render(&mut renderer, &list, 4, 4), texels);

    // Mirrored horizontally, the texels are sampled through the general path.
    let mut list = DrawList::new();
    let mirrored_uv = [
        [0.875, 0.125],
        [-0.125, 0.125],
        [-0.125, 1.125],
        [0.875, 1.125],
    ];
    quad(
        &mut list,
        state,
        rect(0.0, 0.0, 4.0, 4.0),
        mirrored_uv,
        Rgba8::WHITE,
    );
    let expected: Vec<Rgba8> = texels
        .chunks(4)
        .flat_map(|row| row.iter().rev().copied())
        .collect();
    assert_eq!(render(&mut renderer, &list, 4, 4), expected);
}

#[test]
fn magnified_bilinear_weights_neighbors() {
    let mut renderer = SoftwareRenderer::new();
    let id = texture(&mut renderer, 2, 1, &[gray(0), gray(200)]);
    let mut list = DrawList::new();
    quad(
        &mut list,
        DrawState {
            texture: Some(id),
            ..DrawState::default()
        },
        rect(0.0, 0.0, 4.0, 1.0),
        FULL_UV,
        Rgba8::WHITE,
    );

    // Xenos sample points are -0.5, 0, 0.5 and 1 texels, clamped.
    assert_eq!(
        render(&mut renderer, &list, 4, 1),
        [gray(0), gray(0), gray(100), gray(200)]
    );
}

#[test]
fn vertex_color_modulates_texels() {
    let mut renderer = SoftwareRenderer::new();
    let id = texture(&mut renderer, 1, 1, &[Rgba8::new(200, 100, 50, 255)]);
    let mut list = DrawList::new();
    let color = Rgba8::new(128, 128, 128, 128);
    quad(
        &mut list,
        DrawState {
            texture: Some(id),
            ..DrawState::default()
        },
        rect(0.0, 0.0, 1.0, 1.0),
        FULL_UV,
        color,
    );

    assert_eq!(
        render(&mut renderer, &list, 1, 1),
        [Rgba8::new(100, 50, 25, 128)]
    );
}

#[test]
fn vertex_colors_interpolate_at_integer_coordinates() {
    let mut list = DrawList::new();
    let vertices = [
        vertex(0.0, 0.0, 0.0, 0.0, gray(0)),
        vertex(2.0, 0.0, 0.0, 0.0, gray(255)),
        vertex(2.0, 1.0, 0.0, 0.0, gray(255)),
        vertex(0.0, 1.0, 0.0, 0.0, gray(0)),
    ];
    list.push(DrawState::default(), &vertices, &[[0, 1, 2], [0, 2, 3]])
        .unwrap();

    // Values 0 and 127.5 round to the nearest value.
    assert_eq!(
        render(&mut SoftwareRenderer::new(), &list, 2, 1),
        [gray(0), gray(128)]
    );
}

fn address_row(
    address: AddressMode,
    filter: Filter,
    texels: &[Rgba8],
    width: f32,
    uv: [f32; 2],
) -> Vec<Rgba8> {
    let mut renderer = SoftwareRenderer::new();
    let id = texture(&mut renderer, texels.len() as u32, 1, texels);
    let sampler = Sampler {
        filter,
        address_u: address,
        address_v: AddressMode::Clamp,
        mapping: TextureMapping::Direct,
    };
    let mut list = DrawList::new();
    let uvs = [[uv[0], 0.0], [uv[1], 0.0], [uv[1], 1.0], [uv[0], 1.0]];
    quad(
        &mut list,
        DrawState {
            texture: Some(id),
            sampler,
            ..DrawState::default()
        },
        rect(0.0, 0.0, width, 1.0),
        uvs,
        Rgba8::WHITE,
    );

    render(&mut renderer, &list, width as u32, 1)
}

#[test]
fn nearest_addressing_modes() {
    let [a, b, c, d] = [gray(10), gray(20), gray(30), gray(40)];
    let texels = [a, b, c, d];

    assert_eq!(
        address_row(
            AddressMode::Wrap,
            Filter::Nearest,
            &texels,
            12.0,
            [-1.0, 2.0]
        ),
        [a, b, c, d].repeat(3)
    );
    assert_eq!(
        address_row(
            AddressMode::Mirror,
            Filter::Nearest,
            &texels,
            12.0,
            [-1.0, 2.0]
        ),
        [d, c, b, a, a, b, c, d, d, c, b, a]
    );
    assert_eq!(
        address_row(
            AddressMode::Clamp,
            Filter::Nearest,
            &texels,
            12.0,
            [-1.0, 2.0]
        ),
        [a, a, a, a, a, b, c, d, d, d, d, d]
    );
}

#[test]
fn bilinear_addressing_modes_at_edges() {
    let texels = [gray(0), gray(255)];

    // Xenos sample points are -0.5, 0, 0.5 and 1 texels.
    assert_eq!(
        address_row(AddressMode::Wrap, Filter::Linear, &texels, 4.0, [0.0, 1.0]),
        [gray(128), gray(0), gray(128), gray(255)]
    );
    assert_eq!(
        address_row(AddressMode::Clamp, Filter::Linear, &texels, 4.0, [0.0, 1.0]),
        [gray(0), gray(0), gray(128), gray(255)]
    );
    assert_eq!(
        address_row(
            AddressMode::Mirror,
            Filter::Linear,
            &texels,
            4.0,
            [0.0, 1.0]
        ),
        [gray(0), gray(0), gray(128), gray(255)]
    );
}

#[test]
fn wrapped_coordinates_far_from_origin_match_near_ones() {
    let texels = [gray(10), gray(20), gray(30), gray(40), gray(50)];
    let near = address_row(
        AddressMode::Wrap,
        Filter::Linear,
        &texels,
        10.0,
        [0.125, 2.375],
    );
    // 2^16 texture periods away, beyond the 16.16 range; exact in `f32`.
    let far = address_row(
        AddressMode::Wrap,
        Filter::Linear,
        &texels,
        10.0,
        [65536.125, 65538.375],
    );

    assert_eq!(near, far);
}

#[test]
fn strong_minification_addresses_each_pixel() {
    let [a, b, c, d] = [gray(10), gray(20), gray(30), gray(40)];

    // 10001 texels per pixel, beyond the 16.16 stepping range. Texel
    // coordinates 0, 10001, 20002 and 30003 wrap to texels 0 to 3.
    assert_eq!(
        address_row(
            AddressMode::Wrap,
            Filter::Nearest,
            &[a, b, c, d],
            4.0,
            [0.0, 10001.0]
        ),
        [a, b, c, d]
    );
    // Texel coordinates -40000, -20000, 0 and 20000.
    assert_eq!(
        address_row(
            AddressMode::Clamp,
            Filter::Linear,
            &[a, b],
            4.0,
            [-20000.0, 20000.0]
        ),
        [a, a, a, b]
    );
}

#[test]
fn unknown_texture_draws_nothing() {
    let mut renderer = SoftwareRenderer::new();
    let id = texture(&mut renderer, 1, 1, &[Rgba8::WHITE]);
    renderer.destroy_texture(id).unwrap();
    let mut list = DrawList::new();
    quad(
        &mut list,
        DrawState {
            texture: Some(id),
            ..DrawState::default()
        },
        rect(0.0, 0.0, 2.0, 2.0),
        FULL_UV,
        Rgba8::WHITE,
    );

    assert!(
        render(&mut renderer, &list, 2, 2)
            .iter()
            .all(|pixel| *pixel == Rgba8::TRANSPARENT)
    );
}

#[test]
fn bands_render_the_same_pixels_as_the_whole_target() {
    let texels: Vec<Rgba8> = (0..64u32)
        .map(|index| {
            Rgba8::new(
                (index * 4) as u8,
                (index * 7) as u8,
                (255 - index * 3) as u8,
                255,
            )
        })
        .collect();
    let mut renderer = SoftwareRenderer::new();
    let id = texture(&mut renderer, 8, 8, &texels);

    let mut list = DrawList::new();
    list.set_clear_color(Some(gray(20)));
    let rotated = [[3.2, 1.1], [27.9, 6.4], [22.6, 30.2], [-2.3, 25.0]];
    let sampler = Sampler {
        filter: Filter::Linear,
        address_u: AddressMode::Mirror,
        address_v: AddressMode::Wrap,
        mapping: TextureMapping::Direct,
    };
    quad(
        &mut list,
        DrawState {
            texture: Some(id),
            sampler,
            ..DrawState::default()
        },
        rotated,
        [[-0.5, -0.5], [1.5, -0.5], [1.5, 1.5], [-0.5, 1.5]],
        Rgba8::new(200, 180, 160, 220),
    );
    let vertices = [
        vertex(0.0, 31.0, 0.0, 0.0, Rgba8::new(255, 0, 0, 255)),
        vertex(31.0, 0.0, 0.0, 0.0, Rgba8::new(0, 128, 0, 128)),
        vertex(31.0, 31.0, 0.0, 0.0, Rgba8::new(0, 0, 64, 64)),
    ];
    list.push(additive(), &vertices, &[[0, 1, 2]]).unwrap();

    let whole = render(&mut renderer, &list, 32, 32);

    let mut pixels = vec![Rgba8::TRANSPARENT; 32 * 32];
    let mut target = RenderTarget::new(&mut pixels, 32, 32, 32).unwrap();
    for mut band in target.bands(NonZeroU32::new(5).unwrap()) {
        renderer.render_band(&list, &mut band);
    }

    assert_eq!(pixels, whole);
}

#[test]
fn strided_target_leaves_padding_untouched() {
    let mut list = DrawList::new();
    list.set_clear_color(Some(Rgba8::WHITE));
    let sentinel = gray(7);
    let mut pixels = vec![sentinel; 4 * 2 - 1];

    let mut target = RenderTarget::new(&mut pixels, 3, 2, 4).unwrap();
    SoftwareRenderer::new().render(&list, &mut target);

    assert_eq!(
        pixels,
        [
            Rgba8::WHITE,
            Rgba8::WHITE,
            Rgba8::WHITE,
            sentinel,
            Rgba8::WHITE,
            Rgba8::WHITE,
            Rgba8::WHITE
        ]
    );
}

#[test]
fn invalid_targets_are_rejected() {
    let mut pixels = vec![Rgba8::TRANSPARENT; 10];

    assert_eq!(
        RenderTarget::new(&mut pixels, 4, 2, 3).unwrap_err(),
        TargetError::StrideTooSmall {
            stride: 3,
            width: 4
        }
    );
    assert_eq!(
        RenderTarget::new(&mut pixels, 4, 3, 4).unwrap_err(),
        TargetError::BufferTooSmall {
            required: 12,
            actual: 10
        }
    );
}

#[test]
fn stale_texture_ids_draw_nothing_after_slot_reuse() {
    let mut renderer = SoftwareRenderer::new();
    let first = texture(&mut renderer, 1, 1, &[gray(1)]);
    let second = texture(&mut renderer, 1, 1, &[gray(2)]);
    renderer.destroy_texture(first).unwrap();
    let third = texture(&mut renderer, 1, 1, &[gray(3)]);

    assert_ne!(third, first);
    assert_eq!(
        renderer.destroy_texture(first),
        Err(TextureError::UnknownTexture { id: first })
    );
    assert_eq!(
        renderer.update_texture(first, image(1, 1, &[gray(9)])),
        Err(TextureError::UnknownTexture { id: first })
    );

    renderer
        .update_texture(second, image(2, 1, &[gray(4), gray(5)]))
        .unwrap();
    let nearest = Sampler {
        filter: Filter::Nearest,
        ..Sampler::default()
    };
    let mut list = DrawList::new();
    quad(
        &mut list,
        DrawState {
            texture: Some(second),
            sampler: nearest,
            ..DrawState::default()
        },
        rect(0.0, 0.0, 2.0, 1.0),
        FULL_UV,
        Rgba8::WHITE,
    );
    quad(
        &mut list,
        DrawState {
            texture: Some(first),
            sampler: nearest,
            ..DrawState::default()
        },
        rect(0.0, 0.0, 2.0, 1.0),
        FULL_UV,
        Rgba8::WHITE,
    );
    assert_eq!(render(&mut renderer, &list, 2, 1), [gray(4), gray(5)]);
}

#[test]
fn non_power_of_two_textures_sample_exact_texels() {
    let texels: Vec<Rgba8> = (0..49u8)
        .map(|index| Rgba8::new(index * 5, 255 - index * 5, index, 255))
        .collect();
    let mut renderer = SoftwareRenderer::new();
    let id = texture(&mut renderer, 7, 7, &texels);
    let texel_centers = [
        [0.5 / 7.0, 0.5 / 7.0],
        [7.5 / 7.0, 0.5 / 7.0],
        [7.5 / 7.0, 7.5 / 7.0],
        [0.5 / 7.0, 7.5 / 7.0],
    ];
    let mut list = DrawList::new();
    quad(
        &mut list,
        DrawState {
            texture: Some(id),
            ..DrawState::default()
        },
        rect(0.0, 0.0, 7.0, 7.0),
        texel_centers,
        Rgba8::WHITE,
    );

    assert_eq!(render(&mut renderer, &list, 7, 7), texels);
}

#[test]
fn nearest_minification_picks_texels_at_integer_coordinates() {
    let texels: Vec<Rgba8> = (0..196u32).map(|index| gray(index as u8)).collect();
    let mut renderer = SoftwareRenderer::new();
    let id = texture(&mut renderer, 14, 14, &texels);
    let sampler = Sampler {
        filter: Filter::Nearest,
        ..Sampler::default()
    };
    let mut list = DrawList::new();
    quad(
        &mut list,
        DrawState {
            texture: Some(id),
            sampler,
            ..DrawState::default()
        },
        rect(0.0, 0.0, 7.0, 7.0),
        FULL_UV,
        Rgba8::WHITE,
    );

    // Integer target coordinates select texels 0, 2, ..., 12 per axis.
    let expected: Vec<Rgba8> = (0..7)
        .flat_map(|y| (0..7).map(move |x| gray(((2 * y) * 14 + 2 * x) as u8)))
        .collect();
    assert_eq!(render(&mut renderer, &list, 7, 7), expected);
}

#[test]
fn vertex_color_gradient_modulates_texels() {
    let mut renderer = SoftwareRenderer::new();
    let id = texture(&mut renderer, 1, 1, &[gray(200)]);
    let vertices = [
        vertex(0.0, 0.0, 0.0, 0.0, gray(0)),
        vertex(2.0, 0.0, 1.0, 0.0, gray(255)),
        vertex(2.0, 1.0, 1.0, 1.0, gray(255)),
        vertex(0.0, 1.0, 0.0, 1.0, gray(0)),
    ];
    let mut list = DrawList::new();
    list.push(
        DrawState {
            texture: Some(id),
            ..DrawState::default()
        },
        &vertices,
        &[[0, 1, 2], [0, 2, 3]],
    )
    .unwrap();

    // Colors 0 and 128 scale the texel to 0 and 100.
    assert_eq!(render(&mut renderer, &list, 2, 1), [gray(0), gray(100)]);
}

#[test]
fn distant_vertices_cover_like_their_clipped_equivalent() {
    let triangle = |far: bool| {
        let (left, bottom) = if far {
            ([-1.0e30, 2.0e30], 1.0e30)
        } else {
            ([0.0, 300.0], 300.0)
        };
        let vertices = [
            vertex(100.0, 100.0, 0.0, 0.0, ONE_UNIT),
            vertex(100.0, bottom, 0.0, 0.0, ONE_UNIT),
            vertex(left[0], left[1], 0.0, 0.0, ONE_UNIT),
        ];
        let mut list = DrawList::new();
        list.push(additive(), &vertices, &[[0, 1, 2]]).unwrap();
        render(&mut SoftwareRenderer::new(), &list, 128, 128)
    };

    // The slanted edge `2x + y = 300` passes no integer target coordinate.
    let near = triangle(false);
    assert!(near.contains(&ONE_UNIT));
    assert_eq!(triangle(true), near);
}

#[test]
fn guard_band_clipping_of_extreme_triangle_covers_at_most_once() {
    let far = [
        vertex(-12569.3545, -28233.709, 0.0, 0.0, ONE_UNIT),
        vertex(16695.492, 17187.377, 0.0, 0.0, ONE_UNIT),
        vertex(-4.8389093e19, -4.009841e20, 0.0, 0.0, ONE_UNIT),
    ];
    let mut list = DrawList::new();
    list.push(additive(), &far, &[[0, 1, 2]]).unwrap();

    let pixels = render(&mut SoftwareRenderer::new(), &list, 64, 64);

    assert!(
        pixels
            .iter()
            .all(|pixel| *pixel == ONE_UNIT || *pixel == Rgba8::TRANSPARENT)
    );
}

#[test]
fn sliver_with_steep_color_gradient_interpolates_along_its_edge() {
    let sliver = [
        vertex(0.0, 0.0, 0.0, 0.0, Rgba8::WHITE),
        vertex(1.0 / 256.0, 0.0, 0.0, 0.0, Rgba8::WHITE),
        vertex(0.0, 100.0, 0.0, 0.0, Rgba8::new(0, 0, 0, 255)),
    ];
    let mut list = DrawList::new();
    list.push(DrawState::default(), &sliver, &[[0, 1, 2]])
        .unwrap();

    let pixels = render(&mut SoftwareRenderer::new(), &list, 4, 100);

    // Integer coordinates at x = 0 lie on the left edge and are covered;
    // the color falls from white to black down the column.
    let column: Vec<Rgba8> = pixels.iter().step_by(4).copied().collect();
    let expected: Vec<Rgba8> = (0..100u32)
        .map(|row| gray(((255 * (100 - row) + 50) / 100) as u8))
        .collect();
    assert!(
        column
            .iter()
            .zip(&expected)
            .all(|(actual, expected)| actual.red().abs_diff(expected.red()) <= 1),
        "{column:?}"
    );
    for (index, pixel) in pixels.iter().enumerate() {
        assert!(
            index % 4 == 0 || *pixel == Rgba8::TRANSPARENT,
            "pixel {index}: {pixel:?}"
        );
    }
}

#[test]
fn clamped_coordinates_beyond_i64_pick_the_near_edge() {
    let [a, b] = [gray(10), gray(20)];

    assert_eq!(
        address_row(
            AddressMode::Clamp,
            Filter::Linear,
            &[a, b],
            4.0,
            [-1.0e30, 1.0]
        ),
        [a, a, a, a]
    );
    assert_eq!(
        address_row(
            AddressMode::Clamp,
            Filter::Nearest,
            &[a, b],
            4.0,
            [0.0, 1.0e30]
        ),
        [a, b, b, b]
    );
}

#[test]
fn oversized_targets_are_rejected() {
    let mut pixels = [Rgba8::TRANSPARENT; 0];

    assert_eq!(
        RenderTarget::new(
            &mut pixels,
            RenderTarget::MAX_DIMENSION + 1,
            0,
            RenderTarget::MAX_DIMENSION + 1
        )
        .unwrap_err(),
        TargetError::TooLarge {
            width: RenderTarget::MAX_DIMENSION + 1,
            height: 0
        }
    );
}

/// A pixel's value must not depend on where its span starts, so moving the
/// left clip edge leaves every remaining pixel unchanged.
#[test]
fn pixels_do_not_depend_on_span_start() {
    let texels: Vec<Rgba8> = (0..49u32)
        .map(|index| Rgba8::new((index * 5) as u8, (index * 3) as u8, 255 - index as u8, 255))
        .collect();
    let mut renderer = SoftwareRenderer::new();
    let id = texture(&mut renderer, 7, 7, &texels);

    let mut draw = |left: i32| {
        let clip = Some(ClipRect {
            left,
            top: 0,
            right: 200,
            bottom: 8,
        });
        let sampler = Sampler {
            filter: Filter::Nearest,
            address_u: AddressMode::Wrap,
            address_v: AddressMode::Mirror,
            mapping: TextureMapping::Direct,
        };
        let vertices = [
            vertex(0.3, -0.7, 0.0, 0.0, Rgba8::new(255, 0, 0, 255)),
            vertex(199.1, 0.4, 13.7, 0.3, Rgba8::new(0, 255, 0, 255)),
            vertex(198.6, 8.9, 13.1, 2.9, Rgba8::new(0, 0, 255, 255)),
            vertex(0.9, 8.2, -0.4, 2.2, Rgba8::new(128, 128, 128, 255)),
        ];
        let mut list = DrawList::new();
        list.push(
            DrawState {
                texture: Some(id),
                sampler,
                clip,
                ..DrawState::default()
            },
            &vertices,
            &[[0, 1, 2], [0, 2, 3]],
        )
        .unwrap();
        list.push(
            DrawState {
                blend: BlendMode::Additive,
                clip,
                ..DrawState::default()
            },
            &vertices,
            &[[0, 1, 2], [0, 2, 3]],
        )
        .unwrap();
        render(&mut renderer, &list, 200, 8)
    };

    let reference = draw(0);
    for left in [1, 37, 63, 64, 65, 130] {
        let shifted = draw(left);
        for (index, pixel) in shifted.iter().enumerate() {
            if index % 200 >= left as usize {
                assert_eq!(*pixel, reference[index], "left {left}, pixel {index}");
            }
        }
    }
}

#[test]
fn radial_mapping_samples_by_distance_from_center() {
    let ring = [gray(10), gray(20), gray(30), gray(40)];
    let mut renderer = SoftwareRenderer::new();
    let id = texture(&mut renderer, 4, 1, &ring);
    let sampler = Sampler {
        filter: Filter::Nearest,
        mapping: TextureMapping::Radial,
        ..Sampler::default()
    };
    let mut list = DrawList::new();
    quad(
        &mut list,
        DrawState {
            texture: Some(id),
            sampler,
            ..DrawState::default()
        },
        rect(0.0, 0.0, 8.0, 8.0),
        [[0.03, 0.03], [1.03, 0.03], [1.03, 1.03], [0.03, 1.03]],
        Rgba8::WHITE,
    );

    let pixels = render(&mut renderer, &list, 8, 8);

    for (index, pixel) in pixels.iter().enumerate() {
        let offset = |coordinate: usize| coordinate as f64 / 8.0 + 0.03 - 0.5;
        let radius = 2.0 * (offset(index % 8).powi(2) + offset(index / 8).powi(2)).sqrt();
        let column = radius * 4.0;
        assert!(
            (column - column.round()).abs() > 0.01,
            "fixture radius near a texel edge at pixel {index}"
        );
        assert_eq!(*pixel, ring[(column as usize).min(3)], "pixel {index}");
    }
}

/// An opaque rectangle covering the target hides the clear and every
/// earlier command; the result equals drawing it alone.
#[test]
fn opaque_cover_hides_earlier_commands() {
    let texels: Vec<Rgba8> = (0..16u8)
        .map(|index| Rgba8::new(index * 16, 255 - index * 16, 7, 255))
        .collect();
    let mut renderer = SoftwareRenderer::new();
    let id = texture(&mut renderer, 4, 4, &texels);
    let cover = |list: &mut DrawList, rect_corners: [[f32; 2]; 4], clip: Option<ClipRect>| {
        quad(
            list,
            DrawState {
                texture: Some(id),
                clip,
                ..DrawState::default()
            },
            rect_corners,
            FULL_UV,
            Rgba8::WHITE,
        );
    };

    let mut alone = DrawList::new();
    cover(&mut alone, rect(0.0, 0.0, 8.0, 8.0), None);
    let expected = render(&mut renderer, &alone, 8, 8);

    let mut layered = DrawList::new();
    layered.set_clear_color(Some(gray(9)));
    quad(
        &mut layered,
        additive(),
        rect(0.0, 0.0, 8.0, 8.0),
        FULL_UV,
        Rgba8::new(200, 0, 0, 200),
    );
    cover(&mut layered, rect(0.0, 0.0, 8.0, 8.0), None);
    assert_eq!(render(&mut renderer, &layered, 8, 8), expected);

    // Not reaching column 0, or clipped, the earlier content stays visible.
    for (rect_corners, clip) in [
        (rect(1.0, 0.0, 8.0, 8.0), None),
        (
            rect(0.0, 0.0, 8.0, 8.0),
            Some(ClipRect {
                left: 0,
                top: 0,
                right: 8,
                bottom: 7,
            }),
        ),
    ] {
        let mut partial = DrawList::new();
        partial.set_clear_color(Some(gray(9)));
        cover(&mut partial, rect_corners, clip);
        let pixels = render(&mut renderer, &partial, 8, 8);
        assert!(pixels.contains(&gray(9)), "{rect_corners:?} {clip:?}");
    }
}

/// A rectangle whose texture has a transparent texel does not hide what is
/// under it.
#[test]
fn cover_with_transparent_texel_blends() {
    let mut texels = vec![Rgba8::WHITE; 4];
    texels[3] = Rgba8::TRANSPARENT;
    let mut renderer = SoftwareRenderer::new();
    let id = texture(&mut renderer, 2, 2, &texels);
    let nearest = Sampler {
        filter: Filter::Nearest,
        ..Sampler::default()
    };
    let mut list = DrawList::new();
    list.set_clear_color(Some(gray(9)));
    quad(
        &mut list,
        DrawState {
            texture: Some(id),
            sampler: nearest,
            ..DrawState::default()
        },
        rect(0.0, 0.0, 2.0, 2.0),
        FULL_UV,
        Rgba8::WHITE,
    );

    assert_eq!(
        render(&mut renderer, &list, 2, 2),
        [Rgba8::WHITE, Rgba8::WHITE, Rgba8::WHITE, gray(9)]
    );
}

/// The rectangle path covers exactly the pixels of the two triangles, for
/// rectangles at arbitrary subpixel positions.
#[test]
fn aligned_rect_covers_like_its_triangles() {
    let mut state = 0x2545_F491u32;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        (state % 8192) as f32 / 256.0 - 8.0
    };
    for _ in 0..600 {
        let (x0, x1, y0, y1) = (next(), next(), next(), next());
        let (left, right, top, bottom) = (x0.min(x1), x0.max(x1), y0.min(y1), y0.max(y1));
        let corners = rect(left, top, right, bottom);
        let vertices: Vec<Vertex> = corners
            .iter()
            .map(|c| vertex(c[0], c[1], 0.0, 0.0, ONE_UNIT))
            .collect();

        let mut as_rect = DrawList::new();
        as_rect
            .push(additive(), &vertices, &[[0, 1, 2], [0, 2, 3]])
            .unwrap();
        // A degenerate triangle between the halves keeps them apart.
        let mut as_triangles = DrawList::new();
        as_triangles
            .push(additive(), &vertices, &[[0, 1, 2], [0, 0, 0], [0, 2, 3]])
            .unwrap();

        let mut renderer = SoftwareRenderer::new();
        assert_eq!(
            render(&mut renderer, &as_rect, 16, 16),
            render(&mut renderer, &as_triangles, 16, 16),
            "{corners:?}"
        );
    }
}

/// A gradient too steep for 32 fraction bits keeps exact values at covered
/// pixels: the plane switches to 16 fraction bits rather than clamping.
#[test]
fn steep_texture_gradient_keeps_its_zero_crossing() {
    let [a, b] = [gray(10), gray(20)];

    assert_eq!(
        address_row(
            AddressMode::Clamp,
            Filter::Nearest,
            &[a, b],
            8.0,
            [-500_000.0, 500_000.0]
        ),
        [a, a, a, a, a, b, b, b]
    );
}

/// A sliver one subpixel wide away from a chunk anchor: its color step
/// exceeds the fast stepping bounds, and the covered column on its left
/// edge keeps the left vertex color.
#[test]
fn steep_color_gradient_away_from_anchor_keeps_edge_color() {
    let sliver = [
        vertex(10.0, 0.0, 0.0, 0.0, gray(100)),
        vertex(10.0 + 1.0 / 256.0, 0.0, 0.0, 0.0, gray(255)),
        vertex(10.0, 100.0, 0.0, 0.0, gray(100)),
    ];
    let mut list = DrawList::new();
    list.push(DrawState::default(), &sliver, &[[0, 1, 2]])
        .unwrap();

    let pixels = render(&mut SoftwareRenderer::new(), &list, 16, 100);

    let column: Vec<Rgba8> = pixels.iter().skip(10).step_by(16).copied().collect();
    assert!(
        column.iter().filter(|pixel| **pixel == gray(100)).count() >= 90,
        "{column:?}"
    );
    for (index, pixel) in pixels.iter().enumerate() {
        assert!(
            *pixel == Rgba8::TRANSPARENT || (index % 16 == 10 && *pixel == gray(100)),
            "pixel {index}: {pixel:?}"
        );
    }
}

/// An opaque cover stops hiding earlier content once its texture gains a
/// transparent texel, and neither additive blending nor a translucent
/// vertex color hides anything.
#[test]
fn covers_that_are_not_opaque_hide_nothing() {
    let mut renderer = SoftwareRenderer::new();
    let id = texture(&mut renderer, 1, 1, &[Rgba8::WHITE]);
    let nearest = Sampler {
        filter: Filter::Nearest,
        ..Sampler::default()
    };
    let draw = |renderer: &mut SoftwareRenderer, blend: BlendMode, color: Rgba8| {
        let mut list = DrawList::new();
        list.set_clear_color(Some(gray(9)));
        quad(
            &mut list,
            DrawState {
                texture: Some(id),
                sampler: nearest,
                blend,
                ..DrawState::default()
            },
            rect(0.0, 0.0, 2.0, 2.0),
            FULL_UV,
            color,
        );
        render(renderer, &list, 2, 2)
    };

    assert_eq!(
        draw(
            &mut renderer,
            BlendMode::Additive,
            Rgba8::new(10, 10, 10, 255)
        ),
        [gray(19); 4]
    );
    assert_eq!(
        draw(
            &mut renderer,
            BlendMode::Normal,
            Rgba8::new(128, 128, 128, 128)
        ),
        [Rgba8::new(132, 132, 132, 255); 4]
    );

    renderer
        .update_texture(id, image(1, 1, &[Rgba8::new(0, 0, 0, 128)]))
        .unwrap();
    assert_eq!(
        draw(&mut renderer, BlendMode::Normal, Rgba8::WHITE),
        [Rgba8::new(4, 4, 4, 255); 4]
    );
}

/// The list the GPU example draws into its target: a half-transparent red
/// rect, and opaque white modulated by half alpha, sampled 1:1 onto the
/// background.
fn target_example_list(target: TextureId, background: Rgba8) -> DrawList {
    let mut list = DrawList::new();
    list.begin_target(target).unwrap();
    quad(
        &mut list,
        DrawState::default(),
        rect(0.0, 0.0, 2.0, 2.0),
        FULL_UV,
        Rgba8::new(128, 0, 0, 128),
    );
    quad(
        &mut list,
        DrawState::default(),
        rect(2.0, 0.0, 4.0, 2.0),
        FULL_UV,
        Rgba8::WHITE,
    );
    let modulate = DrawState {
        blend: BlendMode::Modulate,
        ..DrawState::default()
    };
    quad(
        &mut list,
        modulate,
        rect(2.0, 0.0, 4.0, 2.0),
        FULL_UV,
        Rgba8::new(255, 255, 255, 128),
    );
    list.end_target();

    list.set_clear_color(Some(background));
    let nearest = DrawState {
        texture: Some(target),
        sampler: Sampler {
            filter: Filter::Nearest,
            ..Sampler::default()
        },
        ..DrawState::default()
    };
    quad(
        &mut list,
        nearest,
        rect(0.0, 0.0, 4.0, 2.0),
        FULL_UV,
        Rgba8::WHITE,
    );

    list
}

#[test]
fn target_pass_squares_source_alpha_and_samples_straight() {
    let mut renderer = SoftwareRenderer::new();
    let size = NonZeroU32::new(4).unwrap();
    let target = renderer
        .create_target(size, NonZeroU32::new(2).unwrap())
        .unwrap();
    let list = target_example_list(target, Rgba8::new(0x10, 0x10, 0x20, 0xff));

    let pixels = render(&mut renderer, &list, 4, 2);

    // Red texel (128, 0, 0, 64): premultiplied (32, 0, 0, 64) over the
    // background. Modulated white texel (255, 255, 255, 128).
    let red = Rgba8::new(44, 12, 24, 255);
    let white = Rgba8::new(136, 136, 144, 255);
    assert_eq!(pixels, [red, red, white, white, red, red, white, white]);
}
/// Xenos evaluates triangle coverage and interpolants at integer target
/// coordinates rather than half-pixel centers.
#[test]
fn target_sampling_matches_xenos_integer_coordinates() {
    let mut renderer = SoftwareRenderer::new();
    let source = texture(&mut renderer, 2, 1, &[gray(0), gray(200)]);
    let target = renderer
        .create_target(NonZeroU32::new(4).unwrap(), NonZeroU32::new(2).unwrap())
        .unwrap();
    let mut list = DrawList::new();
    list.begin_target(target).unwrap();
    let linear = DrawState {
        texture: Some(source),
        sampler: Sampler {
            filter: Filter::Linear,
            ..Sampler::default()
        },
        ..DrawState::default()
    };
    quad(
        &mut list,
        linear,
        rect(0.0, 0.0, 4.0, 1.0),
        FULL_UV,
        Rgba8::WHITE,
    );
    quad(
        &mut list,
        DrawState::default(),
        rect(0.25, 1.0, 3.25, 2.0),
        FULL_UV,
        Rgba8::WHITE,
    );
    list.end_target();

    let background = Rgba8::new(16, 16, 32, 255);
    list.set_clear_color(Some(background));
    let nearest = DrawState {
        texture: Some(target),
        sampler: Sampler {
            filter: Filter::Nearest,
            ..Sampler::default()
        },
        ..DrawState::default()
    };
    quad(
        &mut list,
        nearest,
        rect(0.0, 0.0, 4.0, 2.0),
        FULL_UV,
        Rgba8::WHITE,
    );

    assert_eq!(
        render(&mut renderer, &list, 4, 2),
        [
            gray(0),
            gray(0),
            gray(100),
            gray(200),
            background,
            Rgba8::WHITE,
            Rgba8::WHITE,
            Rgba8::WHITE,
        ]
    );
}

/// The text shadow probe of the GPU example at one pixel per texel: a 4x4
/// alpha texture over gray, in Normal (left) and Modulate (right) blending.
#[test]
fn text_shadow_matches_the_gpu_example() {
    const ALPHA: [[u8; 4]; 4] = [
        [255, 0, 0, 0],
        [0, 128, 0, 0],
        [0, 64, 255, 0],
        [0, 0, 0, 0],
    ];

    let mut renderer = SoftwareRenderer::new();
    let texels: Vec<Rgba8> = ALPHA
        .iter()
        .flatten()
        .map(|&alpha| Rgba8::new(alpha, alpha, alpha, alpha))
        .collect();
    let glyph = texture(&mut renderer, 4, 4, &texels);
    let shadow = Some(TextShadow {
        color: Rgba8::new(20, 40, 240, 180),
    });
    let nearest = Sampler {
        filter: Filter::Nearest,
        ..Sampler::default()
    };
    let normal = DrawState {
        texture: Some(glyph),
        sampler: nearest,
        shadow,
        ..DrawState::default()
    };
    let modulate = DrawState {
        blend: BlendMode::Modulate,
        ..normal
    };
    let mut list = DrawList::new();
    list.set_clear_color(Some(Rgba8::new(128, 128, 128, 255)));
    quad(
        &mut list,
        normal,
        rect(0.0, 0.0, 4.0, 4.0),
        FULL_UV,
        Rgba8::new(157, 78, 39, 200),
    );
    quad(
        &mut list,
        modulate,
        rect(4.0, 0.0, 8.0, 4.0),
        FULL_UV,
        Rgba8::new(200, 100, 50, 200),
    );

    let pixels = render(&mut renderer, &list, 8, 4);

    let rgb: Vec<[u8; 3]> = pixels
        .iter()
        .map(|pixel| [pixel.red(), pixel.green(), pixel.blue()])
        .collect();
    assert_eq!(
        rgb,
        [
            [168, 92, 83],
            [52, 66, 207],
            [128, 128, 128],
            [128, 128, 128],
            [86, 45, 41],
            [10, 20, 120],
            [0, 0, 0],
            [0, 0, 0],
            [52, 66, 207],
            [111, 79, 146],
            [128, 128, 128],
            [128, 128, 128],
            [10, 20, 120],
            [53, 35, 75],
            [0, 0, 0],
            [0, 0, 0],
            [128, 128, 128],
            [142, 123, 113],
            [177, 99, 75],
            [128, 128, 128],
            [0, 0, 0],
            [100, 50, 25],
            [92, 48, 33],
            [0, 0, 0],
            [128, 128, 128],
            [128, 128, 128],
            [109, 112, 147],
            [52, 66, 207],
            [0, 0, 0],
            [0, 0, 0],
            [10, 20, 120],
            [10, 20, 120],
        ]
    );
}
