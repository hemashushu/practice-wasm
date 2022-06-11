(module
    (type $bin_op (func (param i32 i32) (result i32)))
    (import "lib" "pow" (func $pow (type $bin_op)))

    (func $main (result i32)
        (call $test_pow)
    )

    (func $test_pow (result i32)
        (i32.const 2)
        (i32.const 10)
        (call $pow)
    )

    (start $main)
)
