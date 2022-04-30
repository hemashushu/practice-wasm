(module
  (type (;0;) (func (result i32)))
  (import "env" "__linear_memory" (memory (;0;) 0))
  (func (;0;) (type 0) (result i32)
    (local i32)
    i32.const 100
    local.set 0
    local.get 0
    return
  )
)
