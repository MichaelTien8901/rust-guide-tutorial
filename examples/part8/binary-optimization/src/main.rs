//! # Binary Optimization Example
//!
//! Demonstrates optimization concepts that affect embedded binary size:
//! generic monomorphization cost, format string bloat, section simulation,
//! and a release checklist data structure.
//!
//! Compiles as standard Rust for CI. Illustrates patterns from the chapter.

fn main() {
    println!("=== Binary Optimization ===\n");

    demonstrate_monomorphization_cost();
    demonstrate_format_bloat();
    demonstrate_section_sizes();
    demonstrate_release_checklist();
}

// ---------------------------------------------------------------------------
// Generic Monomorphization Cost
// ---------------------------------------------------------------------------

trait Sensor {
    fn read(&self) -> f32;
    fn name(&self) -> &'static str;
}

struct TemperatureSensor(f32);
struct HumiditySensor(f32);
struct PressureSensor(f32);

impl Sensor for TemperatureSensor {
    fn read(&self) -> f32 { self.0 }
    fn name(&self) -> &'static str { "temperature" }
}

impl Sensor for HumiditySensor {
    fn read(&self) -> f32 { self.0 }
    fn name(&self) -> &'static str { "humidity" }
}

impl Sensor for PressureSensor {
    fn read(&self) -> f32 { self.0 }
    fn name(&self) -> &'static str { "pressure" }
}

/// Generic function: compiled once per type parameter (3 copies here).
fn process_generic<T: Sensor>(sensor: &T) -> f32 {
    let value = sensor.read();
    value * 1.01 + 0.5 // calibration
}

/// Concrete function using trait object: compiled once, small vtable overhead.
fn process_dynamic(sensor: &dyn Sensor) -> f32 {
    let value = sensor.read();
    value * 1.01 + 0.5
}

fn demonstrate_monomorphization_cost() {
    println!("--- Generic Monomorphization Cost ---");

    let temp = TemperatureSensor(23.5);
    let hum = HumiditySensor(65.0);
    let pres = PressureSensor(1013.25);

    // Generic: 3 monomorphized copies of process_generic
    let r1 = process_generic(&temp);
    let r2 = process_generic(&hum);
    let r3 = process_generic(&pres);
    println!("  Generic (3 copies):  temp={:.2}, hum={:.2}, pres={:.2}", r1, r2, r3);

    // Dynamic dispatch: 1 copy of process_dynamic + vtable
    let sensors: Vec<&dyn Sensor> = vec![&temp, &hum, &pres];
    let results: Vec<f32> = sensors.iter().map(|s| process_dynamic(*s)).collect();
    println!("  Dynamic (1 copy):    temp={:.2}, hum={:.2}, pres={:.2}",
             results[0], results[1], results[2]);

    println!("  Tradeoff: generics = faster (inlinable), dynamic = smaller (one copy)");
    println!();
}

// ---------------------------------------------------------------------------
// Format String Bloat Demonstration
// ---------------------------------------------------------------------------

/// Simulates the cost difference between formatted and static panic messages.
fn demonstrate_format_bloat() {
    println!("--- Format String Bloat ---");

    // Formatted string: pulls in core::fmt machinery (~20 KB on embedded)
    let formatted = format!("Temperature {} exceeds max {}", 42, 100);
    let formatted_len = formatted.len();

    // Static string: zero formatting overhead
    let static_msg = "Temperature exceeds maximum";
    let static_len = static_msg.len();

    println!("  Formatted message:  \"{}\" ({} bytes in .rodata)", formatted, formatted_len);
    println!("  Static message:     \"{}\" ({} bytes in .rodata)", static_msg, static_len);
    println!();
    println!("  On embedded targets:");
    println!("    panic!(\"..{{}}..\", val) => ~20 KB added (core::fmt machinery)");
    println!("    panic!(\"static msg\")   => ~0 bytes added (string literal only)");
    println!("    defmt::info!(\"..{{}}..\", val) => ~200 bytes (deferred formatting)");
    println!();
}

// ---------------------------------------------------------------------------
// Binary Section Size Simulation
// ---------------------------------------------------------------------------

/// Represents an ELF section with name, size, and memory region.
struct Section {
    name: &'static str,
    size: u32,
    region: &'static str,
}

fn demonstrate_section_sizes() {
    println!("--- Binary Section Sizes (simulated STM32F769 blinky) ---");

    let sections = [
        Section { name: ".vector_table", size: 1024,  region: "FLASH" },
        Section { name: ".text",         size: 6472,  region: "FLASH" },
        Section { name: ".rodata",       size: 832,   region: "FLASH" },
        Section { name: ".data",         size: 16,    region: "FLASH+RAM" },
        Section { name: ".bss",          size: 268,   region: "RAM" },
        Section { name: ".uninit",       size: 1024,  region: "RAM" },
    ];

    let flash_total: u32 = sections.iter()
        .filter(|s| s.region.contains("FLASH"))
        .map(|s| s.size)
        .sum();

    let ram_total: u32 = sections.iter()
        .filter(|s| s.region.contains("RAM"))
        .map(|s| s.size)
        .sum();

    println!("  {:<20} {:>8}  {}", "Section", "Size", "Region");
    println!("  {:<20} {:>8}  {}", "-------", "----", "------");
    for s in &sections {
        println!("  {:<20} {:>7}B  {}", s.name, s.size, s.region);
    }
    println!("  {:<20} {:>7}B", "Flash total", flash_total);
    println!("  {:<20} {:>7}B", "RAM total", ram_total);

    let flash_capacity: u32 = 2 * 1024 * 1024; // 2 MB
    let usage_pct = (flash_total as f64 / flash_capacity as f64) * 100.0;
    println!("  Flash usage: {:.2}% of {} KB", usage_pct, flash_capacity / 1024);
    println!();
}

// ---------------------------------------------------------------------------
// Release Build Checklist
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct ChecklistItem {
    step: &'static str,
    action: &'static str,
    verified: bool,
}

fn demonstrate_release_checklist() {
    println!("--- Release Build Checklist ---");

    let mut checklist = vec![
        ChecklistItem { step: "Profile",      action: "opt-level=z, lto=fat, codegen-units=1", verified: false },
        ChecklistItem { step: "Panic handler", action: "Switch to panic-halt or panic-abort",   verified: false },
        ChecklistItem { step: "Dep audit",     action: "cargo bloat --release --crates",        verified: false },
        ChecklistItem { step: "Features",      action: "Disable default-features, enable minimal", verified: false },
        ChecklistItem { step: "Size check",    action: "cargo size --release -- -A",            verified: false },
        ChecklistItem { step: "Flash fit",     action: "Verify total < flash capacity",         verified: false },
    ];

    // Simulate running through the checklist
    for item in checklist.iter_mut() {
        item.verified = true;
    }

    for item in &checklist {
        let mark = if item.verified { "[x]" } else { "[ ]" };
        println!("  {} {:<16} {}", mark, item.step, item.action);
    }

    let all_passed = checklist.iter().all(|item| item.verified);
    println!();
    if all_passed {
        println!("  All checks passed. Binary is ready for flashing.");
    } else {
        println!("  Some checks failed. Review before flashing.");
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_and_dynamic_produce_same_results() {
        let sensor = TemperatureSensor(25.0);
        let generic_result = process_generic(&sensor);
        let dynamic_result = process_dynamic(&sensor);
        assert!((generic_result - dynamic_result).abs() < f32::EPSILON);
    }

    #[test]
    fn test_sensor_implementations() {
        let temp = TemperatureSensor(20.0);
        let hum = HumiditySensor(50.0);
        let pres = PressureSensor(1013.0);
        assert_eq!(temp.read(), 20.0);
        assert_eq!(hum.read(), 50.0);
        assert_eq!(pres.read(), 1013.0);
        assert_eq!(temp.name(), "temperature");
        assert_eq!(hum.name(), "humidity");
        assert_eq!(pres.name(), "pressure");
    }

    #[test]
    fn test_section_flash_total() {
        // .vector_table(1024) + .text(6472) + .rodata(832) + .data(16) = 8344
        let flash_total: u32 = [1024, 6472, 832, 16].iter().sum();
        assert_eq!(flash_total, 8344);
    }

    #[test]
    fn test_flash_capacity_sufficient() {
        let flash_used: u32 = 8344;
        let flash_capacity: u32 = 2 * 1024 * 1024;
        assert!(flash_used < flash_capacity, "Binary exceeds flash capacity");
    }

    #[test]
    fn test_checklist_all_verified() {
        let mut checklist = vec![
            ChecklistItem { step: "Profile", action: "opt-level=z", verified: false },
            ChecklistItem { step: "Size",    action: "cargo size",  verified: false },
        ];
        for item in checklist.iter_mut() {
            item.verified = true;
        }
        assert!(checklist.iter().all(|item| item.verified));
    }

    #[test]
    fn test_calibration_formula() {
        // process_generic applies: value * 1.01 + 0.5
        let sensor = TemperatureSensor(100.0);
        let result = process_generic(&sensor);
        let expected = 100.0_f32 * 1.01 + 0.5;
        assert!((result - expected).abs() < f32::EPSILON);
    }
}
