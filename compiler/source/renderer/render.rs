use crate::color::palette::{build_palette, resolve_color, Color, ColorError, Palette};
use crate::evaluator::evaluate::Evaluator;
use crate::evaluator::instruction::{EvaluatedProgram, Instruction, Value};
use crate::parser::ast::Point;

#[derive(Debug, PartialEq)]
pub struct RenderError {
    pub message: String,
}

impl From<ColorError> for RenderError {
    fn from(error: ColorError) -> Self {
        RenderError {
            message: error.message,
        }
    }
}

#[derive(Clone)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Option<Color>>>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![None; width as usize]; height as usize],
        }
    }

    fn composite_onto(&self, target: &mut Grid) {
        for y in 0..self.height.min(target.height) {
            for x in 0..self.width.min(target.width) {
                if let Some(color) = &self.pixels[y as usize][x as usize] {
                    target.pixels[y as usize][x as usize] = Some(color.clone());
                }
            }
        }
    }
}

pub struct Frame {
    pub grid: Grid,
    pub delay: u32,
}

pub struct NamedLayer {
    pub name: String,
    pub grid: Grid,
}

pub struct RenderResult {
    pub grid: Grid,
    pub frames: Vec<Frame>,
    pub layers: Vec<NamedLayer>,
    pub has_frames: bool,
    pub has_animated_export: bool,
}

pub struct Renderer;

impl Renderer {
    pub fn render(program: &EvaluatedProgram) -> Result<RenderResult, RenderError> {
        let palette = build_palette(&program.instructions)?;
        let mut grid = Grid::new(program.grid_width, program.grid_height);
        let mut frames: Vec<Frame> = Vec::new();
        let mut layers: Vec<NamedLayer> = Vec::new();
        let mut has_frames = false;
        let has_animated_export = Self::contains_animated_export(&program.instructions);

        Self::execute_instructions(&program.instructions, &mut grid, &palette, &mut frames, &mut layers, &mut has_frames)?;

        Ok(RenderResult {
            grid,
            frames,
            layers,
            has_frames,
            has_animated_export,
        })
    }

    fn contains_animated_export(instructions: &[Instruction]) -> bool {
        for instruction in instructions {
            match instruction {
                Instruction::Export {
                    format: crate::parser::ast::Format::Gif,
                    ..
                } => {
                    return true;
                }
                Instruction::Layer { instructions, .. }
                | Instruction::Mirror { instructions, .. } => {
                    if Self::contains_animated_export(instructions) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn execute_instructions(
        instructions: &[Instruction],
        grid: &mut Grid,
        palette: &Palette,
        frames: &mut Vec<Frame>,
        layers: &mut Vec<NamedLayer>,
        has_frames: &mut bool,
    ) -> Result<(), RenderError> {
        for instruction in instructions {
            match instruction {
                Instruction::Draw { condition, color }
                | Instruction::Pixel { condition, color, .. }
                | Instruction::Line { condition, color, .. }
                | Instruction::Rectangle { condition, color, .. }
                | Instruction::Triangle { condition, color, .. }
                | Instruction::Circle { condition, color, .. } => {
                    let resolved_color = resolve_color(color, palette)?;
                    Self::apply_condition(grid, condition, resolved_color);
                }
                Instruction::Erase { condition } => {
                    Self::erase_condition(grid, condition);
                }
                Instruction::Clear => {
                    for row in &mut grid.pixels {
                        for pixel in row {
                            *pixel = None;
                        }
                    }
                }
                Instruction::Frame { delay } => {
                    *has_frames = true;
                    frames.push(Frame {
                        grid: grid.clone(),
                        delay: *delay,
                    });
                }
                Instruction::Copy { from, to, destination } => {
                    Self::copy_region(grid, from, to, destination, false);
                }
                Instruction::Move { from, to, destination } => {
                    Self::copy_region(grid, from, to, destination, true);
                }
                Instruction::Layer { name, instructions } => {
                    let layer_palette = build_palette(instructions)?;
                    let merged_palette = merge_palettes(palette, &layer_palette);
                    let mut layer_grid = Grid::new(grid.width, grid.height);
                    let mut layer_has_frames = false;
                    Self::execute_instructions(instructions, &mut layer_grid, &merged_palette, frames, layers, &mut layer_has_frames)?;
                    if layer_has_frames {
                        *has_frames = true;
                    }
                    layers.push(NamedLayer {
                        name: name.clone(),
                        grid: layer_grid.clone(),
                    });
                    layer_grid.composite_onto(grid);
                }
                Instruction::Mirror { from, to, instructions } => {
                    let mut mirror_grid = Grid::new(grid.width, grid.height);
                    let mut mirror_has_frames = false;
                    Self::execute_instructions(instructions, &mut mirror_grid, palette, frames, layers, &mut mirror_has_frames)?;
                    if mirror_has_frames {
                        *has_frames = true;
                    }
                    // Draw original
                    mirror_grid.composite_onto(grid);
                    // Draw reflected
                    Self::apply_mirror(grid, &mirror_grid, from, to);
                }
                Instruction::Export { .. } | Instruction::ColorBlock { .. } => {}
            }
        }
        Ok(())
    }

    fn apply_condition(
        grid: &mut Grid,
        condition: &crate::parser::ast::Expression,
        color: Color,
    ) {
        for y in 0..grid.height {
            for x in 0..grid.width {
                if let Ok(Value::Boolean(true)) =
                    Evaluator::evaluate_expression(condition, x as i64, y as i64)
                {
                    grid.pixels[y as usize][x as usize] = Some(color.clone());
                }
            }
        }
    }

    fn erase_condition(grid: &mut Grid, condition: &crate::parser::ast::Expression) {
        for y in 0..grid.height {
            for x in 0..grid.width {
                if let Ok(Value::Boolean(true)) =
                    Evaluator::evaluate_expression(condition, x as i64, y as i64)
                {
                    grid.pixels[y as usize][x as usize] = None;
                }
            }
        }
    }

    fn copy_region(grid: &mut Grid, from: &Point, to: &Point, destination: &Point, erase_source: bool) {
        let min_x = from.x.min(to.x);
        let max_x = from.x.max(to.x);
        let min_y = from.y.min(to.y);
        let max_y = from.y.max(to.y);

        let region_width = max_x - min_x + 1;
        let region_height = max_y - min_y + 1;

        // Copy pixels to a buffer first
        let mut buffer: Vec<Vec<Option<Color>>> = Vec::new();
        for y in min_y..=max_y {
            let mut row = Vec::new();
            for x in min_x..=max_x {
                if (y as usize) < grid.pixels.len() && (x as usize) < grid.pixels[0].len() {
                    row.push(grid.pixels[y as usize][x as usize].clone());
                } else {
                    row.push(None);
                }
            }
            buffer.push(row);
        }

        // Erase source if move
        if erase_source {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if (y as usize) < grid.pixels.len() && (x as usize) < grid.pixels[0].len() {
                        grid.pixels[y as usize][x as usize] = None;
                    }
                }
            }
        }

        // Paste at destination
        for dy in 0..region_height {
            for dx in 0..region_width {
                let dest_x = destination.x + dx;
                let dest_y = destination.y + dy;
                if (dest_y as usize) < grid.pixels.len() && (dest_x as usize) < grid.pixels[0].len()
                    && let Some(color) = &buffer[dy as usize][dx as usize] {
                    grid.pixels[dest_y as usize][dest_x as usize] = Some(color.clone());
                }
            }
        }
    }

    fn apply_mirror(grid: &mut Grid, source: &Grid, axis_from: &Point, axis_to: &Point) {
        let ax = axis_from.x as f64;
        let ay = axis_from.y as f64;
        let bx = axis_to.x as f64;
        let by = axis_to.y as f64;

        let dx = bx - ax;
        let dy = by - ay;
        let length_squared = dx * dx + dy * dy;

        if length_squared == 0.0 {
            return;
        }

        for y in 0..source.height {
            for x in 0..source.width {
                if source.pixels[y as usize][x as usize].is_some() {
                    let px = x as f64;
                    let py = y as f64;

                    // Reflect point across the line
                    let vx = px - ax;
                    let vy = py - ay;
                    let dot = vx * dx + vy * dy;
                    let projection = dot / length_squared;

                    let reflected_x = 2.0 * (ax + projection * dx) - px;
                    let reflected_y = 2.0 * (ay + projection * dy) - py;

                    let rx = reflected_x.round() as i64;
                    let ry = reflected_y.round() as i64;

                    if rx >= 0 && ry >= 0 && (rx as u32) < grid.width && (ry as u32) < grid.height {
                        grid.pixels[ry as usize][rx as usize] =
                            source.pixels[y as usize][x as usize].clone();
                    }
                }
            }
        }
    }
}

fn merge_palettes(base: &Palette, overlay: &Palette) -> Palette {
    let mut merged = base.clone();
    for (name, color) in overlay {
        merged.insert(name.clone(), color.clone());
    }
    merged
}
