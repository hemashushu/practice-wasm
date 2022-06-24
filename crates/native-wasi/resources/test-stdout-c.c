/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#include <stdio.h>

int main()
{
    // puts 函数在输出时会在末尾添加 '\n'
    // puts("Hello world!");
    fputs("Hello world!", stdout);
    return 0;
}

// 如果需要导出其他函数，可以在函数前面添加 `__attribute__((export_name("...")))`
// 例如：
// __attribute__((export_name("add"))) int add(int a, int b) {
//     return a + b;
// }