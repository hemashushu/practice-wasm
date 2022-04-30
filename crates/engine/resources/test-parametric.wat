(module
    (func $f0 (result i32 i32)
        (i32.const 100)
        ;; 测试值不为 0，应该选中替代项 123
        ;;
        ;; -- 栈顶 --
        ;; 1   <-- 测试值
        ;; 456 <-- 结果项
        ;; 123 <-- 替代项
        ;; 100
        ;; -- 栈底 --
        (select (i32.const 123) (i32.const 456) (i32.const 1))
    )

    (func $f1 (result i32 i32)
        (i32.const 100)
        ;; 测试值不为 0，应该选中结果项 456
        (select (i32.const 123) (i32.const 456) (i32.const 0))
    )

    (func $f2 (result i32)
        (i32.const 123)
        (i32.const 456)
        ;; drop 一次，弹出了 456
        (drop)
    )

    (func $f3
        (i32.const 123)
        (i32.const 456)
        ;; drop 两次，清空了操作数栈
        (drop)
        (drop)
    )

    (func $f4 (result i32 i32)
        (i32.const 100)
        ;; -- 栈顶 --
        ;; 456 <-- 被 drop
        ;; 123
        ;; 100
        ;; -- 栈底 --
        (drop (i32.const 123) (i32.const 456))
    )
)
