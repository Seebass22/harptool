use crate::*;
use font_kit::family_name::FamilyName;
use font_kit::loaders::freetype::Font;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use raqote::*;

fn draw_number_row(
    dt: &mut DrawTarget,
    font: &Font,
    hole_size: f32,
    hole_gap: f32,
    y: f32,
    len: usize,
) {
    for i in 1..=len {
        let mut pb = PathBuilder::new();

        let fill_color = Source::Solid(SolidSource::from_unpremultiplied_argb(200, 39, 215, 245));
        // draw square
        let rectpoint = Point::new((i - 1) as f32 * (hole_size + hole_gap), y);
        pb.rect(rectpoint.x, rectpoint.y, hole_size, hole_size);
        let path = pb.finish();
        dt.fill(&path, &fill_color, &DrawOptions::new());

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
    y: f32,
    notes: &Vec<Option<(String, bool)>>,
) {
    for i in 0..notes.len() {
        if let Some((note, is_scale_note)) = notes.get(i).unwrap() {
            let mut pb = PathBuilder::new();

            let fill_color = if *is_scale_note {
                Source::Solid(SolidSource::from_unpremultiplied_argb(200, 129, 255, 124))
            } else {
                Source::Solid(SolidSource::from_unpremultiplied_argb(100, 129, 255, 124))
            };
            // draw square
            let rectpoint = Point::new(i as f32 * (hole_size + hole_gap), y);
            pb.rect(rectpoint.x, rectpoint.y, hole_size, hole_size);
            let path = pb.finish();
            dt.fill(&path, &fill_color, &DrawOptions::new());

            // draw hole number
            dt.draw_text(
                font,
                30.,
                note,
                Point::new(rectpoint.x, rectpoint.y + 30.0),
                &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 0, 0)),
                &DrawOptions::new(),
            );
        }
    }
}

pub fn export_png(tuning_name: &str, tuning: &Tuning, root: &ChromaticScale, setup: &Setup) {
    let mut dt = DrawTarget::new(800, 800);

    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();

    let hole_size = 50.0;
    let hole_gap = 2.0;
    let mut y = 0.0;

    for (i, row) in [
        &tuning.overblows,
        &tuning.blow_bends_full,
        &tuning.blow_bends_half,
        &tuning.blow,
        &tuning.draw,
        &tuning.bends_half,
        &tuning.bends_full,
        &tuning.bends_one_and_half,
    ]
    .iter()
    .enumerate()
    {
        let notes = Tuning::get_row_notes(row, root, setup);
        y += hole_size + hole_gap;
        draw_row(&mut dt, &font, hole_size, hole_gap, y, &notes);
        if i == 3 {
            y += hole_size + hole_gap;
            draw_number_row(&mut dt, &font, hole_size, hole_gap, y, tuning.blow.len());
        }
    }

    let scale_text = if let Some(scale) = &setup.scale {
        format!(", {} scale", scale)
    } else {
        String::from("")
    };
    let caption = format!("{} {} harmonica{}", root.root, tuning_name, scale_text);

    dt.draw_text(
        &font,
        30.,
        &caption,
        Point::new(0., 30.),
        &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 0, 0)),
        &DrawOptions::new(),
    );

    dt.write_png("example.png").unwrap();
}
