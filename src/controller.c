#include "controller.h"
#include "model.h"
#include <pthread.h>
#include <stdio.h>

ssize_t message(int sd, char* str) {
  int length = strlen(str);
  return send(sd, (const void*)str, length, 0);
}

ssize_t broadcast(int sd1, int sd2, char* str) {
  int length = strlen(str);
  int m1 = send(sd1, (const void*)str, length, 0);
  int m2 = send(sd2, (const void*)str, length, 0);
  return m1+m2;
}

int setup_player(map_t* map, int sd, char* renderbuffer) {
  char recvbuff[32];
  render_map(map, renderbuffer, 1);
  printf("%d", (int)message(sd, renderbuffer));
  message(sd, "Place ships like this: B2h (front of ship at B2, horizontally)\n");
  for (int i = 0; i < NUM_SHIPS; i++) {
    while (1) {
      message(sd, "Place your ");
      message(sd, SHIP_NAMES[i]);
      message(sd, ": ");
      int lenrecv = recv(sd, recvbuff, sizeof(recvbuff), 0);
      printf("%s: %d\n", recvbuff, lenrecv);
      break;
    }
    
    render_map(map, renderbuffer, 1);
    printf("%d", (int)message(sd, renderbuffer));
  }
  return 0;
}

void* player_thread_handler(void* args) {
  game_data_t* player_data = (struct game_data*)args;
  int sd = player_data->player_sd;
  map_t* player_map = player_data->player_map; 
  map_t* enemy_map = player_data->enemy_map; 
  char renderbuffer[512];

  setup_player(player_map, sd, renderbuffer);

  return NULL;
}

void rungame(int p1_sd, int p2_sd) {

  map_t* p1_map = malloc(sizeof(map_t));
  map_t* p2_map = malloc(sizeof(map_t));
  game_data_t p1_data = {
    p1_sd,
    p1_map,
    p2_map
  };
  game_data_t p2_data = {
    p2_sd,
    p2_map,
    p1_map
  };

  pthread_t p1;
  pthread_t p2;

  printf("Creating thread for p1\n");
  pthread_create(&p1, NULL, player_thread_handler, (void*)&p1_data);
  printf("Creating thread for p2\n");
  pthread_create(&p2, NULL, player_thread_handler, (void*)&p2_data);

  pthread_join(p1, NULL);
  printf("Thread for p1 has terminated\n");
  pthread_join(p2, NULL);
  printf("Thread for p2 has terminated\n");

  free(p1_map);
  free(p2_map);
}
