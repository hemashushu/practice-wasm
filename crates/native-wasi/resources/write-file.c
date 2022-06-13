/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

// $ wasmtime --dir . crates/native/resources/fileio.wasm

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <errno.h>

int main(int argc, char **argv)
{
    char *ofname = "test.txt";

    int outfd = open(ofname, O_WRONLY | O_CREAT, 0660);
    if (outfd < 0)
    {
        fprintf(stderr, "create file error: %s\n", strerror(errno));
        exit(1);
    }

    char *s = "Hello world!";
    int wbs = write(outfd, s, strlen(s));
    if (wbs < 0)
    {
        fprintf("write file error: %s\n", strerror(errno));
        exit(1);
    }

    close(outfd);

    puts("finish");
    exit(0);
}