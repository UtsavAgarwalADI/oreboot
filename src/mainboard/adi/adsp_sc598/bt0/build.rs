use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const LINKER_SCRIPT: &[u8] = b"
ENTRY(_start);
MEMORY {
    SRAM (rw) : ORIGIN = 0x20080000, LENGTH = 8K
}
SECTIONS {
    .text : {
        KEEP(*(.text.entry))
        *(.text*)
    } > SRAM
    .bss : ALIGN(4) {
        *(.bss*)
        *(COMMON)
    } > SRAM
    .rodata : ALIGN(4) {
        *(.rodata*)
    } > SRAM
    .data : ALIGN(4) {
        *(.data*)
    } > SRAM
}";

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    File::create(out.join("adsp_sc598_link.ld"))
        .unwrap()
        .write_all(LINKER_SCRIPT)
        .expect("Failed to write linker script");

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/*");
}
