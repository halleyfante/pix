use std::fs;
use std::io::Cursor;

use image::codecs::gif::{GifEncoder, Repeat};
use image::{Frame as GifFrame, ImageBuffer, ImageFormat, Rgba, RgbaImage};

use crate::evaluator::instruction::Instruction;
use crate::parser::ast::Format;
use crate::renderer::render::{Frame, Grid, NamedLayer, RenderResult};

#[derive(Debug, PartialEq)]
pub struct ExportError {
    pub message: String,
}

pub struct Exporter;

impl Exporter {
    pub fn export(result: &RenderResult, instructions: &[Instruction]) -> Result<(), ExportError> {
        Self::export_instructions(instructions, &result.grid, &result.frames, &result.layers)?;
        Ok(())
    }

    fn export_instructions(
        instructions: &[Instruction],
        grid: &Grid,
        frames: &[Frame],
        layers: &[NamedLayer],
    ) -> Result<(), ExportError> {
        for instruction in instructions {
            match instruction {
                Instruction::Export {
                    filename,
                    format,
                    scale,
                } => {
                    let scale_factor = scale.unwrap_or(1);
                    match format {
                        Format::Png => Self::export_raster(grid, filename, scale_factor, ImageFormat::Png, "png")?,
                        Format::Webp => Self::export_raster(grid, filename, scale_factor, ImageFormat::WebP, "webp")?,
                        Format::Svg => Self::export_svg(grid, filename, scale_factor, layers)?,
                        Format::Gif => Self::export_gif(frames, filename, scale_factor)?,
                    }
                }
                Instruction::Layer { instructions, .. } => {
                    // Find the layer grid for exports inside this layer
                    let layer_name = match instruction {
                        Instruction::Layer { name, .. } => name,
                        _ => unreachable!(),
                    };
                    if let Some(named_layer) = layers.iter().find(|l| &l.name == layer_name) {
                        Self::export_instructions(instructions, &named_layer.grid, frames, layers)?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn grid_to_image(grid: &Grid, scale: u32) -> RgbaImage {
        let scaled_width = grid.width * scale;
        let scaled_height = grid.height * scale;

        ImageBuffer::from_fn(scaled_width, scaled_height, |x, y| {
            let grid_x = (x / scale) as usize;
            let grid_y = (y / scale) as usize;

            match &grid.pixels[grid_y][grid_x] {
                Some(color) => Rgba([color.red, color.green, color.blue, color.alpha]),
                None => Rgba([0, 0, 0, 0]),
            }
        })
    }

    fn export_raster(grid: &Grid, filename: &str, scale: u32, format: ImageFormat, extension: &str) -> Result<(), ExportError> {
        let image = Self::grid_to_image(grid, scale);

        let output_filename = format!("{}.{}", filename, extension);
        let mut buffer = Cursor::new(Vec::new());
        image.write_to(&mut buffer, format).map_err(|error| ExportError {
            message: format!("failed to encode '{}': {}", output_filename, error),
        })?;
        fs::write(&output_filename, buffer.into_inner()).map_err(|error| ExportError {
            message: format!("failed to save '{}': {}", output_filename, error),
        })?;

        Ok(())
    }

    fn export_gif(frames: &[Frame], filename: &str, scale: u32) -> Result<(), ExportError> {
        if frames.is_empty() {
            return Err(ExportError {
                message: "no frames to export as gif".to_string(),
            });
        }

        let output_filename = format!("{}.gif", filename);
        let mut buffer = Cursor::new(Vec::new());

        {
            let mut encoder = GifEncoder::new_with_speed(&mut buffer, 10);
            encoder.set_repeat(Repeat::Infinite).map_err(|error| ExportError {
                message: format!("failed to configure gif: {}", error),
            })?;

            for frame in frames {
                let image = Self::grid_to_image(&frame.grid, scale);
                let delay = image::Delay::from_numer_denom_ms(frame.delay, 1);
                let gif_frame = GifFrame::from_parts(image, 0, 0, delay);
                encoder.encode_frame(gif_frame).map_err(|error| ExportError {
                    message: format!("failed to encode gif frame: {}", error),
                })?;
            }
        }

        fs::write(&output_filename, buffer.into_inner()).map_err(|error| ExportError {
            message: format!("failed to save '{}': {}", output_filename, error),
        })?;

        Ok(())
    }

    fn export_svg(grid: &Grid, filename: &str, scale: u32, layers: &[NamedLayer]) -> Result<(), ExportError> {
        let pixel_size = scale.max(1);
        let width = grid.width * pixel_size;
        let height = grid.height * pixel_size;

        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" shape-rendering=\"crispEdges\">\n",
            width, height
        );

        if layers.is_empty() {
            Self::write_svg_pixels(&mut svg, grid, pixel_size);
        } else {
            for layer in layers {
                svg.push_str(&format!("  <g id=\"{}\">\n", layer.name));
                Self::write_svg_pixels_indented(&mut svg, &layer.grid, pixel_size, "    ");
                svg.push_str("  </g>\n");
            }
        }

        svg.push_str("</svg>\n");

        let output_filename = format!("{}.svg", filename);
        fs::write(&output_filename, &svg).map_err(|error| ExportError {
            message: format!("failed to save '{}': {}", output_filename, error),
        })?;

        Ok(())
    }

    fn write_svg_pixels(svg: &mut String, grid: &Grid, pixel_size: u32) {
        Self::write_svg_pixels_indented(svg, grid, pixel_size, "  ");
    }

    fn write_svg_pixels_indented(svg: &mut String, grid: &Grid, pixel_size: u32, indent: &str) {
        for y in 0..grid.height {
            for x in 0..grid.width {
                if let Some(color) = &grid.pixels[y as usize][x as usize] {
                    if color.alpha == 255 {
                        svg.push_str(&format!(
                            "{}<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{:02x}{:02x}{:02x}\"/>\n",
                            indent, x * pixel_size, y * pixel_size, pixel_size, pixel_size,
                            color.red, color.green, color.blue
                        ));
                    } else {
                        svg.push_str(&format!(
                            "{}<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{:.3}\"/>\n",
                            indent, x * pixel_size, y * pixel_size, pixel_size, pixel_size,
                            color.red, color.green, color.blue,
                            color.alpha as f64 / 255.0
                        ));
                    }
                }
            }
        }
    }
}
