#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv) {
	int pid = fork();
	if (pid == 0) { // child
		puts("child greets");
		exit(0);
	} else if (pid == -1) {
		perror("fork");
		return 1;
	}
	// parent
	printf("parent says that child's pid is %d\n", pid);
	return 0;
}

