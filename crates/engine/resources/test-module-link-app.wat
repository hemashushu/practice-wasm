(module
    (type $ft0 (func (result i32)))
    (type $ft1 (func (param i32 i32) (result i32)))
    (import "lib" "add" (func $add (type $ft1)))
    (import "lib" "sub" (func $sub (type $ft1)))

    (export "test_add" (func $f0))
    (export "test_sub" (func $f1))

    (func $f0 (type $ft0)
        (i32.const 55)
        (i32.const 22)
        (call $add)
    )

    (func $f1 (type $ft0)
        (i32.const 55)
        (i32.const 22)
        (call $sub)
    )
)
