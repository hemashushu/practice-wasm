(module
    (type $ft0 (func (param i32 i32) (result i32)))
    (table funcref (elem $add $sub $mul $div))

    (func $f0 (result i32)
        (i32.const 0)  ;; $add
        (i32.const 10) ;; lhs
        (i32.const 2)  ;; rhs
        (call $calc)   ;; got 12
    )

    (func $f1 (result i32)
        (i32.const 1)  ;; $sub
        (i32.const 10) ;; lhs
        (i32.const 2)  ;; rhs
        (call $calc)   ;; got 8
    )

    (func $f2 (result i32)
        (i32.const 2)  ;; $mul
        (i32.const 10) ;; lhs
        (i32.const 2)  ;; rhs
        (call $calc)   ;; got 20
    )

    (func $f3 (result i32)
        (i32.const 3)  ;; $div
        (i32.const 10) ;; lhs
        (i32.const 2)  ;; rhs
        (call $calc)   ;; got 5
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

    ;; op 为 0~3，分别表示 add/sub/mul/div 函数
    (func $calc (param $op i32) (param $left i32) (param $right i32) (result i32)
        (local.get $left)
        (local.get $right)
        (local.get $op)
        (call_indirect (type $ft0))
    )
)
