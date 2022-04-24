(module
	(memory 1 8)
	(data (offset (i32.const 100)) "hello")

	(func
		(i32.const 1)
		(i32.const 2)
		(i32.load offset=100)
		(i32.store offset=100)
		(memory.size)
		(drop)
		(i32.const 4)
		(memory.grow)
		(drop)
	)
)

;; 0x0012 | 05 04       | memory section
;; 0x0014 | 01          | 1 count
;; 0x0015 | 01 01 08    | [memory 0] MemoryType { memory64: false, shared: false, initial: 1, maximum: Some(8) }
;; 0x0018 | 0a 16       | code section
;; 0x001a | 01          | 1 count
;; ;; func 0
;; 0x001b | 14          | size of function
;; 0x001c | 00          | 0 local blocks
;; 0x001d | 41 01       | I32Const { value: 1 }
;; 0x001f | 41 02       | I32Const { value: 2 }
;; 0x0021 | 28 02 64    | I32Load { memarg: MemoryImmediate { align: 2, offset: 100, memory: 0 } }
;; 0x0024 | 36 02 64    | I32Store { memarg: MemoryImmediate { align: 2, offset: 100, memory: 0 } }
;; 0x0027 | 3f 00       | MemorySize { mem: 0, mem_byte: 0 }
;; 0x0029 | 1a          | Drop
;; 0x002a | 41 04       | I32Const { value: 4 }
;; 0x002c | 40 00       | MemoryGrow { mem: 0, mem_byte: 0 }
;; 0x002e | 1a          | Drop
;; 0x002f | 0b          | End
;; ;;
;; 0x0030 | 0b 0c       | data section
;; 0x0032 | 01          | 1 count
;; 0x0033 | 00          | data memory[0]
;; 0x0034 | 41 e4 00    | I32Const { value: 100 }
;; 0x0037 | 0b          | End