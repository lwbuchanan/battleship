#include "server.h"


void runserver() {
  int server_sd = socket(AF_INET, SOCK_STREAM, 0);
  if (server_sd < 0) {
    perror("-- Failed to create socket --\n");
    exit(EXIT_FAILURE);
  }

  struct sockaddr_in server_addr;
  server_addr.sin_family = AF_INET;
  server_addr.sin_addr.s_addr = htonl(INADDR_ANY);
  server_addr.sin_port = htons(PORT);

  if (bind(server_sd, (struct sockaddr*)&server_addr, sizeof(struct sockaddr_in)) < 0) {
    perror("-- Failed to bind socket --\n");
    exit(EXIT_FAILURE);
  }

  if (listen(server_sd, 2) < 0) {
    perror("-- Failed to start listening --\n");
    exit(EXIT_FAILURE);
  }
  printf(" : Started listening\n");
  
  
  int p1_sd;
  int p2_sd;
  struct sockaddr_in p1_addr;
  struct sockaddr_in p2_addr;
  while (1) {
    unsigned int sockaddr_len = sizeof(struct sockaddr_in);

    p1_sd = accept(server_sd, (struct sockaddr*)&p1_addr, &sockaddr_len);
    printf(" : Connected to %s\n", inet_ntoa(p1_addr.sin_addr));   
    message(p1_sd, "Waiting for p2...");

    p2_sd = accept(server_sd, (struct sockaddr*)&p2_addr, &sockaddr_len);
    printf(" : Connected to %s\n", inet_ntoa(p2_addr.sin_addr));   
    broadcast(p1_sd, p2_sd, "Game Start!\n");

    printf(" : Game is starting...\n");
    rungame(p1_sd, p2_sd);
    

    // Clean up
    printf(" : Disconnecting from %s\n", inet_ntoa(p1_addr.sin_addr));
    shutdown(p1_sd, SHUT_RDWR);
    close(p1_sd);
    printf(" : Disconnecting from %s\n", inet_ntoa(p2_addr.sin_addr));
    shutdown(p2_sd, SHUT_RDWR);
    close(p2_sd);
  }
  
  close(server_sd);
}
