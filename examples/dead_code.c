int main () {
  return 0;

  // ass will detect and warn you about code that will never execute
  return 1;
}

// It will not warn you about conditional returns. It doesn't know
// for sure whether this will be hit
int func () {
  if (0) {
    return 2;
  }

  for (int i = 0; i < 1; i = i + 1) {
    return 3;
  }
}
