use crate::color::palette::Color;
use crate::evaluator::instruction::Instruction;
use crate::exporter::export::Exporter;
use crate::parser::ast::Format;
use crate::renderer::render::{Frame, Grid, RenderResult};

fn test_grid() -> Grid {
    Grid {
        width: 2,
        height: 2,
        pixels: vec![
            vec![
                Some(Color { red: 255, green: 0, blue: 0, alpha: 255 }),
                None,
            ],
            vec![
                None,
                Some(Color { red: 0, green: 255, blue: 0, alpha: 255 }),
            ],
        ],
    }
}

fn test_result(grid: Grid) -> RenderResult {
    RenderResult {
        grid,
        frames: Vec::new(),
        layers: Vec::new(),
        has_frames: false,
        has_animated_export: false,
    }
}

#[test]
fn export_creates_png_file() {
    let result = test_result(test_grid());
    let instructions = vec![Instruction::Export {
        filename: "test_output".to_string(),
        format: Format::Png,
        scale: None,
    }];

    Exporter::export(&result, &instructions).unwrap();

    let path = std::path::Path::new("test_output.png");
    assert!(path.exists());
    std::fs::remove_file(path).unwrap();
}

#[test]
fn export_with_scale() {
    let result = test_result(test_grid());
    let instructions = vec![Instruction::Export {
        filename: "test_output_scaled".to_string(),
        format: Format::Png,
        scale: Some(4),
    }];

    Exporter::export(&result, &instructions).unwrap();

    let path = std::path::Path::new("test_output_scaled.png");
    assert!(path.exists());

    let image = image::open(path).unwrap();
    assert_eq!(image.width(), 8);
    assert_eq!(image.height(), 8);

    std::fs::remove_file(path).unwrap();
}

#[test]
fn export_creates_svg_file() {
    let result = test_result(test_grid());
    let instructions = vec![Instruction::Export {
        filename: "test_output_svg".to_string(),
        format: Format::Svg,
        scale: None,
    }];

    Exporter::export(&result, &instructions).unwrap();

    let path = std::path::Path::new("test_output_svg.svg");
    assert!(path.exists());

    let content = std::fs::read_to_string(path).unwrap();
    assert!(content.starts_with("<svg"));
    assert!(content.contains("width=\"2\""));
    assert!(content.contains("height=\"2\""));
    assert!(content.contains("ff0000"));
    assert!(content.contains("00ff00"));

    std::fs::remove_file(path).unwrap();
}

#[test]
fn export_svg_with_scale() {
    let result = test_result(test_grid());
    let instructions = vec![Instruction::Export {
        filename: "test_output_svg_scaled".to_string(),
        format: Format::Svg,
        scale: Some(4),
    }];

    Exporter::export(&result, &instructions).unwrap();

    let path = std::path::Path::new("test_output_svg_scaled.svg");
    assert!(path.exists());

    let content = std::fs::read_to_string(path).unwrap();
    assert!(content.contains("width=\"8\""));
    assert!(content.contains("height=\"8\""));

    std::fs::remove_file(path).unwrap();
}

#[test]
fn export_svg_with_transparency() {
    let grid = Grid {
        width: 1,
        height: 1,
        pixels: vec![vec![Some(Color { red: 255, green: 0, blue: 0, alpha: 128 })]],
    };
    let result = test_result(grid);
    let instructions = vec![Instruction::Export {
        filename: "test_output_svg_alpha".to_string(),
        format: Format::Svg,
        scale: None,
    }];

    Exporter::export(&result, &instructions).unwrap();

    let path = std::path::Path::new("test_output_svg_alpha.svg");
    let content = std::fs::read_to_string(path).unwrap();
    assert!(content.contains("fill-opacity"));

    std::fs::remove_file(path).unwrap();
}

#[test]
fn export_creates_webp_file() {
    let result = test_result(test_grid());
    let instructions = vec![Instruction::Export {
        filename: "test_output_webp".to_string(),
        format: Format::Webp,
        scale: None,
    }];

    Exporter::export(&result, &instructions).unwrap();

    let path = std::path::Path::new("test_output_webp.webp");
    assert!(path.exists());
    std::fs::remove_file(path).unwrap();
}

#[test]
fn export_webp_with_scale() {
    let result = test_result(test_grid());
    let instructions = vec![Instruction::Export {
        filename: "test_output_webp_scaled".to_string(),
        format: Format::Webp,
        scale: Some(4),
    }];

    Exporter::export(&result, &instructions).unwrap();

    let path = std::path::Path::new("test_output_webp_scaled.webp");
    assert!(path.exists());

    let image = image::open(path).unwrap();
    assert_eq!(image.width(), 8);
    assert_eq!(image.height(), 8);

    std::fs::remove_file(path).unwrap();
}

#[test]
fn export_creates_gif_file() {
    let grid = test_grid();
    let result = RenderResult {
        grid: grid.clone(),
        frames: vec![
            Frame { grid: grid.clone(), delay: 100 },
            Frame { grid: grid.clone(), delay: 200 },
        ],
        layers: Vec::new(),
        has_frames: true,
        has_animated_export: true,
    };
    let instructions = vec![Instruction::Export {
        filename: "test_output_gif".to_string(),
        format: Format::Gif,
        scale: None,
    }];

    Exporter::export(&result, &instructions).unwrap();

    let path = std::path::Path::new("test_output_gif.gif");
    assert!(path.exists());
    std::fs::remove_file(path).unwrap();
}

#[test]
fn export_gif_no_frames_errors() {
    let result = test_result(test_grid());
    let instructions = vec![Instruction::Export {
        filename: "test_output_gif_empty".to_string(),
        format: Format::Gif,
        scale: None,
    }];

    let error = Exporter::export(&result, &instructions).unwrap_err();
    assert_eq!(error.message, "no frames to export as gif");
}
