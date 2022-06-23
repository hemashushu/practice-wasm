/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#include <stdio.h>
#include <stdlib.h>

int main()
{
    printf("number: %d, string: %s\n", 123, "foo");
    printf("end of stdout");

    fprintf(stderr, "number: %d, string: %s\n", 456, "bar");
    fprintf(stderr, "end of stderr");

    // linux 平台返回码是 (number % 256)
    // https://doc.rust-lang.org/stable/std/process/fn.exit.html
    exit(66);
    printf("unreachable!");
}