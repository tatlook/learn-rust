#include <unistd.h>
#include <stdio.h>
#include <sys/socket.h>
#include <sys/un.h>

int main(int argc, char **argv) {
    int fd = socket(PF_UNIX, SOCK_STREAM, 0);
    if (fd == -1) {
        perror("client: socket");
        return 1;
    }
    struct sockaddr_un addr = {
        .sun_family = AF_UNIX,
        .sun_path = "./socket"
    };
    
    if (connect(fd, (struct sockaddr *) &addr, sizeof(addr))) {
        perror("client: connect");
        return 1;
    }
    write(fd, "Hello, World!", 13);
    printf("Connected to server\n");
    close(fd);
    return 0;
}
