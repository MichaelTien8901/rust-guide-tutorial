//! UEFI Programming Concepts (Reference Example)
//!
//! Demonstrates UEFI patterns and structures.
//!
//! # UEFI Boot Flow
//! ```text
//!     ┌─────────────────────────────────────────────────────────┐
//!     │                    UEFI Boot Flow                       │
//!     └─────────────────────────────────────────────────────────┘
//!                            │
//!                            ▼
//!     ┌─────────────────────────────────────────────────────────┐
//!     │  1. Firmware Initialization                             │
//!     │     - Hardware initialization                           │
//!     │     - Memory detection                                  │
//!     └─────────────────────────────────────────────────────────┘
//!                            │
//!                            ▼
//!     ┌─────────────────────────────────────────────────────────┐
//!     │  2. Boot Services                                       │
//!     │     - Memory allocation                                 │
//!     │     - Protocol handling                                 │
//!     │     - Event services                                    │
//!     └─────────────────────────────────────────────────────────┘
//!                            │
//!                            ▼
//!     ┌─────────────────────────────────────────────────────────┐
//!     │  3. UEFI Application (Bootloader/Your Code)             │
//!     │     - Access Boot Services                              │
//!     │     - Load OS kernel                                    │
//!     │     - ExitBootServices()                                │
//!     └─────────────────────────────────────────────────────────┘
//!                            │
//!                            ▼
//!     ┌─────────────────────────────────────────────────────────┐
//!     │  4. Runtime Services (After ExitBootServices)           │
//!     │     - Time services                                     │
//!     │     - Variable services                                 │
//!     │     - Reset system                                      │
//!     └─────────────────────────────────────────────────────────┘
//! ```
//!
//! Note: This example shows UEFI concepts but compiles as
//! a standard binary for learning purposes.

use std::collections::HashMap;

fn main() {
    println!("=== UEFI Programming Concepts ===\n");

    println!("--- UEFI Types and Structures ---");
    uefi_types();

    println!("\n--- UEFI Status Codes ---");
    uefi_status_codes();

    println!("\n--- Protocol Pattern ---");
    protocol_pattern();

    println!("\n--- Memory Map Simulation ---");
    memory_map_simulation();

    println!("\n--- UEFI Variables ---");
    uefi_variables();

    println!("\n--- Boot Services Pattern ---");
    boot_services_pattern();
}

// ============================================
// UEFI Types and Structures
// ============================================

/// UEFI uses specific integer types
type UefiUint8 = u8;
type UefiUint16 = u16;
type UefiUint32 = u32;
type UefiUint64 = u64;
type UefiUintn = usize; // Platform-dependent size

/// UEFI Handle - opaque pointer to a protocol collection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Handle(usize);

/// GUID - Globally Unique Identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl Guid {
    const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Guid {
            data1,
            data2,
            data3,
            data4,
        }
    }
}

// Well-known GUIDs
const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID: Guid = Guid::new(
    0x387477c2,
    0x69c7,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: Guid = Guid::new(
    0x9042a9de,
    0x23dc,
    0x4a38,
    [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
);

fn uefi_types() {
    println!("  UEFI Integer Types:");
    println!("    UINT8:  {} bytes", std::mem::size_of::<UefiUint8>());
    println!("    UINT16: {} bytes", std::mem::size_of::<UefiUint16>());
    println!("    UINT32: {} bytes", std::mem::size_of::<UefiUint32>());
    println!("    UINT64: {} bytes", std::mem::size_of::<UefiUint64>());
    println!(
        "    UINTN:  {} bytes (platform-dependent)",
        std::mem::size_of::<UefiUintn>()
    );

    println!("\n  GUID Examples:");
    println!("    SimpleTextOutput: {:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
        EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID.data1,
        EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID.data2,
        EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID.data3,
        EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID.data4[0],
        EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID.data4[1],
        EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID.data4[2],
        EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID.data4[3],
        EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID.data4[4],
        EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID.data4[5],
        EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID.data4[6],
        EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID.data4[7],
    );
}

// ============================================
// UEFI Status Codes
// ============================================

/// UEFI Status type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Status(usize);

impl Status {
    const SUCCESS: Status = Status(0);
    const LOAD_ERROR: Status = Status(1);
    const INVALID_PARAMETER: Status = Status(2);
    const UNSUPPORTED: Status = Status(3);
    const BAD_BUFFER_SIZE: Status = Status(4);
    const BUFFER_TOO_SMALL: Status = Status(5);
    const NOT_READY: Status = Status(6);
    const DEVICE_ERROR: Status = Status(7);
    const NOT_FOUND: Status = Status(14);

    fn is_success(&self) -> bool {
        self.0 == 0
    }

    fn is_error(&self) -> bool {
        (self.0 & (1 << (usize::BITS - 1))) != 0 || self.0 != 0
    }

    fn description(&self) -> &'static str {
        match self.0 {
            0 => "Success",
            1 => "Load Error",
            2 => "Invalid Parameter",
            3 => "Unsupported",
            4 => "Bad Buffer Size",
            5 => "Buffer Too Small",
            6 => "Not Ready",
            7 => "Device Error",
            14 => "Not Found",
            _ => "Unknown Error",
        }
    }
}

fn uefi_status_codes() {
    let statuses = [
        Status::SUCCESS,
        Status::INVALID_PARAMETER,
        Status::NOT_FOUND,
        Status::DEVICE_ERROR,
    ];

    for status in &statuses {
        println!(
            "  Status {}: {} (success: {})",
            status.0,
            status.description(),
            status.is_success()
        );
    }

    // Using Result pattern with Status
    fn uefi_operation() -> Result<u32, Status> {
        // Simulate successful operation
        Ok(42)
    }

    match uefi_operation() {
        Ok(value) => println!("  Operation succeeded: {}", value),
        Err(status) => println!("  Operation failed: {}", status.description()),
    }
}

// ============================================
// Protocol Pattern
// ============================================

/// Trait representing a UEFI protocol
trait Protocol {
    fn guid() -> Guid;
}

/// Simple Text Output Protocol (simulated)
struct SimpleTextOutput {
    cursor_row: u32,
    cursor_col: u32,
    max_rows: u32,
    max_cols: u32,
}

impl Protocol for SimpleTextOutput {
    fn guid() -> Guid {
        EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID
    }
}

impl SimpleTextOutput {
    fn new(max_rows: u32, max_cols: u32) -> Self {
        SimpleTextOutput {
            cursor_row: 0,
            cursor_col: 0,
            max_rows,
            max_cols,
        }
    }

    fn output_string(&mut self, s: &str) -> Status {
        println!("    [Console] {}", s);
        Status::SUCCESS
    }

    fn set_cursor_position(&mut self, row: u32, col: u32) -> Status {
        if row >= self.max_rows || col >= self.max_cols {
            return Status::INVALID_PARAMETER;
        }
        self.cursor_row = row;
        self.cursor_col = col;
        Status::SUCCESS
    }

    fn clear_screen(&mut self) -> Status {
        println!("    [Console] Screen cleared");
        self.cursor_row = 0;
        self.cursor_col = 0;
        Status::SUCCESS
    }
}

fn protocol_pattern() {
    let mut console = SimpleTextOutput::new(25, 80);

    println!("  Using SimpleTextOutput protocol:");

    console.clear_screen();
    console.set_cursor_position(0, 0);
    console.output_string("Hello, UEFI!");
    console.output_string("This is a UEFI application.");

    println!("\n  Protocol GUID: {:?}", SimpleTextOutput::guid());
}

// ============================================
// Memory Map Simulation
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MemoryType {
    Reserved,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
}

#[derive(Debug, Clone)]
struct MemoryDescriptor {
    memory_type: MemoryType,
    physical_start: u64,
    virtual_start: u64,
    number_of_pages: u64,
    attribute: u64,
}

impl MemoryDescriptor {
    fn size_bytes(&self) -> u64 {
        self.number_of_pages * 4096 // Page size is 4KB
    }
}

fn memory_map_simulation() {
    let memory_map = vec![
        MemoryDescriptor {
            memory_type: MemoryType::ConventionalMemory,
            physical_start: 0x0000_0000,
            virtual_start: 0,
            number_of_pages: 256, // 1 MB
            attribute: 0xF,
        },
        MemoryDescriptor {
            memory_type: MemoryType::LoaderCode,
            physical_start: 0x0010_0000,
            virtual_start: 0,
            number_of_pages: 64, // 256 KB
            attribute: 0xF,
        },
        MemoryDescriptor {
            memory_type: MemoryType::BootServicesData,
            physical_start: 0x0014_0000,
            virtual_start: 0,
            number_of_pages: 128, // 512 KB
            attribute: 0xF,
        },
        MemoryDescriptor {
            memory_type: MemoryType::ConventionalMemory,
            physical_start: 0x0020_0000,
            virtual_start: 0,
            number_of_pages: 30720, // ~120 MB
            attribute: 0xF,
        },
    ];

    println!("  Simulated Memory Map:");
    println!("  {:-^70}", "");
    println!(
        "  {:20} {:16} {:16} {:>10}",
        "Type", "Physical Start", "Pages", "Size"
    );
    println!("  {:-^70}", "");

    let mut total_conventional = 0u64;

    for desc in &memory_map {
        println!(
            "  {:20?} 0x{:014X} {:16} {:>10}",
            desc.memory_type,
            desc.physical_start,
            desc.number_of_pages,
            format_size(desc.size_bytes())
        );

        if desc.memory_type == MemoryType::ConventionalMemory {
            total_conventional += desc.size_bytes();
        }
    }

    println!("  {:-^70}", "");
    println!(
        "  Total conventional memory: {}",
        format_size(total_conventional)
    );
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 {
        format!("{} MB", bytes / (1024 * 1024))
    } else if bytes >= 1024 {
        format!("{} KB", bytes / 1024)
    } else {
        format!("{} B", bytes)
    }
}

// ============================================
// UEFI Variables
// ============================================

struct VariableStore {
    variables: HashMap<(String, Guid), Vec<u8>>,
}

impl VariableStore {
    fn new() -> Self {
        VariableStore {
            variables: HashMap::new(),
        }
    }

    fn get_variable(&self, name: &str, guid: &Guid) -> Result<&[u8], Status> {
        self.variables
            .get(&(name.to_string(), *guid))
            .map(|v| v.as_slice())
            .ok_or(Status::NOT_FOUND)
    }

    fn set_variable(&mut self, name: &str, guid: &Guid, data: Vec<u8>) -> Status {
        self.variables.insert((name.to_string(), *guid), data);
        Status::SUCCESS
    }
}

const EFI_GLOBAL_VARIABLE_GUID: Guid = Guid::new(
    0x8BE4DF61,
    0x93CA,
    0x11d2,
    [0xAA, 0x0D, 0x00, 0xE0, 0x98, 0x03, 0x2B, 0x8C],
);

fn uefi_variables() {
    let mut store = VariableStore::new();

    // Set some variables
    store.set_variable(
        "BootOrder",
        &EFI_GLOBAL_VARIABLE_GUID,
        vec![0x00, 0x00, 0x01, 0x00],
    );
    store.set_variable("Timeout", &EFI_GLOBAL_VARIABLE_GUID, vec![0x05, 0x00]);

    println!("  UEFI Variables:");

    // Read BootOrder
    match store.get_variable("BootOrder", &EFI_GLOBAL_VARIABLE_GUID) {
        Ok(data) => {
            println!("    BootOrder: {:02X?}", data);
        }
        Err(status) => {
            println!("    BootOrder: {}", status.description());
        }
    }

    // Read Timeout
    match store.get_variable("Timeout", &EFI_GLOBAL_VARIABLE_GUID) {
        Ok(data) => {
            let timeout = u16::from_le_bytes([data[0], data[1]]);
            println!("    Timeout: {} seconds", timeout);
        }
        Err(status) => {
            println!("    Timeout: {}", status.description());
        }
    }

    // Try to read non-existent variable
    match store.get_variable("NonExistent", &EFI_GLOBAL_VARIABLE_GUID) {
        Ok(_) => println!("    NonExistent: found"),
        Err(status) => println!("    NonExistent: {}", status.description()),
    }
}

// ============================================
// Boot Services Pattern
// ============================================

/// Simulated Boot Services table
struct BootServices {
    memory_map: Vec<MemoryDescriptor>,
    protocols: HashMap<(Handle, Guid), Box<dyn std::any::Any>>,
    next_handle: usize,
}

impl BootServices {
    fn new() -> Self {
        BootServices {
            memory_map: Vec::new(),
            protocols: HashMap::new(),
            next_handle: 1,
        }
    }

    fn allocate_pages(&mut self, pages: u64) -> Result<u64, Status> {
        // Simplified allocation
        let address = 0x1000_0000 + (self.next_handle as u64 * 0x1000);
        self.next_handle += pages as usize;
        println!("    Allocated {} pages at 0x{:X}", pages, address);
        Ok(address)
    }

    fn free_pages(&mut self, address: u64, _pages: u64) -> Status {
        println!("    Freed pages at 0x{:X}", address);
        Status::SUCCESS
    }

    fn create_handle(&mut self) -> Handle {
        let handle = Handle(self.next_handle);
        self.next_handle += 1;
        handle
    }

    fn install_protocol(
        &mut self,
        handle: Handle,
        guid: Guid,
        protocol: Box<dyn std::any::Any>,
    ) -> Status {
        self.protocols.insert((handle, guid), protocol);
        Status::SUCCESS
    }

    fn locate_protocol(&self, guid: &Guid) -> Result<&dyn std::any::Any, Status> {
        for ((_, g), proto) in &self.protocols {
            if g == guid {
                return Ok(proto.as_ref());
            }
        }
        Err(Status::NOT_FOUND)
    }
}

fn boot_services_pattern() {
    let mut bs = BootServices::new();

    println!("  Boot Services operations:");

    // Allocate memory
    let addr = bs.allocate_pages(10).unwrap();

    // Create and install a protocol
    let handle = bs.create_handle();
    let console = Box::new(SimpleTextOutput::new(25, 80));
    bs.install_protocol(handle, EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID, console);
    println!("    Installed SimpleTextOutput on handle {:?}", handle);

    // Locate protocol
    match bs.locate_protocol(&EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID) {
        Ok(_) => println!("    Located SimpleTextOutput protocol"),
        Err(status) => println!("    Protocol not found: {}", status.description()),
    }

    // Free memory
    bs.free_pages(addr, 10);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_codes() {
        assert!(Status::SUCCESS.is_success());
        assert!(!Status::INVALID_PARAMETER.is_success());
    }

    #[test]
    fn test_guid_equality() {
        let guid1 = EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID;
        let guid2 = EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID;
        assert_eq!(guid1, guid2);
    }

    #[test]
    fn test_memory_descriptor_size() {
        let desc = MemoryDescriptor {
            memory_type: MemoryType::ConventionalMemory,
            physical_start: 0,
            virtual_start: 0,
            number_of_pages: 256,
            attribute: 0,
        };
        assert_eq!(desc.size_bytes(), 256 * 4096);
    }

    #[test]
    fn test_variable_store() {
        let mut store = VariableStore::new();
        let guid = EFI_GLOBAL_VARIABLE_GUID;

        store.set_variable("Test", &guid, vec![1, 2, 3]);

        let data = store.get_variable("Test", &guid).unwrap();
        assert_eq!(data, &[1, 2, 3]);

        assert!(store.get_variable("NonExistent", &guid).is_err());
    }
}
