(module
    ;; 测试 memory, table, function type, function, local variable, block 等的名称
    (memory $mem0 1 2)          ;; 名称 mem0
    (table $tab0 2 4 funcref)   ;; 名称 tab0
    (type $type0 (func (param $ta i32) (param $tb i32) (result i32) (result i64))) ;; 名称 typ0，注 type 里面的参数名称将被忽略
    (func $fun0 (param $a i32) (param $b i64) (result i32)
        (local $var2 f32)
        (block $b0
            (block $b1
                (block $b2
                    (i32.const  2)
                    (br $b1)
                )
            )
            (if ;; 自动的 label 索引 3
                (then (i32.const 3))
                (else (i32.const 4))
            )
        )
        (block $4
            (block $5
                (br $4)
            )
        )
    )
    (func $fun1 (type $type0)
        (local $var0 i32)
        (local $var1 i64)
        (i32.const 100)
        (loop $l0 (type $type0)
            nop
            br 0
        )
    )

    (elem $elem_one (offset (i32.const 1)) $fun0)
    (elem $elem_two (offset (i32.const 3)) $fun1)
    (data $data_foo 0 (offset (i32.const 10)) "foo")
    (data $data_bar 0 (offset (i32.const 20)) "bar")
)