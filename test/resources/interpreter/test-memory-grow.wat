(module
    (memory 1)
    (func $f0 (result i32 i32 i32 i32)
        ;; 写入两个 i32 数

        (i32.const 0)
        (i32.const 0xaabbccdd)
        (i32.store)

        (i32.const 32)
        (i32.const 0x10012002)
        (i32.store)

        ;; 增加页面
        (i32.const 5)
        (memory.grow)   ;; 压入数字 1
        (memory.size)   ;; 压入数字 6

        ;; 读原先的那两个 i32 数

        (i32.const 0)
        (i32.load)      ;; 压入数字 0xaabbccdd

        (i32.const 32)
        (i32.load)      ;; 压入数字 0x10012002
    )
)
