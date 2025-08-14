use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const LINKER_SCRIPT_NAME: &str = "adsp_sc598_link.ld";

const LINKER_SCRIPT: &[u8] = b"
ENTRY(_start);
MEMORY {
    SRAM (rw) : ORIGIN = 0x20080000, LENGTH = 2M
}
SECTIONS {
    .text : {
        KEEP(*(.text.entry))
        KEEP(*(.text.*))
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
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join(LINKER_SCRIPT_NAME))
        .unwrap()
        .write_all(LINKER_SCRIPT)
        .expect("Failed to write linker script");

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/*");
}
