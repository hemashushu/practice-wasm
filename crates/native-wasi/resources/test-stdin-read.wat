(module
    (import "wasi_snapshot_preview1" "fd_read"
        (func $fd_read
            (param $fd i32)
            (param $iovs_addr i32)
            (param $iovs_len i32)
            (param $result.size i32)
            (result (;$errno;) i32)))

    (memory 1)
    (export "memory" (memory 0))

    ;; stdin 提供输入数据 "0123456789"
    (func (export "read_string") (result i32 i32 i32 i32 i32 i32)
        ;; new io vector
        (i32.store (i32.const 100) (i32.const 1000))    ;; iov.offset
        (i32.store (i32.const 104) (i32.const 4))       ;; iov.len

        ;; call $fd_read
        (call $fd_read
            (i32.const 0)   ;; fd
            (i32.const 100) ;; iovs_addr
            (i32.const 1)   ;; iovs_len
            (i32.const 10)  ;; result.size
        )

        (i32.load (i32.const 10))

        (i32.load8_u (i32.const 1000))
        (i32.load8_u (i32.const 1001))
        (i32.load8_u (i32.const 1002))
        (i32.load8_u (i32.const 1003))

        ;; 返回 (errno, 已读取字节数, '0', '1' ,'2', '3')
    )

    (func (export "read_multiple_parts") (result i32 i32 i32 i32 i32 i32)
        ;; new io vector(s)
        (i32.store (i32.const 200) (i32.const 1200))    ;; iov.offset
        (i32.store (i32.const 204) (i32.const 1))       ;; iov.len
        (i32.store (i32.const 208) (i32.const 1202))    ;; iov.offset
        (i32.store (i32.const 212) (i32.const 2))       ;; iov.len

        ;; call $fd_read
        (call $fd_read
            (i32.const 0)   ;; fd
            (i32.const 200) ;; iovs_addr
            (i32.const 2)   ;; iovs_len
            (i32.const 20)  ;; result.size
        )

        (i32.load (i32.const 20))

        (i32.load8_u (i32.const 1200))
        (i32.load8_u (i32.const 1201))      ;; 这个值应该是不确定的
        (i32.load8_u (i32.const 1202))
        (i32.load8_u (i32.const 1203))

        ;; 返回 (errno, 已读取字节数, '0', '?' ,'1', '2')
    )
)