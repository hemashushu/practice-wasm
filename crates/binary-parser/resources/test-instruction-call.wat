(module
    (type $ft0 (func))
    (table funcref (elem $f1 $f1 $f1))
    (func $f0
        (call $f1)
        (call_indirect (type $ft0) (i32.const 2))
    )
    (func $f1
        (i32.const 100)
    )
)

;; 0x0000 | 00 61 73 6d | version 1 (Module)
;;        | 01 00 00 00
;; 0x0008 | 01 04       | type section
;; 0x000a | 01          | 1 count
;; 0x000b | 60 00 00    | [type 0] Func(FuncType { params: [], returns: [] })
;; 0x000e | 03 03       | func section
;; 0x0010 | 02          | 2 count
;; 0x0011 | 00          | [func 0] type 0
;; 0x0012 | 00          | [func 1] type 0
;; 0x0013 | 04 05       | table section
;; 0x0015 | 01          | 1 count
;; 0x0016 | 70 01 03 03 | [table 0] TableType { element_type: FuncRef, initial: 3, maximum: Some(3) }
;; 0x001a | 09 09       | element section
;; 0x001c | 01          | 1 count
;; 0x001d | 00          | element FuncRef table[0]
;; 0x001e | 41 00       | I32Const { value: 0 }
;; 0x0020 | 0b          | End
;; 0x0021 | 03          | 3 items
;; 0x0022 | 01          | item Func(1)
;; 0x0023 | 01          | item Func(1)
;; 0x0024 | 01          | item Func(1)
;; 0x0025 | 0a 11       | code section
;; 0x0027 | 02          | 2 count
;; ============== func 0 ====================
;; 0x0028 | 09          | size of function
;; 0x0029 | 00          | 0 local blocks
;; 0x002a | 10 01       | Call { function_index: 1 }
;; 0x002c | 41 02       | I32Const { value: 2 }
;; 0x002e | 11 00 00    | CallIndirect { index: 0, table_index: 0, table_byte: 0 }
;; 0x0031 | 0b          | End
;; ============== func 1 ====================
;; 0x0032 | 05          | size of function
;; 0x0033 | 00          | 0 local blocks
;; 0x0034 | 41 e4 00    | I32Const { value: 100 }
;; 0x0037 | 0b          | End
;; 0x0038 | 00 18       | custom section
;; 0x003a | 04 6e 61 6d | name: "name"
;;        | 65
;; 0x003f | 01 09       | function names
;; 0x0041 | 02          | 2 count
;; 0x0042 | 00 02 66 30 | Naming { index: 0, name: "f0" }
;; 0x0046 | 01 02 66 31 | Naming { index: 1, name: "f1" }
;; 0x004a | 04 06       | type names
;; 0x004c | 01          | 1 count
;; 0x004d | 00 03 66 74 | Naming { index: 0, name: "ft0" }
;;        | 30