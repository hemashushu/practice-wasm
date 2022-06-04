(module
    ;; 测试 br_table
    (func (param i32) (result i32)
        (block
            (block
                (block
                    (i32.const 55)
                    (local.get 0)
                    (br_table 0 1 2 3)
                ) ;; <-- 0
                (i32.const 22)
                (return)
            ) ;; <-- 1
            (i32.const 33)
            (return)
        ) ;; <-- 2
        (i32.const 44)
    ) ;; <-- 3, 4...
)
