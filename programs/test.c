long long factorial(long long n);
long long fibonacci(long long n);
void _start(void) __attribute__((naked));

#ifdef __riscv_xlen
void _start(void) {
  // setup the stack with inline assembly
  asm volatile("auipc sp, 0");       // set sp to the current pc
  asm volatile("addi sp, sp, 1024"); // move sp up by 1024 to create a stack
  asm volatile("addi sp, sp, 1024"); // move sp up by 1024 to create a stack
  asm volatile("addi sp, sp, 1024"); // move sp up by 1024 to create a stack
  asm volatile("addi sp, sp, 1024"); // move sp up by 1024 to create a stack
  asm volatile("addi sp, sp, 1024"); // move sp up by 1024 to create a stack
  asm volatile("call main");
  asm volatile("ebreak"); // halt the program after main returns
}
#else
#include <stdio.h>
#endif

long long complex_mathematical_function(long long n);

int main() {
  for (int i = 0; i < 16; i++) {
    volatile long long a =
        complex_mathematical_function((long long)i * 123456789);
#ifdef __riscv_xlen
    asm volatile("mv a0, %0" ::"r"(
        a)); // move the result into a0 for inspection in the debugger
    asm volatile(
        "ecall"); // prevent the compiler from optimizing away the calculations
#else
    printf("%lld\n", a);
#endif
  }
  return 0;
}

static long long helper(long long x) {
  if (x <= 1)
    return x + 1;

  return helper(x / 2) + (x % 7);
}

long long complex_mathematical_function(long long n) {
  static const int table[16] = {3,  5,  7,  11, 13, 17, 19, 23,
                                29, 31, 37, 41, 43, 47, 53, 59};

  long long arr[64];

  // initialize array
  for (int i = 0; i < 64; i++) {
    arr[i] = (n + i * 17) ^ (i * i + 123);
  }

  long long result = 0x123456789ABCDEFLL;

  // nested loops
  for (int round = 0; round < 10; round++) {
    for (int i = 0; i < 64; i++) {
      long long v = arr[i];

      // arithmetic
      v += table[(i + round) & 15];
      v *= (i + 3);

      // bitwise
      v ^= (v << 7);
      v ^= (v >> 11);
      v ^= (v << 3);

      // modulo + branch
      if ((v & 1) == 0) {
        v += (v % 97);
      } else {
        v -= (v % 89);
      }

      // switch
      switch ((v ^ i) & 3) {
      case 0:
        v += 111;
        break;

      case 1:
        v ^= 0x55AA55AA;
        break;

      case 2:
        v -= 777;
        break;

      default:
        v += i * round;
        break;
      }

      arr[i] = v;
      result ^= v;
      result += (v >> (i & 7));
    }
  }

  // pointer walk
  long long *p = arr;
  for (int i = 0; i < 64; i++) {
    result += *(p + i);
    result ^= (*(p + i) << (i & 15));
  }

  // while loop
  int k = 0;
  while (k < 64) {
    result += arr[k] % (k + 1);
    k++;
  }

  // recursion
  result += helper((n & 1023) + 100);

  // reduction pass
  for (int i = 1; i < 64; i++) {
    result ^= arr[i] * (i + 1);
    result += arr[i - 1] / (i + 1);
  }

  return result;
}