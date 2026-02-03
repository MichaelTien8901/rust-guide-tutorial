//! UEFI Graphics Example
//!
//! Demonstrates UEFI Graphics Output Protocol (GOP) patterns.
//!
//! # GOP Architecture
//! ```text
//!     ┌─────────────────────────────────────────────────────────────┐
//!     │              Graphics Output Protocol (GOP)                 │
//!     ├─────────────────────────────────────────────────────────────┤
//!     │                                                             │
//!     │  Application                                                │
//!     │       │                                                     │
//!     │       ▼                                                     │
//!     │  GOP Protocol                                               │
//!     │    ├── QueryMode() - Get mode information                   │
//!     │    ├── SetMode() - Change display mode                      │
//!     │    ├── Blt() - Block transfer operations                    │
//!     │    └── Mode->FrameBufferBase - Direct framebuffer access    │
//!     │       │                                                     │
//!     │       ▼                                                     │
//!     │  Framebuffer (Linear memory)                                │
//!     │                                                             │
//!     └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! Note: This example demonstrates concepts in a std environment.
//! Real UEFI graphics requires the uefi crate and no_std.

use std::fmt;

fn main() {
    println!("=== UEFI Graphics Concepts ===\n");

    println!("--- Pixel Formats ---");
    pixel_formats();

    println!("\n--- Graphics Modes ---");
    graphics_modes();

    println!("\n--- Framebuffer Operations ---");
    framebuffer_operations();

    println!("\n--- Drawing Primitives ---");
    drawing_primitives();

    println!("\n--- BLT Operations ---");
    blt_operations();

    println!("\n--- Double Buffering ---");
    double_buffering();
}

// ============================================
// Pixel Formats
// ============================================

/// UEFI pixel format types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PixelFormat {
    /// Red-Green-Blue-Reserved 8-bit per color
    RedGreenBlueReserved8BitPerColor,
    /// Blue-Green-Red-Reserved 8-bit per color
    BlueGreenRedReserved8BitPerColor,
    /// Pixel format defined by bitmask
    BitMask,
    /// Only valid for Blt() operations
    BltOnly,
}

impl fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PixelFormat::RedGreenBlueReserved8BitPerColor => write!(f, "RGBX (32-bit)"),
            PixelFormat::BlueGreenRedReserved8BitPerColor => write!(f, "BGRX (32-bit)"),
            PixelFormat::BitMask => write!(f, "BitMask"),
            PixelFormat::BltOnly => write!(f, "BltOnly"),
        }
    }
}

/// Pixel bitmask for custom formats
#[derive(Debug, Clone, Copy)]
struct PixelBitmask {
    red_mask: u32,
    green_mask: u32,
    blue_mask: u32,
    reserved_mask: u32,
}

impl PixelBitmask {
    /// Standard RGB888 mask
    fn rgb888() -> Self {
        PixelBitmask {
            red_mask: 0x00FF0000,
            green_mask: 0x0000FF00,
            blue_mask: 0x000000FF,
            reserved_mask: 0xFF000000,
        }
    }

    /// Standard BGR888 mask
    fn bgr888() -> Self {
        PixelBitmask {
            red_mask: 0x000000FF,
            green_mask: 0x0000FF00,
            blue_mask: 0x00FF0000,
            reserved_mask: 0xFF000000,
        }
    }
}

/// Color representation
#[derive(Debug, Clone, Copy, Default)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
    reserved: u8,
}

impl Color {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Color {
            red,
            green,
            blue,
            reserved: 0,
        }
    }

    fn white() -> Self {
        Color::new(255, 255, 255)
    }

    fn black() -> Self {
        Color::new(0, 0, 0)
    }

    fn red() -> Self {
        Color::new(255, 0, 0)
    }

    fn green() -> Self {
        Color::new(0, 255, 0)
    }

    fn blue() -> Self {
        Color::new(0, 0, 255)
    }

    /// Convert to 32-bit pixel value based on format
    fn to_pixel(&self, format: PixelFormat) -> u32 {
        match format {
            PixelFormat::RedGreenBlueReserved8BitPerColor => {
                ((self.red as u32) << 0)
                    | ((self.green as u32) << 8)
                    | ((self.blue as u32) << 16)
                    | ((self.reserved as u32) << 24)
            }
            PixelFormat::BlueGreenRedReserved8BitPerColor => {
                ((self.blue as u32) << 0)
                    | ((self.green as u32) << 8)
                    | ((self.red as u32) << 16)
                    | ((self.reserved as u32) << 24)
            }
            _ => 0,
        }
    }

    /// Create from 32-bit pixel value
    fn from_pixel(pixel: u32, format: PixelFormat) -> Self {
        match format {
            PixelFormat::RedGreenBlueReserved8BitPerColor => Color {
                red: (pixel & 0xFF) as u8,
                green: ((pixel >> 8) & 0xFF) as u8,
                blue: ((pixel >> 16) & 0xFF) as u8,
                reserved: ((pixel >> 24) & 0xFF) as u8,
            },
            PixelFormat::BlueGreenRedReserved8BitPerColor => Color {
                blue: (pixel & 0xFF) as u8,
                green: ((pixel >> 8) & 0xFF) as u8,
                red: ((pixel >> 16) & 0xFF) as u8,
                reserved: ((pixel >> 24) & 0xFF) as u8,
            },
            _ => Color::black(),
        }
    }
}

fn pixel_formats() {
    println!("  Supported UEFI pixel formats:");
    println!("    - {}", PixelFormat::RedGreenBlueReserved8BitPerColor);
    println!("    - {}", PixelFormat::BlueGreenRedReserved8BitPerColor);
    println!("    - {}", PixelFormat::BitMask);

    let red = Color::red();
    let format_rgb = PixelFormat::RedGreenBlueReserved8BitPerColor;
    let format_bgr = PixelFormat::BlueGreenRedReserved8BitPerColor;

    println!("\n  Color conversion examples:");
    println!("    Red in RGBX: 0x{:08X}", red.to_pixel(format_rgb));
    println!("    Red in BGRX: 0x{:08X}", red.to_pixel(format_bgr));

    let bitmask = PixelBitmask::rgb888();
    println!("\n  RGB888 bitmask:");
    println!("    Red:   0x{:08X}", bitmask.red_mask);
    println!("    Green: 0x{:08X}", bitmask.green_mask);
    println!("    Blue:  0x{:08X}", bitmask.blue_mask);
}

// ============================================
// Graphics Modes
// ============================================

/// Graphics mode information
#[derive(Debug, Clone)]
struct ModeInfo {
    version: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: PixelFormat,
    pixel_bitmask: Option<PixelBitmask>,
    pixels_per_scan_line: u32,
}

impl ModeInfo {
    fn new(width: u32, height: u32, format: PixelFormat) -> Self {
        ModeInfo {
            version: 0,
            horizontal_resolution: width,
            vertical_resolution: height,
            pixel_format: format,
            pixel_bitmask: None,
            pixels_per_scan_line: width,
        }
    }

    fn framebuffer_size(&self) -> usize {
        (self.pixels_per_scan_line * self.vertical_resolution * 4) as usize
    }
}

impl fmt::Display for ModeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}x{} {} (stride: {})",
            self.horizontal_resolution,
            self.vertical_resolution,
            self.pixel_format,
            self.pixels_per_scan_line
        )
    }
}

/// Simulated GOP protocol
struct GraphicsOutput {
    modes: Vec<ModeInfo>,
    current_mode: usize,
    framebuffer: Vec<u32>,
}

impl GraphicsOutput {
    fn new() -> Self {
        let modes = vec![
            ModeInfo::new(640, 480, PixelFormat::BlueGreenRedReserved8BitPerColor),
            ModeInfo::new(800, 600, PixelFormat::BlueGreenRedReserved8BitPerColor),
            ModeInfo::new(1024, 768, PixelFormat::BlueGreenRedReserved8BitPerColor),
            ModeInfo::new(1280, 720, PixelFormat::BlueGreenRedReserved8BitPerColor),
            ModeInfo::new(1920, 1080, PixelFormat::BlueGreenRedReserved8BitPerColor),
        ];

        let default_mode = &modes[0];
        let fb_size =
            (default_mode.pixels_per_scan_line * default_mode.vertical_resolution) as usize;

        GraphicsOutput {
            modes,
            current_mode: 0,
            framebuffer: vec![0; fb_size],
        }
    }

    fn max_mode(&self) -> usize {
        self.modes.len()
    }

    fn query_mode(&self, mode_number: usize) -> Option<&ModeInfo> {
        self.modes.get(mode_number)
    }

    fn set_mode(&mut self, mode_number: usize) -> Result<(), &'static str> {
        if mode_number >= self.modes.len() {
            return Err("Invalid mode number");
        }

        self.current_mode = mode_number;
        let mode = &self.modes[mode_number];
        let fb_size = (mode.pixels_per_scan_line * mode.vertical_resolution) as usize;
        self.framebuffer = vec![0; fb_size];

        Ok(())
    }

    fn current_mode_info(&self) -> &ModeInfo {
        &self.modes[self.current_mode]
    }

    fn framebuffer_base(&self) -> *const u32 {
        self.framebuffer.as_ptr()
    }

    fn framebuffer_mut(&mut self) -> &mut [u32] {
        &mut self.framebuffer
    }
}

fn graphics_modes() {
    let gop = GraphicsOutput::new();

    println!("  Available graphics modes ({} total):", gop.max_mode());
    for i in 0..gop.max_mode() {
        if let Some(mode) = gop.query_mode(i) {
            let current = if i == gop.current_mode { " *" } else { "" };
            println!("    Mode {}: {}{}", i, mode, current);
        }
    }

    println!("\n  Current mode details:");
    let current = gop.current_mode_info();
    println!(
        "    Resolution: {}x{}",
        current.horizontal_resolution, current.vertical_resolution
    );
    println!("    Pixel format: {}", current.pixel_format);
    println!("    Pixels per scan line: {}", current.pixels_per_scan_line);
    println!("    Framebuffer size: {} bytes", current.framebuffer_size());
}

// ============================================
// Framebuffer Operations
// ============================================

fn framebuffer_operations() {
    let mut gop = GraphicsOutput::new();

    // Set mode to 800x600
    gop.set_mode(1).unwrap();
    let mode = gop.current_mode_info().clone();

    println!("  Mode set to: {}", mode);
    println!("  Framebuffer base: {:p}", gop.framebuffer_base());

    // Calculate pixel position
    let x = 100u32;
    let y = 50u32;
    let offset = (y * mode.pixels_per_scan_line + x) as usize;

    println!("\n  Pixel addressing:");
    println!("    Pixel at ({}, {}) = offset {}", x, y, offset);
    println!(
        "    Formula: y * stride + x = {} * {} + {} = {}",
        y, mode.pixels_per_scan_line, x, offset
    );

    // Write a pixel
    let fb = gop.framebuffer_mut();
    let color = Color::red();
    fb[offset] = color.to_pixel(mode.pixel_format);

    println!("\n  Wrote red pixel at ({}, {})", x, y);
    println!("    Pixel value: 0x{:08X}", fb[offset]);

    // Fill a rectangle
    let rect_x = 200u32;
    let rect_y = 100u32;
    let rect_w = 50u32;
    let rect_h = 30u32;
    let fill_color = Color::blue().to_pixel(mode.pixel_format);

    for row in 0..rect_h {
        let row_offset = ((rect_y + row) * mode.pixels_per_scan_line + rect_x) as usize;
        for col in 0..rect_w {
            fb[row_offset + col as usize] = fill_color;
        }
    }

    println!(
        "  Filled rectangle at ({}, {}) size {}x{}",
        rect_x, rect_y, rect_w, rect_h
    );
}

// ============================================
// Drawing Primitives
// ============================================

/// Simple framebuffer wrapper for drawing
struct Canvas {
    width: u32,
    height: u32,
    stride: u32,
    format: PixelFormat,
    pixels: Vec<u32>,
}

impl Canvas {
    fn new(width: u32, height: u32, format: PixelFormat) -> Self {
        Canvas {
            width,
            height,
            stride: width,
            format,
            pixels: vec![0; (width * height) as usize],
        }
    }

    fn clear(&mut self, color: Color) {
        let pixel = color.to_pixel(self.format);
        self.pixels.fill(pixel);
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            let offset = (y * self.stride + x) as usize;
            self.pixels[offset] = color.to_pixel(self.format);
        }
    }

    fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        if x < self.width && y < self.height {
            let offset = (y * self.stride + x) as usize;
            Some(Color::from_pixel(self.pixels[offset], self.format))
        } else {
            None
        }
    }

    fn draw_hline(&mut self, x: u32, y: u32, length: u32, color: Color) {
        let pixel = color.to_pixel(self.format);
        for i in 0..length {
            if x + i < self.width && y < self.height {
                let offset = (y * self.stride + x + i) as usize;
                self.pixels[offset] = pixel;
            }
        }
    }

    fn draw_vline(&mut self, x: u32, y: u32, length: u32, color: Color) {
        let pixel = color.to_pixel(self.format);
        for i in 0..length {
            if x < self.width && y + i < self.height {
                let offset = ((y + i) * self.stride + x) as usize;
                self.pixels[offset] = pixel;
            }
        }
    }

    fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: Color) {
        self.draw_hline(x, y, width, color);
        self.draw_hline(x, y + height - 1, width, color);
        self.draw_vline(x, y, height, color);
        self.draw_vline(x + width - 1, y, height, color);
    }

    fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: Color) {
        let pixel = color.to_pixel(self.format);
        for row in 0..height {
            if y + row >= self.height {
                break;
            }
            let row_offset = ((y + row) * self.stride + x) as usize;
            for col in 0..width {
                if x + col >= self.width {
                    break;
                }
                self.pixels[row_offset + col as usize] = pixel;
            }
        }
    }

    /// Count non-zero pixels (for verification)
    fn count_filled_pixels(&self) -> usize {
        self.pixels.iter().filter(|&&p| p != 0).count()
    }
}

fn drawing_primitives() {
    let mut canvas = Canvas::new(320, 200, PixelFormat::BlueGreenRedReserved8BitPerColor);

    // Clear to black
    canvas.clear(Color::black());
    println!("  Created {}x{} canvas", canvas.width, canvas.height);

    // Draw some primitives
    canvas.fill_rect(10, 10, 100, 80, Color::blue());
    println!("  Drew filled blue rectangle at (10, 10) size 100x80");

    canvas.draw_rect(50, 50, 80, 60, Color::red());
    println!("  Drew red rectangle outline at (50, 50) size 80x60");

    canvas.draw_hline(0, 100, 320, Color::green());
    println!("  Drew green horizontal line at y=100");

    canvas.draw_vline(160, 0, 200, Color::white());
    println!("  Drew white vertical line at x=160");

    // Set individual pixels
    for i in 0..10 {
        canvas.set_pixel(200 + i * 5, 150, Color::new(255, 255, 0));
    }
    println!("  Drew 10 yellow pixels");

    let filled = canvas.count_filled_pixels();
    println!("\n  Total filled pixels: {}", filled);
}

// ============================================
// BLT Operations
// ============================================

/// BLT operation types (Block Transfer)
#[derive(Debug, Clone, Copy)]
enum BltOperation {
    /// Write data from buffer to video
    VideoFill,
    /// Copy from buffer to video
    BufferToVideo,
    /// Copy from video to buffer
    VideoToBuffer,
    /// Copy within video memory
    VideoToVideo,
}

/// BLT pixel (BGRX format)
#[derive(Debug, Clone, Copy, Default)]
struct BltPixel {
    blue: u8,
    green: u8,
    red: u8,
    reserved: u8,
}

impl BltPixel {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        BltPixel {
            blue,
            green,
            red,
            reserved: 0,
        }
    }

    fn from_color(color: Color) -> Self {
        BltPixel::new(color.red, color.green, color.blue)
    }
}

fn blt_operations() {
    println!("  BLT (Block Transfer) Operations:");
    println!("    - VideoFill: Fill area with single color");
    println!("    - BufferToVideo: Copy buffer to screen");
    println!("    - VideoToBuffer: Copy screen to buffer");
    println!("    - VideoToVideo: Copy within screen");

    // Simulate a framebuffer
    let width = 100u32;
    let height = 100u32;
    let mut framebuffer = vec![BltPixel::default(); (width * height) as usize];

    // VideoFill - fill a rectangle
    println!("\n  Simulating VideoFill:");
    let fill_color = BltPixel::new(0, 128, 255); // Orange
    let dest_x = 10u32;
    let dest_y = 10u32;
    let fill_width = 30u32;
    let fill_height = 20u32;

    for row in 0..fill_height {
        let offset = ((dest_y + row) * width + dest_x) as usize;
        for col in 0..fill_width {
            framebuffer[offset + col as usize] = fill_color;
        }
    }
    println!(
        "    Filled {}x{} area at ({}, {}) with orange",
        fill_width, fill_height, dest_x, dest_y
    );

    // BufferToVideo - copy from source buffer
    println!("\n  Simulating BufferToVideo:");
    let src_buffer: Vec<BltPixel> = (0..16)
        .map(|i| BltPixel::new((i * 16) as u8, 0, 0))
        .collect();

    let dest_x = 50u32;
    let dest_y = 50u32;
    for (i, pixel) in src_buffer.iter().enumerate() {
        let x = dest_x + (i % 4) as u32;
        let y = dest_y + (i / 4) as u32;
        let offset = (y * width + x) as usize;
        framebuffer[offset] = *pixel;
    }
    println!(
        "    Copied 4x4 gradient pattern to ({}, {})",
        dest_x, dest_y
    );

    // VideoToVideo - copy within framebuffer
    println!("\n  Simulating VideoToVideo:");
    let src_x = 10u32;
    let src_y = 10u32;
    let copy_width = 15u32;
    let copy_height = 10u32;
    let new_dest_x = 60u32;
    let new_dest_y = 20u32;

    // Copy to temp buffer first (avoid overlap issues)
    let mut temp = vec![BltPixel::default(); (copy_width * copy_height) as usize];
    for row in 0..copy_height {
        let src_offset = ((src_y + row) * width + src_x) as usize;
        let temp_offset = (row * copy_width) as usize;
        for col in 0..copy_width {
            temp[temp_offset + col as usize] = framebuffer[src_offset + col as usize];
        }
    }

    // Copy from temp to destination
    for row in 0..copy_height {
        let dest_offset = ((new_dest_y + row) * width + new_dest_x) as usize;
        let temp_offset = (row * copy_width) as usize;
        for col in 0..copy_width {
            framebuffer[dest_offset + col as usize] = temp[temp_offset + col as usize];
        }
    }
    println!(
        "    Copied {}x{} from ({}, {}) to ({}, {})",
        copy_width, copy_height, src_x, src_y, new_dest_x, new_dest_y
    );

    // Count non-black pixels
    let filled = framebuffer
        .iter()
        .filter(|p| p.red != 0 || p.green != 0 || p.blue != 0)
        .count();
    println!("\n  Total non-black pixels: {}", filled);
}

// ============================================
// Double Buffering
// ============================================

/// Double buffer for flicker-free rendering
struct DoubleBuffer {
    front: Vec<u32>,
    back: Vec<u32>,
    width: u32,
    height: u32,
}

impl DoubleBuffer {
    fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        DoubleBuffer {
            front: vec![0; size],
            back: vec![0; size],
            width,
            height,
        }
    }

    /// Get the back buffer for drawing
    fn back_buffer(&mut self) -> &mut [u32] {
        &mut self.back
    }

    /// Swap front and back buffers
    fn swap(&mut self) {
        std::mem::swap(&mut self.front, &mut self.back);
    }

    /// Get the front buffer (for display)
    fn front_buffer(&self) -> &[u32] {
        &self.front
    }

    /// Clear the back buffer
    fn clear_back(&mut self, color: u32) {
        self.back.fill(color);
    }
}

fn double_buffering() {
    let mut db = DoubleBuffer::new(320, 200);

    println!("  Double buffering simulation:");
    println!("    Buffer size: {}x{}", db.width, db.height);

    // Frame 1: Draw to back buffer
    db.clear_back(0x00000000); // Black
    {
        let back = db.back_buffer();
        // Draw a simple pattern
        for y in 50..100 {
            for x in 50..150 {
                let offset = y * 320 + x;
                back[offset as usize] = 0x00FF0000; // Red in BGRX
            }
        }
    }
    println!("    Frame 1: Drew red rectangle to back buffer");

    // Swap
    db.swap();
    println!("    Swapped buffers - frame 1 now visible");

    // Frame 2: Draw different content
    db.clear_back(0x00000000);
    {
        let back = db.back_buffer();
        for y in 100..150 {
            for x in 100..200 {
                let offset = y * 320 + x;
                back[offset as usize] = 0x0000FF00; // Green in BGRX
            }
        }
    }
    println!("    Frame 2: Drew green rectangle to back buffer");

    // Check front buffer still has frame 1
    let front_red_count = db
        .front_buffer()
        .iter()
        .filter(|&&p| p == 0x00FF0000)
        .count();
    println!("    Front buffer red pixels (frame 1): {}", front_red_count);

    // Swap again
    db.swap();
    println!("    Swapped buffers - frame 2 now visible");

    let front_green_count = db
        .front_buffer()
        .iter()
        .filter(|&&p| p == 0x0000FF00)
        .count();
    println!(
        "    Front buffer green pixels (frame 2): {}",
        front_green_count
    );

    println!("\n  Benefits of double buffering:");
    println!("    - No screen tearing");
    println!("    - Smooth animations");
    println!("    - Draw complex scenes without visible artifacts");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversion() {
        let red = Color::red();
        let format = PixelFormat::BlueGreenRedReserved8BitPerColor;

        let pixel = red.to_pixel(format);
        let back = Color::from_pixel(pixel, format);

        assert_eq!(back.red, red.red);
        assert_eq!(back.green, red.green);
        assert_eq!(back.blue, red.blue);
    }

    #[test]
    fn test_canvas_operations() {
        let mut canvas = Canvas::new(100, 100, PixelFormat::BlueGreenRedReserved8BitPerColor);

        canvas.set_pixel(50, 50, Color::white());
        let pixel = canvas.get_pixel(50, 50).unwrap();

        assert_eq!(pixel.red, 255);
        assert_eq!(pixel.green, 255);
        assert_eq!(pixel.blue, 255);
    }

    #[test]
    fn test_canvas_bounds() {
        let mut canvas = Canvas::new(100, 100, PixelFormat::BlueGreenRedReserved8BitPerColor);

        // Should not panic for out-of-bounds
        canvas.set_pixel(200, 200, Color::red());
        assert!(canvas.get_pixel(200, 200).is_none());
    }

    #[test]
    fn test_double_buffer_swap() {
        let mut db = DoubleBuffer::new(10, 10);

        db.back_buffer()[0] = 0xDEADBEEF;
        assert_eq!(db.front_buffer()[0], 0);

        db.swap();
        assert_eq!(db.front_buffer()[0], 0xDEADBEEF);
    }
}
