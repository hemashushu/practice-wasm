/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

// 在函数前面添加 `__attribute__((export_name("mul")))` 可以表示
// 该函数需要导出。

int add(int a, int b)
{
    return a + b;
}

int mul(int a, int b)
{
    return a * b;
}