(module
    (type (;0;) (func (param i32 i32) (result i32)))
    (type (;1;) (func (param i32) (result i32)))
    (type (;2;) (func))
    (memory (;0;) 16)
    (global $__stack_pointer (mut i32) i32.const 1048576)
    (global (;1;) i32 i32.const 1048576)
    (global (;2;) i32 i32.const 1048576)
    (export "memory" (memory 0))
    (export "add" (func $add))
    (export "sub" (func $sub))
    (export "inc" (func $inc))
    (export "show" (func $show))
    (export "__data_end" (global 1))
    (export "__heap_base" (global 2))
    (func $add (;type 0;) (param (;0;) i32) (param (;1;) i32) (result i32)
        local.get 1
        local.get 0
        i32.add
    )
    (func $sub (;type 0;) (param (;0;) i32) (param (;1;) i32) (result i32)
        local.get 0
        local.get 1
        i32.sub
    )
    (func $inc (;type 1;) (param (;0;) i32) (result i32)
        local.get 0
        i32.const 1
        i32.add
    )
    (func $show (;type 2;)
    )
)