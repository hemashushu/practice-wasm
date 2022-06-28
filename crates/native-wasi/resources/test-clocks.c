/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#define _XOPEN_SOURCE 600
#include <time.h>
#include <stdio.h>
#include <stdlib.h>
// #include <stdbool.h>
// #include <unistd.h>

#define SECONDS_IN_DAY (24 * 60 * 60)

void show_clock(clockid_t clock_id)
{
    struct timespec ts;

    // time

    if (clock_gettime(clock_id, &ts) == -1)
    {
        perror("clock_gettime");
        exit(EXIT_FAILURE);
    }

    printf("tv_sec: %ld\n", (long)ts.tv_sec); // 秒数的部分
    printf("tv_nsec: %ld\n", ts.tv_nsec);     // 纳秒的部分

    // long days = ts.tv_sec / SECONDS_IN_DAY;
    // long hours = (ts.tv_sec % SECONDS_IN_DAY) / 3600;
    // long minutes = (ts.tv_sec % 3600) / 60;
    // long seconds = ts.tv_sec % 60;
    //
    // printf("days: %ld\n", days);
    // printf("hours: %ld\n", hours);
    // printf("minutes: %ld\n", minutes);
    // printf("seconds: %ld\n", seconds);

    // resolution

    if (clock_getres(clock_id, &ts) == -1)
    {
        perror("clock_getres");
        exit(EXIT_FAILURE);
    }

    printf("tv_sec: %ld\n", (long)ts.tv_sec);
    printf("tv_nsec: %ld\n", ts.tv_nsec);
}

int main(int argc, char *argv[])
{
    show_clock(CLOCK_REALTIME);

    // other clock id:
    // - CLOCK_MONOTONIC
    // - CLOCK_TAI
    // - CLOCK_BOOTTIME

    exit(EXIT_SUCCESS);
}
