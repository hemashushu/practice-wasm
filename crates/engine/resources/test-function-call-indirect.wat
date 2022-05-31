(module
    (type $ft0 (func (param i32 i32) (result i32)))
    (table funcref (elem $add $sub $mul $div))

    ;; op 为 0~3，分别表示 add/sub/mul/div 函数
    (func $calc (param $op i32) (param $left i32) (param $right i32) (result i32)
        (local.get $left)
        (local.get $right)
        (local.get $op)
        (call_indirect (type $ft0))
    )

    (func $add (type $ft0)
        (i32.add (local.get 0) (local.get 1))
    )

    (func $sub (type $ft0)
        (i32.sub (local.get 0) (local.get 1))
    )

    (func $mul (type $ft0)
        (i32.mul (local.get 0) (local.get 1))
    )

    (func $div (type $ft0)
        (i32.div_s (local.get 0) (local.get 1))
    )
)
