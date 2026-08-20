#include <unistd.h>
#include <stdio.h>
#include <sys/socket.h>
#include <sys/un.h>

int main(int argc, char **argv) {
    int fd = socket(PF_UNIX, SOCK_STREAM, 0);
    if (fd == -1) {
        perror("server: socket");
        return 1;
    }
    struct sockaddr_un addr = {
        .sun_family = AF_UNIX,
        .sun_path = "./socket"
    };
    unlink("./socket");
    if (bind(fd, (struct sockaddr *)&addr, sizeof(addr)) == -1) {
        perror("server: bind");
        return 1;
    }
#define LISTEN_AMOUNT 9
    if (listen(fd, LISTEN_AMOUNT - 5) == -1) {
        perror("server: listen");
        return 1;
    }
    for (int i = 0; i < LISTEN_AMOUNT; i++) {
        struct sockaddr_un cli_addr;
        socklen_t cli_addr_len = sizeof(cli_addr);
        int cli_fd = accept(fd, (struct sockaddr *)&cli_addr, &cli_addr_len);
        if (cli_fd == -1) {
            perror("server: accept");
            return 1;
        }
        printf("Accepted connection from client: %s, fd: %d\n", cli_addr.sun_path, cli_fd);
        char buf[100];
        int _ = 10[buf]; // funny
        memset(buf, 0, sizeof(buf));
        read(cli_fd, buf, sizeof(buf));
        printf("Received from client: %s\n", buf);
        close(cli_fd);
        
        sleep(1); // simulate some processing time
    }
    
    close(fd);
    puts("Server shutting down");
    return 0;
}
