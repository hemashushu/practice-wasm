;; 测试 memory, data, table, elem 以及 start 等指令
(module
  (type $ft0 (func (result i32)))
  (type $ft1 (func))

  (import "env" "putc" (func $fputc (type $ft0)))
  (import "env" "print" (func (type $ft0)))

  (memory 1 8)
  (data (offset (i32.const 100)) "hello")
  (data (offset (i32.const 200)) "\50\60\70")

  (table 2 4 funcref)
  (elem (offset (i32.const 1)) $f2)
  (elem (offset (i32.const 3)) $f3)

  (func $f2 (type 1)
    (i32.load offset=100)
    (call $fputc)
    (call $f3)
  )

  (func $f3 (type 1)
    (i32.load offset=200 align=8)
    (i64.load offset=400)
  )

  (start $f3)

  (export "i_f2" (func $f2))
  (export "re_putc" (func $fputc)) ;; 导入的函数可以再次导出
)
