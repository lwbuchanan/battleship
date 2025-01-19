#include "model.h"

// enum tilecontents tile_contains(map_t* map, int x, int y) {
//   if (x > 9 || y > 9) return oob;
//   for (int i = 0; i < map->numships; i++) {
//     for (int j = 0; j < map->ships[i].length; j++) {
//       if ((map->ships[i].xpos + j * (map->ships[i].orientation == vertical) == x) &&
//           (map->ships[i].ypos + j * (map->ships[i].orientation == horizontal) == y)) {
//         return ship;
//       }
//     }
//   }
//   return sea;
// }

ship_t* ship_at(map_t* map, int x, int y) {
  for (int i = 0; i < map->numships; i++) {
    for (int j = 0; j < map->ships[i].length; j++) {
      if ((map->ships[i].xpos + j * (map->ships[i].orientation == vertical) == x) &&
          (map->ships[i].ypos + j * (map->ships[i].orientation == horizontal) == y)) {
        return &map->ships[i];
      }
    }
  }
  return NULL;
}

int add_ship(map_t* map, enum orientation orientation, int length, int xpos, int ypos) {
  if ((orientation == vertical && length + xpos > 10) || 
      (orientation == horizontal && length + ypos > 10)) {
    return -1;
  }

  ship_t newship;
  newship.orientation = orientation;
  newship.length = length;
  newship.xpos = xpos;
  newship.ypos = ypos;
  newship.health = length;

  map->ships[map->numships] = newship;
  map->numships++;
  map->livingships++;
  return 0;
}

int shoot(map_t* map, int x, int y) {
  if (map->tiles[x][y] != empty) 
    return -1;

  ship_t* ship = ship_at(map, x, y);
  if (ship == NULL) {
    map->tiles[x][y] = miss;
    return 1;
  }

  ship->health--;
  if (ship->health <= 0) {
    map->tiles[x][y] = sunk;
    map->livingships--;
  }
  else {
    map->tiles[x][y] = hit;
  }
  return 0;
}

void render_map(map_t* map, char* buff, int show_ships) {
  int offset = 0;
  offset += sprintf(buff + offset, "   1  2  3  4  5  6  7  8  9  10\n");
  offset += sprintf(buff + offset, " --------------------------------\n");
  for (int i = 0; i < 10; i++) {
    offset += sprintf(buff+offset, "%c|", 'A'+i);
    for (int j = 0; j < 10; j++) {
      ship_t* ship_here = ship_at(map, i, j);
      switch (map->tiles[i][j]) {
        case empty:
          if (ship_here != NULL && show_ships) {
            offset += sprintf(buff + offset, " # ");
          } else {
            offset += sprintf(buff + offset, " . ");
          } 
          break;
        case hit:
          if (ship_here->health <= 0) {
            offset += sprintf(buff + offset, " @ ");
          } else {
            offset += sprintf(buff + offset, " X ");
          }
          break;
        case miss:
          offset += sprintf(buff + offset, " O ");
          break;
        case sunk:
          offset += sprintf(buff + offset, " @ ");
          break;
      }      
    }
    offset += sprintf(buff + offset, "|\n");
  }
  offset += sprintf(buff + offset, " --------------------------------\n");
}


// int main() {
//   map_t* map = malloc(sizeof(map_t));
//   map->numships = 0;
//   add_ship(map, vertical, 5, 0, 0);
//   add_ship(map, horizontal, 4, 3, 3);
//   add_ship(map, horizontal, 3, 7, 4);
//   add_ship(map, vertical, 3, 7, 9);
//   add_ship(map, horizontal, 2, 0, 8);

//   shoot(map, 0,8);
//   shoot(map, 0,9);

//   char* renderbuffer = malloc(512);
//   render_map(map, renderbuffer, 1);
//   printf("%s", renderbuffer);

//   free(renderbuffer);    
//   free(map);
//   return 0;
// }
