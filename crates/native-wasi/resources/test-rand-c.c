/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#include <stdio.h>
#include <time.h>
#include <stdlib.h>

int main()
{
    srand(time(NULL));          // 初始化
    for (int i = 0; i < 4; i++) // 产生 4 个随机 int
    {
        // 返回 [0, RAND_MAX) 之间的随机数
        // RAND_MAX 默认值为 INT_MAX，即 2147483647

        int next = rand();
        printf("%d\n", next);
    }

    return 0;
}
