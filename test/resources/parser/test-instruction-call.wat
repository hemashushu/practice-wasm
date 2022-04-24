(module
    (type $ft1 (func))
    (type $ft2 (func))
    (table funcref (elem $f1 $f1 $f1))
    (func $f1
        (call $f1)
        (call_indirect (type $ft2) (i32.const 2))
    )
)

;; 0x0008 | 01 07       | type section
;; 0x000a | 02          | 2 count
;; 0x000b | 60 00 00    | [type 0] Func(FuncType { params: [], returns: [] })
;; 0x000e | 60 00 00    | [type 1] Func(FuncType { params: [], returns: [] })
;; 0x0011 | 03 02       | func section
;; 0x0013 | 01          | 1 count
;; 0x0014 | 00          | [func 0] type 0
;; 0x0015 | 04 05       | table section
;; 0x0017 | 01          | 1 count
;; 0x0018 | 70 01 03 03 | [table 0] TableType { element_type: FuncRef, initial: 3, maximum: Some(3) }
;; 0x001c | 09 09       | element section
;; 0x001e | 01          | 1 count
;; 0x001f | 00          | element FuncRef table[0]
;; 0x0020 | 41 00       | I32Const { value: 0 }
;; 0x0022 | 0b          | End
;; 0x0023 | 03          | 3 items
;; 0x0024 | 00          | item Func(0)
;; 0x0025 | 00          | item Func(0)
;; 0x0026 | 00          | item Func(0)
;; 0x0027 | 0a 0b       | code section
;; 0x0029 | 01          | 1 count
;; ============== func 0 ====================
;; 0x002a | 09          | size of function
;; 0x002b | 00          | 0 local blocks
;; 0x002c | 10 00       | Call { function_index: 0 }
;; 0x002e | 41 02       | I32Const { value: 2 }
;; 0x0030 | 11 01 00    | CallIndirect { index: 1, table_index: 0, table_byte: 0 }
;; 0x0033 | 0b          | End