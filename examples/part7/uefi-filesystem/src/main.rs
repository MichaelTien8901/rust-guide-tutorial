//! UEFI Filesystem Example
//!
//! Demonstrates UEFI file system access patterns and concepts.
//!
//! # UEFI Filesystem Architecture
//! ```text
//!     ┌─────────────────────────────────────────────────────────────┐
//!     │                 UEFI Filesystem Stack                       │
//!     ├─────────────────────────────────────────────────────────────┤
//!     │                                                             │
//!     │  Application                                                │
//!     │       │                                                     │
//!     │       ▼                                                     │
//!     │  Simple File System Protocol (EFI_SIMPLE_FILE_SYSTEM)       │
//!     │       │                                                     │
//!     │       ▼                                                     │
//!     │  File Protocol (EFI_FILE_PROTOCOL)                          │
//!     │       │                                                     │
//!     │       ▼                                                     │
//!     │  Block I/O Protocol                                         │
//!     │       │                                                     │
//!     │       ▼                                                     │
//!     │  Disk I/O Protocol                                          │
//!     │                                                             │
//!     └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! Note: This example demonstrates concepts in a std environment.
//! Real UEFI filesystem access requires the uefi crate and no_std.

use std::collections::HashMap;
use std::fmt;

fn main() {
    println!("=== UEFI Filesystem Concepts ===\n");

    println!("--- File Attributes ---");
    file_attributes();

    println!("\n--- File Protocol Operations ---");
    file_protocol_ops();

    println!("\n--- Directory Operations ---");
    directory_operations();

    println!("\n--- Path Handling ---");
    path_handling();

    println!("\n--- File Information ---");
    file_information();

    println!("\n--- Simulated Filesystem ---");
    simulated_filesystem();
}

// ============================================
// UEFI File Attributes
// ============================================

/// UEFI file attribute flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileAttributes(u64);

impl FileAttributes {
    const READ_ONLY: u64 = 0x0000000000000001;
    const HIDDEN: u64 = 0x0000000000000002;
    const SYSTEM: u64 = 0x0000000000000004;
    const RESERVED: u64 = 0x0000000000000008;
    const DIRECTORY: u64 = 0x0000000000000010;
    const ARCHIVE: u64 = 0x0000000000000020;
    const VALID_ATTR: u64 = 0x0000000000000037;

    fn new() -> Self {
        FileAttributes(0)
    }

    fn read_only(self) -> Self {
        FileAttributes(self.0 | Self::READ_ONLY)
    }

    fn hidden(self) -> Self {
        FileAttributes(self.0 | Self::HIDDEN)
    }

    fn directory(self) -> Self {
        FileAttributes(self.0 | Self::DIRECTORY)
    }

    fn is_directory(&self) -> bool {
        (self.0 & Self::DIRECTORY) != 0
    }

    fn is_read_only(&self) -> bool {
        (self.0 & Self::READ_ONLY) != 0
    }

    fn is_hidden(&self) -> bool {
        (self.0 & Self::HIDDEN) != 0
    }
}

impl fmt::Display for FileAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut attrs = Vec::new();
        if self.is_read_only() {
            attrs.push("READ_ONLY");
        }
        if self.is_hidden() {
            attrs.push("HIDDEN");
        }
        if (self.0 & Self::SYSTEM) != 0 {
            attrs.push("SYSTEM");
        }
        if self.is_directory() {
            attrs.push("DIRECTORY");
        }
        if (self.0 & Self::ARCHIVE) != 0 {
            attrs.push("ARCHIVE");
        }
        if attrs.is_empty() {
            write!(f, "NONE")
        } else {
            write!(f, "{}", attrs.join(" | "))
        }
    }
}

fn file_attributes() {
    let regular = FileAttributes::new();
    println!("  Regular file: {}", regular);

    let readonly = FileAttributes::new().read_only();
    println!("  Read-only file: {}", readonly);

    let dir = FileAttributes::new().directory();
    println!("  Directory: {}", dir);

    let hidden_dir = FileAttributes::new().directory().hidden();
    println!("  Hidden directory: {}", hidden_dir);

    println!("\n  Checking attributes:");
    println!("    readonly.is_read_only() = {}", readonly.is_read_only());
    println!("    dir.is_directory() = {}", dir.is_directory());
    println!("    hidden_dir.is_hidden() = {}", hidden_dir.is_hidden());
}

// ============================================
// File Protocol Operations
// ============================================

/// UEFI file open modes
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum OpenMode {
    Read,
    ReadWrite,
    Create,
}

impl OpenMode {
    fn to_flags(&self) -> u64 {
        match self {
            OpenMode::Read => 0x0000000000000001,
            OpenMode::ReadWrite => 0x0000000000000003,
            OpenMode::Create => 0x8000000000000000 | 0x0000000000000003,
        }
    }
}

/// Simulated UEFI file status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Status(usize);

impl Status {
    const SUCCESS: Status = Status(0);
    const NOT_FOUND: Status = Status(14);
    const WRITE_PROTECTED: Status = Status(8);
    const VOLUME_FULL: Status = Status(10);
    const END_OF_FILE: Status = Status(31);

    fn is_success(&self) -> bool {
        self.0 == 0
    }

    fn description(&self) -> &'static str {
        match self.0 {
            0 => "Success",
            8 => "Write Protected",
            10 => "Volume Full",
            14 => "Not Found",
            31 => "End of File",
            _ => "Unknown Error",
        }
    }
}

/// Simulated file handle
struct FileHandle {
    name: String,
    position: u64,
    size: u64,
    data: Vec<u8>,
    attributes: FileAttributes,
    is_open: bool,
}

impl FileHandle {
    fn new(name: &str, attributes: FileAttributes) -> Self {
        FileHandle {
            name: name.to_string(),
            position: 0,
            size: 0,
            data: Vec::new(),
            attributes,
            is_open: true,
        }
    }

    fn with_data(name: &str, data: Vec<u8>) -> Self {
        let size = data.len() as u64;
        FileHandle {
            name: name.to_string(),
            position: 0,
            size,
            data,
            attributes: FileAttributes::new(),
            is_open: true,
        }
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Status> {
        if !self.is_open {
            return Err(Status::NOT_FOUND);
        }

        let remaining = self.size.saturating_sub(self.position) as usize;
        let to_read = buffer.len().min(remaining);

        if to_read == 0 {
            return Err(Status::END_OF_FILE);
        }

        let start = self.position as usize;
        buffer[..to_read].copy_from_slice(&self.data[start..start + to_read]);
        self.position += to_read as u64;

        Ok(to_read)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, Status> {
        if !self.is_open {
            return Err(Status::NOT_FOUND);
        }

        if self.attributes.is_read_only() {
            return Err(Status::WRITE_PROTECTED);
        }

        let start = self.position as usize;
        let end = start + data.len();

        // Extend data if necessary
        if end > self.data.len() {
            self.data.resize(end, 0);
        }

        self.data[start..end].copy_from_slice(data);
        self.position = end as u64;
        self.size = self.data.len() as u64;

        Ok(data.len())
    }

    fn seek(&mut self, position: u64) -> Result<(), Status> {
        if !self.is_open {
            return Err(Status::NOT_FOUND);
        }

        self.position = position.min(self.size);
        Ok(())
    }

    fn get_position(&self) -> u64 {
        self.position
    }

    fn close(&mut self) {
        self.is_open = false;
    }
}

fn file_protocol_ops() {
    // Create a file with some data
    let mut file = FileHandle::with_data("test.txt", b"Hello, UEFI World!".to_vec());

    println!("  File: {}", file.name);
    println!("  Size: {} bytes", file.size);

    // Read from file
    let mut buffer = [0u8; 5];
    match file.read(&mut buffer) {
        Ok(bytes_read) => {
            println!(
                "  Read {} bytes: {:?}",
                bytes_read,
                String::from_utf8_lossy(&buffer)
            );
        }
        Err(e) => println!("  Read error: {}", e.description()),
    }

    // Check position
    println!("  Position after read: {}", file.get_position());

    // Seek to beginning
    file.seek(0).unwrap();
    println!("  Position after seek(0): {}", file.get_position());

    // Read entire content
    let mut full_buffer = vec![0u8; file.size as usize];
    match file.read(&mut full_buffer) {
        Ok(bytes_read) => {
            println!(
                "  Full content ({} bytes): {}",
                bytes_read,
                String::from_utf8_lossy(&full_buffer)
            );
        }
        Err(e) => println!("  Read error: {}", e.description()),
    }

    // Try to read past EOF
    match file.read(&mut buffer) {
        Ok(_) => println!("  Unexpected read success"),
        Err(e) => println!("  Expected EOF: {}", e.description()),
    }

    // Write to file
    file.seek(file.size).unwrap(); // Seek to end
    match file.write(b" Appended!") {
        Ok(bytes_written) => println!("  Wrote {} bytes", bytes_written),
        Err(e) => println!("  Write error: {}", e.description()),
    }

    // Read new content
    file.seek(0).unwrap();
    let mut new_buffer = vec![0u8; file.size as usize];
    file.read(&mut new_buffer).unwrap();
    println!("  New content: {}", String::from_utf8_lossy(&new_buffer));

    // Close file
    file.close();
    println!("  File closed");
}

// ============================================
// Directory Operations
// ============================================

/// Directory entry
#[derive(Debug, Clone)]
struct DirEntry {
    name: String,
    size: u64,
    attributes: FileAttributes,
}

impl DirEntry {
    fn file(name: &str, size: u64) -> Self {
        DirEntry {
            name: name.to_string(),
            size,
            attributes: FileAttributes::new(),
        }
    }

    fn directory(name: &str) -> Self {
        DirEntry {
            name: name.to_string(),
            size: 0,
            attributes: FileAttributes::new().directory(),
        }
    }
}

/// Simulated directory handle
struct DirectoryHandle {
    name: String,
    entries: Vec<DirEntry>,
    position: usize,
}

impl DirectoryHandle {
    fn new(name: &str) -> Self {
        DirectoryHandle {
            name: name.to_string(),
            entries: vec![DirEntry::directory("."), DirEntry::directory("..")],
            position: 0,
        }
    }

    fn add_entry(&mut self, entry: DirEntry) {
        self.entries.push(entry);
    }

    fn read_entry(&mut self) -> Option<&DirEntry> {
        if self.position < self.entries.len() {
            let entry = &self.entries[self.position];
            self.position += 1;
            Some(entry)
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.position = 0;
    }
}

fn directory_operations() {
    // Create a directory with entries
    let mut dir = DirectoryHandle::new("\\EFI\\BOOT");

    // Add some entries
    dir.add_entry(DirEntry::file("BOOTX64.EFI", 102400));
    dir.add_entry(DirEntry::file("startup.nsh", 256));
    dir.add_entry(DirEntry::directory("drivers"));
    dir.add_entry(DirEntry::file("config.txt", 1024));

    println!("  Directory: {}", dir.name);
    println!("  Entries:");

    // Enumerate entries
    while let Some(entry) = dir.read_entry() {
        let type_str = if entry.attributes.is_directory() {
            "<DIR>"
        } else {
            "     "
        };
        println!(
            "    {} {:20} {:>10}",
            type_str,
            entry.name,
            if entry.attributes.is_directory() {
                String::new()
            } else {
                format!("{} bytes", entry.size)
            }
        );
    }

    // Reset and count
    dir.reset();
    let count = dir.entries.len();
    println!("\n  Total entries: {}", count);
}

// ============================================
// Path Handling
// ============================================

/// UEFI uses backslash as path separator
fn normalize_path(path: &str) -> String {
    // Convert forward slashes to backslashes
    let normalized = path.replace('/', "\\");

    // Remove duplicate separators
    let mut result = String::new();
    let mut prev_sep = false;

    for c in normalized.chars() {
        if c == '\\' {
            if !prev_sep {
                result.push(c);
            }
            prev_sep = true;
        } else {
            result.push(c);
            prev_sep = false;
        }
    }

    result
}

/// Split path into components
fn split_path(path: &str) -> Vec<&str> {
    path.split('\\').filter(|s| !s.is_empty()).collect()
}

/// Join path components
fn join_path(components: &[&str]) -> String {
    if components.is_empty() {
        "\\".to_string()
    } else {
        format!("\\{}", components.join("\\"))
    }
}

/// Get parent directory
fn parent_path(path: &str) -> Option<String> {
    let components: Vec<&str> = split_path(path);
    if components.len() <= 1 {
        None
    } else {
        Some(join_path(&components[..components.len() - 1]))
    }
}

/// Get filename from path
fn filename(path: &str) -> Option<&str> {
    split_path(path).last().copied()
}

fn path_handling() {
    let paths = [
        "\\EFI\\BOOT\\BOOTX64.EFI",
        "/EFI/BOOT/grub.cfg",
        "\\EFI\\\\ubuntu\\\\shimx64.efi",
        "\\startup.nsh",
    ];

    for path in &paths {
        println!("  Original: {}", path);
        let normalized = normalize_path(path);
        println!("    Normalized: {}", normalized);

        let components = split_path(&normalized);
        println!("    Components: {:?}", components);

        if let Some(parent) = parent_path(&normalized) {
            println!("    Parent: {}", parent);
        }

        if let Some(name) = filename(&normalized) {
            println!("    Filename: {}", name);
        }

        println!();
    }
}

// ============================================
// File Information Structures
// ============================================

/// UEFI file information
#[derive(Debug, Clone)]
struct FileInfo {
    size: u64,
    file_size: u64,
    physical_size: u64,
    create_time: EfiTime,
    last_access_time: EfiTime,
    modification_time: EfiTime,
    attributes: FileAttributes,
    filename: String,
}

/// UEFI time structure
#[derive(Debug, Clone, Copy, Default)]
struct EfiTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl EfiTime {
    fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        EfiTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
}

impl fmt::Display for EfiTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

impl FileInfo {
    fn new(filename: &str, file_size: u64, attributes: FileAttributes) -> Self {
        let now = EfiTime::new(2024, 1, 15, 10, 30, 0);
        FileInfo {
            size: std::mem::size_of::<Self>() as u64,
            file_size,
            physical_size: ((file_size + 4095) / 4096) * 4096, // Round up to cluster
            create_time: now,
            last_access_time: now,
            modification_time: now,
            attributes,
            filename: filename.to_string(),
        }
    }
}

fn file_information() {
    let file_info = FileInfo::new("BOOTX64.EFI", 102400, FileAttributes::new().read_only());

    println!("  File Information:");
    println!("    Filename: {}", file_info.filename);
    println!("    File Size: {} bytes", file_info.file_size);
    println!("    Physical Size: {} bytes", file_info.physical_size);
    println!("    Attributes: {}", file_info.attributes);
    println!("    Created: {}", file_info.create_time);
    println!("    Modified: {}", file_info.modification_time);
    println!("    Accessed: {}", file_info.last_access_time);

    // Directory info
    let dir_info = FileInfo::new("BOOT", 0, FileAttributes::new().directory());

    println!("\n  Directory Information:");
    println!("    Name: {}", dir_info.filename);
    println!("    Attributes: {}", dir_info.attributes);
    println!("    Is Directory: {}", dir_info.attributes.is_directory());
}

// ============================================
// Simulated Filesystem
// ============================================

/// Simple filesystem simulation
struct SimpleFilesystem {
    files: HashMap<String, Vec<u8>>,
    directories: Vec<String>,
}

impl SimpleFilesystem {
    fn new() -> Self {
        let mut fs = SimpleFilesystem {
            files: HashMap::new(),
            directories: vec![
                "\\".to_string(),
                "\\EFI".to_string(),
                "\\EFI\\BOOT".to_string(),
            ],
        };

        // Add some default files
        fs.files.insert(
            "\\EFI\\BOOT\\BOOTX64.EFI".to_string(),
            vec![0x4D, 0x5A], // MZ header start
        );
        fs.files.insert(
            "\\startup.nsh".to_string(),
            b"echo Hello from UEFI Shell".to_vec(),
        );

        fs
    }

    fn open(&self, path: &str, mode: OpenMode) -> Result<FileHandle, Status> {
        let normalized = normalize_path(path);

        if let Some(data) = self.files.get(&normalized) {
            Ok(FileHandle::with_data(&normalized, data.clone()))
        } else if matches!(mode, OpenMode::Create) {
            Ok(FileHandle::new(&normalized, FileAttributes::new()))
        } else {
            Err(Status::NOT_FOUND)
        }
    }

    fn exists(&self, path: &str) -> bool {
        let normalized = normalize_path(path);
        self.files.contains_key(&normalized) || self.directories.contains(&normalized)
    }

    fn is_directory(&self, path: &str) -> bool {
        let normalized = normalize_path(path);
        self.directories.contains(&normalized)
    }

    fn list_directory(&self, path: &str) -> Result<Vec<DirEntry>, Status> {
        let normalized = normalize_path(path);

        if !self.directories.contains(&normalized) {
            return Err(Status::NOT_FOUND);
        }

        let mut entries = vec![DirEntry::directory("."), DirEntry::directory("..")];

        let prefix = if normalized == "\\" {
            "\\".to_string()
        } else {
            format!("{}\\", normalized)
        };

        // Find subdirectories
        for dir in &self.directories {
            if dir.starts_with(&prefix) && dir != &normalized {
                let relative = &dir[prefix.len()..];
                if !relative.contains('\\') {
                    entries.push(DirEntry::directory(relative));
                }
            }
        }

        // Find files
        for (file_path, data) in &self.files {
            if file_path.starts_with(&prefix) {
                let relative = &file_path[prefix.len()..];
                if !relative.contains('\\') {
                    entries.push(DirEntry::file(relative, data.len() as u64));
                }
            }
        }

        Ok(entries)
    }
}

fn simulated_filesystem() {
    let fs = SimpleFilesystem::new();

    // Check if paths exist
    let paths = [
        "\\EFI\\BOOT\\BOOTX64.EFI",
        "\\startup.nsh",
        "\\nonexistent.txt",
        "\\EFI\\BOOT",
    ];

    println!("  Checking paths:");
    for path in &paths {
        let exists = fs.exists(path);
        let is_dir = fs.is_directory(path);
        println!("    {} - exists: {}, directory: {}", path, exists, is_dir);
    }

    // List root directory
    println!("\n  Root directory contents:");
    if let Ok(entries) = fs.list_directory("\\") {
        for entry in entries {
            let type_str = if entry.attributes.is_directory() {
                "<DIR>"
            } else {
                "     "
            };
            println!("    {} {}", type_str, entry.name);
        }
    }

    // List EFI\\BOOT directory
    println!("\n  \\EFI\\BOOT directory contents:");
    if let Ok(entries) = fs.list_directory("\\EFI\\BOOT") {
        for entry in entries {
            let type_str = if entry.attributes.is_directory() {
                "<DIR>"
            } else {
                "     "
            };
            println!("    {} {}", type_str, entry.name);
        }
    }

    // Open and read a file
    println!("\n  Reading \\startup.nsh:");
    if let Ok(mut file) = fs.open("\\startup.nsh", OpenMode::Read) {
        let mut buffer = vec![0u8; 100];
        if let Ok(bytes) = file.read(&mut buffer) {
            buffer.truncate(bytes);
            println!("    Content: {}", String::from_utf8_lossy(&buffer));
        }
    }

    // Try to open non-existent file
    println!("\n  Opening non-existent file:");
    match fs.open("\\missing.txt", OpenMode::Read) {
        Ok(_) => println!("    Unexpected success"),
        Err(e) => println!("    Error: {}", e.description()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_attributes() {
        let attr = FileAttributes::new().directory().read_only();
        assert!(attr.is_directory());
        assert!(attr.is_read_only());
        assert!(!attr.is_hidden());
    }

    #[test]
    fn test_file_read_write() {
        let mut file = FileHandle::new("test.txt", FileAttributes::new());

        // Write data
        let written = file.write(b"Hello").unwrap();
        assert_eq!(written, 5);

        // Seek and read
        file.seek(0).unwrap();
        let mut buffer = [0u8; 5];
        let read = file.read(&mut buffer).unwrap();
        assert_eq!(read, 5);
        assert_eq!(&buffer, b"Hello");
    }

    #[test]
    fn test_path_operations() {
        assert_eq!(normalize_path("/EFI/BOOT"), "\\EFI\\BOOT");
        assert_eq!(normalize_path("\\\\EFI\\\\BOOT"), "\\EFI\\BOOT");

        let components = split_path("\\EFI\\BOOT\\file.efi");
        assert_eq!(components, vec!["EFI", "BOOT", "file.efi"]);

        assert_eq!(
            parent_path("\\EFI\\BOOT\\file.efi"),
            Some("\\EFI\\BOOT".to_string())
        );

        assert_eq!(filename("\\EFI\\BOOT\\file.efi"), Some("file.efi"));
    }

    #[test]
    fn test_filesystem() {
        let fs = SimpleFilesystem::new();

        assert!(fs.exists("\\EFI\\BOOT\\BOOTX64.EFI"));
        assert!(!fs.exists("\\nonexistent.txt"));
        assert!(fs.is_directory("\\EFI\\BOOT"));
        assert!(!fs.is_directory("\\startup.nsh"));
    }
}
