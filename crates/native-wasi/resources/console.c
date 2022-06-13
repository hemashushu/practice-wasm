/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#include <stdio.h>
#include <unistd.h>

#define BUF_SIZE 4096

int main(void)
{
    puts("gets and puts");
    puts("input a word:");
    char s[10];
    fgets(s, 10, stdin);
    puts(s);

    puts("read and write, press Ctrl+D to exit");

    int n;
    char buf[BUF_SIZE];

    while ((n = read(STDIN_FILENO, buf, BUF_SIZE)) > 0)
    {
        if (write(STDOUT_FILENO, buf, n) != n)
        {
            puts("write error");
            return 0;
        }
    }

    if (n < 0)
    {
        puts("read error");
    }

    return 0;
}