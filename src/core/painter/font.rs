use anyhow::Error;
use cosmic_text::{
    Align, Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, SwashCache, Weight, Wrap,
};
use image::{imageops, Pixel, Rgb, RgbImage, Rgba, RgbaImage};

pub fn draw_text_with_background(
    primary_headline: &str,
    secondary_headline: &str,
    background_buffer: &mut RgbImage,
    text_color: Color,
    bounds_primary: (f32, f32, f32, f32), // (x, y, width, height)
    bounds_secondary: (f32, f32, f32, f32), // (x, y, width, height)
    highlight_words: &[String],
    highlight_color: Color,
    font_size: f32,
    line_height: f32,
) -> Result<(), Error> {
    let primary_font_dir = "assets/FontName.ttf";
    let secondary_font_dir = "assets/FontName.ttf";
    let mut primary_font = FontSystem::new();
    let mut secondary_font = FontSystem::new();
    let fontdb = primary_font.db_mut();
    fontdb.load_font_file(primary_font_dir)?;
    let fontdb = secondary_font.db_mut();
    fontdb.load_font_file(secondary_font_dir)?;

    let metrics_primary = Metrics::new(font_size, line_height);
    let metrics_secondary = Metrics::new(font_size * 0.5, line_height * 0.5);
    let mut swash_primary = SwashCache::new();
    let mut swash_secondary = SwashCache::new();

    let mut buffer_primary = Buffer::new(&mut primary_font, metrics_primary);
    let mut buffer_primary = buffer_primary.borrow_with(&mut primary_font);
    let mut buffer_secondary = Buffer::new(&mut secondary_font, metrics_secondary);
    let mut buffer_secondary = buffer_secondary.borrow_with(&mut secondary_font);

    // Set the buffers to match the bounding box dimensions.
    buffer_primary.set_size(Some(bounds_primary.2), Some(bounds_primary.3));
    buffer_secondary.set_size(Some(bounds_secondary.2), Some(bounds_secondary.3));

    let attrs = Attrs::new()
        .family(Family::Name("Outfit"))
        .weight(Weight::BLACK);

    buffer_primary.set_rich_text(
        [(
            primary_headline,
            attrs.color(text_color).family(Family::Name("Outfit")),
        )],
        attrs,
        Shaping::Advanced,
    );
    buffer_primary.set_wrap(Wrap::WordOrGlyph);

    buffer_secondary.set_rich_text(
        [(
            secondary_headline,
            attrs
                .color(text_color)
                .family(Family::Name("Outfit"))
                .weight(Weight::NORMAL),
        )],
        attrs,
        Shaping::Advanced,
    );
    buffer_secondary.set_wrap(Wrap::WordOrGlyph);

    // Set center alignment for each line
    for line in &mut buffer_primary.lines {
        line.set_align(Some(Align::Center));
    }
    buffer_primary.shape_until_scroll(true);

    for line in &mut buffer_secondary.lines {
        line.set_align(Some(Align::Center));
    }
    buffer_secondary.shape_until_scroll(true);

    // Containers for both primary and secondary headline.
    let (img_width, img_height) = background_buffer.dimensions();
    let mut primary_headline_img = RgbaImage::new(img_width, img_height);
    for pixel in primary_headline_img.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0]); // transparent background to only draw text; helpful while extracting drawn pixels.
    }

    let mut secondary_headline_img = RgbaImage::new(img_width, img_height);
    for pixel in secondary_headline_img.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0]); // transparent background to only draw text; helpful while extracting drawn pixels.
    }

    // Draw the respective contents in their containers.
    buffer_primary.draw(&mut swash_primary, text_color, |x, y, _, _, color| {
        // Adjust x and y coordinates to be relative to the bounding box
        let x = (x as f32 + bounds_primary.0).floor() as i32;
        let y = (y as f32 + bounds_primary.1).floor() as i32;

        // Check if the pixel is within the bounding box
        if x >= bounds_primary.0 as i32
            && y >= bounds_primary.1 as i32
            && x < (bounds_primary.0 + bounds_primary.2) as i32
            && y < (bounds_primary.1 + bounds_primary.3) as i32
        {
            primary_headline_img.put_pixel(
                x as u32,
                y as u32,
                Rgba([color.r(), color.g(), color.b(), color.a()]),
            );
        }
    });

    buffer_secondary.draw(&mut swash_secondary, text_color, |x, y, _, _, color| {
        // Adjust x and y coordinates to be relative to the bounding box
        let x = (x as f32 + bounds_secondary.0).floor() as i32;
        let y = (y as f32 + bounds_secondary.1).floor() as i32;

        // Check if the pixel is within the bounding box
        if x >= bounds_secondary.0 as i32
            && y >= bounds_secondary.1 as i32
            && x < (bounds_secondary.0 + bounds_secondary.2) as i32
            && y < (bounds_secondary.1 + bounds_secondary.3) as i32
        {
            secondary_headline_img.put_pixel(
                x as u32,
                y as u32,
                Rgba([color.r(), color.g(), color.b(), color.a()]),
            );
        }
    });

    // Calculate the number of characters per line - primary headline.
    let mut chars_per_line = Vec::<usize>::new();
    for run in buffer_primary.layout_runs() {
        let mut chars_count = 0;

        for glyphs in run.glyphs.iter() {
            let char = &run.text[glyphs.start..glyphs.end];
            chars_count += char.len();
        }

        chars_per_line.push(chars_count);
    }

    // Calculate bounding boxes for each line in primary headline,
    // to bound the highlighted word.
    let mut word_positions = Vec::<(f32, f32, f32, f32)>::new();
    let mut highlight_char_count = 0;
    let mut current_line = 0;
    for run in buffer_primary.layout_runs() {
        let mut word = String::new();
        let mut current_char_count = 0;
        let mut word_left_boundary = 0;
        let mut word_width = 0;
        let mut make_next_boundary_left = false;

        for glyphs in run.glyphs.iter() {
            if current_char_count == 0 || make_next_boundary_left {
                word_left_boundary = glyphs.x as u32;
                make_next_boundary_left = false;
            }

            let char = &run.text[glyphs.start..glyphs.end];
            word.push_str(char);
            current_char_count += char.len();
            word_width += glyphs.w as u32;

            if char == " " || current_char_count == chars_per_line[current_line] {
                let current_word = word.trim();

                if current_char_count == chars_per_line[current_line] {
                    word_width += glyphs.w as u32 / 2;
                }

                if highlight_words.contains(&current_word.to_lowercase()) {
                    highlight_char_count += current_word.len();
                    word_positions.push((
                        word_left_boundary as f32,
                        run.line_top as f32,
                        word_width as f32,
                        glyphs.font_size,
                    ));
                }

                word.clear();
                word_width = 0;
                make_next_boundary_left = true;
            }
        }
        current_line += 1;
    }

    // Calculate the average width of the highlighted words - primary headline.
    let total_width = word_positions
        .iter()
        .map(|(_, _, width, _)| width)
        .sum::<f32>();
    let per_char_width_avg = total_width / highlight_char_count as f32;

    // a dark gradient for lower half, behind the text.
    let mut grad = RgbaImage::new(img_width, img_height / 2);
    let start = Rgba::from_slice(&[0, 0, 0, 0]);
    let end = Rgba::from_slice(&[0, 0, 0, 255]);
    imageops::vertical_gradient(&mut grad, start, end);

    // Paint the gradient onto the background buffer
    for (x, y, pixel) in grad.enumerate_pixels() {
        let y_offset = (img_height / 2) as u32;
        let bg_pixel = background_buffer.get_pixel(x, y + y_offset);
        let alpha = pixel[3] as f32 / 255.0;

        let blended_color = [
            (pixel[0] as f32 * alpha + bg_pixel[0] as f32 * (1.0 - alpha)) as u8,
            (pixel[1] as f32 * alpha + bg_pixel[1] as f32 * (1.0 - alpha)) as u8,
            (pixel[2] as f32 * alpha + bg_pixel[2] as f32 * (1.0 - alpha)) as u8,
        ];

        background_buffer.put_pixel(x, y + y_offset, Rgb(blended_color));
    }

    // Drawing bounding boxes for each highlighted word.
    for (x, y, width, height) in word_positions {
        let x = x as u32 + bounds_primary.0 as u32;
        let y = y as u32 + bounds_primary.1 as u32;
        let width = width as u32;
        let height = height as u32;

        let shifted_x = (x as f32 - per_char_width_avg / 3.0).ceil() as u32;
        let shifted_y =
            y + (metrics_primary.font_size - metrics_primary.line_height).abs() as u32 / 2;
        let new_width = width + per_char_width_avg.ceil() as u32 / 3;

        for i in shifted_x..background_buffer.width().min(shifted_x + new_width) {
            for j in shifted_y..background_buffer.height().min(shifted_y + height) {
                // if let Some(bg_pixel) = background_buffer.get_pixel_checked(i, j) {
                if let Some(_) = background_buffer.get_pixel_checked(i, j) {
                    // let bg_color = [bg_pixel[0], bg_pixel[1], bg_pixel[2]];

                    // Alpha blending with actual background color
                    // let blended_color = [
                    //     ((highlight_color.r() as f32 * 0.5) + (bg_color[0] as f32 * 0.5)) as u8,
                    //     ((highlight_color.g() as f32 * 0.5) + (bg_color[1] as f32 * 0.5)) as u8,
                    //     ((highlight_color.b() as f32 * 0.5) + (bg_color[2] as f32 * 0.5)) as u8,
                    // ];
                    let blended_color = [
                        highlight_color.r(),
                        highlight_color.g(),
                        highlight_color.b(),
                    ];

                    background_buffer.put_pixel(i, j, Rgb(blended_color));
                }
            }
        }
    }

    // Painting primary headline onto main background buffer.
    for (x, y, pixel) in primary_headline_img.enumerate_pixels() {
        if pixel[3] > 0 {
            let alpha = pixel[3] as f32 / 255.0;
            let text_color = [pixel[0], pixel[1], pixel[2]];
            let bg_pixel = background_buffer.get_pixel(x, y);
            let bg_color = [bg_pixel[0], bg_pixel[1], bg_pixel[2]];

            // Improved alpha blending with actual background color
            let blended_color = [
                (text_color[0] as f32 * alpha + bg_color[0] as f32 * (1.0 - alpha)) as u8,
                (text_color[1] as f32 * alpha + bg_color[1] as f32 * (1.0 - alpha)) as u8,
                (text_color[2] as f32 * alpha + bg_color[2] as f32 * (1.0 - alpha)) as u8,
            ];

            background_buffer.put_pixel(x, y, Rgb(blended_color));
        }
    }

    // Painting secondary headline onto main background buffer.
    for (x, y, pixel) in secondary_headline_img.enumerate_pixels() {
        if pixel[3] > 0 {
            let alpha = pixel[3] as f32 / 255.0;
            let text_color = [pixel[0], pixel[1], pixel[2]];
            let bg_pixel = background_buffer.get_pixel(x, y);
            let bg_color = [bg_pixel[0], bg_pixel[1], bg_pixel[2]];

            // Improved alpha blending with actual background color
            let blended_color = [
                (text_color[0] as f32 * alpha + bg_color[0] as f32 * (1.0 - alpha)) as u8,
                (text_color[1] as f32 * alpha + bg_color[1] as f32 * (1.0 - alpha)) as u8,
                (text_color[2] as f32 * alpha + bg_color[2] as f32 * (1.0 - alpha)) as u8,
            ];

            background_buffer.put_pixel(x, y, Rgb(blended_color));
        }
    }

    Ok(())
}
