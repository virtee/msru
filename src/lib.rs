// SPDX-License-Identifier: Apache-2.0

//! As most of the existing crates require kernel-mode, this provides a
//! Rust-friendly interface for reading and writing to MSRs while in
//! user-space. This does require the `msr` kernel module to be loaded.
//!
//! Currently this crate only supports Linux.

use std::{
    convert::From,
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom},
    os::unix::fs::FileExt,
    path::Path,
};

#[derive(Debug)]
pub enum MsrError {
    IoError(std::io::Error),
    MissingKernelModule,
    UnknownError,
}

impl std::error::Error for MsrError {}

impl std::fmt::Display for MsrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MsrError::IoError(io_error) => write!(f, "IoError Encountered: {io_error}"),
            MsrError::MissingKernelModule => write!(f, "MSR Kernel Module not loaded!"),
            MsrError::UnknownError => write!(f, "An unknown error was encountered!"),
        }
    }
}

impl From<std::io::Error> for MsrError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

type Result<T> = std::result::Result<T, MsrError>;

/// A Rust-friendly MSR structure.
pub struct Msr {
    /// A model specific register address we would like to read.
    pub reg: u32,
    fh: File,
    buffer: [u8; 8],
}

impl Msr {
    /// Construct an Msr for a specified register and CPU.
    pub fn new(reg: u32, cpu: u16) -> Result<Self> {
        let cpu_msr_path: String = format!("/dev/cpu/{cpu}/msr");
        if !Path::new(&cpu_msr_path).exists() {
            return Err(MsrError::MissingKernelModule);
        }
        Ok(Self {
            reg,
            fh: OpenOptions::new()
                .read(true)
                .write(true)
                .open(cpu_msr_path)?,
            buffer: [0; 8],
        })
    }

    /// Returns a u64 value from the bytes buffer.
    pub fn read_value(&mut self) -> u64 {
        u64::from_ne_bytes(self.buffer)
    }

    /// Update the byte buffer with the specified value to be written to the
    /// MSR.
    pub fn set_value(&mut self, value: u64) {
        self.buffer = value.to_ne_bytes();
    }
}

pub trait Accessor {
    fn read(&mut self) -> Result<u64>;
    fn write(&self) -> Result<()>;
}

impl Accessor for Msr {
    /// Read the bytes from the MSR at the specified CPU and return the value.
    /// - Expects the a file-handle to have already been opened.
    fn read(&mut self) -> Result<u64> {
        self.fh.seek(SeekFrom::Start(self.reg.into()))?;
        self.fh.read_exact(&mut self.buffer)?;
        Ok(self.read_value())
    }

    /// Write the bytes buffer into the MSR at the specified CPU.
    /// Expects the a file-handle to have already been opened.
    fn write(&self) -> Result<()> {
        // Make sure the buffer is updated for writing.
        self.fh.write_all_at(&self.buffer, self.reg.into())?;
        Ok(())
    }
}
