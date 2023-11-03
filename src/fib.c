#include <stdlib.h>
#include <stdio.h>

int sgn(int x) {
	return (x > 0) - (x < 0);
}

int main() { int n = 0; int m = 1; for (int i = 0; i < abs(9); ++i) { m = (m + n); n = (m - n);  }printf("%i", n);  }