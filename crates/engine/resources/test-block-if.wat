(module
    ;; 测试 if
    (func $0 (result i32)
        (i32.const 11)
        (i32.const 22)

        (block (param i32 i32) (result i32)
            (i32.gt_s)
            (if (result i32)
                (then (i32.const 1))
                (else (i32.const 2)) ;; got
            )
        )
    )

    ;; 测试 if
    (func $1 (result i32)
        (i32.const 22)
        (i32.const 11)

        (block (param i32 i32) (result i32)
            (i32.gt_s)
            (if (result i32)
                (then (i32.const 1)) ;; got
                (else (i32.const 2))
            )
        )
    )
)
