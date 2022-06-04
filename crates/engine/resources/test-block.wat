(module
    ;; 测试 return 指令
    (func $0 (result i32)
        (i32.const 1) ;; got
        (return)
        (i32.const 10)
    )

    ;; 测试 return 指令
    (func $1 (result i32)
        (i32.const 1)
        (block
            (i32.const 2) ;; got
            (return)
            (i32.const 3)
        )
        (i32.const 10)
    )

    ;; 测试 return 指令
    (func $2 (result i32)
        (i32.const 1)
        (block
            (i32.const 2)
            (block
                (i32.const 3) ;; got
                (return)
                (i32.const 4)
            )
        )
        (i32.const 10)
    )

    ;; 测试 br 指令
    (func $3 (result i32)
        (i32.const 1)
        (block
            (i32.const 2)
            (br 0)
            (i32.const 3)
        ) ;; <-- br to here
        (i32.const 4) ;; got
    )

    ;; 测试 br 指令
    (func $4 (result i32)
        (i32.const 1)
        (block
            (i32.const 2) ;; got
            (br 1)
            (i32.const 3)
        )
        (i32.const 4)
    ) ;; <-- br to here

    ;; 测试 br 指令
    (func $5 (result i32)
        (i32.const 1)
        (block
            (i32.const 2)
            (block
                (i32.const 3)
                (block
                    (i32.const 4)
                    (br 0)
                    (i32.const 5)
                ) ;; <-- br to here
                (i32.const 11) ;; got
                (return)
            )
            (i32.const 12)
            (return)
        )
        (i32.const 13)
    )

    ;; 测试 br 指令
    (func $6 (result i32)
        (i32.const 1)
        (block
            (i32.const 2)
            (block
                (i32.const 3)
                (block
                    (i32.const 4)
                    (br 1)
                    (i32.const 5)
                )
                (i32.const 11)
                (return)
            ) ;; <-- br to here
            (i32.const 12)  ;; got
            (return)
        )
        (i32.const 13)
    )

    ;; 测试 br 指令
    (func $7 (result i32)
        (i32.const 1)
        (block
            (i32.const 2)
            (block
                (i32.const 3)
                (block
                    (i32.const 4)
                    (br 2)
                    (i32.const 5)
                )
                (i32.const 11)
                (return)
            )
            (i32.const 12)
            (return)
        ) ;; <-- br to here
        (i32.const 13) ;; got
    )

    ;; 测试 br_if
    (func $8 (result i32)
        (local $i i32)
        (local $sum i32)

        (loop (result i32)
            ;; i = i + 1
            (local.get $i)
            (i32.const 1)
            (i32.add)
            (local.set $i)

            ;; sum = sum +i
            (local.get $i)
            (local.get $sum)
            (i32.add)
            (local.set $sum)

            ;; 下面局部变量作为返回值，同时当需要重复执行
            ;; 流程控制结构块时，起掺杂作用
            (local.get $sum)

            ;; (i <= 10)?
            (local.get $i)
            (i32.const 10)
            (i32.lt_u)
            (br_if 0)
        )
        ;; got $sum = 1 + 2 + .. + 10 = 55
    )

    ;; 测试在结构块里访问函数的局部变量，以及返回值的类型等
    ;; 使用 (55,66) 调用本函数，返回 77
    (func $9 (param i32 i32) (result i32)
        (i32.const 11)
        (i64.const 22)
        (block (param i64) (result i64)
            (local.get 0)       ;; 55
            (i64.extend_i32_s)  ;; 55
            (i64.add)           ;; 22 + 55
            (i32.wrap_i64)
            (br 1)
        )
        (i32.const 99)
    )
)
