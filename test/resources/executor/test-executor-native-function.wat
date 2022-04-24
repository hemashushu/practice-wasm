(module
    (type $ft0 (func (result i32)))
    (type $ft1 (func (param i32 i32) (result i32)))
    (import "env" "add_i32" (func $add_i32 (type $ft1)))
    (export "test_add" (func $f0))

    (func $f0 (type $ft0)
        (i32.const 11)
        (i32.const 22)
        (call $add_i32)
    )
)
