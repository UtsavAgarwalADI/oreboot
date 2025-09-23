use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const LINKER_SCRIPT_FILE: &str = "link-h616-bt0.ld";

const LINKER_SCRIPT: &[u8] = b"
OUTPUT_ARCH(aarch64)
ENTRY(start)
MEMORY {
    SRAM : ORIGIN = 0x00020800, LENGTH = 30K
}
SECTIONS {
    . = ORIGIN(SRAM);
    .magic : ALIGN(4) {
        LONG(0xEAEAFBFB);
    } > SRAM
    .text : ALIGN(4) {
        KEEP(*(.text.entry))
        *(.text .text.*)
    } > SRAM
    .rodata : ALIGN(4) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4);
        erodata = .;
    } > SRAM
    .data : ALIGN(4) {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4);
        edata = .;
    } > SRAM
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(4) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > SRAM
    /DISCARD/ : {
        *(.eh_frame)
    }
}";

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join(LINKER_SCRIPT_FILE))
        .unwrap()
        .write_all(LINKER_SCRIPT)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
