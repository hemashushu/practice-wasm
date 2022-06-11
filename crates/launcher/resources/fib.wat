(module
    (func $main (result i32)
        (i32.const 10)
        (call $fib)
    )

    ;; 计算 `斐波那契数`（`fib`）的函数
    ;;
    ;; 0、 1、 1、 2、 3、 5、 8、 13、 21、 34、 55、 89、 144、 233、 377、 610
    ;; ^   ^
    ;; |   \-- 第 1 项
    ;; \------ 第 0 项
    (func $fib (param $n i32) (result i32)
        (if (result i32)
            (i32.le_s (local.get $n) (i32.const 1))
            (then
                (local.get $n)
            )
            (else
                (i32.add
                    (i32.sub (local.get $n) (i32.const 1))
                    (call $fib)
                    (i32.sub (local.get $n) (i32.const 2))
                    (call $fib)
                )
            )
        )
    )

    (start $main)

    (export "main" (func $main))
    (export "fib" (func $fib))

    ;; 我使用几个自制的解析器分别测试 fib 函数的运行速度，测试结果挺有趣的
    ;; fib(32) benchmark:
    ;;
    ;; go      vm           0m1.185s
    ;; go      interpreter  0m4.899s
    ;; rust    vm           0m2.148s
    ;; rust    interpreter  0m8.869s
    ;; js      interpreter  0m3.381s
    ;;
    ;; 需要指出的是，这几个解析器我都没有进行过优化，大概是怎样简单实现就怎么写，所以
    ;; 测试结果只反映我了乱写代码的水平。
)
