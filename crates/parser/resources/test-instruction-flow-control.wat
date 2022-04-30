(module
    (func (result i32)                          ;; (result i32) 是 block return type
        (block (result i32)                     ;;
            (i32.const 1)
            (loop (result i32)                  ;;
                (if (result i32) (i32.const 2)  ;; (i32.const 2) 是测试部分
                    (then (i32.const 3))        ;; then
                    (else (i32.const 4))        ;; else
                )
            )
        )
        (drop)
    )
)

;; 0x0016 | 15          | size of function
;; 0x0017 | 00          | 0 local blocks
;; 0x0018 | 02 7f       | Block { ty: Type(I32) }
;; 0x001a | 41 01       |   I32Const { value: 1 }
;; 0x001c | 03 7f       |   Loop { ty: Type(I32) }
;; 0x001e | 41 02       |     I32Const { value: 2 }
;; 0x0020 | 04 7f       |     If { ty: Type(I32) }
;; 0x0022 | 41 03       |       I32Const { value: 3 }
;; 0x0024 | 05          |     Else
;; 0x0025 | 41 04       |       I32Const { value: 4 }
;; 0x0027 | 0b          |     End
;; 0x0028 | 0b          |   End
;; 0x0029 | 0b          | End
;; 0x002a | 1a          | Drop
;; 0x002b | 0b          | End