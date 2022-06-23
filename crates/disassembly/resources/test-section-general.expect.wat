(module
    (type $ft0 (func (result i32)))
    (type $ft1 (func))
    (import "env" "putc" (func $fputc (type $ft0)))
    (import "env" "print" (func (;1;) (type $ft0)))
    (table (;0;) 2 4 funcref)
    (memory (;0;) 1 8)
    (export "i_f2" (func $f2))
    (export "re_putc" (func $fputc))
    (start $f3)
    (elem (;0;) (offset (i32.const 1)) $f2)
    (elem (;1;) (offset (i32.const 3)) $f3)
    (func $f2 (;type $ft1;)
        i32.load offset=100 align=4
        call $fputc
        call $f3
    )
    (func $f3 (;type $ft1;)
        i32.load offset=200 align=8
        i64.load offset=400 align=8
    )
    (data (;0;) (offset (i32.const 100)) "\68\65\6c\6c\6f")
    (data (;1;) (offset (i32.const 200)) "\50\60\70")
)