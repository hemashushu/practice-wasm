(module
    (func
        (block
            (i32.const 100)
            (br 0)
            (i32.const 101)
        )
        (loop
            (i32.const 200)
            (br 0)
            (i32.const 201)
        )
        (if (i32.eqz (i32.const 300))
            (then (i32.const 400) (br 0) (i32.const 401))
            (else (i32.const 500) (br 0) (i32.const 501))
        )
    )

;; 0x0016 | 30          | size of function
;; 0x0017 | 00          | 0 local blocks
;; 0x0018 | 02 40       |   Block { ty: Empty }
;; 0x001a | 41 e4 00    |     I32Const { value: 100 }
;; 0x001d | 0c 00       |     Br { relative_depth: 0 }
;; 0x001f | 41 e5 00    |     I32Const { value: 101 }
;; 0x0022 | 0b          |   End
;; 0x0023 | 03 40       |   Loop { ty: Empty }
;; 0x0025 | 41 c8 01    |     I32Const { value: 200 }
;; 0x0028 | 0c 00       |     Br { relative_depth: 0 }
;; 0x002a | 41 c9 01    |     I32Const { value: 201 }
;; 0x002d | 0b          |   End
;; 0x002e | 41 ac 02    |   I32Const { value: 300 }
;; 0x0031 | 45          |   I32Eqz
;; 0x0032 | 04 40       |   If { ty: Empty }
;; 0x0034 | 41 90 03    |     I32Const { value: 400 }
;; 0x0037 | 0c 00       |     Br { relative_depth: 0 }
;; 0x0039 | 41 91 03    |     I32Const { value: 401 }
;; 0x003c | 05          |   Else
;; 0x003d | 41 f4 03    |     I32Const { value: 500 }
;; 0x0040 | 0c 00       |     Br { relative_depth: 0 }
;; 0x0042 | 41 f5 03    |     I32Const { value: 501 }
;; 0x0045 | 0b          |   End
;; 0x0046 | 0b          | End

    (func
        (block
            (block
                (block
                    (br 1)
                    (br_if 2 (i32.const 100))
                    (br_table 0 1 2 3) ;; 3 是默认标签
                    (return)
                )
            )
        )
    )


;; 0x0047 | 19          | size of function
;; 0x0048 | 00          | 0 local blocks
;; 0x0049 | 02 40       | Block { ty: Empty }
;; 0x004b | 02 40       |   Block { ty: Empty }
;; 0x004d | 02 40       |     Block { ty: Empty }
;; 0x004f | 0c 01       |       Br { relative_depth: 1 }
;; 0x0051 | 41 e4 00    |       I32Const { value: 100 }
;; 0x0054 | 0d 02       |       BrIf { relative_depth: 2 }
;; 0x0056 | 0e 03 00 01 |       BrTable { table: BrTable { count: 3, default: 3, targets: [0, 1, 2] } }
;;        | 02 03
;; 0x005c | 0f          |       Return
;; 0x005d | 0b          |     End
;; 0x005e | 0b          |   End
;; 0x005f | 0b          | End
;; 0x0060 | 0b          | End

)
