int myVar = 3;

// Sets myVar to 4
int realAssignment () {
  myVar = 4;
}
// Creates a local variable called myVar with the value 5
int shadowedAssignment () {
  int myVar = 5;
}

// Should return 4
int main() {
  realAssignment();
  shadowedAssignment();
  return myVar;
}
