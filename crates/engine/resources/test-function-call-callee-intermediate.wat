(module
    (type $a (func (param i32 i32) (result i32)))

    (import "math" "sub" (func $im_sub (type $a)))
    (import "callee" "mul" (func $im_mul (type $a)))
    (import "callee" "div" (func $im_div (type $a)))

    (func $re_div (type $a)
        (local.get 0)
        (local.get 1)
        (call $im_div)
    )

    (export "re_sub" (func $im_sub))    ;; 将 native function 重新导出
    (export "re_mul" (func $im_mul))    ;; 将导入的 function 重新导出
    (export "re_div" (func $re_div))    ;; 一个调用了导入 function 的 function
)
