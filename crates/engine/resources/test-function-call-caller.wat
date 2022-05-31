(module
    (type $a (func (param i32 i32) (result i32)))

    ;; 导入 native function
    (import "math" "add" (func $na_add (type $a)))

    ;; 导入外部 function
    (import "callee" "mul" (func $ex_mul (type $a)))
    (import "callee" "div" (func $ex_div (type $a)))

    ;; 导入外部 function （这些 function 是重新导出的 function）
    (import "intermediate" "re_sub" (func $re_sub (type $a)))
    (import "intermediate" "re_mul" (func $re_mul (type $a)))
    (import "intermediate" "re_div" (func $re_div (type $a)))

    ;; 调用 $na_add
    (func $0 (type $a)
        (local.get 0)
        (local.get 1)
        (call $na_add)
    )

    ;; 调用 $ex_mul
    (func $1 (type $a)
        (local.get 0)
        (local.get 1)
        (call $ex_mul)
    )

    ;; 调用 $ex_div
    (func $2 (type $a)
        (local.get 0)
        (local.get 1)
        (call $ex_div)
    )

    ;; 调用 $re_sub
    (func $3 (type $a)
        (local.get 0)
        (local.get 1)
        (call $re_sub)
    )

    ;; 调用 $re_mul
    (func $4 (type $a)
        (local.get 0)
        (local.get 1)
        (call $re_mul)
    )

    ;; 调用 $re_div
    (func $5 (type $a)
        (local.get 0)
        (local.get 1)
        (call $re_div)
    )
)
