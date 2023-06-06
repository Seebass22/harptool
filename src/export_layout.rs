use crate::*;
use font_kit::loaders::freetype::Font;
use raqote::*;
use std::sync::Arc;

fn draw_number_row(
    dt: &mut DrawTarget,
    font: &Font,
    hole_size: f32,
    hole_gap: f32,
    pos: Point,
    len: usize,
) {
    for i in 1..=len {
        let fill_color = Source::Solid(SolidSource::from_unpremultiplied_argb(200, 39, 215, 245));
        let rectpoint = Point::new(pos.x + (i - 1) as f32 * (hole_size + hole_gap), pos.y);
        draw_square(dt, hole_size, rectpoint, fill_color);

        // draw hole number
        dt.draw_text(
            font,
            30.,
            i.to_string().as_ref(),
            Point::new(rectpoint.x, rectpoint.y + 30.0),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 0, 0)),
            &DrawOptions::new(),
        );
    }
}

fn draw_row(
    dt: &mut DrawTarget,
    font: &Font,
    hole_size: f32,
    hole_gap: f32,
    pos: Point,
    notes: &Vec<Option<(String, bool)>>,
) {
    for i in 0..notes.len() {
        if let Some((note, is_scale_note)) = notes.get(i).unwrap() {
            let fill_color = if *is_scale_note {
                Source::Solid(SolidSource::from_unpremultiplied_argb(255, 129, 255, 124))
            } else {
                Source::Solid(SolidSource::from_unpremultiplied_argb(100, 129, 255, 124))
            };

            let rectpoint = Point::new(pos.x + i as f32 * (hole_size + hole_gap), pos.y);
            draw_square(dt, hole_size, rectpoint, fill_color);

            // draw hole number
            dt.draw_text(
                font,
                30.,
                note,
                Point::new(rectpoint.x + 2.0, rectpoint.y + 30.0),
                &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 0, 0)),
                &DrawOptions::new(),
            );
        }
    }
}

fn draw_square(dt: &mut DrawTarget, size: f32, point: Point, fill_color: Source) {
    let mut pb = PathBuilder::new();
    pb.rect(point.x, point.y, size, size);
    let path = pb.finish();
    dt.fill(&path, &fill_color, &DrawOptions::new());
    let stroke_color = Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 0, 0));
    dt.stroke(
        &path,
        &stroke_color,
        &StrokeStyle::default(),
        &DrawOptions::new(),
    );
}

/// exports a PNG image of the note layout as "layout.png"
pub fn export_png(
    tuning_name: &str,
    tuning: &Tuning,
    root: &Option<ChromaticScale>,
    setup: &Setup,
    should_draw_row_labels: bool,
) {
    let mut dt = DrawTarget::new(1024, 600);

    let font = include_bytes!("dejavu-sans-font/DejaVuSans.ttf");
    let font = font_kit::font::Font::from_bytes(Arc::new(font.to_vec()), 0).unwrap();

    let hole_size = 50.0;
    let hole_gap = 4.0;
    let mut y = 0.0;
    let x = if should_draw_row_labels { 190.0 } else { 5.0 };

    if should_draw_row_labels {
        draw_background(Point::new(0., 50.), 185., 550., &mut dt);
    }

    for (i, (row, label)) in [
        (&tuning.overblows, "overblows"),
        (&tuning.blow_bends_full, "blow bends full step"),
        (&tuning.blow_bends_half, "blow bends half step"),
        (&tuning.blow, "blow"),
        (&tuning.draw, "draw"),
        (&tuning.bends_half, "bends half step"),
        (&tuning.bends_full, "bends full step"),
        (&tuning.bends_one_and_half, "bends 1 1/2 step"),
        (&tuning.overdraws, "overdraws"),
    ]
    .iter()
    .enumerate()
    {
        y += hole_size + hole_gap;

        let notes = if let Some(root) = root {
            Tuning::get_row_notes(row, root, setup)
        } else {
            Tuning::get_row_degrees(row, setup)
        };

        if should_draw_row_labels {
            draw_row_label(&mut dt, &font, label, y + 35.0);
        }
        draw_row(
            &mut dt,
            &font,
            hole_size,
            hole_gap,
            Point::new(x, y),
            &notes,
        );

        if i == 3 {
            y += hole_size + hole_gap;
            draw_number_row(
                &mut dt,
                &font,
                hole_size,
                hole_gap,
                Point::new(x, y),
                tuning.blow.len(),
            );
        }
    }

    draw_background(Point::new(0., 0.), 1024., 50., &mut dt);
    let caption = get_caption(tuning_name, root, setup);
    dt.draw_text(
        &font,
        30.,
        &caption,
        Point::new(0., 30.),
        &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 0, 0)),
        &DrawOptions::new(),
    );

    dt.write_png("layout.png").unwrap();
}

fn draw_background(pos: Point, width: f32, height: f32, dt: &mut DrawTarget) {
    let mut pb = PathBuilder::new();
    pb.rect(pos.x, pos.y, width, height);
    let path = pb.finish();
    dt.fill(
        &path,
        &Source::Solid(SolidSource::from_unpremultiplied_argb(150, 255, 255, 255)),
        &DrawOptions::new(),
    );
}

fn draw_row_label(dt: &mut DrawTarget, font: &Font, label: &str, y: f32) {
    dt.draw_text(
        font,
        18.,
        label,
        Point::new(0., y),
        &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 0, 0)),
        &DrawOptions::new(),
    );
}

fn to_ordinal(n: usize) -> String {
    let suffix = match n {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    format!("{}{}", n, suffix)
}

fn get_caption(tuning_name: &str, root: &Option<ChromaticScale>, setup: &Setup) -> String {
    let scale_text = if let Some(scale) = &setup.scale {
        format!(", {} scale", scale)
    } else {
        String::from("")
    };

    let harmonica_type = if let Some(root) = root {
        format!("{} {} harmonica", root.root, tuning_name)
    } else {
        format!("{} harmonica", tuning_name)
    };

    let position = to_ordinal(setup.position);
    format!("{}{}, {} position", harmonica_type, scale_text, position)
}
