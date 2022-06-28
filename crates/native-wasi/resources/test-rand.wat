(module
    (type (;0;) (func (param i32 i32) (result i32)))
    (import "wasi_snapshot_preview1" "random_get" (func (;0;) (type 0)))

    (memory 1)

    ;; 因为 wasi_sdk 编译 C 的 rand() 函数时并不调用 "random_get" API，
    ;; 而是自带了一个随机产生器。
    ;; 所以这里专门写了一个函数来测试该 API

    (func (;1;) (result i32 i32 i32)
        (i32.const 100)     ;; 目标地址
        (i32.const 8)       ;; 2 个 i32 数字的长度
        (call 0)
        (i32.load (i32.const 100))
        (i32.load (i32.const 104))

        ;; 返回 (errno, r0, r1)
    )

    (export "get_two_rand" (func 1))
)
