CC := riscv64-unknown-elf-gcc
OBJCOPY := riscv64-unknown-elf-objcopy
LD := riscv64-unknown-elf-ld

CFLAGS := -g -O0 -ffreestanding -nostdlib -nostartfiles -I ../include -march=rv64ima_zicsr_zifencei -mabi=lp64 -mcmodel=medany 
CFLAGS += -Wall -Werror -Wextra
LDFLAGS := -Ttext=0x80000000 -nostdlib 

test: test.c
	mkdir -p artifacts
	$(CC) $(CFLAGS) -S -o artifacts/test.S test.c
	$(CC) $(CFLAGS) -c -o artifacts/test.o artifacts/test.S
	$(LD) $(LDFLAGS) -o artifacts/test.elf artifacts/test.o
	$(OBJCOPY) -O binary artifacts/test.elf artifacts/test.bin