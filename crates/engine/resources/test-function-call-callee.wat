(module
    (type $a (func (param i32 i32) (result i32)))

    (func $mul (type $a)
        (local.get 0)
        (local.get 1)
        (i32.mul)
    )

    (func $div (type $a)
        (local.get 0)
        (local.get 1)
        (i32.div_s)
    )

    (export "mul" (func $mul))
    (export "div" (func $div))
)
