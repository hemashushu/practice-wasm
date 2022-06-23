(module
    (import "wasi_snapshot_preview1" "fd_write"
        (func $fd_write
            (param $fd i32)
            (param $iovs_addr i32)
            (param $iovs_len i32)
            (param $result.size i32)
            (result (;$errno;) i32)))

    (memory 1)
    (export "memory" (memory 0))

    (data (i32.const 1100) "hello world")
    (data (i32.const 1200) "你好，世界")
    (data (i32.const 1300) "part1\n")
    (data (i32.const 1400) "part2\n")

    (func (export "write_string") (result i32 i32)
        ;; new io vector
        (i32.store (i32.const 100) (i32.const 1100))    ;; iov.offset, "hello world" 的起始地址
        (i32.store (i32.const 104) (i32.const 11))      ;; iov.len, "hello world" 的长度

        ;; call $fd_write
        (call $fd_write
            (i32.const 1)   ;; fd
            (i32.const 100) ;; iovs_addr
            (i32.const 1)   ;; iovs_len
            (i32.const 10)  ;; result.size
        )

        (i32.load (i32.const 10))

        ;; 返回 (errno, 已写入字节数)
    )

    (func (export "write_utf8") (result i32 i32)
        ;; new io vector
        (i32.store (i32.const 200) (i32.const 1200))    ;; iov.offset, "你好，世界" 的起始地址
        (i32.store (i32.const 204) (i32.const 15))      ;; iov.len, "你好，世界" 的长度

        ;; call $fd_write
        (call $fd_write
            (i32.const 1)   ;; fd
            (i32.const 200) ;; iovs_addr
            (i32.const 1)   ;; iovs_len
            (i32.const 20)  ;; result.size
        )

        (i32.load (i32.const 20))

        ;; 返回 (errno, 已写入字节数)
    )

    (func (export "write_multiple_parts") (result i32 i32)
        ;; new io vector(s)
        (i32.store (i32.const 300) (i32.const 1300))    ;; iov.offset, "part1\n" 的起始地址
        (i32.store (i32.const 304) (i32.const 6))       ;; iov.len, "part1\n" 的长度
        (i32.store (i32.const 308) (i32.const 1400))    ;; iov.offset, "part2\n" 的起始地址
        (i32.store (i32.const 312) (i32.const 6))       ;; iov.len, "part2\n" 的长度

        ;; call $fd_write
        (call $fd_write
            (i32.const 1)   ;; fd
            (i32.const 300) ;; iovs_addr
            (i32.const 2)   ;; iovs_len
            (i32.const 30)  ;; result.size
        )

        (i32.load (i32.const 30))

        ;; 返回 (errno, 已写入字节数)
    )
)