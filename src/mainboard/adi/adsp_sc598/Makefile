OREBOOT=$(abspath $(CURDIR)/../../../../)
TARGET     = aarch64-unknown-none
# full image path, including multiple stages
IMAGE_BASE  = $(OREBOOT)/target/$(TARGET)/release/oreboot-sc5xx
IMAGE_BT0  := $(IMAGE_BASE)-bt0
IMAGE      := $(IMAGE_BASE).bin

.PHONY: bt0 main clean objdump cibuild

cibuild: bt0

ciclippy: nop
citest: nop
checkformat: nop


mainboard:
	cargo -Z unstable-options -C main build --release

bt0:
	cargo -Z unstable-options -C bt0 build --release
	cargo objcopy --release -- -O binary "$(IMAGE)"
	cp "$(IMAGE)" /tftpboot/bt0

objdump: bt0
	cargo objdump -D "$(IMAGE_BT0)"

clean:
	cargo -Z unstable-options -C bt0 clean
