(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (func $add (type 0) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.add
  )
  (func $mul (type 0) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.mul
  )
  (memory (;0;) 16)
  (global $__stack_pointer (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048576)
  (global (;2;) i32 i32.const 1048576)
  (export "memory" (memory 0))
  (export "add" (func $add))
  (export "mul" (func $mul))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
)
