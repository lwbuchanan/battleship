#ifndef __CONTROLLER_H__
#define __CONTROLLER_H__

#include "model.h"
#include <stdlib.h>
#include <sys/socket.h>
#include <string.h>
#include <pthread.h>

typedef struct game_data {
  int player_sd;
  map_t* player_map;
  map_t* enemy_map;
} game_data_t;

ssize_t message(int sd, char* str); 

ssize_t broadcast(int sd1, int sd2, char* str);

int setup_player(map_t* map, int sd, char* renderbuffer);

void rungame(int p1_sd, int p2_sd);

#endif 
