(module
    (type $ft0 (func (param i32 i32) (result i32)))
    (export "add" (func $add))
    (export "sub" (func $sub))

    (func $add (type $ft0)
        (local.get 0) ;; 注，这两句 local.get 不是必须的，因为实参已经放置好在栈的顶端，
        (local.get 1) ;; 其中栈顶是最右侧的参数的值，这两句是为了测试 local.get 指令。
        (i32.add)
    )

    (func $sub (type $ft0)
        (local.get 0) ;; 注，这两句 local.get 不是必须的，因为实参已经放置好在栈的顶端，
        (local.get 1) ;; 其中栈顶是最右侧的参数的值，这两句是为了测试 local.get 指令。
        (i32.sub)
    )
)
