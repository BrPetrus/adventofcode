#include <stdio.h>
#include <stdlib.h>

const char FILEPATH[] = "input.txt";

int main(void) {
	FILE *input = fopen(FILEPATH, "r");
	if (input == NULL) {
		printf("Error opening file %s\n", FILEPATH);
		return EXIT_FAILURE;
	}

	int solution = 0;

	char buffer[256] = { 0 };
	while (fgets(buffer, 256, input) != NULL) {
		printf("%s", buffer);

		// Find first digit from left
		int i = 0;
		int a = 0;
		while (buffer[i] != '\0') {
			if (buffer[i] >= '0' && buffer[i] <= '9') {
				a = buffer[i] - '0';
				break;
			}
			i++;
		}

		// Find last digit from right
		int j = i;
		int b = 0;
		while (buffer[j] != '\0') {
			if (buffer[j] >= '0' && buffer[j] <= '9') {
				b = buffer[j] - '0';	
			}
			j++;
		}

		// Convert to integer
		//int number = buffer[j] - '0' + 10 * (buffer[i] - '0');
		int number = b + 10 * a;

		printf("Number: %d\n", number);
		printf("i: %d\n", i);
		printf("j: %d\n", j);
		printf("a: %d\n", a);
		printf("b: %d\n", b);

		// Add
		solution += number;
	}
	fclose(input);
	printf("Solution: %d\n", solution);
	return EXIT_SUCCESS;
}

