#ifndef __SERVER_H__
#define __SERVER_H__

#include <netinet/in.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>
#include "controller.h"

#define PORT 8081


void runserver();

#endif

