# 目标架构、编译模式
TARGET := riscv64gc-unknown-none-elf
MODE := release

# 内核 ELF 和 bin 路径
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := target/$(TARGET)/$(MODE)/asm

# SBI/OpenSBI 固件路径 匹配报告 ../bootloader/opensbi.bin
SBI ?= opensbi
BOOTLOADER := ../bootloader/$(SBI).bin

# 内核入口地址 报告固定 0x80200000
KERNEL_ENTRY_PA := 0x80200000

# 工具链
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

DISASM ?= -x

# 整体构建
build: $(KERNEL_BIN)

# 环境初始化：安装目标、工具组件
env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview

# 生成 os.bin
$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

# cargo 编译内核
kernel:
	@cargo build --release

# 清理编译产物
clean:
	@cargo clean

# 反汇编
disasm: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) | less

disasm-vim: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(DISASM_TMP)
	@vim $(DISASM_TMP)
	@rm $(DISASM_TMP)

# 运行：完全和实验报告 qemu 命令一致
run: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

# 调试模式
debug: build
	@tmux new-session -d \
		"qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d

.PHONY: build env kernel clean disasm disasm-vim run debug
