CC := riscv64-unknown-elf-gcc
OBJCOPY := riscv64-unknown-elf-objcopy
LD := riscv64-unknown-elf-ld

CFLAGS := -O2 -ffreestanding -nostdlib -nostartfiles -I ../include -march=rv64i_zicsr -mabi=lp64
CFLAGS += -Wall -Werror -Wextra
LDFLAGS := -Ttext=0x80000000 -nostdlib 

test: test.S
	mkdir -p artifacts
	$(CC) $(CFLAGS) -c -o artifacts/test.o test.S
	$(LD) $(LDFLAGS) -o artifacts/test.elf artifacts/test.o
	$(OBJCOPY) -O binary artifacts/test.elf artifacts/test.bin