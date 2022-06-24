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

__attribute__((export_name("echo_by_std"))) int echo_by_std()
{

    int c;

    // 通过输入 Ctrl+D 可以中断循环
    // 如果缓存的存在，在 shell 运行这个函数时需要输入换行符之后 `fgetc` 才会返回
    while ((c = fgetc(stdin)) != EOF)
    {
        if (fputc(c, stdout) == EOF)
        {
            fputs("write error", stderr);
            return 1;
        }
    }

    if (ferror(stdin))
    {
        fputs("read error", stderr);
        return 1;
    }

    // 检测到 EOF，比如到达文件末尾，或者用户按了 Ctrl+D
    // if (feof(stdin))
    // {
    //     ...
    // }

    return 0;
}

__attribute__((export_name("echo_by_syscall"))) int echo_by_syscall()
{
    int n;
    char buffer[BUF_SIZE];

    // 通过输入 Ctrl+D 可以中断循环
    while ((n = read(STDIN_FILENO, buffer, BUF_SIZE)) > 0)
    {
        if (write(STDOUT_FILENO, buffer, n) != n)
        {
            fputs("write error", stderr);
            return 1;
        }
    }

    if (n < 0)
    {
        fputs("read error", stderr);
        return 1;
    }

    return 0;
}

int main()
{
    return echo_by_std();
    // return echo_by_syscall();
}