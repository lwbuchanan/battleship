#ifndef __MODEL_H__
#define __MODEL_H__

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define NUM_SHIPS 5
#define SHIP_NAMES (char*[]){"Carrier", "Battleship", "Destroyer", "Submarine", "Patrol Boat"}
#define SHIP_SIZES (int[]){5, 4, 3, 3, 2}

enum orientation {horizontal = 0, vertical = 1};
enum tilestatus {empty, hit, miss, sunk};

typedef struct ship {
  enum orientation orientation;
  int length;
  int xpos;
  int ypos;
  int health;
} ship_t;

typedef struct map_t {
  enum tilestatus tiles[10][10];
  ship_t ships[NUM_SHIPS];
  int numships;
  int livingships;
} map_t;

ship_t* ship_at(map_t* map, int x, int y);

int add_ship(map_t* map, enum orientation orientation, int length, int xpos, int ypos);

int shoot(map_t* map, int x, int y);

void render_map(map_t* map, char* buff, int show_ships);

#endif
