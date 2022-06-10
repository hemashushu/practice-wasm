(module
    (type $bin_op (func (param i32 i32) (result i32)))
    (import "math" "pow" (func $pow (type $bin_op)))

    (func $main (result i32)
        (i32.const 100)
    )

    (func $test_add (type $bin_op)
        (local.get 0)
        (local.get 1)
        (i32.add)
    )

    (func $test_pow (result i32)
        (i32.const 2)
        (i32.const 10)
        (call $pow)
    )

    (start $main)

    (export "test_add" (func $test_add))
    (export "test_pow" (func $test_pow))
)
