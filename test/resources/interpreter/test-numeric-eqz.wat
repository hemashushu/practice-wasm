(module
    (func $f0 (result i32 i32)
        (i32.const 10)
        (i32.const 0)
        (i32.eqz)
    )

    (func $f1 (result i32 i32)
        (i32.const 10)
        (i32.const 1)
        ;; 不为 0 则压入 0
        (i32.eqz)
    )

    (func $f2 (result i32 i32)
        (i32.const 10)
        (i32.const 20)
        ;; 不为 0 则压入 0
        (i32.eqz)
    )
)
