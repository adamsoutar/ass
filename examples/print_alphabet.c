int putchar (int c);

int main () {
  int n = 0;
  while (n < 26) {
    putchar(65 + n);
    n = n + 1;
  }
  return 0;
}
