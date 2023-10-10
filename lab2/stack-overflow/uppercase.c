#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>

void my_strcpy(char *dest, const char *src) {
	size_t i = 0;
	while (true) {
		dest[i] = src[i];
		if (src[i] == '\0') {
			break;
		}
		i++;
	}
}

int main(int argc, char *argv[]) {
	if (argc < 2) {
		printf("Usage: %s <string to uppercase>\n", argv[0]);
		return 1;
	}
    
	// strlen(argv[1]) does not include '\0'
	char uppercase[strlen(argv[1])];

	// zanimljivo:
	// 1 byte stack overflow because argv[1] is +1 longer than uppercase
	// stack overflow happened because uppercase is allocated on stack
	my_strcpy(uppercase, argv[1]);

	for (int i = 0; uppercase[i] != '\0'; i++) {
		if(uppercase[i] >= 'a' && uppercase[i] <= 'z') {
			uppercase[i] = uppercase[i] - ('a' - 'A');
		}
	}

	printf("%s\n", uppercase);
}