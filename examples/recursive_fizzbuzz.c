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

int doFizzBuzz (int n) {
  int didPrint = 0;
  if (n % 3 == 0) {
    printFizz(); didPrint = 1;
  }
  if (n % 5 == 0) {
    printBuzz(); didPrint = 1;
  }
  if (!didPrint) {
    printNumber(n);
  }
  printNewLine();

  if (n < 100) {
    doFizzBuzz(n + 1);
  }
}

int main () {
  doFizzBuzz(3);
}
