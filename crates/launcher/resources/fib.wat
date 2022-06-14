(module
    (func $main
        (i32.const 10)
        (call $fib)
        (drop)
    )

    ;; 计算 `斐波那契数`（`fib`）的函数
    ;;
    ;; 0、 1、 1、 2、 3、 5、 8、 13、 21、 34、 55、 89、 144、 233、 377、 610
    ;; ^   ^                                   ^
    ;; |   \-- 第 1 项                          \-- 第 10 项
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

    ;; 我使用几个自制的解析器分别测试 fib 函数的运行速度，测试结果挺有趣的：
    ;;
    ;; fib(32)
    ;;
    ;; go       interpreter 0m4.899s
    ;; rust     interpreter 0m8.869s
    ;; nodejs   interpreter 0m3.381s
    ;; go       vm          0m1.185s
    ;; rust     wasm        0m2.148s
    ;;
    ;; 需要指出的是，这几个解析器我都没有进行过优化，没有使用到 AOT 或者 JIT 等技术，
    ;; 大概是怎样简单就怎么写，所以测试结果只反映了我这堆乱写的代码的运行情况。
    ;;
    ;; 下面是使用其他 WASM 虚拟机的运行情况：
    ;;
    ;; c++      wasm        0m1.473s    (wabt)
    ;; rust     jit         0m0.021s    (wasmtime)
    ;; c        wasm        0m0.263s    (wamrsm-micro-runtime without aot jit)
    ;; c        wasm        0m0.080s    (wasm3)
)
