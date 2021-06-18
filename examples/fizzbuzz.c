int putchar (int c);

int printFizz () {
  putchar(70);
  putchar(105);
  putchar(122);
  putchar(122);
}
int printBuzz () {
  putchar(66);
  putchar(117);
  putchar(122);
  putchar(122);
}
int printNewLine () {
  putchar(10);
}

int printNumber (int n) {
  if (n > 9) {
    printNumber(n / 10);
    n = n - n / 10 * 10;
  }
  putchar(n + 48);
}

int main () {
  for (int i = 1; i <= 100; i = i + 1) {
    int didPrint = 0;
    if (i % 3 == 0) {
      printFizz(); didPrint = 1;
    }
    if (i % 5 == 0) {
      printBuzz(); didPrint = 1;
    }
    if (!didPrint) {
      printNumber(i);
    }
    printNewLine();
  }
  return 0;
}
