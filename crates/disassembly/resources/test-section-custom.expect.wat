(module
    (type $typ0 (func (param i32 i32) (result i32 i64)))
    (type (;1;) (func (param i32)))
    (type (;2;) (func (param i32 i64) (result i32)))
    (import "env" "putc" (func $fputc (type 1)))
    (table $tab0 2 4 funcref)
    (memory $mem0 1 2)
    (elem $elem_one (offset (i32.const 1)) $fun0)
    (elem $elem_two (offset (i32.const 3)) $fun1)
    (func $fun0 (;type 2;) (param $a i32) (param $b i64) (result i32)
        (local $var2 f32)
        block $b0
            block $b1
                block $b2
                    i32.const 2
                    br 1
                end
            end
            if
                i32.const 3
            else
                i32.const 4
            end
        end
        block $4
            block $5
                br 1
            end
        end
    )
    (func $fun1 (;type $typ0;) (param $var0 i32) (param $var1 i32) (result i32 i64)
        (local (;2;) i32)
        (local (;3;) i64)
        i32.const 100
        loop $l0 (type $typ0)
            nop
            br 0
        end
    )
    (data $data_foo (offset (i32.const 10)) "\66\6f\6f")
    (data $data_bar (offset (i32.const 20)) "\62\61\72")
)