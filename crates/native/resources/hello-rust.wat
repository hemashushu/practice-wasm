(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func (param i32 i32 i32) (result i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32) (result i64)))
  (type (;4;) (func))
  (type (;5;) (func (param i32 i32)))
  (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;7;) (func (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32)))
  (type (;8;) (func (param i32) (result i32)))
  (type (;9;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;10;) (func (param i32 i32 i32) (result i64)))
  (type (;11;) (func (param i32 i32 i32)))
  (type (;12;) (func (param i32 i32 i32 i32 i32)))
  (type (;13;) (func (param i64 i32 i32) (result i32)))
  (type (;14;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;15;) (func (param i32 i32 i32 i32 i32 i32 i32)))
  (type (;16;) (func (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;17;) (func (result i32)))
  (type (;18;) (func (param i32 i32 i32 i32)))
  (type (;19;) (func (param i32 i32) (result i64)))
  (import "wasi_snapshot_preview1" "fd_write" (func $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h6a67043e9c1785e7E (type 6)))
  (import "wasi_snapshot_preview1" "path_open" (func $_ZN4wasi13lib_generated22wasi_snapshot_preview19path_open17hc00f9638ef14f9b0E (type 7)))
  (import "wasi_snapshot_preview1" "environ_get" (func $__imported_wasi_snapshot_preview1_environ_get (type 2)))
  (import "wasi_snapshot_preview1" "environ_sizes_get" (func $__imported_wasi_snapshot_preview1_environ_sizes_get (type 2)))
  (import "wasi_snapshot_preview1" "fd_close" (func $__imported_wasi_snapshot_preview1_fd_close (type 8)))
  (import "wasi_snapshot_preview1" "fd_prestat_get" (func $__imported_wasi_snapshot_preview1_fd_prestat_get (type 2)))
  (import "wasi_snapshot_preview1" "fd_prestat_dir_name" (func $__imported_wasi_snapshot_preview1_fd_prestat_dir_name (type 1)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__imported_wasi_snapshot_preview1_proc_exit (type 0)))
  (func $__wasm_call_ctors (type 4)
    call $__wasilibc_populate_preopens
  )
  (func $undefined_weak:__wasilibc_find_relpath_alloc (type 9) (param i32 i32 i32 i32 i32) (result i32)
    unreachable
  )
  (func $_start (type 4)
    (local i32)
    block  ;; label = @1
      call $__original_main
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      call $exit
      unreachable
    end
  )
  (func $_ZN4core3ptr76drop_in_place$LT$std..io..Write..write_fmt..Adapter$LT$std..fs..File$GT$$GT$17hd00642ff593fcf18E (type 0) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=4
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.const 8
      i32.add
      i32.load
      local.tee 1
      i32.load
      local.get 1
      i32.load offset=4
      i32.load
      call_indirect (type 0)
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        i32.load offset=4
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load
        call $free
      end
      local.get 0
      i32.load offset=8
      call $free
    end
  )
  (func $_ZN3std2io5Write9write_all17h80d03cd6417d9f63E (type 10) (param i32 i32 i32) (result i64)
    (local i32 i64 i64 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i64.const 4
    local.set 4
    i64.const 0
    local.set 5
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load
        local.set 6
        loop  ;; label = @3
          local.get 3
          local.get 2
          i32.store offset=4
          local.get 3
          local.get 1
          i32.store
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 6
                local.get 3
                i32.const 1
                local.get 3
                i32.const 12
                i32.add
                call $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h6a67043e9c1785e7E
                local.tee 0
                br_if 0 (;@6;)
                local.get 3
                i32.load offset=12
                local.tee 0
                br_if 1 (;@5;)
                i32.const 1054672
                i64.extend_i32_u
                i64.const 32
                i64.shl
                local.set 5
                i64.const 2
                local.set 4
                br 4 (;@2;)
              end
              local.get 0
              i32.const 65535
              i32.and
              local.tee 0
              i32.const 27
              i32.eq
              br_if 1 (;@4;)
              local.get 0
              i64.extend_i32_u
              i64.const 32
              i64.shl
              local.set 5
              i64.const 0
              local.set 4
              br 3 (;@2;)
            end
            local.get 2
            local.get 0
            i32.lt_u
            br_if 3 (;@1;)
            local.get 1
            local.get 0
            i32.add
            local.set 1
            local.get 2
            local.get 0
            i32.sub
            local.set 2
          end
          local.get 2
          br_if 0 (;@3;)
        end
      end
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 5
      local.get 4
      i64.or
      return
    end
    local.get 0
    local.get 2
    i32.const 1048652
    call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
    unreachable
  )
  (func $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E (type 11) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 44
    i32.add
    i32.const 1
    i32.store
    local.get 3
    i64.const 2
    i64.store offset=12 align=4
    local.get 3
    i32.const 1049876
    i32.store offset=8
    local.get 3
    i32.const 1
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=40
    local.get 3
    local.get 3
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
    unreachable
  )
  (func $__rust_alloc (type 2) (param i32 i32) (result i32)
    block  ;; label = @1
      local.get 1
      local.get 0
      i32.le_u
      br_if 0 (;@1;)
      local.get 1
      local.get 0
      call $aligned_alloc
      return
    end
    local.get 0
    call $malloc
  )
  (func $__rust_alloc_error_handler (type 5) (param i32 i32)
    local.get 0
    local.get 1
    call $__rg_oom
    unreachable
  )
  (func $__rg_oom (type 5) (param i32 i32)
    local.get 0
    local.get 1
    call $rust_oom
    unreachable
  )
  (func $_ZN5alloc7raw_vec17capacity_overflow17h0f57087aa53660d1E (type 4)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 28
    i32.add
    i32.const 0
    i32.store
    local.get 0
    i32.const 1053116
    i32.store offset=24
    local.get 0
    i64.const 1
    i64.store offset=12 align=4
    local.get 0
    i32.const 1048740
    i32.store offset=8
    local.get 0
    i32.const 8
    i32.add
    i32.const 1048748
    call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
    unreachable
  )
  (func $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E (type 5) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 1
    i32.store8 offset=24
    local.get 2
    local.get 1
    i32.store offset=20
    local.get 2
    local.get 0
    i32.store offset=16
    local.get 2
    i32.const 1048992
    i32.store offset=12
    local.get 2
    i32.const 1053116
    i32.store offset=8
    local.get 2
    i32.const 8
    i32.add
    call $rust_begin_unwind
    unreachable
  )
  (func $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E (type 5) (param i32 i32)
    local.get 0
    local.get 1
    call $_ZN4core10intrinsics17const_eval_select17h4f8d9384ae361685E
    unreachable
  )
  (func $_ZN4core10intrinsics17const_eval_select17h4f8d9384ae361685E (type 5) (param i32 i32)
    local.get 0
    local.get 1
    call $_ZN4core3ops8function6FnOnce9call_once17hb6981449096bf8f1E
    unreachable
  )
  (func $_ZN4core3ops8function6FnOnce9call_once17hb6981449096bf8f1E (type 5) (param i32 i32)
    local.get 0
    local.get 1
    call $_ZN5alloc5alloc18handle_alloc_error8rt_error17h9e693526ac08cff3E
    unreachable
  )
  (func $_ZN5alloc5alloc18handle_alloc_error8rt_error17h9e693526ac08cff3E (type 5) (param i32 i32)
    local.get 0
    local.get 1
    call $__rust_alloc_error_handler
    unreachable
  )
  (func $rust_oom (type 5) (param i32 i32)
    local.get 0
    local.get 1
    call $_ZN3std5alloc24default_alloc_error_hook17ha1b29e04979abe0bE
    call $_ZN3std7process5abort17hc143e6d6381800dcE
    unreachable
  )
  (func $_ZN5alloc7raw_vec11finish_grow17hb55c4957f32437bbE (type 12) (param i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 0
        i32.ge_s
        br_if 0 (;@2;)
        i32.const 1
        local.set 2
        i32.const 0
        local.set 1
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                br_if 2 (;@4;)
                local.get 1
                br_if 1 (;@5;)
                i32.const 1
                local.set 2
                br 4 (;@2;)
              end
              local.get 1
              br_if 0 (;@5;)
              i32.const 1
              local.set 2
              br 3 (;@2;)
            end
            local.get 1
            call $malloc
            local.tee 2
            i32.eqz
            br_if 1 (;@3;)
            br 2 (;@2;)
          end
          local.get 2
          local.get 1
          call $realloc
          local.tee 2
          br_if 1 (;@2;)
        end
        local.get 0
        local.get 1
        i32.store offset=4
        i32.const 1
        local.set 1
        i32.const 1
        local.set 2
        br 1 (;@1;)
      end
      local.get 0
      local.get 2
      i32.store offset=4
      i32.const 0
      local.set 2
    end
    local.get 0
    local.get 2
    i32.store
    local.get 0
    i32.const 8
    i32.add
    local.get 1
    i32.store
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h6d66c53f3322ec2aE (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.const 128
              i32.lt_u
              br_if 0 (;@5;)
              local.get 2
              i32.const 0
              i32.store offset=12
              local.get 1
              i32.const 2048
              i32.lt_u
              br_if 1 (;@4;)
              local.get 1
              i32.const 65536
              i32.ge_u
              br_if 2 (;@3;)
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              local.set 1
              br 3 (;@2;)
            end
            block  ;; label = @5
              local.get 0
              i32.load offset=8
              local.tee 3
              local.get 0
              i32.const 4
              i32.add
              i32.load
              i32.ne
              br_if 0 (;@5;)
              local.get 0
              local.get 3
              call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h2033c1963b518374E
              local.get 0
              i32.load offset=8
              local.set 3
            end
            local.get 0
            local.get 3
            i32.const 1
            i32.add
            i32.store offset=8
            local.get 0
            i32.load
            local.get 3
            i32.add
            local.get 1
            i32.store8
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          local.set 1
          br 1 (;@2;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=15
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const 240
        i32.or
        i32.store8 offset=12
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=14
        local.get 2
        local.get 1
        i32.const 12
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        i32.const 4
        local.set 1
      end
      block  ;; label = @2
        local.get 0
        i32.const 4
        i32.add
        i32.load
        local.get 0
        i32.const 8
        i32.add
        local.tee 4
        i32.load
        local.tee 3
        i32.sub
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        local.get 1
        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h9e9d16cf4f4f67caE
        local.get 4
        i32.load
        local.set 3
      end
      local.get 0
      i32.load
      local.get 3
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $memcpy
      drop
      local.get 4
      local.get 3
      local.get 1
      i32.add
      i32.store
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    i32.const 0
  )
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h2033c1963b518374E (type 5) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      local.get 1
      i32.const 1
      i32.add
      local.tee 3
      local.get 1
      i32.lt_u
      br_if 0 (;@1;)
      local.get 2
      local.get 0
      i32.const 4
      i32.add
      local.tee 4
      i32.load
      local.tee 1
      i32.const 1
      i32.shl
      local.tee 5
      local.get 3
      local.get 5
      local.get 3
      i32.gt_u
      select
      local.tee 3
      i32.const 8
      local.get 3
      i32.const 8
      i32.gt_u
      select
      local.tee 3
      local.get 0
      i32.load
      i32.const 0
      local.get 1
      select
      local.get 1
      i32.const 1
      call $_ZN5alloc7raw_vec11finish_grow17hb55c4957f32437bbE
      block  ;; label = @2
        local.get 2
        i32.load
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.const 8
        i32.add
        i32.load
        local.tee 0
        i32.eqz
        br_if 1 (;@1;)
        local.get 2
        i32.load offset=4
        local.get 0
        call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
        unreachable
      end
      local.get 2
      i32.load offset=4
      local.set 1
      local.get 4
      local.get 3
      i32.store
      local.get 0
      local.get 1
      i32.store
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    call $_ZN5alloc7raw_vec17capacity_overflow17h0f57087aa53660d1E
    unreachable
  )
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h9e9d16cf4f4f67caE (type 11) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      local.get 1
      local.get 2
      i32.add
      local.tee 2
      local.get 1
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      local.get 0
      i32.const 4
      i32.add
      local.tee 4
      i32.load
      local.tee 1
      i32.const 1
      i32.shl
      local.tee 5
      local.get 2
      local.get 5
      local.get 2
      i32.gt_u
      select
      local.tee 2
      i32.const 8
      local.get 2
      i32.const 8
      i32.gt_u
      select
      local.tee 2
      local.get 0
      i32.load
      i32.const 0
      local.get 1
      select
      local.get 1
      i32.const 1
      call $_ZN5alloc7raw_vec11finish_grow17hb55c4957f32437bbE
      block  ;; label = @2
        local.get 3
        i32.load
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 8
        i32.add
        i32.load
        local.tee 0
        i32.eqz
        br_if 1 (;@1;)
        local.get 3
        i32.load offset=4
        local.get 0
        call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
        unreachable
      end
      local.get 3
      i32.load offset=4
      local.set 1
      local.get 4
      local.get 2
      i32.store
      local.get 0
      local.get 1
      i32.store
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    call $_ZN5alloc7raw_vec17capacity_overflow17h0f57087aa53660d1E
    unreachable
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17he7114d91aaeb75baE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1048872
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN4core3fmt5write17h8776c655b56f9e02E (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 36
    i32.add
    local.get 1
    i32.store
    local.get 3
    i32.const 3
    i32.store8 offset=40
    local.get 3
    i64.const 137438953472
    i64.store offset=8
    local.get 3
    local.get 0
    i32.store offset=32
    i32.const 0
    local.set 4
    local.get 3
    i32.const 0
    i32.store offset=24
    local.get 3
    i32.const 0
    i32.store offset=16
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load offset=8
            local.tee 5
            br_if 0 (;@4;)
            local.get 2
            i32.const 20
            i32.add
            i32.load
            local.tee 6
            i32.eqz
            br_if 1 (;@3;)
            local.get 2
            i32.load
            local.set 1
            local.get 2
            i32.load offset=16
            local.set 0
            local.get 6
            i32.const -1
            i32.add
            i32.const 536870911
            i32.and
            i32.const 1
            i32.add
            local.tee 4
            local.set 6
            loop  ;; label = @5
              block  ;; label = @6
                local.get 1
                i32.const 4
                i32.add
                i32.load
                local.tee 7
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                i32.load offset=32
                local.get 1
                i32.load
                local.get 7
                local.get 3
                i32.load offset=36
                i32.load offset=12
                call_indirect (type 1)
                br_if 4 (;@2;)
              end
              local.get 0
              i32.load
              local.get 3
              i32.const 8
              i32.add
              local.get 0
              i32.const 4
              i32.add
              i32.load
              call_indirect (type 2)
              br_if 3 (;@2;)
              local.get 0
              i32.const 8
              i32.add
              local.set 0
              local.get 1
              i32.const 8
              i32.add
              local.set 1
              local.get 6
              i32.const -1
              i32.add
              local.tee 6
              br_if 0 (;@5;)
              br 2 (;@3;)
            end
          end
          local.get 2
          i32.const 12
          i32.add
          i32.load
          local.tee 0
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.const 5
          i32.shl
          local.set 8
          local.get 0
          i32.const -1
          i32.add
          i32.const 134217727
          i32.and
          i32.const 1
          i32.add
          local.set 4
          local.get 2
          i32.load
          local.set 1
          i32.const 0
          local.set 6
          loop  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.const 4
              i32.add
              i32.load
              local.tee 0
              i32.eqz
              br_if 0 (;@5;)
              local.get 3
              i32.load offset=32
              local.get 1
              i32.load
              local.get 0
              local.get 3
              i32.load offset=36
              i32.load offset=12
              call_indirect (type 1)
              br_if 3 (;@2;)
            end
            local.get 3
            local.get 5
            local.get 6
            i32.add
            local.tee 0
            i32.const 28
            i32.add
            i32.load8_u
            i32.store8 offset=40
            local.get 3
            local.get 0
            i32.const 4
            i32.add
            i64.load align=4
            i64.const 32
            i64.rotl
            i64.store offset=8
            local.get 0
            i32.const 24
            i32.add
            i32.load
            local.set 9
            local.get 2
            i32.load offset=16
            local.set 10
            i32.const 0
            local.set 11
            i32.const 0
            local.set 7
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.const 20
                  i32.add
                  i32.load
                  br_table 1 (;@6;) 0 (;@7;) 2 (;@5;) 1 (;@6;)
                end
                local.get 9
                i32.const 3
                i32.shl
                local.set 12
                i32.const 0
                local.set 7
                local.get 10
                local.get 12
                i32.add
                local.tee 12
                i32.load offset=4
                i32.const 2
                i32.ne
                br_if 1 (;@5;)
                local.get 12
                i32.load
                i32.load
                local.set 9
              end
              i32.const 1
              local.set 7
            end
            local.get 3
            local.get 9
            i32.store offset=20
            local.get 3
            local.get 7
            i32.store offset=16
            local.get 0
            i32.const 16
            i32.add
            i32.load
            local.set 7
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.const 12
                  i32.add
                  i32.load
                  br_table 1 (;@6;) 0 (;@7;) 2 (;@5;) 1 (;@6;)
                end
                local.get 7
                i32.const 3
                i32.shl
                local.set 9
                local.get 10
                local.get 9
                i32.add
                local.tee 9
                i32.load offset=4
                i32.const 2
                i32.ne
                br_if 1 (;@5;)
                local.get 9
                i32.load
                i32.load
                local.set 7
              end
              i32.const 1
              local.set 11
            end
            local.get 3
            local.get 7
            i32.store offset=28
            local.get 3
            local.get 11
            i32.store offset=24
            local.get 10
            local.get 0
            i32.load
            i32.const 3
            i32.shl
            i32.add
            local.tee 0
            i32.load
            local.get 3
            i32.const 8
            i32.add
            local.get 0
            i32.load offset=4
            call_indirect (type 2)
            br_if 2 (;@2;)
            local.get 1
            i32.const 8
            i32.add
            local.set 1
            local.get 8
            local.get 6
            i32.const 32
            i32.add
            local.tee 6
            i32.ne
            br_if 0 (;@4;)
          end
        end
        i32.const 0
        local.set 0
        local.get 4
        local.get 2
        i32.load offset=4
        i32.lt_u
        local.tee 1
        i32.eqz
        br_if 1 (;@1;)
        local.get 3
        i32.load offset=32
        local.get 2
        i32.load
        local.get 4
        i32.const 3
        i32.shl
        i32.add
        i32.const 0
        local.get 1
        select
        local.tee 1
        i32.load
        local.get 1
        i32.load offset=4
        local.get 3
        i32.load offset=36
        i32.load offset=12
        call_indirect (type 1)
        i32.eqz
        br_if 1 (;@1;)
      end
      i32.const 1
      local.set 0
    end
    local.get 3
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core3ops8function6FnOnce9call_once17h0d605c8afa86665cE (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    drop
    loop (result i32)  ;; label = @1
      br 0 (;@1;)
    end
  )
  (func $_ZN4core9panicking18panic_bounds_check17h8c564c2b20bece92E (type 11) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 44
    i32.add
    i32.const 1
    i32.store
    local.get 3
    i64.const 2
    i64.store offset=12 align=4
    local.get 3
    i32.const 1049092
    i32.store offset=8
    local.get 3
    i32.const 1
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.store offset=40
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
    unreachable
  )
  (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h110f0df6209a13b2E (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load32_u
    i32.const 1
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417hcb4d540e45bed3d8E
  )
  (func $_ZN4core3fmt3num3imp7fmt_u6417hcb4d540e45bed3d8E (type 13) (param i64 i32 i32) (result i32)
    (local i32 i32 i64 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i32.const 39
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i64.const 10000
        i64.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 5
        br 1 (;@1;)
      end
      i32.const 39
      local.set 4
      loop  ;; label = @2
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.add
        local.tee 6
        i32.const -4
        i32.add
        local.get 0
        i64.const 10000
        i64.div_u
        local.tee 5
        i64.const -10000
        i64.mul
        local.get 0
        i64.add
        i32.wrap_i64
        local.tee 7
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 8
        i32.const 1
        i32.shl
        i32.const 1049450
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 6
        i32.const -2
        i32.add
        local.get 8
        i32.const -100
        i32.mul
        local.get 7
        i32.add
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.const 1049450
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 4
        i32.const -4
        i32.add
        local.set 4
        local.get 0
        i64.const 99999999
        i64.gt_u
        local.set 6
        local.get 5
        local.set 0
        local.get 6
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      local.get 5
      i32.wrap_i64
      local.tee 6
      i32.const 99
      i32.le_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 9
      i32.add
      local.get 4
      i32.const -2
      i32.add
      local.tee 4
      i32.add
      local.get 5
      i32.wrap_i64
      local.tee 7
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee 6
      i32.const -100
      i32.mul
      local.get 7
      i32.add
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.const 1049450
      i32.add
      i32.load16_u align=1
      i32.store16 align=1
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 6
        i32.const 10
        i32.lt_u
        br_if 0 (;@2;)
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.const -2
        i32.add
        local.tee 4
        i32.add
        local.get 6
        i32.const 1
        i32.shl
        i32.const 1049450
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        br 1 (;@1;)
      end
      local.get 3
      i32.const 9
      i32.add
      local.get 4
      i32.const -1
      i32.add
      local.tee 4
      i32.add
      local.get 6
      i32.const 48
      i32.add
      i32.store8
    end
    local.get 2
    local.get 1
    i32.const 1053116
    i32.const 0
    local.get 3
    i32.const 9
    i32.add
    local.get 4
    i32.add
    i32.const 39
    local.get 4
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h88d3d7a17289fb2aE
    local.set 4
    local.get 3
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 4
  )
  (func $rust_begin_unwind (type 0) (param i32)
    (local i32)
    local.get 0
    i32.load offset=12
    local.set 1
    local.get 0
    i32.load offset=8
    call $_ZN4core6option15Option$LT$T$GT$6unwrap17hef65611489e1fddeE
    local.get 0
    local.get 1
    call $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h8e8491797c8758c1E
    unreachable
  )
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h3f4d0c27f5d87265E (type 3) (param i32) (result i64)
    i64.const -2167809825003555871
  )
  (func $_ZN4core3fmt9Formatter12pad_integral17h88d3d7a17289fb2aE (type 14) (param i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        i32.const 43
        i32.const 1114112
        local.get 0
        i32.load
        local.tee 6
        i32.const 1
        i32.and
        local.tee 1
        select
        local.set 7
        local.get 1
        local.get 5
        i32.add
        local.set 8
        br 1 (;@1;)
      end
      local.get 5
      i32.const 1
      i32.add
      local.set 8
      local.get 0
      i32.load
      local.set 6
      i32.const 45
      local.set 7
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 6
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          br_if 0 (;@3;)
          i32.const 0
          local.set 9
          br 1 (;@2;)
        end
        block  ;; label = @3
          local.get 3
          i32.const 3
          i32.and
          local.tee 10
          br_if 0 (;@3;)
          br 1 (;@2;)
        end
        i32.const 0
        local.set 9
        local.get 2
        local.set 1
        loop  ;; label = @3
          local.get 9
          local.get 1
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 9
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 10
          i32.const -1
          i32.add
          local.tee 10
          br_if 0 (;@3;)
        end
      end
      local.get 9
      local.get 8
      i32.add
      local.set 8
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=8
        br_if 0 (;@2;)
        i32.const 1
        local.set 1
        local.get 0
        local.get 7
        local.get 2
        local.get 3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h71de40f5dcad219aE
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 4
        local.get 5
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        return
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.const 12
                i32.add
                i32.load
                local.tee 9
                local.get 8
                i32.le_u
                br_if 0 (;@6;)
                local.get 6
                i32.const 8
                i32.and
                br_if 4 (;@2;)
                i32.const 0
                local.set 1
                local.get 9
                local.get 8
                i32.sub
                local.tee 10
                local.set 6
                i32.const 1
                local.get 0
                i32.load8_u offset=32
                local.tee 9
                local.get 9
                i32.const 3
                i32.eq
                select
                i32.const 3
                i32.and
                br_table 3 (;@3;) 1 (;@5;) 2 (;@4;) 3 (;@3;)
              end
              i32.const 1
              local.set 1
              local.get 0
              local.get 7
              local.get 2
              local.get 3
              call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h71de40f5dcad219aE
              br_if 4 (;@1;)
              local.get 0
              i32.load offset=24
              local.get 4
              local.get 5
              local.get 0
              i32.const 28
              i32.add
              i32.load
              i32.load offset=12
              call_indirect (type 1)
              return
            end
            i32.const 0
            local.set 6
            local.get 10
            local.set 1
            br 1 (;@3;)
          end
          local.get 10
          i32.const 1
          i32.shr_u
          local.set 1
          local.get 10
          i32.const 1
          i32.add
          i32.const 1
          i32.shr_u
          local.set 6
        end
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 0
        i32.const 28
        i32.add
        i32.load
        local.set 10
        local.get 0
        i32.load offset=4
        local.set 9
        local.get 0
        i32.load offset=24
        local.set 8
        block  ;; label = @3
          loop  ;; label = @4
            local.get 1
            i32.const -1
            i32.add
            local.tee 1
            i32.eqz
            br_if 1 (;@3;)
            local.get 8
            local.get 9
            local.get 10
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          i32.const 1
          return
        end
        i32.const 1
        local.set 1
        local.get 9
        i32.const 1114112
        i32.eq
        br_if 1 (;@1;)
        local.get 0
        local.get 7
        local.get 2
        local.get 3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h71de40f5dcad219aE
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 4
        local.get 5
        local.get 0
        i32.load offset=28
        i32.load offset=12
        call_indirect (type 1)
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=28
        local.set 10
        local.get 0
        i32.load offset=24
        local.set 0
        i32.const 0
        local.set 1
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              local.get 6
              local.get 1
              i32.ne
              br_if 0 (;@5;)
              local.get 6
              local.set 1
              br 2 (;@3;)
            end
            local.get 1
            i32.const 1
            i32.add
            local.set 1
            local.get 0
            local.get 9
            local.get 10
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          local.get 1
          i32.const -1
          i32.add
          local.set 1
        end
        local.get 1
        local.get 6
        i32.lt_u
        local.set 1
        br 1 (;@1;)
      end
      local.get 0
      i32.load offset=4
      local.set 6
      local.get 0
      i32.const 48
      i32.store offset=4
      local.get 0
      i32.load8_u offset=32
      local.set 11
      i32.const 1
      local.set 1
      local.get 0
      i32.const 1
      i32.store8 offset=32
      local.get 0
      local.get 7
      local.get 2
      local.get 3
      call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h71de40f5dcad219aE
      br_if 0 (;@1;)
      i32.const 0
      local.set 1
      local.get 9
      local.get 8
      i32.sub
      local.tee 10
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 1
            local.get 0
            i32.load8_u offset=32
            local.tee 9
            local.get 9
            i32.const 3
            i32.eq
            select
            i32.const 3
            i32.and
            br_table 2 (;@2;) 0 (;@4;) 1 (;@3;) 2 (;@2;)
          end
          i32.const 0
          local.set 3
          local.get 10
          local.set 1
          br 1 (;@2;)
        end
        local.get 10
        i32.const 1
        i32.shr_u
        local.set 1
        local.get 10
        i32.const 1
        i32.add
        i32.const 1
        i32.shr_u
        local.set 3
      end
      local.get 1
      i32.const 1
      i32.add
      local.set 1
      local.get 0
      i32.const 28
      i32.add
      i32.load
      local.set 10
      local.get 0
      i32.load offset=4
      local.set 9
      local.get 0
      i32.load offset=24
      local.set 8
      block  ;; label = @2
        loop  ;; label = @3
          local.get 1
          i32.const -1
          i32.add
          local.tee 1
          i32.eqz
          br_if 1 (;@2;)
          local.get 8
          local.get 9
          local.get 10
          i32.load offset=16
          call_indirect (type 2)
          i32.eqz
          br_if 0 (;@3;)
        end
        i32.const 1
        return
      end
      i32.const 1
      local.set 1
      local.get 9
      i32.const 1114112
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=24
      local.get 4
      local.get 5
      local.get 0
      i32.load offset=28
      i32.load offset=12
      call_indirect (type 1)
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=28
      local.set 1
      local.get 0
      i32.load offset=24
      local.set 8
      i32.const 0
      local.set 10
      block  ;; label = @2
        loop  ;; label = @3
          local.get 3
          local.get 10
          i32.eq
          br_if 1 (;@2;)
          local.get 10
          i32.const 1
          i32.add
          local.set 10
          local.get 8
          local.get 9
          local.get 1
          i32.load offset=16
          call_indirect (type 2)
          i32.eqz
          br_if 0 (;@3;)
        end
        i32.const 1
        local.set 1
        local.get 10
        i32.const -1
        i32.add
        local.get 3
        i32.lt_u
        br_if 1 (;@1;)
      end
      local.get 0
      local.get 11
      i32.store8 offset=32
      local.get 0
      local.get 6
      i32.store offset=4
      i32.const 0
      return
    end
    local.get 1
  )
  (func $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h71de40f5dcad219aE (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 1114112
          i32.eq
          br_if 0 (;@3;)
          i32.const 1
          local.set 4
          local.get 0
          i32.load offset=24
          local.get 1
          local.get 0
          i32.const 28
          i32.add
          i32.load
          i32.load offset=16
          call_indirect (type 2)
          br_if 1 (;@2;)
        end
        local.get 2
        br_if 1 (;@1;)
        i32.const 0
        local.set 4
      end
      local.get 4
      return
    end
    local.get 0
    i32.load offset=24
    local.get 2
    local.get 3
    local.get 0
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 1)
  )
  (func $_ZN4core5slice5index24slice_end_index_len_fail17h53a611cf4b2e1c2bE (type 11) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 44
    i32.add
    i32.const 1
    i32.store
    local.get 3
    i64.const 2
    i64.store offset=12 align=4
    local.get 3
    i32.const 1049908
    i32.store offset=8
    local.get 3
    i32.const 1
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=40
    local.get 3
    local.get 3
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
    unreachable
  )
  (func $_ZN4core3fmt9Formatter3pad17h0b95dbdf7f5bfcd3E (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.load offset=16
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=8
                local.tee 4
                i32.const 1
                i32.eq
                br_if 0 (;@6;)
                local.get 3
                i32.const 1
                i32.ne
                br_if 1 (;@5;)
              end
              local.get 3
              i32.const 1
              i32.ne
              br_if 3 (;@2;)
              local.get 1
              local.get 2
              i32.add
              local.set 5
              local.get 0
              i32.const 20
              i32.add
              i32.load
              local.tee 6
              br_if 1 (;@4;)
              i32.const 0
              local.set 7
              local.get 1
              local.set 8
              br 2 (;@3;)
            end
            local.get 0
            i32.load offset=24
            local.get 1
            local.get 2
            local.get 0
            i32.const 28
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type 1)
            local.set 3
            br 3 (;@1;)
          end
          i32.const 0
          local.set 7
          local.get 1
          local.set 8
          loop  ;; label = @4
            local.get 8
            local.tee 3
            local.get 5
            i32.eq
            br_if 2 (;@2;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.load8_s
                local.tee 8
                i32.const -1
                i32.le_s
                br_if 0 (;@6;)
                local.get 3
                i32.const 1
                i32.add
                local.set 8
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 8
                i32.const -32
                i32.ge_u
                br_if 0 (;@6;)
                local.get 3
                i32.const 2
                i32.add
                local.set 8
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 8
                i32.const -16
                i32.ge_u
                br_if 0 (;@6;)
                local.get 3
                i32.const 3
                i32.add
                local.set 8
                br 1 (;@5;)
              end
              local.get 3
              i32.load8_u offset=2
              i32.const 63
              i32.and
              i32.const 6
              i32.shl
              local.get 3
              i32.load8_u offset=1
              i32.const 63
              i32.and
              i32.const 12
              i32.shl
              i32.or
              local.get 3
              i32.load8_u offset=3
              i32.const 63
              i32.and
              i32.or
              local.get 8
              i32.const 255
              i32.and
              i32.const 18
              i32.shl
              i32.const 1835008
              i32.and
              i32.or
              i32.const 1114112
              i32.eq
              br_if 3 (;@2;)
              local.get 3
              i32.const 4
              i32.add
              local.set 8
            end
            local.get 7
            local.get 3
            i32.sub
            local.get 8
            i32.add
            local.set 7
            local.get 6
            i32.const -1
            i32.add
            local.tee 6
            br_if 0 (;@4;)
          end
        end
        local.get 8
        local.get 5
        i32.eq
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 8
          i32.load8_s
          local.tee 3
          i32.const -1
          i32.gt_s
          br_if 0 (;@3;)
          local.get 3
          i32.const -32
          i32.lt_u
          br_if 0 (;@3;)
          local.get 3
          i32.const -16
          i32.lt_u
          br_if 0 (;@3;)
          local.get 8
          i32.load8_u offset=2
          i32.const 63
          i32.and
          i32.const 6
          i32.shl
          local.get 8
          i32.load8_u offset=1
          i32.const 63
          i32.and
          i32.const 12
          i32.shl
          i32.or
          local.get 8
          i32.load8_u offset=3
          i32.const 63
          i32.and
          i32.or
          local.get 3
          i32.const 255
          i32.and
          i32.const 18
          i32.shl
          i32.const 1835008
          i32.and
          i32.or
          i32.const 1114112
          i32.eq
          br_if 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              br_if 0 (;@5;)
              i32.const 0
              local.set 8
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 7
              local.get 2
              i32.lt_u
              br_if 0 (;@5;)
              i32.const 0
              local.set 3
              local.get 2
              local.set 8
              local.get 7
              local.get 2
              i32.eq
              br_if 1 (;@4;)
              br 2 (;@3;)
            end
            i32.const 0
            local.set 3
            local.get 7
            local.set 8
            local.get 1
            local.get 7
            i32.add
            i32.load8_s
            i32.const -64
            i32.lt_s
            br_if 1 (;@3;)
          end
          local.get 8
          local.set 7
          local.get 1
          local.set 3
        end
        local.get 7
        local.get 2
        local.get 3
        select
        local.set 2
        local.get 3
        local.get 1
        local.get 3
        select
        local.set 1
      end
      block  ;; label = @2
        local.get 4
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 2
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        return
      end
      local.get 0
      i32.const 12
      i32.add
      i32.load
      local.set 9
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.const 16
              i32.lt_u
              br_if 0 (;@5;)
              local.get 2
              local.get 1
              i32.const 3
              i32.add
              i32.const -4
              i32.and
              local.tee 3
              local.get 1
              i32.sub
              local.tee 5
              i32.lt_u
              br_if 2 (;@3;)
              local.get 5
              i32.const 4
              i32.gt_u
              br_if 2 (;@3;)
              local.get 2
              local.get 5
              i32.sub
              local.tee 4
              i32.const 4
              i32.lt_u
              br_if 2 (;@3;)
              local.get 4
              i32.const 3
              i32.and
              local.set 10
              i32.const 0
              local.set 11
              i32.const 0
              local.set 8
              block  ;; label = @6
                local.get 5
                i32.eqz
                br_if 0 (;@6;)
                local.get 5
                i32.const 3
                i32.and
                local.set 7
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 3
                    local.get 1
                    i32.const -1
                    i32.xor
                    i32.add
                    i32.const 3
                    i32.ge_u
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 8
                    local.get 1
                    local.set 3
                    br 1 (;@7;)
                  end
                  local.get 5
                  i32.const -4
                  i32.and
                  local.set 6
                  i32.const 0
                  local.set 8
                  local.get 1
                  local.set 3
                  loop  ;; label = @8
                    local.get 8
                    local.get 3
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.get 3
                    i32.const 1
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.get 3
                    i32.const 2
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.get 3
                    i32.const 3
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.set 8
                    local.get 3
                    i32.const 4
                    i32.add
                    local.set 3
                    local.get 6
                    i32.const -4
                    i32.add
                    local.tee 6
                    br_if 0 (;@8;)
                  end
                end
                local.get 7
                i32.eqz
                br_if 0 (;@6;)
                loop  ;; label = @7
                  local.get 8
                  local.get 3
                  i32.load8_s
                  i32.const -65
                  i32.gt_s
                  i32.add
                  local.set 8
                  local.get 3
                  i32.const 1
                  i32.add
                  local.set 3
                  local.get 7
                  i32.const -1
                  i32.add
                  local.tee 7
                  br_if 0 (;@7;)
                end
              end
              local.get 1
              local.get 5
              i32.add
              local.set 3
              block  ;; label = @6
                local.get 10
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                local.get 4
                i32.const -4
                i32.and
                i32.add
                local.tee 7
                i32.load8_s
                i32.const -65
                i32.gt_s
                local.set 11
                local.get 10
                i32.const 1
                i32.eq
                br_if 0 (;@6;)
                local.get 11
                local.get 7
                i32.load8_s offset=1
                i32.const -65
                i32.gt_s
                i32.add
                local.set 11
                local.get 10
                i32.const 2
                i32.eq
                br_if 0 (;@6;)
                local.get 11
                local.get 7
                i32.load8_s offset=2
                i32.const -65
                i32.gt_s
                i32.add
                local.set 11
              end
              local.get 4
              i32.const 2
              i32.shr_u
              local.set 4
              local.get 11
              local.get 8
              i32.add
              local.set 6
              loop  ;; label = @6
                local.get 3
                local.set 10
                local.get 4
                i32.eqz
                br_if 4 (;@2;)
                local.get 4
                i32.const 192
                local.get 4
                i32.const 192
                i32.lt_u
                select
                local.tee 11
                i32.const 3
                i32.and
                local.set 12
                local.get 11
                i32.const 2
                i32.shl
                local.set 13
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 11
                    i32.const 252
                    i32.and
                    local.tee 14
                    i32.const 2
                    i32.shl
                    local.tee 3
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 8
                    br 1 (;@7;)
                  end
                  local.get 10
                  local.get 3
                  i32.add
                  local.set 5
                  i32.const 0
                  local.set 8
                  local.get 10
                  local.set 3
                  loop  ;; label = @8
                    local.get 3
                    i32.const 12
                    i32.add
                    i32.load
                    local.tee 7
                    i32.const -1
                    i32.xor
                    i32.const 7
                    i32.shr_u
                    local.get 7
                    i32.const 6
                    i32.shr_u
                    i32.or
                    i32.const 16843009
                    i32.and
                    local.get 3
                    i32.const 8
                    i32.add
                    i32.load
                    local.tee 7
                    i32.const -1
                    i32.xor
                    i32.const 7
                    i32.shr_u
                    local.get 7
                    i32.const 6
                    i32.shr_u
                    i32.or
                    i32.const 16843009
                    i32.and
                    local.get 3
                    i32.const 4
                    i32.add
                    i32.load
                    local.tee 7
                    i32.const -1
                    i32.xor
                    i32.const 7
                    i32.shr_u
                    local.get 7
                    i32.const 6
                    i32.shr_u
                    i32.or
                    i32.const 16843009
                    i32.and
                    local.get 3
                    i32.load
                    local.tee 7
                    i32.const -1
                    i32.xor
                    i32.const 7
                    i32.shr_u
                    local.get 7
                    i32.const 6
                    i32.shr_u
                    i32.or
                    i32.const 16843009
                    i32.and
                    local.get 8
                    i32.add
                    i32.add
                    i32.add
                    i32.add
                    local.set 8
                    local.get 3
                    i32.const 16
                    i32.add
                    local.tee 3
                    local.get 5
                    i32.ne
                    br_if 0 (;@8;)
                  end
                end
                local.get 10
                local.get 13
                i32.add
                local.set 3
                local.get 4
                local.get 11
                i32.sub
                local.set 4
                local.get 8
                i32.const 8
                i32.shr_u
                i32.const 16711935
                i32.and
                local.get 8
                i32.const 16711935
                i32.and
                i32.add
                i32.const 65537
                i32.mul
                i32.const 16
                i32.shr_u
                local.get 6
                i32.add
                local.set 6
                local.get 12
                i32.eqz
                br_if 0 (;@6;)
              end
              local.get 10
              local.get 14
              i32.const 2
              i32.shl
              i32.add
              local.set 3
              local.get 12
              i32.const 1073741823
              i32.add
              local.tee 11
              i32.const 1073741823
              i32.and
              local.tee 8
              i32.const 1
              i32.add
              local.tee 7
              i32.const 3
              i32.and
              local.set 4
              block  ;; label = @6
                local.get 8
                i32.const 3
                i32.ge_u
                br_if 0 (;@6;)
                i32.const 0
                local.set 8
                br 2 (;@4;)
              end
              local.get 7
              i32.const 2147483644
              i32.and
              local.set 7
              i32.const 0
              local.set 8
              loop  ;; label = @6
                local.get 3
                i32.const 12
                i32.add
                i32.load
                local.tee 5
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get 5
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get 3
                i32.const 8
                i32.add
                i32.load
                local.tee 5
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get 5
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get 3
                i32.const 4
                i32.add
                i32.load
                local.tee 5
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get 5
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get 3
                i32.load
                local.tee 5
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get 5
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get 8
                i32.add
                i32.add
                i32.add
                i32.add
                local.set 8
                local.get 3
                i32.const 16
                i32.add
                local.set 3
                local.get 7
                i32.const -4
                i32.add
                local.tee 7
                br_if 0 (;@6;)
                br 2 (;@4;)
              end
            end
            block  ;; label = @5
              local.get 2
              br_if 0 (;@5;)
              i32.const 0
              local.set 6
              br 3 (;@2;)
            end
            local.get 2
            i32.const 3
            i32.and
            local.set 8
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.const -1
                i32.add
                i32.const 3
                i32.ge_u
                br_if 0 (;@6;)
                i32.const 0
                local.set 6
                local.get 1
                local.set 3
                br 1 (;@5;)
              end
              local.get 2
              i32.const -4
              i32.and
              local.set 7
              i32.const 0
              local.set 6
              local.get 1
              local.set 3
              loop  ;; label = @6
                local.get 6
                local.get 3
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get 3
                i32.const 1
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get 3
                i32.const 2
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get 3
                i32.const 3
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.set 6
                local.get 3
                i32.const 4
                i32.add
                local.set 3
                local.get 7
                i32.const -4
                i32.add
                local.tee 7
                br_if 0 (;@6;)
              end
            end
            local.get 8
            i32.eqz
            br_if 2 (;@2;)
            loop  ;; label = @5
              local.get 6
              local.get 3
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.set 6
              local.get 3
              i32.const 1
              i32.add
              local.set 3
              local.get 8
              i32.const -1
              i32.add
              local.tee 8
              br_if 0 (;@5;)
              br 3 (;@2;)
            end
          end
          block  ;; label = @4
            local.get 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 11
            i32.const -1073741823
            i32.add
            local.set 7
            loop  ;; label = @5
              local.get 3
              i32.load
              local.tee 5
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 5
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 8
              i32.add
              local.set 8
              local.get 3
              i32.const 4
              i32.add
              local.set 3
              local.get 7
              i32.const -1
              i32.add
              local.tee 7
              br_if 0 (;@5;)
            end
          end
          local.get 8
          i32.const 8
          i32.shr_u
          i32.const 16711935
          i32.and
          local.get 8
          i32.const 16711935
          i32.and
          i32.add
          i32.const 65537
          i32.mul
          i32.const 16
          i32.shr_u
          local.get 6
          i32.add
          local.set 6
          br 1 (;@2;)
        end
        local.get 2
        i32.const -4
        i32.and
        local.set 8
        i32.const 0
        local.set 6
        local.get 1
        local.set 3
        loop  ;; label = @3
          local.get 6
          local.get 3
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 3
          i32.const 1
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 3
          i32.const 2
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 3
          i32.const 3
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 6
          local.get 3
          i32.const 4
          i32.add
          local.set 3
          local.get 8
          i32.const -4
          i32.add
          local.tee 8
          br_if 0 (;@3;)
        end
        local.get 2
        i32.const 3
        i32.and
        local.tee 7
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 8
        loop  ;; label = @3
          local.get 6
          local.get 3
          local.get 8
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 6
          local.get 7
          local.get 8
          i32.const 1
          i32.add
          local.tee 8
          i32.ne
          br_if 0 (;@3;)
        end
      end
      block  ;; label = @2
        local.get 9
        local.get 6
        i32.le_u
        br_if 0 (;@2;)
        i32.const 0
        local.set 3
        local.get 9
        local.get 6
        i32.sub
        local.tee 8
        local.set 5
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              i32.const 0
              local.get 0
              i32.load8_u offset=32
              local.tee 7
              local.get 7
              i32.const 3
              i32.eq
              select
              i32.const 3
              i32.and
              br_table 2 (;@3;) 0 (;@5;) 1 (;@4;) 2 (;@3;)
            end
            i32.const 0
            local.set 5
            local.get 8
            local.set 3
            br 1 (;@3;)
          end
          local.get 8
          i32.const 1
          i32.shr_u
          local.set 3
          local.get 8
          i32.const 1
          i32.add
          i32.const 1
          i32.shr_u
          local.set 5
        end
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 0
        i32.const 28
        i32.add
        i32.load
        local.set 7
        local.get 0
        i32.load offset=4
        local.set 8
        local.get 0
        i32.load offset=24
        local.set 6
        block  ;; label = @3
          loop  ;; label = @4
            local.get 3
            i32.const -1
            i32.add
            local.tee 3
            i32.eqz
            br_if 1 (;@3;)
            local.get 6
            local.get 8
            local.get 7
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          i32.const 1
          return
        end
        i32.const 1
        local.set 3
        local.get 8
        i32.const 1114112
        i32.eq
        br_if 1 (;@1;)
        local.get 6
        local.get 1
        local.get 2
        local.get 7
        i32.load offset=12
        call_indirect (type 1)
        br_if 1 (;@1;)
        i32.const 0
        local.set 3
        loop  ;; label = @3
          block  ;; label = @4
            local.get 5
            local.get 3
            i32.ne
            br_if 0 (;@4;)
            local.get 5
            local.get 5
            i32.lt_u
            return
          end
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 6
          local.get 8
          local.get 7
          i32.load offset=16
          call_indirect (type 2)
          i32.eqz
          br_if 0 (;@3;)
        end
        local.get 3
        i32.const -1
        i32.add
        local.get 5
        i32.lt_u
        return
      end
      local.get 0
      i32.load offset=24
      local.get 1
      local.get 2
      local.get 0
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 1)
      return
    end
    local.get 3
  )
  (func $_ZN4core9panicking5panic17h66d99d0b614b1c7cE (type 11) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 20
    i32.add
    i32.const 0
    i32.store
    local.get 3
    i32.const 1053116
    i32.store offset=16
    local.get 3
    i64.const 1
    i64.store offset=4 align=4
    local.get 3
    local.get 1
    i32.store offset=28
    local.get 3
    local.get 0
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 24
    i32.add
    i32.store
    local.get 3
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
    unreachable
  )
  (func $_ZN4core9panicking19assert_failed_inner17h8160b73ca348502bE (type 15) (param i32 i32 i32 i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 112
    i32.sub
    local.tee 7
    global.set $__stack_pointer
    local.get 7
    local.get 2
    i32.store offset=12
    local.get 7
    local.get 1
    i32.store offset=8
    local.get 7
    local.get 4
    i32.store offset=20
    local.get 7
    local.get 3
    i32.store offset=16
    local.get 7
    i32.const 2
    i32.store offset=28
    local.get 7
    i32.const 1049124
    i32.const 1049126
    local.get 0
    i32.const 255
    i32.and
    select
    i32.store offset=24
    block  ;; label = @1
      local.get 5
      i32.load
      br_if 0 (;@1;)
      local.get 7
      i32.const 56
      i32.add
      i32.const 20
      i32.add
      i32.const 3
      i32.store
      local.get 7
      i32.const 68
      i32.add
      i32.const 3
      i32.store
      local.get 7
      i32.const 88
      i32.add
      i32.const 20
      i32.add
      i32.const 3
      i32.store
      local.get 7
      i64.const 4
      i64.store offset=92 align=4
      local.get 7
      i32.const 1049224
      i32.store offset=88
      local.get 7
      i32.const 4
      i32.store offset=60
      local.get 7
      local.get 7
      i32.const 56
      i32.add
      i32.store offset=104
      local.get 7
      local.get 7
      i32.const 16
      i32.add
      i32.store offset=72
      local.get 7
      local.get 7
      i32.const 8
      i32.add
      i32.store offset=64
      local.get 7
      local.get 7
      i32.const 24
      i32.add
      i32.store offset=56
      local.get 7
      i32.const 88
      i32.add
      local.get 6
      call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
      unreachable
    end
    local.get 7
    i32.const 32
    i32.add
    i32.const 16
    i32.add
    local.get 5
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 7
    i32.const 32
    i32.add
    i32.const 8
    i32.add
    local.get 5
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 7
    local.get 5
    i64.load align=4
    i64.store offset=32
    local.get 7
    i32.const 88
    i32.add
    i32.const 20
    i32.add
    i32.const 4
    i32.store
    local.get 7
    i32.const 84
    i32.add
    i32.const 5
    i32.store
    local.get 7
    i32.const 56
    i32.add
    i32.const 20
    i32.add
    i32.const 3
    i32.store
    local.get 7
    i32.const 68
    i32.add
    i32.const 3
    i32.store
    local.get 7
    i64.const 4
    i64.store offset=92 align=4
    local.get 7
    i32.const 1049188
    i32.store offset=88
    local.get 7
    i32.const 4
    i32.store offset=60
    local.get 7
    local.get 7
    i32.const 56
    i32.add
    i32.store offset=104
    local.get 7
    local.get 7
    i32.const 32
    i32.add
    i32.store offset=80
    local.get 7
    local.get 7
    i32.const 16
    i32.add
    i32.store offset=72
    local.get 7
    local.get 7
    i32.const 8
    i32.add
    i32.store offset=64
    local.get 7
    local.get 7
    i32.const 24
    i32.add
    i32.store offset=56
    local.get 7
    i32.const 88
    i32.add
    local.get 6
    call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
    unreachable
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h3c603a185463c3edE (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 2)
  )
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h0f2edd5418c91d47E (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_ZN4core3fmt9Formatter3pad17h0b95dbdf7f5bfcd3E
  )
  (func $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17h26ce3e7a201dd52aE (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 1
    i32.const 28
    i32.add
    i32.load
    local.set 3
    local.get 1
    i32.load offset=24
    local.set 1
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 0
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 0
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 0
    i64.load align=4
    i64.store offset=8
    local.get 1
    local.get 3
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 0
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core5slice5index22slice_index_order_fail17hd6544278d56731b9E (type 11) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 44
    i32.add
    i32.const 1
    i32.store
    local.get 3
    i64.const 2
    i64.store offset=12 align=4
    local.get 3
    i32.const 1049960
    i32.store offset=8
    local.get 3
    i32.const 1
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=40
    local.get 3
    local.get 3
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
    unreachable
  )
  (func $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17h6605a725a9142927E (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.load offset=24
    i32.const 1048952
    i32.const 14
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 1)
  )
  (func $_ZN4core6option13expect_failed17h492d2d279a279116E (type 4)
    call $_ZN4core9panicking9panic_str17ha13f0c5548e3f147E
    unreachable
  )
  (func $_ZN4core9panicking9panic_str17ha13f0c5548e3f147E (type 4)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 94
    i32.store offset=12
    local.get 0
    i32.const 1053116
    i32.store offset=8
    local.get 0
    i32.const 8
    i32.add
    call $_ZN4core9panicking13panic_display17hcb4b42a07a33cc05E
    unreachable
  )
  (func $_ZN4core9panicking13panic_display17hcb4b42a07a33cc05E (type 0) (param i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    i32.const 20
    i32.add
    i32.const 1
    i32.store
    local.get 1
    i64.const 1
    i64.store offset=4 align=4
    local.get 1
    i32.const 1053444
    i32.store
    local.get 1
    i32.const 4
    i32.store offset=28
    local.get 1
    local.get 0
    i32.store offset=24
    local.get 1
    local.get 1
    i32.const 24
    i32.add
    i32.store offset=16
    local.get 1
    i32.const 1053240
    call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
    unreachable
  )
  (func $_ZN73_$LT$core..panic..panic_info..PanicInfo$u20$as$u20$core..fmt..Display$GT$3fmt17hf6025793d8663473E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 1
    local.set 3
    block  ;; label = @1
      local.get 1
      i32.load offset=24
      local.tee 4
      i32.const 1049008
      i32.const 12
      local.get 1
      i32.const 28
      i32.add
      i32.load
      local.tee 1
      i32.load offset=12
      call_indirect (type 1)
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=8
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          i32.store offset=12
          local.get 2
          i32.const 6
          i32.store offset=20
          local.get 2
          local.get 2
          i32.const 12
          i32.add
          i32.store offset=16
          i32.const 1
          local.set 3
          local.get 2
          i32.const 60
          i32.add
          i32.const 1
          i32.store
          local.get 2
          i64.const 2
          i64.store offset=44 align=4
          local.get 2
          i32.const 1049024
          i32.store offset=40
          local.get 2
          local.get 2
          i32.const 16
          i32.add
          i32.store offset=56
          local.get 4
          local.get 1
          local.get 2
          i32.const 40
          i32.add
          call $_ZN4core3fmt5write17h8776c655b56f9e02E
          i32.eqz
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        local.get 0
        i32.load
        local.tee 3
        local.get 0
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 3)
        i64.const -8867930603987265711
        i64.ne
        br_if 0 (;@2;)
        local.get 2
        local.get 3
        i32.store offset=12
        local.get 2
        i32.const 7
        i32.store offset=20
        local.get 2
        local.get 2
        i32.const 12
        i32.add
        i32.store offset=16
        i32.const 1
        local.set 3
        local.get 2
        i32.const 60
        i32.add
        i32.const 1
        i32.store
        local.get 2
        i64.const 2
        i64.store offset=44 align=4
        local.get 2
        i32.const 1049024
        i32.store offset=40
        local.get 2
        local.get 2
        i32.const 16
        i32.add
        i32.store offset=56
        local.get 4
        local.get 1
        local.get 2
        i32.const 40
        i32.add
        call $_ZN4core3fmt5write17h8776c655b56f9e02E
        br_if 1 (;@1;)
      end
      local.get 0
      i32.load offset=12
      local.set 3
      local.get 2
      i32.const 16
      i32.add
      i32.const 20
      i32.add
      i32.const 1
      i32.store
      local.get 2
      i32.const 16
      i32.add
      i32.const 12
      i32.add
      i32.const 1
      i32.store
      local.get 2
      local.get 3
      i32.const 12
      i32.add
      i32.store offset=32
      local.get 2
      local.get 3
      i32.const 8
      i32.add
      i32.store offset=24
      local.get 2
      i32.const 4
      i32.store offset=20
      local.get 2
      local.get 3
      i32.store offset=16
      local.get 2
      i32.const 40
      i32.add
      i32.const 20
      i32.add
      i32.const 3
      i32.store
      local.get 2
      i64.const 3
      i64.store offset=44 align=4
      local.get 2
      i32.const 1048968
      i32.store offset=40
      local.get 2
      local.get 2
      i32.const 16
      i32.add
      i32.store offset=56
      local.get 4
      local.get 1
      local.get 2
      i32.const 40
      i32.add
      call $_ZN4core3fmt5write17h8776c655b56f9e02E
      local.set 3
    end
    local.get 2
    i32.const 64
    i32.add
    global.set $__stack_pointer
    local.get 3
  )
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h7fb80c02ed96c242E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 1
    i32.const 28
    i32.add
    i32.load
    local.set 3
    local.get 1
    i32.load offset=24
    local.set 4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 0
    i32.load
    local.tee 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 4
    local.get 3
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h174d90cd314a320fE (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.tee 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_ZN4core3fmt9Formatter3pad17h0b95dbdf7f5bfcd3E
  )
  (func $_ZN4core6result13unwrap_failed17h1b005d733a1499a6E (type 12) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    local.get 1
    i32.store offset=12
    local.get 5
    local.get 0
    i32.store offset=8
    local.get 5
    local.get 3
    i32.store offset=20
    local.get 5
    local.get 2
    i32.store offset=16
    local.get 5
    i32.const 44
    i32.add
    i32.const 2
    i32.store
    local.get 5
    i32.const 60
    i32.add
    i32.const 3
    i32.store
    local.get 5
    i64.const 2
    i64.store offset=28 align=4
    local.get 5
    i32.const 1049256
    i32.store offset=24
    local.get 5
    i32.const 4
    i32.store offset=52
    local.get 5
    local.get 5
    i32.const 48
    i32.add
    i32.store offset=40
    local.get 5
    local.get 5
    i32.const 16
    i32.add
    i32.store offset=56
    local.get 5
    local.get 5
    i32.const 8
    i32.add
    i32.store offset=48
    local.get 5
    i32.const 24
    i32.add
    local.get 4
    call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
    unreachable
  )
  (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hb1584ad756e22b8fE (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=4
        local.set 3
        local.get 0
        i32.load
        local.set 4
        local.get 0
        i32.load offset=8
        local.set 5
        loop  ;; label = @3
          block  ;; label = @4
            local.get 5
            i32.load8_u
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            i32.const 1049296
            i32.const 4
            local.get 3
            i32.load offset=12
            call_indirect (type 1)
            i32.eqz
            br_if 0 (;@4;)
            i32.const 1
            return
          end
          i32.const 0
          local.set 6
          local.get 2
          local.set 7
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  loop  ;; label = @8
                    local.get 1
                    local.get 6
                    i32.add
                    local.set 8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 7
                              i32.const 8
                              i32.lt_u
                              br_if 0 (;@13;)
                              block  ;; label = @14
                                local.get 8
                                i32.const 3
                                i32.add
                                i32.const -4
                                i32.and
                                local.get 8
                                i32.sub
                                local.tee 0
                                br_if 0 (;@14;)
                                local.get 7
                                i32.const -8
                                i32.add
                                local.set 9
                                i32.const 0
                                local.set 0
                                br 3 (;@11;)
                              end
                              local.get 7
                              local.get 0
                              local.get 0
                              local.get 7
                              i32.gt_u
                              select
                              local.set 0
                              i32.const 0
                              local.set 10
                              loop  ;; label = @14
                                local.get 8
                                local.get 10
                                i32.add
                                i32.load8_u
                                i32.const 10
                                i32.eq
                                br_if 5 (;@9;)
                                local.get 0
                                local.get 10
                                i32.const 1
                                i32.add
                                local.tee 10
                                i32.eq
                                br_if 2 (;@12;)
                                br 0 (;@14;)
                              end
                            end
                            local.get 7
                            i32.eqz
                            br_if 5 (;@7;)
                            i32.const 0
                            local.set 10
                            local.get 8
                            i32.load8_u
                            i32.const 10
                            i32.eq
                            br_if 3 (;@9;)
                            local.get 7
                            i32.const 1
                            i32.eq
                            br_if 5 (;@7;)
                            i32.const 1
                            local.set 10
                            local.get 8
                            i32.load8_u offset=1
                            i32.const 10
                            i32.eq
                            br_if 3 (;@9;)
                            local.get 7
                            i32.const 2
                            i32.eq
                            br_if 5 (;@7;)
                            i32.const 2
                            local.set 10
                            local.get 8
                            i32.load8_u offset=2
                            i32.const 10
                            i32.eq
                            br_if 3 (;@9;)
                            local.get 7
                            i32.const 3
                            i32.eq
                            br_if 5 (;@7;)
                            i32.const 3
                            local.set 10
                            local.get 8
                            i32.load8_u offset=3
                            i32.const 10
                            i32.eq
                            br_if 3 (;@9;)
                            local.get 7
                            i32.const 4
                            i32.eq
                            br_if 5 (;@7;)
                            i32.const 4
                            local.set 10
                            local.get 8
                            i32.load8_u offset=4
                            i32.const 10
                            i32.eq
                            br_if 3 (;@9;)
                            local.get 7
                            i32.const 5
                            i32.eq
                            br_if 5 (;@7;)
                            i32.const 5
                            local.set 10
                            local.get 8
                            i32.load8_u offset=5
                            i32.const 10
                            i32.eq
                            br_if 3 (;@9;)
                            local.get 7
                            i32.const 6
                            i32.eq
                            br_if 5 (;@7;)
                            i32.const 6
                            local.set 10
                            local.get 8
                            i32.load8_u offset=6
                            i32.const 10
                            i32.ne
                            br_if 5 (;@7;)
                            br 3 (;@9;)
                          end
                          local.get 0
                          local.get 7
                          i32.const -8
                          i32.add
                          local.tee 9
                          i32.gt_u
                          br_if 1 (;@10;)
                        end
                        block  ;; label = @11
                          loop  ;; label = @12
                            local.get 8
                            local.get 0
                            i32.add
                            local.tee 10
                            i32.load
                            local.tee 11
                            i32.const -1
                            i32.xor
                            local.get 11
                            i32.const 168430090
                            i32.xor
                            i32.const -16843009
                            i32.add
                            i32.and
                            local.get 10
                            i32.const 4
                            i32.add
                            i32.load
                            local.tee 10
                            i32.const -1
                            i32.xor
                            local.get 10
                            i32.const 168430090
                            i32.xor
                            i32.const -16843009
                            i32.add
                            i32.and
                            i32.or
                            i32.const -2139062144
                            i32.and
                            br_if 1 (;@11;)
                            local.get 0
                            i32.const 8
                            i32.add
                            local.tee 0
                            local.get 9
                            i32.le_u
                            br_if 0 (;@12;)
                          end
                        end
                        local.get 0
                        local.get 7
                        i32.le_u
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 7
                        i32.const 1049776
                        call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
                        unreachable
                      end
                      local.get 0
                      local.get 7
                      i32.eq
                      br_if 2 (;@7;)
                      local.get 0
                      local.get 7
                      i32.sub
                      local.set 11
                      local.get 8
                      local.get 0
                      i32.add
                      local.set 8
                      i32.const 0
                      local.set 10
                      block  ;; label = @10
                        loop  ;; label = @11
                          local.get 8
                          local.get 10
                          i32.add
                          i32.load8_u
                          i32.const 10
                          i32.eq
                          br_if 1 (;@10;)
                          local.get 11
                          local.get 10
                          i32.const 1
                          i32.add
                          local.tee 10
                          i32.add
                          br_if 0 (;@11;)
                          br 4 (;@7;)
                        end
                      end
                      local.get 0
                      local.get 10
                      i32.add
                      local.set 10
                    end
                    block  ;; label = @9
                      local.get 10
                      local.get 6
                      i32.add
                      local.tee 0
                      i32.const 1
                      i32.add
                      local.tee 6
                      local.get 0
                      i32.lt_u
                      br_if 0 (;@9;)
                      local.get 2
                      local.get 6
                      i32.lt_u
                      br_if 0 (;@9;)
                      local.get 1
                      local.get 0
                      i32.add
                      i32.load8_u
                      i32.const 10
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 1
                      i32.store8
                      local.get 2
                      local.get 6
                      i32.le_u
                      br_if 3 (;@6;)
                      local.get 6
                      local.set 0
                      local.get 1
                      local.get 6
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 4 (;@5;)
                      br 5 (;@4;)
                    end
                    local.get 2
                    local.get 6
                    i32.sub
                    local.set 7
                    local.get 2
                    local.get 6
                    i32.ge_u
                    br_if 0 (;@8;)
                  end
                end
                local.get 5
                i32.const 0
                i32.store8
                local.get 2
                local.set 6
              end
              local.get 2
              local.set 0
              local.get 2
              local.get 6
              i32.eq
              br_if 1 (;@4;)
            end
            local.get 1
            local.get 2
            i32.const 0
            local.get 6
            i32.const 1049332
            call $_ZN4core3str16slice_error_fail17h1967a032a7ff118eE
            unreachable
          end
          block  ;; label = @4
            local.get 4
            local.get 1
            local.get 0
            local.get 3
            i32.load offset=12
            call_indirect (type 1)
            i32.eqz
            br_if 0 (;@4;)
            i32.const 1
            return
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              local.get 0
              i32.gt_u
              br_if 0 (;@5;)
              local.get 2
              local.get 0
              i32.eq
              br_if 1 (;@4;)
              br 4 (;@1;)
            end
            local.get 1
            local.get 0
            i32.add
            i32.load8_s
            i32.const -65
            i32.le_s
            br_if 3 (;@1;)
          end
          local.get 1
          local.get 0
          i32.add
          local.set 1
          local.get 2
          local.get 0
          i32.sub
          local.tee 2
          br_if 0 (;@3;)
        end
      end
      i32.const 0
      return
    end
    local.get 1
    local.get 2
    local.get 0
    local.get 2
    i32.const 1049348
    call $_ZN4core3str16slice_error_fail17h1967a032a7ff118eE
    unreachable
  )
  (func $_ZN4core3str16slice_error_fail17h1967a032a7ff118eE (type 12) (param i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 112
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    local.get 3
    i32.store offset=12
    local.get 5
    local.get 2
    i32.store offset=8
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 257
        i32.lt_u
        br_if 0 (;@2;)
        i32.const 256
        local.set 6
        block  ;; label = @3
          local.get 0
          i32.load8_s offset=256
          i32.const -65
          i32.gt_s
          br_if 0 (;@3;)
          i32.const 255
          local.set 6
          local.get 0
          i32.load8_s offset=255
          i32.const -65
          i32.gt_s
          br_if 0 (;@3;)
          i32.const 254
          local.set 6
          local.get 0
          i32.load8_s offset=254
          i32.const -65
          i32.gt_s
          br_if 0 (;@3;)
          i32.const 253
          local.set 6
        end
        local.get 5
        local.get 6
        i32.store offset=20
        local.get 5
        local.get 0
        i32.store offset=16
        i32.const 5
        local.set 6
        i32.const 1050232
        local.set 7
        br 1 (;@1;)
      end
      local.get 5
      local.get 1
      i32.store offset=20
      local.get 5
      local.get 0
      i32.store offset=16
      i32.const 0
      local.set 6
      i32.const 1053116
      local.set 7
    end
    local.get 5
    local.get 6
    i32.store offset=28
    local.get 5
    local.get 7
    i32.store offset=24
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            local.get 1
            i32.gt_u
            local.tee 6
            br_if 0 (;@4;)
            local.get 3
            local.get 1
            i32.gt_u
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 2
              local.get 3
              i32.gt_u
              br_if 0 (;@5;)
              block  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  i32.eqz
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    local.get 2
                    local.get 1
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 1
                    local.get 2
                    i32.eq
                    br_if 1 (;@7;)
                    br 2 (;@6;)
                  end
                  local.get 0
                  local.get 2
                  i32.add
                  i32.load8_s
                  i32.const -64
                  i32.lt_s
                  br_if 1 (;@6;)
                end
                local.get 3
                local.set 2
              end
              local.get 5
              local.get 2
              i32.store offset=32
              local.get 1
              local.set 3
              block  ;; label = @6
                local.get 2
                local.get 1
                i32.ge_u
                br_if 0 (;@6;)
                local.get 2
                i32.const 1
                i32.add
                local.tee 6
                i32.const 0
                local.get 2
                i32.const -3
                i32.add
                local.tee 3
                local.get 3
                local.get 2
                i32.gt_u
                select
                local.tee 3
                i32.lt_u
                br_if 3 (;@3;)
                block  ;; label = @7
                  local.get 3
                  local.get 6
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 6
                  i32.add
                  local.get 0
                  local.get 3
                  i32.add
                  local.tee 8
                  i32.sub
                  local.set 6
                  block  ;; label = @8
                    local.get 0
                    local.get 2
                    i32.add
                    local.tee 9
                    i32.load8_s
                    i32.const -65
                    i32.le_s
                    br_if 0 (;@8;)
                    local.get 6
                    i32.const -1
                    i32.add
                    local.set 7
                    br 1 (;@7;)
                  end
                  local.get 3
                  local.get 2
                  i32.eq
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    local.get 9
                    i32.const -1
                    i32.add
                    local.tee 2
                    i32.load8_s
                    i32.const -65
                    i32.le_s
                    br_if 0 (;@8;)
                    local.get 6
                    i32.const -2
                    i32.add
                    local.set 7
                    br 1 (;@7;)
                  end
                  local.get 8
                  local.get 2
                  i32.eq
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    local.get 9
                    i32.const -2
                    i32.add
                    local.tee 2
                    i32.load8_s
                    i32.const -65
                    i32.le_s
                    br_if 0 (;@8;)
                    local.get 6
                    i32.const -3
                    i32.add
                    local.set 7
                    br 1 (;@7;)
                  end
                  local.get 8
                  local.get 2
                  i32.eq
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    local.get 9
                    i32.const -3
                    i32.add
                    local.tee 2
                    i32.load8_s
                    i32.const -65
                    i32.le_s
                    br_if 0 (;@8;)
                    local.get 6
                    i32.const -4
                    i32.add
                    local.set 7
                    br 1 (;@7;)
                  end
                  local.get 8
                  local.get 2
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 6
                  i32.const -5
                  i32.add
                  local.set 7
                end
                local.get 7
                local.get 3
                i32.add
                local.set 3
              end
              block  ;; label = @6
                local.get 3
                i32.eqz
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 3
                  local.get 1
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 3
                  local.get 1
                  i32.eq
                  br_if 1 (;@6;)
                  br 6 (;@1;)
                end
                local.get 0
                local.get 3
                i32.add
                i32.load8_s
                i32.const -65
                i32.le_s
                br_if 5 (;@1;)
              end
              local.get 3
              local.get 1
              i32.eq
              br_if 3 (;@2;)
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      local.get 3
                      i32.add
                      local.tee 1
                      i32.load8_s
                      local.tee 2
                      i32.const -1
                      i32.gt_s
                      br_if 0 (;@9;)
                      local.get 1
                      i32.load8_u offset=1
                      i32.const 63
                      i32.and
                      local.set 0
                      local.get 2
                      i32.const 31
                      i32.and
                      local.set 6
                      local.get 2
                      i32.const -33
                      i32.gt_u
                      br_if 1 (;@8;)
                      local.get 6
                      i32.const 6
                      i32.shl
                      local.get 0
                      i32.or
                      local.set 1
                      br 2 (;@7;)
                    end
                    local.get 5
                    local.get 2
                    i32.const 255
                    i32.and
                    i32.store offset=36
                    i32.const 1
                    local.set 2
                    br 2 (;@6;)
                  end
                  local.get 0
                  i32.const 6
                  i32.shl
                  local.get 1
                  i32.load8_u offset=2
                  i32.const 63
                  i32.and
                  i32.or
                  local.set 0
                  block  ;; label = @8
                    local.get 2
                    i32.const -16
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 6
                    i32.const 12
                    i32.shl
                    i32.or
                    local.set 1
                    br 1 (;@7;)
                  end
                  local.get 0
                  i32.const 6
                  i32.shl
                  local.get 1
                  i32.load8_u offset=3
                  i32.const 63
                  i32.and
                  i32.or
                  local.get 6
                  i32.const 18
                  i32.shl
                  i32.const 1835008
                  i32.and
                  i32.or
                  local.tee 1
                  i32.const 1114112
                  i32.eq
                  br_if 5 (;@2;)
                end
                local.get 5
                local.get 1
                i32.store offset=36
                i32.const 1
                local.set 2
                local.get 1
                i32.const 128
                i32.lt_u
                br_if 0 (;@6;)
                i32.const 2
                local.set 2
                local.get 1
                i32.const 2048
                i32.lt_u
                br_if 0 (;@6;)
                i32.const 3
                i32.const 4
                local.get 1
                i32.const 65536
                i32.lt_u
                select
                local.set 2
              end
              local.get 5
              local.get 3
              i32.store offset=40
              local.get 5
              local.get 2
              local.get 3
              i32.add
              i32.store offset=44
              local.get 5
              i32.const 48
              i32.add
              i32.const 20
              i32.add
              i32.const 5
              i32.store
              local.get 5
              i32.const 108
              i32.add
              i32.const 4
              i32.store
              local.get 5
              i32.const 100
              i32.add
              i32.const 4
              i32.store
              local.get 5
              i32.const 72
              i32.add
              i32.const 20
              i32.add
              i32.const 8
              i32.store
              local.get 5
              i32.const 84
              i32.add
              i32.const 9
              i32.store
              local.get 5
              i64.const 5
              i64.store offset=52 align=4
              local.get 5
              i32.const 1050416
              i32.store offset=48
              local.get 5
              i32.const 1
              i32.store offset=76
              local.get 5
              local.get 5
              i32.const 72
              i32.add
              i32.store offset=64
              local.get 5
              local.get 5
              i32.const 24
              i32.add
              i32.store offset=104
              local.get 5
              local.get 5
              i32.const 16
              i32.add
              i32.store offset=96
              local.get 5
              local.get 5
              i32.const 40
              i32.add
              i32.store offset=88
              local.get 5
              local.get 5
              i32.const 36
              i32.add
              i32.store offset=80
              local.get 5
              local.get 5
              i32.const 32
              i32.add
              i32.store offset=72
              local.get 5
              i32.const 48
              i32.add
              local.get 4
              call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
              unreachable
            end
            local.get 5
            i32.const 100
            i32.add
            i32.const 4
            i32.store
            local.get 5
            i32.const 72
            i32.add
            i32.const 20
            i32.add
            i32.const 4
            i32.store
            local.get 5
            i32.const 84
            i32.add
            i32.const 1
            i32.store
            local.get 5
            i32.const 48
            i32.add
            i32.const 20
            i32.add
            i32.const 4
            i32.store
            local.get 5
            i64.const 4
            i64.store offset=52 align=4
            local.get 5
            i32.const 1050332
            i32.store offset=48
            local.get 5
            i32.const 1
            i32.store offset=76
            local.get 5
            local.get 5
            i32.const 72
            i32.add
            i32.store offset=64
            local.get 5
            local.get 5
            i32.const 24
            i32.add
            i32.store offset=96
            local.get 5
            local.get 5
            i32.const 16
            i32.add
            i32.store offset=88
            local.get 5
            local.get 5
            i32.const 12
            i32.add
            i32.store offset=80
            local.get 5
            local.get 5
            i32.const 8
            i32.add
            i32.store offset=72
            local.get 5
            i32.const 48
            i32.add
            local.get 4
            call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
            unreachable
          end
          local.get 5
          local.get 2
          local.get 3
          local.get 6
          select
          i32.store offset=40
          local.get 5
          i32.const 48
          i32.add
          i32.const 20
          i32.add
          i32.const 3
          i32.store
          local.get 5
          i32.const 72
          i32.add
          i32.const 20
          i32.add
          i32.const 4
          i32.store
          local.get 5
          i32.const 84
          i32.add
          i32.const 4
          i32.store
          local.get 5
          i64.const 3
          i64.store offset=52 align=4
          local.get 5
          i32.const 1050272
          i32.store offset=48
          local.get 5
          i32.const 1
          i32.store offset=76
          local.get 5
          local.get 5
          i32.const 72
          i32.add
          i32.store offset=64
          local.get 5
          local.get 5
          i32.const 24
          i32.add
          i32.store offset=88
          local.get 5
          local.get 5
          i32.const 16
          i32.add
          i32.store offset=80
          local.get 5
          local.get 5
          i32.const 40
          i32.add
          i32.store offset=72
          local.get 5
          i32.const 48
          i32.add
          local.get 4
          call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
          unreachable
        end
        local.get 3
        local.get 6
        i32.const 1050484
        call $_ZN4core5slice5index22slice_index_order_fail17hd6544278d56731b9E
        unreachable
      end
      i32.const 1055427
      i32.const 43
      local.get 4
      call $_ZN4core9panicking5panic17h66d99d0b614b1c7cE
      unreachable
    end
    local.get 0
    local.get 1
    local.get 3
    local.get 1
    local.get 4
    call $_ZN4core3str16slice_error_fail17h1967a032a7ff118eE
    unreachable
  )
  (func $_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hdb42396925a5638aE (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 1
    local.set 3
    block  ;; label = @1
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h0b19f84ca4cda0d0E
      br_if 0 (;@1;)
      local.get 1
      i32.const 28
      i32.add
      i32.load
      local.set 4
      local.get 1
      i32.load offset=24
      local.set 5
      local.get 2
      i32.const 28
      i32.add
      i32.const 0
      i32.store
      local.get 2
      i32.const 1053116
      i32.store offset=24
      local.get 2
      i64.const 1
      i64.store offset=12 align=4
      local.get 2
      i32.const 1048900
      i32.store offset=8
      local.get 5
      local.get 4
      local.get 2
      i32.const 8
      i32.add
      call $_ZN4core3fmt5write17h8776c655b56f9e02E
      br_if 0 (;@1;)
      local.get 0
      i32.const 4
      i32.add
      local.get 1
      call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h0b19f84ca4cda0d0E
      local.set 3
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 3
  )
  (func $_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17h6c3bcede51e1d38fE (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 1
    local.set 3
    block  ;; label = @1
      local.get 1
      i32.load offset=24
      local.tee 4
      i32.const 39
      local.get 1
      i32.const 28
      i32.add
      i32.load
      i32.load offset=16
      local.tee 5
      call_indirect (type 2)
      br_if 0 (;@1;)
      local.get 2
      local.get 0
      i32.load
      i32.const 257
      call $_ZN4core4char7methods22_$LT$impl$u20$char$GT$16escape_debug_ext17h835a9a7c4404e562E
      local.get 2
      i32.const 12
      i32.add
      i32.load8_u
      local.set 6
      local.get 2
      i32.const 8
      i32.add
      i32.load
      local.set 7
      local.get 2
      i32.load
      local.set 1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load offset=4
            local.tee 8
            i32.const 1114112
            i32.eq
            br_if 0 (;@4;)
            loop  ;; label = @5
              local.get 1
              local.set 0
              i32.const 92
              local.set 3
              i32.const 1
              local.set 1
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      br_table 7 (;@2;) 1 (;@8;) 3 (;@6;) 0 (;@9;) 7 (;@2;)
                    end
                    local.get 6
                    i32.const 255
                    i32.and
                    local.set 0
                    i32.const 0
                    local.set 6
                    i32.const 3
                    local.set 1
                    i32.const 125
                    local.set 3
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 0
                          br_table 9 (;@2;) 5 (;@6;) 4 (;@7;) 0 (;@11;) 1 (;@10;) 2 (;@9;) 9 (;@2;)
                        end
                        i32.const 2
                        local.set 6
                        i32.const 123
                        local.set 3
                        br 4 (;@6;)
                      end
                      i32.const 3
                      local.set 1
                      i32.const 117
                      local.set 3
                      i32.const 3
                      local.set 6
                      br 3 (;@6;)
                    end
                    i32.const 4
                    local.set 6
                    i32.const 92
                    local.set 3
                    br 2 (;@6;)
                  end
                  i32.const 0
                  local.set 1
                  local.get 8
                  local.set 3
                  br 1 (;@6;)
                end
                i32.const 2
                i32.const 1
                local.get 7
                select
                local.set 6
                i32.const 48
                i32.const 87
                local.get 8
                local.get 7
                i32.const 2
                i32.shl
                i32.shr_u
                i32.const 15
                i32.and
                local.tee 3
                i32.const 10
                i32.lt_u
                select
                local.get 3
                i32.add
                local.set 3
                local.get 7
                i32.const -1
                i32.add
                i32.const 0
                local.get 7
                select
                local.set 7
              end
              local.get 4
              local.get 3
              local.get 5
              call_indirect (type 2)
              i32.eqz
              br_if 0 (;@5;)
              br 2 (;@3;)
            end
          end
          loop  ;; label = @4
            local.get 1
            local.set 0
            i32.const 92
            local.set 3
            i32.const 1
            local.set 1
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                br_table 4 (;@2;) 4 (;@2;) 1 (;@5;) 0 (;@6;) 4 (;@2;)
              end
              local.get 6
              i32.const 255
              i32.and
              local.set 0
              i32.const 0
              local.set 6
              i32.const 3
              local.set 1
              i32.const 125
              local.set 3
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      br_table 7 (;@2;) 4 (;@5;) 3 (;@6;) 2 (;@7;) 1 (;@8;) 0 (;@9;) 7 (;@2;)
                    end
                    i32.const 4
                    local.set 6
                    i32.const 92
                    local.set 3
                    br 3 (;@5;)
                  end
                  i32.const 3
                  local.set 1
                  i32.const 117
                  local.set 3
                  i32.const 3
                  local.set 6
                  br 2 (;@5;)
                end
                i32.const 2
                local.set 6
                i32.const 123
                local.set 3
                br 1 (;@5;)
              end
              i32.const 2
              i32.const 1
              local.get 7
              select
              local.set 6
              i32.const 1114112
              local.get 7
              i32.const 2
              i32.shl
              i32.shr_u
              i32.const 1
              i32.and
              i32.const 48
              i32.or
              local.set 3
              local.get 7
              i32.const -1
              i32.add
              i32.const 0
              local.get 7
              select
              local.set 7
            end
            local.get 4
            local.get 3
            local.get 5
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
        end
        i32.const 1
        local.set 3
        br 1 (;@1;)
      end
      local.get 4
      i32.const 39
      local.get 5
      call_indirect (type 2)
      local.set 3
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 3
  )
  (func $_ZN4core4char7methods22_$LT$impl$u20$char$GT$16escape_debug_ext17h835a9a7c4404e562E (type 11) (param i32 i32 i32)
    (local i32 i32 i32 i32 i64)
    i32.const 116
    local.set 3
    i32.const 2
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 1
                    i32.const -9
                    i32.add
                    br_table 7 (;@1;) 2 (;@6;) 5 (;@3;) 5 (;@3;) 1 (;@7;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 3 (;@5;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 5 (;@3;) 4 (;@4;) 0 (;@8;)
                  end
                  i32.const 92
                  local.set 3
                  local.get 1
                  i32.const 92
                  i32.eq
                  br_if 5 (;@2;)
                  br 4 (;@3;)
                end
                i32.const 114
                local.set 3
                br 4 (;@2;)
              end
              i32.const 110
              local.set 3
              br 3 (;@2;)
            end
            local.get 2
            i32.const 65536
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            i32.const 34
            local.set 3
            br 2 (;@2;)
          end
          local.get 2
          i32.const 256
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          i32.const 39
          local.set 3
          br 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    i32.const 1
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 11
                    i32.shl
                    local.set 5
                    i32.const 0
                    local.set 3
                    i32.const 32
                    local.set 2
                    i32.const 32
                    local.set 4
                    block  ;; label = @9
                      block  ;; label = @10
                        loop  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 2
                              i32.const 1
                              i32.shr_u
                              local.get 3
                              i32.add
                              local.tee 2
                              i32.const 2
                              i32.shl
                              i32.const 1052104
                              i32.add
                              i32.load
                              i32.const 11
                              i32.shl
                              local.tee 6
                              local.get 5
                              i32.lt_u
                              br_if 0 (;@13;)
                              local.get 6
                              local.get 5
                              i32.eq
                              br_if 3 (;@10;)
                              local.get 2
                              local.set 4
                              br 1 (;@12;)
                            end
                            local.get 2
                            i32.const 1
                            i32.add
                            local.set 3
                          end
                          local.get 4
                          local.get 3
                          i32.sub
                          local.set 2
                          local.get 4
                          local.get 3
                          i32.gt_u
                          br_if 0 (;@11;)
                          br 2 (;@9;)
                        end
                      end
                      local.get 2
                      i32.const 1
                      i32.add
                      local.set 3
                    end
                    local.get 3
                    i32.const 31
                    i32.gt_u
                    br_if 1 (;@7;)
                    local.get 3
                    i32.const 2
                    i32.shl
                    local.set 2
                    i32.const 707
                    local.set 4
                    block  ;; label = @9
                      local.get 3
                      i32.const 31
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 2
                      i32.const 1052108
                      i32.add
                      i32.load
                      i32.const 21
                      i32.shr_u
                      local.set 4
                    end
                    i32.const 0
                    local.set 5
                    block  ;; label = @9
                      local.get 3
                      i32.const -1
                      i32.add
                      local.tee 6
                      local.get 3
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 6
                      i32.const 32
                      i32.ge_u
                      br_if 3 (;@6;)
                      local.get 6
                      i32.const 2
                      i32.shl
                      i32.const 1052104
                      i32.add
                      i32.load
                      i32.const 2097151
                      i32.and
                      local.set 5
                    end
                    block  ;; label = @9
                      local.get 4
                      local.get 2
                      i32.const 1052104
                      i32.add
                      i32.load
                      i32.const 21
                      i32.shr_u
                      local.tee 3
                      i32.const -1
                      i32.xor
                      i32.add
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 1
                      local.get 5
                      i32.sub
                      local.set 5
                      local.get 3
                      i32.const 707
                      local.get 3
                      i32.const 707
                      i32.gt_u
                      select
                      local.set 2
                      local.get 4
                      i32.const -1
                      i32.add
                      local.set 6
                      i32.const 0
                      local.set 4
                      loop  ;; label = @10
                        local.get 2
                        local.get 3
                        i32.eq
                        br_if 5 (;@5;)
                        local.get 4
                        local.get 3
                        i32.const 1052232
                        i32.add
                        i32.load8_u
                        i32.add
                        local.tee 4
                        local.get 5
                        i32.gt_u
                        br_if 1 (;@9;)
                        local.get 6
                        local.get 3
                        i32.const 1
                        i32.add
                        local.tee 3
                        i32.ne
                        br_if 0 (;@10;)
                      end
                      local.get 6
                      local.set 3
                    end
                    local.get 3
                    i32.const 1
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 1
                    i32.or
                    i32.clz
                    i32.const 2
                    i32.shr_u
                    i32.const 7
                    i32.xor
                    i64.extend_i32_u
                    i64.const 21474836480
                    i64.or
                    local.set 7
                    br 5 (;@3;)
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 1
                        i32.const 65536
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 1
                        i32.const 131072
                        i32.ge_u
                        br_if 1 (;@9;)
                        local.get 1
                        i32.const 1051243
                        i32.const 42
                        i32.const 1051327
                        i32.const 192
                        i32.const 1051519
                        i32.const 438
                        call $_ZN4core7unicode9printable5check17h02b73d51ac387bb4E
                        br_if 6 (;@4;)
                        br 2 (;@8;)
                      end
                      local.get 1
                      i32.const 1050572
                      i32.const 40
                      i32.const 1050652
                      i32.const 288
                      i32.const 1050940
                      i32.const 303
                      call $_ZN4core7unicode9printable5check17h02b73d51ac387bb4E
                      i32.eqz
                      br_if 1 (;@8;)
                      br 5 (;@4;)
                    end
                    local.get 1
                    i32.const 2097120
                    i32.and
                    i32.const 173792
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const -177984
                    i32.add
                    i32.const -8
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 2097150
                    i32.and
                    i32.const 178206
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const -183984
                    i32.add
                    i32.const -15
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const -194560
                    i32.add
                    i32.const -3104
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const -196608
                    i32.add
                    i32.const -1507
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const -917760
                    i32.add
                    i32.const -716214
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 918000
                    i32.lt_u
                    br_if 4 (;@4;)
                  end
                  local.get 1
                  i32.const 1
                  i32.or
                  i32.clz
                  i32.const 2
                  i32.shr_u
                  i32.const 7
                  i32.xor
                  i64.extend_i32_u
                  i64.const 21474836480
                  i64.or
                  local.set 7
                  br 4 (;@3;)
                end
                local.get 3
                i32.const 32
                i32.const 1052000
                call $_ZN4core9panicking18panic_bounds_check17h8c564c2b20bece92E
                unreachable
              end
              local.get 6
              i32.const 32
              i32.const 1052032
              call $_ZN4core9panicking18panic_bounds_check17h8c564c2b20bece92E
              unreachable
            end
            local.get 2
            i32.const 707
            i32.const 1052016
            call $_ZN4core9panicking18panic_bounds_check17h8c564c2b20bece92E
            unreachable
          end
          i32.const 1
          local.set 4
          local.get 1
          local.set 3
          br 1 (;@2;)
        end
        i32.const 3
        local.set 4
        local.get 1
        local.set 3
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 4
    i32.store
    local.get 0
    i32.const 8
    i32.add
    local.get 7
    i64.store align=4
  )
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h0b19f84ca4cda0d0E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load
              local.tee 3
              i32.const 16
              i32.and
              br_if 0 (;@5;)
              local.get 3
              i32.const 32
              i32.and
              br_if 1 (;@4;)
              local.get 0
              i64.load32_u
              i32.const 1
              local.get 1
              call $_ZN4core3fmt3num3imp7fmt_u6417hcb4d540e45bed3d8E
              local.set 0
              br 4 (;@1;)
            end
            local.get 0
            i32.load
            local.set 0
            i32.const 0
            local.set 3
            loop  ;; label = @5
              local.get 2
              local.get 3
              i32.add
              i32.const 127
              i32.add
              i32.const 48
              i32.const 87
              local.get 0
              i32.const 15
              i32.and
              local.tee 4
              i32.const 10
              i32.lt_u
              select
              local.get 4
              i32.add
              i32.store8
              local.get 3
              i32.const -1
              i32.add
              local.set 3
              local.get 0
              i32.const 15
              i32.gt_u
              local.set 4
              local.get 0
              i32.const 4
              i32.shr_u
              local.set 0
              local.get 4
              br_if 0 (;@5;)
            end
            local.get 3
            i32.const 128
            i32.add
            local.tee 0
            i32.const 129
            i32.ge_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 1
            i32.const 1049448
            i32.const 2
            local.get 2
            local.get 3
            i32.add
            i32.const 128
            i32.add
            i32.const 0
            local.get 3
            i32.sub
            call $_ZN4core3fmt9Formatter12pad_integral17h88d3d7a17289fb2aE
            local.set 0
            br 3 (;@1;)
          end
          local.get 0
          i32.load
          local.set 0
          i32.const 0
          local.set 3
          loop  ;; label = @4
            local.get 2
            local.get 3
            i32.add
            i32.const 127
            i32.add
            i32.const 48
            i32.const 55
            local.get 0
            i32.const 15
            i32.and
            local.tee 4
            i32.const 10
            i32.lt_u
            select
            local.get 4
            i32.add
            i32.store8
            local.get 3
            i32.const -1
            i32.add
            local.set 3
            local.get 0
            i32.const 15
            i32.gt_u
            local.set 4
            local.get 0
            i32.const 4
            i32.shr_u
            local.set 0
            local.get 4
            br_if 0 (;@4;)
          end
          local.get 3
          i32.const 128
          i32.add
          local.tee 0
          i32.const 129
          i32.ge_u
          br_if 1 (;@2;)
          local.get 1
          i32.const 1
          i32.const 1049448
          i32.const 2
          local.get 2
          local.get 3
          i32.add
          i32.const 128
          i32.add
          i32.const 0
          local.get 3
          i32.sub
          call $_ZN4core3fmt9Formatter12pad_integral17h88d3d7a17289fb2aE
          local.set 0
          br 2 (;@1;)
        end
        local.get 0
        i32.const 128
        i32.const 1049432
        call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
        unreachable
      end
      local.get 0
      i32.const 128
      i32.const 1049432
      call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
      unreachable
    end
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core7unicode9printable5check17h02b73d51ac387bb4E (type 16) (param i32 i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 1
    local.get 2
    i32.const 1
    i32.shl
    i32.add
    local.set 7
    local.get 0
    i32.const 65280
    i32.and
    i32.const 8
    i32.shr_u
    local.set 8
    i32.const 0
    local.set 9
    local.get 0
    i32.const 255
    i32.and
    local.set 10
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          loop  ;; label = @4
            local.get 1
            i32.const 2
            i32.add
            local.set 11
            local.get 9
            local.get 1
            i32.load8_u offset=1
            local.tee 2
            i32.add
            local.set 12
            block  ;; label = @5
              local.get 1
              i32.load8_u
              local.tee 1
              local.get 8
              i32.eq
              br_if 0 (;@5;)
              local.get 1
              local.get 8
              i32.gt_u
              br_if 3 (;@2;)
              local.get 12
              local.set 9
              local.get 11
              local.set 1
              local.get 11
              local.get 7
              i32.ne
              br_if 1 (;@4;)
              br 3 (;@2;)
            end
            block  ;; label = @5
              local.get 12
              local.get 9
              i32.lt_u
              br_if 0 (;@5;)
              local.get 12
              local.get 4
              i32.gt_u
              br_if 2 (;@3;)
              local.get 3
              local.get 9
              i32.add
              local.set 1
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 2
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 2
                  i32.const -1
                  i32.add
                  local.set 2
                  local.get 1
                  i32.load8_u
                  local.set 9
                  local.get 1
                  i32.const 1
                  i32.add
                  local.set 1
                  local.get 9
                  local.get 10
                  i32.ne
                  br_if 0 (;@7;)
                end
                i32.const 0
                local.set 2
                br 5 (;@1;)
              end
              local.get 12
              local.set 9
              local.get 11
              local.set 1
              local.get 11
              local.get 7
              i32.ne
              br_if 1 (;@4;)
              br 3 (;@2;)
            end
          end
          local.get 9
          local.get 12
          i32.const 1050540
          call $_ZN4core5slice5index22slice_index_order_fail17hd6544278d56731b9E
          unreachable
        end
        local.get 12
        local.get 4
        i32.const 1050540
        call $_ZN4core5slice5index24slice_end_index_len_fail17h53a611cf4b2e1c2bE
        unreachable
      end
      local.get 0
      i32.const 65535
      i32.and
      local.set 9
      local.get 5
      local.get 6
      i32.add
      local.set 12
      i32.const 1
      local.set 2
      block  ;; label = @2
        loop  ;; label = @3
          local.get 5
          i32.const 1
          i32.add
          local.set 10
          block  ;; label = @4
            block  ;; label = @5
              local.get 5
              i32.load8_u
              local.tee 1
              i32.const 24
              i32.shl
              i32.const 24
              i32.shr_s
              local.tee 11
              i32.const 0
              i32.lt_s
              br_if 0 (;@5;)
              local.get 10
              local.set 5
              br 1 (;@4;)
            end
            local.get 10
            local.get 12
            i32.eq
            br_if 2 (;@2;)
            local.get 11
            i32.const 127
            i32.and
            i32.const 8
            i32.shl
            local.get 5
            i32.load8_u offset=1
            i32.or
            local.set 1
            local.get 5
            i32.const 2
            i32.add
            local.set 5
          end
          local.get 9
          local.get 1
          i32.sub
          local.tee 9
          i32.const 0
          i32.lt_s
          br_if 2 (;@1;)
          local.get 2
          i32.const 1
          i32.xor
          local.set 2
          local.get 5
          local.get 12
          i32.ne
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      i32.const 1055427
      i32.const 43
      i32.const 1050556
      call $_ZN4core9panicking5panic17h66d99d0b614b1c7cE
      unreachable
    end
    local.get 2
    i32.const 1
    i32.and
  )
  (func $_ZN4core3fmt8builders11DebugStruct5field17h8a23062b858eb435E (type 9) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i64 i64)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    i32.const 1
    local.set 6
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=4
      br_if 0 (;@1;)
      local.get 0
      i32.load8_u offset=5
      local.set 7
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 8
        i32.load
        local.tee 9
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 6
        local.get 8
        i32.load offset=24
        i32.const 1049369
        i32.const 1049371
        local.get 7
        i32.const 255
        i32.and
        local.tee 7
        select
        i32.const 2
        i32.const 3
        local.get 7
        select
        local.get 8
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        br_if 1 (;@1;)
        i32.const 1
        local.set 6
        local.get 8
        i32.load offset=24
        local.get 1
        local.get 2
        local.get 8
        i32.load offset=28
        i32.load offset=12
        call_indirect (type 1)
        br_if 1 (;@1;)
        i32.const 1
        local.set 6
        local.get 8
        i32.load offset=24
        i32.const 1053452
        i32.const 2
        local.get 8
        i32.load offset=28
        i32.load offset=12
        call_indirect (type 1)
        br_if 1 (;@1;)
        local.get 3
        local.get 8
        local.get 4
        i32.load offset=12
        call_indirect (type 2)
        local.set 6
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 7
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 6
        local.get 8
        i32.load offset=24
        i32.const 1049364
        i32.const 3
        local.get 8
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        br_if 1 (;@1;)
        local.get 8
        i32.load
        local.set 9
      end
      i32.const 1
      local.set 6
      local.get 5
      i32.const 1
      i32.store8 offset=23
      local.get 5
      i32.const 52
      i32.add
      i32.const 1049272
      i32.store
      local.get 5
      i32.const 16
      i32.add
      local.get 5
      i32.const 23
      i32.add
      i32.store
      local.get 5
      local.get 9
      i32.store offset=24
      local.get 5
      local.get 8
      i64.load offset=24 align=4
      i64.store offset=8
      local.get 8
      i64.load offset=8 align=4
      local.set 10
      local.get 8
      i64.load offset=16 align=4
      local.set 11
      local.get 5
      local.get 8
      i32.load8_u offset=32
      i32.store8 offset=56
      local.get 5
      local.get 8
      i32.load offset=4
      i32.store offset=28
      local.get 5
      local.get 11
      i64.store offset=40
      local.get 5
      local.get 10
      i64.store offset=32
      local.get 5
      local.get 5
      i32.const 8
      i32.add
      i32.store offset=48
      local.get 5
      i32.const 8
      i32.add
      local.get 1
      local.get 2
      call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hb1584ad756e22b8fE
      br_if 0 (;@1;)
      local.get 5
      i32.const 8
      i32.add
      i32.const 1053452
      i32.const 2
      call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hb1584ad756e22b8fE
      br_if 0 (;@1;)
      local.get 3
      local.get 5
      i32.const 24
      i32.add
      local.get 4
      i32.load offset=12
      call_indirect (type 2)
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=48
      i32.const 1049367
      i32.const 2
      local.get 5
      i32.load offset=52
      i32.load offset=12
      call_indirect (type 1)
      local.set 6
    end
    local.get 0
    i32.const 1
    i32.store8 offset=5
    local.get 0
    local.get 6
    i32.store8 offset=4
    local.get 5
    i32.const 64
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core3fmt5Write10write_char17h48f1f74822f1d37aE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 2 (;@2;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.store8 offset=12
          i32.const 1
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=15
      local.get 2
      local.get 1
      i32.const 18
      i32.shr_u
      i32.const 240
      i32.or
      i32.store8 offset=12
      local.get 2
      local.get 1
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=14
      local.get 2
      local.get 1
      i32.const 12
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=13
      i32.const 4
      local.set 1
    end
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    local.get 1
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hb1584ad756e22b8fE
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN4core3fmt5Write9write_fmt17hd301836d8c71db60E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1049652
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h8d24c0da07505a80E (type 1) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 2
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hb1584ad756e22b8fE
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hd434d93e5b266e4aE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 2 (;@2;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.store8 offset=12
          i32.const 1
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=15
      local.get 2
      local.get 1
      i32.const 18
      i32.shr_u
      i32.const 240
      i32.or
      i32.store8 offset=12
      local.get 2
      local.get 1
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=14
      local.get 2
      local.get 1
      i32.const 12
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=13
      i32.const 4
      local.set 1
    end
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    local.get 1
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hb1584ad756e22b8fE
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h12d461925b966368E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1049652
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN4core3fmt8builders10DebugTuple5field17h4b7d21df77126ad7E (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i64 i64)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=8
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=4
        local.set 4
        i32.const 1
        local.set 5
        br 1 (;@1;)
      end
      local.get 0
      i32.load offset=4
      local.set 4
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 6
        i32.load
        local.tee 7
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 5
        local.get 6
        i32.load offset=24
        i32.const 1049369
        i32.const 1049386
        local.get 4
        select
        i32.const 2
        i32.const 1
        local.get 4
        select
        local.get 6
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        br_if 1 (;@1;)
        local.get 1
        local.get 6
        local.get 2
        i32.load offset=12
        call_indirect (type 2)
        local.set 5
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 4
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 6
          i32.load offset=24
          i32.const 1049384
          i32.const 2
          local.get 6
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 1)
          i32.eqz
          br_if 0 (;@3;)
          i32.const 1
          local.set 5
          i32.const 0
          local.set 4
          br 2 (;@1;)
        end
        local.get 6
        i32.load
        local.set 7
      end
      i32.const 1
      local.set 5
      local.get 3
      i32.const 1
      i32.store8 offset=23
      local.get 3
      i32.const 52
      i32.add
      i32.const 1049272
      i32.store
      local.get 3
      i32.const 16
      i32.add
      local.get 3
      i32.const 23
      i32.add
      i32.store
      local.get 3
      local.get 7
      i32.store offset=24
      local.get 3
      local.get 6
      i64.load offset=24 align=4
      i64.store offset=8
      local.get 6
      i64.load offset=8 align=4
      local.set 8
      local.get 6
      i64.load offset=16 align=4
      local.set 9
      local.get 3
      local.get 6
      i32.load8_u offset=32
      i32.store8 offset=56
      local.get 3
      local.get 6
      i32.load offset=4
      i32.store offset=28
      local.get 3
      local.get 9
      i64.store offset=40
      local.get 3
      local.get 8
      i64.store offset=32
      local.get 3
      local.get 3
      i32.const 8
      i32.add
      i32.store offset=48
      local.get 1
      local.get 3
      i32.const 24
      i32.add
      local.get 2
      i32.load offset=12
      call_indirect (type 2)
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=48
      i32.const 1049367
      i32.const 2
      local.get 3
      i32.load offset=52
      i32.load offset=12
      call_indirect (type 1)
      local.set 5
    end
    local.get 0
    local.get 5
    i32.store8 offset=8
    local.get 0
    local.get 4
    i32.const 1
    i32.add
    i32.store offset=4
    local.get 3
    i32.const 64
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17hecac681ab20f31d6E (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i32.const 1
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load offset=24
        local.tee 5
        i32.const 34
        local.get 2
        i32.const 28
        i32.add
        i32.load
        local.tee 6
        i32.load offset=16
        local.tee 7
        call_indirect (type 2)
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            br_if 0 (;@4;)
            i32.const 0
            local.set 2
            br 1 (;@3;)
          end
          local.get 0
          local.get 1
          i32.add
          local.set 8
          local.get 0
          local.set 9
          i32.const 0
          local.set 10
          i32.const 0
          local.set 11
          block  ;; label = @4
            block  ;; label = @5
              loop  ;; label = @6
                local.get 9
                local.set 12
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 9
                    i32.load8_s
                    local.tee 2
                    i32.const -1
                    i32.le_s
                    br_if 0 (;@8;)
                    local.get 9
                    i32.const 1
                    i32.add
                    local.set 9
                    local.get 2
                    i32.const 255
                    i32.and
                    local.set 13
                    br 1 (;@7;)
                  end
                  local.get 9
                  i32.load8_u offset=1
                  i32.const 63
                  i32.and
                  local.set 14
                  local.get 2
                  i32.const 31
                  i32.and
                  local.set 15
                  block  ;; label = @8
                    local.get 2
                    i32.const -33
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 15
                    i32.const 6
                    i32.shl
                    local.get 14
                    i32.or
                    local.set 13
                    local.get 9
                    i32.const 2
                    i32.add
                    local.set 9
                    br 1 (;@7;)
                  end
                  local.get 14
                  i32.const 6
                  i32.shl
                  local.get 9
                  i32.load8_u offset=2
                  i32.const 63
                  i32.and
                  i32.or
                  local.set 14
                  block  ;; label = @8
                    local.get 2
                    i32.const -16
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 14
                    local.get 15
                    i32.const 12
                    i32.shl
                    i32.or
                    local.set 13
                    local.get 9
                    i32.const 3
                    i32.add
                    local.set 9
                    br 1 (;@7;)
                  end
                  local.get 14
                  i32.const 6
                  i32.shl
                  local.get 9
                  i32.load8_u offset=3
                  i32.const 63
                  i32.and
                  i32.or
                  local.get 15
                  i32.const 18
                  i32.shl
                  i32.const 1835008
                  i32.and
                  i32.or
                  local.tee 13
                  i32.const 1114112
                  i32.eq
                  br_if 3 (;@4;)
                  local.get 9
                  i32.const 4
                  i32.add
                  local.set 9
                end
                local.get 3
                local.get 13
                i32.const 65537
                call $_ZN4core4char7methods22_$LT$impl$u20$char$GT$16escape_debug_ext17h835a9a7c4404e562E
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 3
                        i32.load
                        local.tee 2
                        br_table 1 (;@9;) 2 (;@8;) 1 (;@9;) 0 (;@10;) 1 (;@9;)
                      end
                      local.get 3
                      i32.load offset=8
                      local.get 3
                      i32.load8_u offset=12
                      i32.add
                      i32.const 1
                      i32.eq
                      br_if 1 (;@8;)
                    end
                    local.get 11
                    local.get 10
                    i32.lt_u
                    br_if 3 (;@5;)
                    block  ;; label = @9
                      local.get 10
                      i32.eqz
                      br_if 0 (;@9;)
                      block  ;; label = @10
                        local.get 10
                        local.get 1
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 10
                        local.get 1
                        i32.eq
                        br_if 1 (;@9;)
                        br 5 (;@5;)
                      end
                      local.get 0
                      local.get 10
                      i32.add
                      i32.load8_s
                      i32.const -64
                      i32.lt_s
                      br_if 4 (;@5;)
                    end
                    block  ;; label = @9
                      local.get 11
                      i32.eqz
                      br_if 0 (;@9;)
                      block  ;; label = @10
                        local.get 11
                        local.get 1
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 11
                        local.get 1
                        i32.ne
                        br_if 5 (;@5;)
                        br 1 (;@9;)
                      end
                      local.get 0
                      local.get 11
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 4 (;@5;)
                    end
                    local.get 5
                    local.get 0
                    local.get 10
                    i32.add
                    local.get 11
                    local.get 10
                    i32.sub
                    local.get 6
                    i32.load offset=12
                    call_indirect (type 1)
                    br_if 1 (;@7;)
                    local.get 3
                    i32.load8_u offset=12
                    local.set 14
                    local.get 3
                    i32.load offset=8
                    local.set 16
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 3
                        i32.load offset=4
                        local.tee 17
                        i32.const 1114112
                        i32.ne
                        br_if 0 (;@10;)
                        loop  ;; label = @11
                          local.get 2
                          local.set 15
                          i32.const 1
                          local.set 2
                          i32.const 92
                          local.set 10
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 15
                              br_table 4 (;@9;) 4 (;@9;) 1 (;@12;) 0 (;@13;) 4 (;@9;)
                            end
                            local.get 14
                            i32.const 255
                            i32.and
                            local.set 15
                            i32.const 3
                            local.set 2
                            i32.const 0
                            local.set 14
                            i32.const 125
                            local.set 10
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 15
                                    br_table 7 (;@9;) 4 (;@12;) 3 (;@13;) 2 (;@14;) 1 (;@15;) 0 (;@16;) 7 (;@9;)
                                  end
                                  i32.const 4
                                  local.set 14
                                  i32.const 92
                                  local.set 10
                                  br 3 (;@12;)
                                end
                                i32.const 3
                                local.set 14
                                i32.const 117
                                local.set 10
                                i32.const 3
                                local.set 2
                                br 2 (;@12;)
                              end
                              i32.const 2
                              local.set 14
                              i32.const 123
                              local.set 10
                              br 1 (;@12;)
                            end
                            i32.const 2
                            i32.const 1
                            local.get 16
                            select
                            local.set 14
                            i32.const 1114112
                            local.get 16
                            i32.const 2
                            i32.shl
                            i32.shr_u
                            i32.const 1
                            i32.and
                            i32.const 48
                            i32.or
                            local.set 10
                            local.get 16
                            i32.const -1
                            i32.add
                            i32.const 0
                            local.get 16
                            select
                            local.set 16
                          end
                          local.get 5
                          local.get 10
                          local.get 7
                          call_indirect (type 2)
                          i32.eqz
                          br_if 0 (;@11;)
                          br 4 (;@7;)
                        end
                      end
                      loop  ;; label = @10
                        local.get 2
                        local.set 15
                        i32.const 1
                        local.set 2
                        i32.const 92
                        local.set 10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 15
                                br_table 5 (;@9;) 1 (;@13;) 3 (;@11;) 0 (;@14;) 5 (;@9;)
                              end
                              local.get 14
                              i32.const 255
                              i32.and
                              local.set 15
                              i32.const 3
                              local.set 2
                              i32.const 0
                              local.set 14
                              i32.const 125
                              local.set 10
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 15
                                    br_table 7 (;@9;) 5 (;@11;) 4 (;@12;) 0 (;@16;) 1 (;@15;) 2 (;@14;) 7 (;@9;)
                                  end
                                  i32.const 2
                                  local.set 14
                                  i32.const 123
                                  local.set 10
                                  br 4 (;@11;)
                                end
                                i32.const 3
                                local.set 14
                                i32.const 117
                                local.set 10
                                i32.const 3
                                local.set 2
                                br 3 (;@11;)
                              end
                              i32.const 4
                              local.set 14
                              i32.const 92
                              local.set 10
                              br 2 (;@11;)
                            end
                            i32.const 0
                            local.set 2
                            local.get 17
                            local.set 10
                            br 1 (;@11;)
                          end
                          i32.const 2
                          i32.const 1
                          local.get 16
                          select
                          local.set 14
                          i32.const 48
                          i32.const 87
                          local.get 17
                          local.get 16
                          i32.const 2
                          i32.shl
                          i32.shr_u
                          i32.const 15
                          i32.and
                          local.tee 10
                          i32.const 10
                          i32.lt_u
                          select
                          local.get 10
                          i32.add
                          local.set 10
                          local.get 16
                          i32.const -1
                          i32.add
                          i32.const 0
                          local.get 16
                          select
                          local.set 16
                        end
                        local.get 5
                        local.get 10
                        local.get 7
                        call_indirect (type 2)
                        br_if 3 (;@7;)
                        br 0 (;@10;)
                      end
                    end
                    i32.const 1
                    local.set 2
                    block  ;; label = @9
                      local.get 13
                      i32.const 128
                      i32.lt_u
                      br_if 0 (;@9;)
                      i32.const 2
                      local.set 2
                      local.get 13
                      i32.const 2048
                      i32.lt_u
                      br_if 0 (;@9;)
                      i32.const 3
                      i32.const 4
                      local.get 13
                      i32.const 65536
                      i32.lt_u
                      select
                      local.set 2
                    end
                    local.get 2
                    local.get 11
                    i32.add
                    local.set 10
                  end
                  local.get 11
                  local.get 12
                  i32.sub
                  local.get 9
                  i32.add
                  local.set 11
                  local.get 9
                  local.get 8
                  i32.ne
                  br_if 1 (;@6;)
                  br 3 (;@4;)
                end
              end
              i32.const 1
              local.set 4
              br 3 (;@2;)
            end
            local.get 0
            local.get 1
            local.get 10
            local.get 11
            i32.const 1049712
            call $_ZN4core3str16slice_error_fail17h1967a032a7ff118eE
            unreachable
          end
          block  ;; label = @4
            local.get 10
            br_if 0 (;@4;)
            i32.const 0
            local.set 2
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 10
            local.get 1
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            local.set 2
            local.get 10
            local.get 1
            i32.eq
            br_if 1 (;@3;)
            br 3 (;@1;)
          end
          local.get 0
          local.get 10
          i32.add
          i32.load8_s
          i32.const -65
          i32.le_s
          br_if 2 (;@1;)
          local.get 10
          local.set 2
        end
        local.get 5
        local.get 0
        local.get 2
        i32.add
        local.get 1
        local.get 2
        i32.sub
        local.get 6
        i32.load offset=12
        call_indirect (type 1)
        br_if 0 (;@2;)
        local.get 5
        i32.const 34
        local.get 7
        call_indirect (type 2)
        local.set 4
      end
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 4
      return
    end
    local.get 0
    local.get 1
    local.get 10
    local.get 1
    i32.const 1049728
    call $_ZN4core3str16slice_error_fail17h1967a032a7ff118eE
    unreachable
  )
  (func $_ZN4core3str8converts9from_utf817h5e14e3bada571d6dE (type 11) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i64 i32 i32)
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      local.get 2
      i32.const -7
      i32.add
      local.tee 3
      local.get 3
      local.get 2
      i32.gt_u
      select
      local.set 4
      local.get 1
      i32.const 3
      i32.add
      i32.const -4
      i32.and
      local.get 1
      i32.sub
      local.set 5
      i32.const 0
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            loop  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 1
                    local.get 3
                    i32.add
                    i32.load8_u
                    local.tee 6
                    i32.const 24
                    i32.shl
                    i32.const 24
                    i32.shr_s
                    local.tee 7
                    i32.const 0
                    i32.lt_s
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const -1
                    i32.eq
                    br_if 1 (;@7;)
                    local.get 5
                    local.get 3
                    i32.sub
                    i32.const 3
                    i32.and
                    br_if 1 (;@7;)
                    block  ;; label = @9
                      local.get 3
                      local.get 4
                      i32.ge_u
                      br_if 0 (;@9;)
                      loop  ;; label = @10
                        local.get 1
                        local.get 3
                        i32.add
                        local.tee 6
                        i32.load
                        local.get 6
                        i32.const 4
                        i32.add
                        i32.load
                        i32.or
                        i32.const -2139062144
                        i32.and
                        br_if 1 (;@9;)
                        local.get 3
                        i32.const 8
                        i32.add
                        local.tee 3
                        local.get 4
                        i32.lt_u
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 3
                    local.get 2
                    i32.ge_u
                    br_if 2 (;@6;)
                    loop  ;; label = @9
                      local.get 1
                      local.get 3
                      i32.add
                      i32.load8_s
                      i32.const 0
                      i32.lt_s
                      br_if 3 (;@6;)
                      local.get 2
                      local.get 3
                      i32.const 1
                      i32.add
                      local.tee 3
                      i32.ne
                      br_if 0 (;@9;)
                      br 8 (;@1;)
                    end
                  end
                  i64.const 1
                  local.set 8
                  i32.const 1
                  local.set 9
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 6
                                    i32.const 1049976
                                    i32.add
                                    i32.load8_u
                                    i32.const -2
                                    i32.add
                                    br_table 0 (;@16;) 1 (;@15;) 2 (;@14;) 14 (;@2;)
                                  end
                                  local.get 3
                                  i32.const 1
                                  i32.add
                                  local.tee 6
                                  local.get 2
                                  i32.lt_u
                                  br_if 6 (;@9;)
                                  i32.const 0
                                  local.set 9
                                  i64.const 0
                                  local.set 8
                                  br 13 (;@2;)
                                end
                                i32.const 0
                                local.set 9
                                i64.const 0
                                local.set 8
                                local.get 3
                                i32.const 1
                                i32.add
                                local.tee 10
                                local.get 2
                                i32.ge_u
                                br_if 12 (;@2;)
                                local.get 1
                                local.get 10
                                i32.add
                                i32.load8_s
                                local.set 10
                                local.get 6
                                i32.const -224
                                i32.add
                                br_table 1 (;@13;) 3 (;@11;) 3 (;@11;) 3 (;@11;) 3 (;@11;) 3 (;@11;) 3 (;@11;) 3 (;@11;) 3 (;@11;) 3 (;@11;) 3 (;@11;) 3 (;@11;) 3 (;@11;) 2 (;@12;) 3 (;@11;)
                              end
                              i32.const 0
                              local.set 9
                              i64.const 0
                              local.set 8
                              local.get 3
                              i32.const 1
                              i32.add
                              local.tee 10
                              local.get 2
                              i32.ge_u
                              br_if 11 (;@2;)
                              local.get 1
                              local.get 10
                              i32.add
                              i32.load8_s
                              local.set 10
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      local.get 6
                                      i32.const -240
                                      i32.add
                                      br_table 1 (;@16;) 0 (;@17;) 0 (;@17;) 0 (;@17;) 2 (;@15;) 0 (;@17;)
                                    end
                                    local.get 7
                                    i32.const 15
                                    i32.add
                                    i32.const 255
                                    i32.and
                                    i32.const 2
                                    i32.gt_u
                                    br_if 13 (;@3;)
                                    local.get 10
                                    i32.const -1
                                    i32.gt_s
                                    br_if 13 (;@3;)
                                    local.get 10
                                    i32.const -64
                                    i32.ge_u
                                    br_if 13 (;@3;)
                                    br 2 (;@14;)
                                  end
                                  local.get 10
                                  i32.const 112
                                  i32.add
                                  i32.const 255
                                  i32.and
                                  i32.const 48
                                  i32.ge_u
                                  br_if 12 (;@3;)
                                  br 1 (;@14;)
                                end
                                local.get 10
                                i32.const -113
                                i32.gt_s
                                br_if 11 (;@3;)
                              end
                              local.get 3
                              i32.const 2
                              i32.add
                              local.tee 6
                              local.get 2
                              i32.ge_u
                              br_if 11 (;@2;)
                              local.get 1
                              local.get 6
                              i32.add
                              i32.load8_s
                              i32.const -65
                              i32.gt_s
                              br_if 9 (;@4;)
                              local.get 3
                              i32.const 3
                              i32.add
                              local.tee 6
                              local.get 2
                              i32.ge_u
                              br_if 11 (;@2;)
                              local.get 1
                              local.get 6
                              i32.add
                              i32.load8_s
                              i32.const -65
                              i32.le_s
                              br_if 5 (;@8;)
                              i64.const 3
                              local.set 8
                              i32.const 1
                              local.set 9
                              br 11 (;@2;)
                            end
                            local.get 10
                            i32.const -32
                            i32.and
                            i32.const -96
                            i32.ne
                            br_if 9 (;@3;)
                            br 2 (;@10;)
                          end
                          local.get 10
                          i32.const -96
                          i32.ge_s
                          br_if 8 (;@3;)
                          br 1 (;@10;)
                        end
                        block  ;; label = @11
                          local.get 7
                          i32.const 31
                          i32.add
                          i32.const 255
                          i32.and
                          i32.const 12
                          i32.lt_u
                          br_if 0 (;@11;)
                          local.get 7
                          i32.const -2
                          i32.and
                          i32.const -18
                          i32.ne
                          br_if 8 (;@3;)
                          local.get 10
                          i32.const -1
                          i32.gt_s
                          br_if 8 (;@3;)
                          local.get 10
                          i32.const -64
                          i32.ge_u
                          br_if 8 (;@3;)
                          br 1 (;@10;)
                        end
                        local.get 10
                        i32.const -65
                        i32.gt_s
                        br_if 7 (;@3;)
                      end
                      local.get 3
                      i32.const 2
                      i32.add
                      local.tee 6
                      local.get 2
                      i32.ge_u
                      br_if 7 (;@2;)
                      local.get 1
                      local.get 6
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.gt_s
                      br_if 5 (;@4;)
                      br 1 (;@8;)
                    end
                    local.get 1
                    local.get 6
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    br_if 5 (;@3;)
                  end
                  local.get 6
                  i32.const 1
                  i32.add
                  local.set 3
                  br 1 (;@6;)
                end
                local.get 3
                i32.const 1
                i32.add
                local.set 3
              end
              local.get 3
              local.get 2
              i32.lt_u
              br_if 0 (;@5;)
              br 4 (;@1;)
            end
          end
          i64.const 2
          local.set 8
          i32.const 1
          local.set 9
          br 1 (;@2;)
        end
        i64.const 1
        local.set 8
        i32.const 1
        local.set 9
      end
      local.get 0
      local.get 3
      i32.store offset=4
      local.get 0
      i32.const 11
      i32.add
      i32.const 0
      i32.store8
      local.get 0
      i32.const 9
      i32.add
      local.get 8
      i64.store16 align=1
      local.get 0
      i32.const 8
      i32.add
      local.get 9
      i32.store8
      local.get 0
      i32.const 1
      i32.store
      return
    end
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    i32.const 8
    i32.add
    local.get 2
    i32.store
    local.get 0
    i32.const 0
    i32.store
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hbf33bc403fdf05b1E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load
              local.tee 3
              i32.const 16
              i32.and
              br_if 0 (;@5;)
              local.get 3
              i32.const 32
              i32.and
              br_if 1 (;@4;)
              local.get 0
              i64.load8_u
              i32.const 1
              local.get 1
              call $_ZN4core3fmt3num3imp7fmt_u6417hcb4d540e45bed3d8E
              local.set 0
              br 2 (;@3;)
            end
            local.get 0
            i32.load8_u
            local.set 3
            i32.const 0
            local.set 0
            loop  ;; label = @5
              local.get 2
              local.get 0
              i32.add
              i32.const 127
              i32.add
              i32.const 48
              i32.const 87
              local.get 3
              i32.const 15
              i32.and
              local.tee 4
              i32.const 10
              i32.lt_u
              select
              local.get 4
              i32.add
              i32.store8
              local.get 0
              i32.const -1
              i32.add
              local.set 0
              local.get 3
              i32.const 255
              i32.and
              local.tee 4
              i32.const 4
              i32.shr_u
              local.set 3
              local.get 4
              i32.const 15
              i32.gt_u
              br_if 0 (;@5;)
            end
            local.get 0
            i32.const 128
            i32.add
            local.tee 3
            i32.const 129
            i32.ge_u
            br_if 2 (;@2;)
            local.get 1
            i32.const 1
            i32.const 1049448
            i32.const 2
            local.get 2
            local.get 0
            i32.add
            i32.const 128
            i32.add
            i32.const 0
            local.get 0
            i32.sub
            call $_ZN4core3fmt9Formatter12pad_integral17h88d3d7a17289fb2aE
            local.set 0
            br 1 (;@3;)
          end
          local.get 0
          i32.load8_u
          local.set 3
          i32.const 0
          local.set 0
          loop  ;; label = @4
            local.get 2
            local.get 0
            i32.add
            i32.const 127
            i32.add
            i32.const 48
            i32.const 55
            local.get 3
            i32.const 15
            i32.and
            local.tee 4
            i32.const 10
            i32.lt_u
            select
            local.get 4
            i32.add
            i32.store8
            local.get 0
            i32.const -1
            i32.add
            local.set 0
            local.get 3
            i32.const 255
            i32.and
            local.tee 4
            i32.const 4
            i32.shr_u
            local.set 3
            local.get 4
            i32.const 15
            i32.gt_u
            br_if 0 (;@4;)
          end
          local.get 0
          i32.const 128
          i32.add
          local.tee 3
          i32.const 129
          i32.ge_u
          br_if 2 (;@1;)
          local.get 1
          i32.const 1
          i32.const 1049448
          i32.const 2
          local.get 2
          local.get 0
          i32.add
          i32.const 128
          i32.add
          i32.const 0
          local.get 0
          i32.sub
          call $_ZN4core3fmt9Formatter12pad_integral17h88d3d7a17289fb2aE
          local.set 0
        end
        local.get 2
        i32.const 128
        i32.add
        global.set $__stack_pointer
        local.get 0
        return
      end
      local.get 3
      i32.const 128
      i32.const 1049432
      call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
      unreachable
    end
    local.get 3
    i32.const 128
    i32.const 1049432
    call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
    unreachable
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he9943eac91fbe3f2E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h0b19f84ca4cda0d0E
  )
  (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h8cd11e9fef8d604aE (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.tee 0
    i64.extend_i32_u
    local.get 0
    i32.const -1
    i32.xor
    i64.extend_i32_s
    i64.const 1
    i64.add
    local.get 0
    i32.const -1
    i32.gt_s
    local.tee 0
    select
    local.get 0
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417hcb4d540e45bed3d8E
  )
  (func $_ZN53_$LT$core..fmt..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h20b05970f39fae96E (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.load offset=24
    i32.const 1054513
    i32.const 5
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 1)
  )
  (func $_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h1654d1d6387babd5E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i64.const 4294967296
    i64.const 0
    local.get 1
    i32.load offset=24
    i32.const 1052056
    i32.const 9
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 1)
    select
    local.get 1
    i64.extend_i32_u
    i64.or
    i64.store
    local.get 2
    local.get 0
    i32.store offset=12
    local.get 2
    i32.const 1052065
    i32.const 11
    local.get 2
    i32.const 12
    i32.add
    i32.const 1049108
    call $_ZN4core3fmt8builders11DebugStruct5field17h8a23062b858eb435E
    local.set 1
    local.get 2
    local.get 0
    i32.const 4
    i32.add
    i32.store offset=12
    local.get 1
    i32.const 1052076
    i32.const 9
    local.get 2
    i32.const 12
    i32.add
    i32.const 1052088
    call $_ZN4core3fmt8builders11DebugStruct5field17h8a23062b858eb435E
    local.tee 0
    i32.load8_u offset=4
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=5
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 255
      i32.and
      local.set 0
      i32.const 1
      local.set 1
      local.get 0
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 2
        i32.load
        local.tee 1
        i32.load8_u
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        local.get 1
        i32.load offset=24
        i32.const 1049382
        i32.const 2
        local.get 1
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        local.set 1
        br 1 (;@1;)
      end
      local.get 1
      i32.load offset=24
      i32.const 1049374
      i32.const 1
      local.get 1
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 1)
      local.set 1
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
    i32.const 255
    i32.and
    i32.const 0
    i32.ne
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha1290dd7e4804cd6E (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 0
        i32.load8_u
        br_if 0 (;@2;)
        local.get 1
        i32.load offset=24
        i32.const 1052052
        i32.const 4
        local.get 1
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.load offset=24
      i32.const 1052048
      i32.const 4
      local.get 1
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 1)
      i32.store8 offset=8
      local.get 2
      local.get 1
      i32.store
      local.get 2
      i32.const 0
      i32.store8 offset=9
      local.get 2
      i32.const 0
      i32.store offset=4
      i32.const 1
      local.set 1
      local.get 2
      local.get 0
      i32.const 1
      i32.add
      i32.store offset=12
      local.get 2
      local.get 2
      i32.const 12
      i32.add
      i32.const 1049388
      call $_ZN4core3fmt8builders10DebugTuple5field17h4b7d21df77126ad7E
      drop
      local.get 2
      i32.load8_u offset=8
      local.set 0
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.load offset=4
          local.tee 3
          br_if 0 (;@3;)
          local.get 0
          local.set 1
          br 1 (;@2;)
        end
        local.get 0
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        local.get 2
        i32.load
        local.set 0
        block  ;; label = @3
          local.get 3
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          local.get 2
          i32.load8_u offset=9
          i32.const 255
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.load8_u
          i32.const 4
          i32.and
          br_if 0 (;@3;)
          i32.const 1
          local.set 1
          local.get 0
          i32.load offset=24
          i32.const 1049387
          i32.const 1
          local.get 0
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 1)
          br_if 1 (;@2;)
        end
        local.get 0
        i32.load offset=24
        i32.const 1054619
        i32.const 1
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        local.set 1
      end
      local.get 1
      i32.const 255
      i32.and
      i32.const 0
      i32.ne
      local.set 1
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $__original_main (type 17) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 5
          call $malloc
          local.tee 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.const 1852399981
          i32.store align=1
          local.get 0
          i64.const 17179869189
          i64.store offset=60 align=4
          local.get 0
          local.get 1
          i32.store offset=56
          local.get 0
          i32.const 8
          i32.add
          local.get 0
          i32.const 56
          i32.add
          call $_ZN3std3ffi5c_str7CString19_from_vec_unchecked17h0219136b109b7902E
          local.get 0
          i32.load offset=8
          local.get 0
          i32.load offset=12
          call $_ZN3std6thread6Thread3new17hab16636756d5a840E
          local.set 1
          i32.const 0
          i32.load offset=1059912
          br_if 1 (;@2;)
          i32.const 0
          i32.const -1
          i32.store offset=1059912
          i32.const 0
          i32.load offset=1059916
          br_if 2 (;@1;)
          i32.const 0
          local.get 1
          i32.store offset=1059916
          i32.const 0
          i32.const 0
          i32.store offset=1059912
          i32.const 10
          call $_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hd35f5e32e989082cE
          block  ;; label = @4
            i32.const 0
            i32.load offset=1059896
            i32.const 3
            i32.eq
            br_if 0 (;@4;)
            local.get 0
            i32.const 1
            i32.store8 offset=24
            local.get 0
            local.get 0
            i32.const 24
            i32.add
            i32.store offset=56
            i32.const 1059896
            i32.const 0
            local.get 0
            i32.const 56
            i32.add
            i32.const 1055136
            call $_ZN3std4sync4once4Once10call_inner17h06906d6ec9740340E
          end
          local.get 0
          i32.const 80
          i32.add
          global.set $__stack_pointer
          i32.const 0
          return
        end
        i32.const 5
        i32.const 1
        call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
        unreachable
      end
      i32.const 1053428
      i32.const 16
      local.get 0
      i32.const 56
      i32.add
      i32.const 1055820
      i32.const 1055756
      call $_ZN4core6result13unwrap_failed17h1b005d733a1499a6E
      unreachable
    end
    local.get 0
    i32.const 24
    i32.add
    i32.const 20
    i32.add
    i32.const 1
    i32.store
    local.get 0
    i32.const 56
    i32.add
    i32.const 20
    i32.add
    i32.const 0
    i32.store
    local.get 0
    i64.const 2
    i64.store offset=28 align=4
    local.get 0
    i32.const 1055492
    i32.store offset=24
    local.get 0
    i32.const 5
    i32.store offset=52
    local.get 0
    i32.const 1053116
    i32.store offset=72
    local.get 0
    i64.const 1
    i64.store offset=60 align=4
    local.get 0
    i32.const 1055812
    i32.store offset=56
    local.get 0
    local.get 0
    i32.const 48
    i32.add
    i32.store offset=40
    local.get 0
    local.get 0
    i32.const 56
    i32.add
    i32.store offset=48
    local.get 0
    local.get 0
    i32.const 56
    i32.add
    local.get 0
    i32.const 24
    i32.add
    call $_ZN3std2io5Write9write_fmt17h49c2f05193f87c52E
    i64.store offset=16
    local.get 0
    i32.const 16
    i32.add
    call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17hb77b520a01bfdb66E
    call $_ZN3std3sys4wasi14abort_internal17h6f2ef412fa287f15E
    unreachable
  )
  (func $_ZN3std3ffi5c_str7CString19_from_vec_unchecked17h0219136b109b7902E (type 5) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                i32.const 4
                i32.add
                i32.load
                local.tee 3
                local.get 1
                i32.load offset=8
                local.tee 4
                i32.ne
                br_if 0 (;@6;)
                local.get 4
                i32.const 1
                i32.add
                local.tee 3
                local.get 4
                i32.lt_u
                br_if 4 (;@2;)
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 4
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 5
                    br 1 (;@7;)
                  end
                  local.get 1
                  i32.load
                  local.set 5
                end
                local.get 2
                local.get 3
                local.get 5
                local.get 4
                i32.const 1
                call $_ZN5alloc7raw_vec11finish_grow17hb55c4957f32437bbE
                local.get 2
                i32.load
                br_if 1 (;@5;)
                local.get 2
                i32.load offset=4
                local.set 5
                local.get 1
                i32.const 4
                i32.add
                local.get 3
                i32.store
                local.get 1
                local.get 5
                i32.store
              end
              block  ;; label = @6
                local.get 4
                local.get 3
                i32.ne
                br_if 0 (;@6;)
                local.get 1
                local.get 4
                call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h2033c1963b518374E
                local.get 1
                i32.const 4
                i32.add
                i32.load
                local.set 3
                local.get 1
                i32.load offset=8
                local.set 4
              end
              local.get 1
              local.get 4
              i32.const 1
              i32.add
              local.tee 5
              i32.store offset=8
              local.get 1
              i32.load
              local.tee 1
              local.get 4
              i32.add
              i32.const 0
              i32.store8
              local.get 3
              local.get 5
              i32.le_u
              br_if 2 (;@3;)
              local.get 5
              br_if 1 (;@4;)
              local.get 1
              call $free
              i32.const 1
              local.set 1
              br 2 (;@3;)
            end
            local.get 2
            i32.const 8
            i32.add
            i32.load
            local.tee 1
            i32.eqz
            br_if 2 (;@2;)
            local.get 2
            i32.load offset=4
            local.get 1
            call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
            unreachable
          end
          local.get 1
          local.get 5
          call $realloc
          local.tee 1
          i32.eqz
          br_if 2 (;@1;)
        end
        local.get 0
        local.get 5
        i32.store offset=4
        local.get 0
        local.get 1
        i32.store
        local.get 2
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h0f57087aa53660d1E
      unreachable
    end
    local.get 5
    i32.const 1
    call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
    unreachable
  )
  (func $_ZN3std6thread6Thread3new17hab16636756d5a840E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 0
    i32.load8_u offset=1059864
    local.set 3
    i32.const 0
    i32.const 1
    i32.store8 offset=1059864
    local.get 2
    local.get 3
    i32.store8 offset=7
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i64.load offset=1059840
            local.tee 4
            i64.const -1
            i64.eq
            br_if 0 (;@4;)
            i32.const 0
            local.get 4
            i64.const 1
            i64.add
            i64.store offset=1059840
            local.get 4
            i64.const 0
            i64.ne
            br_if 1 (;@3;)
            i32.const 1055427
            i32.const 43
            i32.const 1053336
            call $_ZN4core9panicking5panic17h66d99d0b614b1c7cE
            unreachable
          end
          i32.const 0
          i32.const 0
          i32.store8 offset=1059864
          local.get 2
          i32.const 28
          i32.add
          i32.const 0
          i32.store
          local.get 2
          i32.const 1053116
          i32.store offset=24
          local.get 2
          i64.const 1
          i64.store offset=12 align=4
          local.get 2
          i32.const 1053312
          i32.store offset=8
          local.get 2
          i32.const 8
          i32.add
          i32.const 1053320
          call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
          unreachable
        end
        i32.const 0
        i32.const 0
        i32.store8 offset=1059864
        i32.const 32
        call $malloc
        local.tee 3
        i32.eqz
        br_if 1 (;@1;)
        local.get 3
        i64.const 0
        i64.store offset=24
        local.get 3
        local.get 1
        i32.store offset=20
        local.get 3
        local.get 0
        i32.store offset=16
        local.get 3
        local.get 4
        i64.store offset=8
        local.get 3
        i64.const 4294967297
        i64.store
        local.get 2
        i32.const 32
        i32.add
        global.set $__stack_pointer
        local.get 3
        return
      end
      local.get 2
      i32.const 28
      i32.add
      i32.const 0
      i32.store
      local.get 2
      i32.const 24
      i32.add
      i32.const 1053116
      i32.store
      local.get 2
      i64.const 1
      i64.store offset=12 align=4
      local.get 2
      i32.const 1057332
      i32.store offset=8
      local.get 2
      i32.const 7
      i32.add
      local.get 2
      i32.const 8
      i32.add
      call $_ZN4core9panicking13assert_failed17hfcfdb11ed0fde9ebE
      unreachable
    end
    i32.const 32
    i32.const 8
    call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
    unreachable
  )
  (func $_ZN10hello_rust4main17hb681533656a5f6b2E (type 4)
    (local i32 i32 i32 i64 i64 i32 i32 i32 i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 96
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 6
    i32.store offset=60
    local.get 0
    i32.const 1054796
    i32.store offset=56
    block  ;; label = @1
      i32.const 0
      i32.load offset=1059868
      i32.const 3
      i32.eq
      br_if 0 (;@1;)
      i32.const 0
      i32.load offset=1059868
      i32.const 3
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      i32.const 1059872
      i32.store offset=8
      local.get 0
      local.get 0
      i32.const 8
      i32.add
      i32.store offset=72
      i32.const 1059868
      i32.const 1
      local.get 0
      i32.const 72
      i32.add
      i32.const 1055200
      call $_ZN3std4sync4once4Once10call_inner17h06906d6ec9740340E
    end
    local.get 0
    i32.const 1059872
    i32.store offset=32
    local.get 0
    i32.const 4
    i32.store8 offset=12
    local.get 0
    local.get 0
    i32.const 32
    i32.add
    i32.store offset=8
    local.get 0
    i32.const 92
    i32.add
    i32.const 0
    i32.store
    local.get 0
    i32.const 1053116
    i32.store offset=88
    local.get 0
    i64.const 1
    i64.store offset=76 align=4
    local.get 0
    i32.const 1052952
    i32.store offset=72
    local.get 0
    i32.const 8
    i32.add
    i32.const 1054896
    local.get 0
    i32.const 72
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 1
    local.get 0
    i32.load8_u offset=12
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 1
                                    i32.eqz
                                    br_if 0 (;@16;)
                                    local.get 2
                                    i32.const 255
                                    i32.and
                                    i32.const 4
                                    i32.ne
                                    br_if 1 (;@15;)
                                    i32.const 1054860
                                    i64.extend_i32_u
                                    i64.const 32
                                    i64.shl
                                    i64.const 8
                                    i64.shr_u
                                    local.set 3
                                    i64.const 2
                                    local.set 4
                                    br 2 (;@14;)
                                  end
                                  block  ;; label = @16
                                    local.get 2
                                    i32.const 255
                                    i32.and
                                    i32.const 3
                                    i32.ne
                                    br_if 0 (;@16;)
                                    local.get 0
                                    i32.const 16
                                    i32.add
                                    i32.load
                                    local.tee 2
                                    i32.load
                                    local.get 2
                                    i32.load offset=4
                                    i32.load
                                    call_indirect (type 0)
                                    block  ;; label = @17
                                      local.get 2
                                      i32.load offset=4
                                      i32.load offset=4
                                      i32.eqz
                                      br_if 0 (;@17;)
                                      local.get 2
                                      i32.load
                                      call $free
                                    end
                                    local.get 0
                                    i32.load offset=16
                                    call $free
                                  end
                                  local.get 0
                                  i32.const 72
                                  i32.add
                                  i32.const 1052960
                                  i32.const 13
                                  call $_ZN70_$LT$$RF$str$u20$as$u20$std..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17he72994ed41181ff3E
                                  local.get 0
                                  i32.load offset=72
                                  i32.eqz
                                  br_if 2 (;@13;)
                                  block  ;; label = @16
                                    local.get 0
                                    i32.const 84
                                    i32.add
                                    i32.load
                                    i32.eqz
                                    br_if 0 (;@16;)
                                    local.get 0
                                    i32.const 80
                                    i32.add
                                    i32.load
                                    local.tee 5
                                    br_if 4 (;@12;)
                                  end
                                  i32.const 2
                                  local.set 6
                                  i32.const 1053584
                                  local.set 1
                                  br 13 (;@2;)
                                end
                                local.get 2
                                i64.extend_i32_u
                                i64.const 255
                                i64.and
                                local.set 4
                                local.get 0
                                i64.load32_u offset=13 align=1
                                local.get 0
                                i32.const 17
                                i32.add
                                i64.load16_u align=1
                                local.get 0
                                i32.const 19
                                i32.add
                                i64.load8_u
                                i64.const 16
                                i64.shl
                                i64.or
                                i64.const 32
                                i64.shl
                                i64.or
                                local.set 3
                              end
                              local.get 0
                              local.get 3
                              i64.const 24
                              i64.shr_u
                              i64.store32 offset=36
                              local.get 0
                              local.get 3
                              i64.const 8
                              i64.shl
                              local.get 4
                              i64.or
                              i64.store32 offset=32
                              local.get 0
                              i32.const 92
                              i32.add
                              i32.const 2
                              i32.store
                              local.get 0
                              i32.const 20
                              i32.add
                              i32.const 11
                              i32.store
                              local.get 0
                              i64.const 2
                              i64.store offset=76 align=4
                              local.get 0
                              i32.const 1054764
                              i32.store offset=72
                              local.get 0
                              i32.const 4
                              i32.store offset=12
                              local.get 0
                              local.get 0
                              i32.const 8
                              i32.add
                              i32.store offset=88
                              local.get 0
                              local.get 0
                              i32.const 32
                              i32.add
                              i32.store offset=16
                              local.get 0
                              local.get 0
                              i32.const 56
                              i32.add
                              i32.store offset=8
                              local.get 0
                              i32.const 72
                              i32.add
                              i32.const 1054780
                              call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
                              unreachable
                            end
                            local.get 0
                            local.get 0
                            i32.load offset=76
                            local.tee 1
                            i32.store offset=24
                            local.get 0
                            local.get 0
                            i32.const 80
                            i32.add
                            i32.load
                            i32.store offset=28
                            i32.const 512
                            local.set 7
                            i32.const 512
                            call $malloc
                            local.tee 8
                            i32.eqz
                            br_if 1 (;@11;)
                            local.get 0
                            i32.const 512
                            i32.store offset=36
                            local.get 0
                            local.get 8
                            i32.store offset=32
                            local.get 0
                            local.get 8
                            i32.store offset=48
                            i32.const 0
                            local.set 2
                            local.get 0
                            i32.const 0
                            i32.store offset=52
                            local.get 1
                            local.get 0
                            i32.const 52
                            i32.add
                            local.get 0
                            i32.const 48
                            i32.add
                            i32.const 512
                            call $__wasilibc_find_relpath
                            local.tee 6
                            i32.const -1
                            i32.ne
                            br_if 3 (;@9;)
                            i32.const 512
                            local.set 7
                            i32.const 0
                            i32.load offset=1060428
                            i32.const 48
                            i32.ne
                            br_if 2 (;@10;)
                            i32.const 512
                            local.set 7
                            loop  ;; label = @13
                              local.get 0
                              local.get 7
                              i32.store offset=40
                              local.get 0
                              i32.const 32
                              i32.add
                              local.get 7
                              i32.const 1
                              call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h9e9d16cf4f4f67caE
                              local.get 0
                              local.get 0
                              i32.load offset=32
                              local.tee 8
                              i32.store offset=48
                              local.get 0
                              i32.const 0
                              i32.store offset=52
                              local.get 1
                              local.get 0
                              i32.const 52
                              i32.add
                              local.get 0
                              i32.const 48
                              i32.add
                              local.get 0
                              i32.load offset=36
                              local.tee 7
                              call $__wasilibc_find_relpath
                              local.tee 6
                              i32.const -1
                              i32.ne
                              br_if 4 (;@9;)
                              i32.const 0
                              i32.load offset=1060428
                              i32.const 48
                              i32.ne
                              br_if 3 (;@10;)
                              br 0 (;@13;)
                            end
                          end
                          i32.const 1053584
                          local.set 1
                          i32.const 2
                          local.set 6
                          i32.const 1
                          local.set 2
                          br 4 (;@7;)
                        end
                        i32.const 512
                        i32.const 1
                        call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
                        unreachable
                      end
                      local.get 0
                      i32.const 12
                      i32.store offset=60
                      local.get 0
                      local.get 0
                      i32.const 24
                      i32.add
                      i32.store offset=56
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              i32.const 148
                              call $malloc
                              local.tee 2
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 0
                              i64.const 148
                              i64.store offset=12 align=4
                              local.get 0
                              local.get 2
                              i32.store offset=8
                              local.get 0
                              local.get 0
                              i32.const 8
                              i32.add
                              i32.store offset=68
                              local.get 0
                              i32.const 92
                              i32.add
                              i32.const 1
                              i32.store
                              local.get 0
                              i64.const 2
                              i64.store offset=76 align=4
                              local.get 0
                              i32.const 1057284
                              i32.store offset=72
                              local.get 0
                              local.get 0
                              i32.const 56
                              i32.add
                              i32.store offset=88
                              local.get 0
                              i32.const 68
                              i32.add
                              i32.const 1048872
                              local.get 0
                              i32.const 72
                              i32.add
                              call $_ZN4core3fmt5write17h8776c655b56f9e02E
                              br_if 1 (;@12;)
                              i32.const 12
                              call $malloc
                              local.tee 2
                              i32.eqz
                              br_if 2 (;@11;)
                              local.get 2
                              local.get 0
                              i64.load offset=8
                              i64.store align=4
                              local.get 2
                              i32.const 8
                              i32.add
                              local.get 0
                              i32.const 8
                              i32.add
                              i32.const 8
                              i32.add
                              i32.load
                              i32.store
                              i32.const 12
                              call $malloc
                              local.tee 1
                              i32.eqz
                              br_if 3 (;@10;)
                              local.get 1
                              i32.const 40
                              i32.store8 offset=8
                              local.get 1
                              i32.const 1053384
                              i32.store offset=4
                              local.get 1
                              local.get 2
                              i32.store
                              i32.const 3
                              local.set 6
                              i32.const 1
                              local.set 2
                              br 5 (;@8;)
                            end
                            i32.const 148
                            i32.const 1
                            call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
                            unreachable
                          end
                          i32.const 1048764
                          i32.const 51
                          local.get 0
                          i32.const 72
                          i32.add
                          i32.const 1048856
                          i32.const 1048840
                          call $_ZN4core6result13unwrap_failed17h1b005d733a1499a6E
                          unreachable
                        end
                        i32.const 12
                        i32.const 4
                        call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
                        unreachable
                      end
                      i32.const 12
                      i32.const 4
                      call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
                      unreachable
                    end
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        i32.load offset=48
                        local.tee 9
                        i32.load8_u
                        br_if 0 (;@10;)
                        i32.const 1
                        local.set 1
                        br 1 (;@9;)
                      end
                      local.get 9
                      i32.const 1
                      i32.add
                      local.set 10
                      i32.const 0
                      local.set 2
                      loop  ;; label = @10
                        local.get 10
                        local.get 2
                        i32.add
                        local.set 1
                        local.get 2
                        i32.const 1
                        i32.add
                        local.tee 5
                        local.set 2
                        local.get 1
                        i32.load8_u
                        br_if 0 (;@10;)
                      end
                      i32.const 0
                      local.set 2
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 5
                          i32.const 0
                          i32.lt_s
                          br_if 0 (;@11;)
                          local.get 5
                          br_if 1 (;@10;)
                          i32.const 1
                          local.set 1
                          br 2 (;@9;)
                        end
                        call $_ZN5alloc7raw_vec17capacity_overflow17h0f57087aa53660d1E
                        unreachable
                      end
                      local.get 5
                      call $malloc
                      local.tee 1
                      i32.eqz
                      br_if 4 (;@5;)
                      local.get 5
                      local.set 2
                    end
                    local.get 1
                    local.get 9
                    local.get 2
                    call $memcpy
                    drop
                    local.get 2
                    i64.extend_i32_u
                    local.tee 3
                    i64.const 32
                    i64.shl
                    local.get 3
                    i64.or
                    local.set 4
                    i32.const 0
                    local.set 2
                  end
                  block  ;; label = @8
                    local.get 7
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 8
                    call $free
                  end
                  local.get 0
                  i32.load offset=24
                  i32.const 0
                  i32.store8
                  local.get 0
                  i32.load offset=28
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 0
                  i32.load offset=24
                  local.set 5
                end
                local.get 5
                call $free
              end
              local.get 2
              br_if 3 (;@2;)
              local.get 0
              i32.const 72
              i32.add
              local.get 1
              local.get 4
              i64.const 32
              i64.shr_u
              i32.wrap_i64
              call $_ZN4core3str8converts9from_utf817h5e14e3bada571d6dE
              i32.const 1054940
              i64.extend_i32_u
              i64.const 32
              i64.shl
              i64.const 2
              i64.or
              local.set 3
              i32.const 1
              local.set 2
              block  ;; label = @6
                local.get 0
                i32.load offset=72
                br_if 0 (;@6;)
                local.get 0
                i32.load offset=76
                local.tee 5
                i32.eqz
                br_if 0 (;@6;)
                i32.const 1
                local.set 2
                block  ;; label = @7
                  local.get 6
                  i32.const 1
                  local.get 5
                  local.get 0
                  i32.const 80
                  i32.add
                  i32.load
                  i32.const 9
                  i64.const 266846205
                  i64.const 266846205
                  i32.const 0
                  local.get 0
                  i32.const 72
                  i32.add
                  call $_ZN4wasi13lib_generated22wasi_snapshot_preview19path_open17hc00f9638ef14f9b0E
                  local.tee 5
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 0
                  i32.load offset=72
                  local.tee 2
                  i32.store offset=8
                  local.get 2
                  i32.const -1
                  i32.eq
                  br_if 3 (;@4;)
                  local.get 2
                  i64.extend_i32_u
                  local.set 3
                  i32.const 0
                  local.set 2
                  br 1 (;@6;)
                end
                local.get 5
                i32.const 65535
                i32.and
                i64.extend_i32_u
                i64.const 32
                i64.shl
                local.set 3
              end
              block  ;; label = @6
                local.get 4
                i32.wrap_i64
                i32.eqz
                br_if 0 (;@6;)
                local.get 1
                call $free
              end
              local.get 3
              local.get 3
              i64.const 4294967295
              i64.and
              local.get 2
              select
              local.set 3
              local.get 2
              br_if 4 (;@1;)
              local.get 0
              local.get 3
              i64.store32 offset=32
              local.get 0
              i32.const 4
              i32.store8 offset=12
              local.get 0
              local.get 0
              i32.const 32
              i32.add
              i32.store offset=8
              local.get 0
              i32.const 92
              i32.add
              i32.const 0
              i32.store
              local.get 0
              i32.const 1053116
              i32.store offset=88
              local.get 0
              i64.const 1
              i64.store offset=76 align=4
              local.get 0
              i32.const 1052952
              i32.store offset=72
              local.get 0
              i32.const 8
              i32.add
              i32.const 1048668
              local.get 0
              i32.const 72
              i32.add
              call $_ZN4core3fmt5write17h8776c655b56f9e02E
              local.set 1
              local.get 0
              i32.load8_u offset=12
              local.set 2
              local.get 1
              br_if 2 (;@3;)
              block  ;; label = @6
                local.get 2
                i32.const 255
                i32.and
                i32.const 3
                i32.ne
                br_if 0 (;@6;)
                local.get 0
                i32.const 16
                i32.add
                i32.load
                local.tee 2
                i32.load
                local.get 2
                i32.load offset=4
                i32.load
                call_indirect (type 0)
                block  ;; label = @7
                  local.get 2
                  i32.load offset=4
                  i32.load offset=4
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 2
                  i32.load
                  call $free
                end
                local.get 0
                i32.load offset=16
                call $free
              end
              local.get 0
              i32.load offset=32
              call $close
              drop
              local.get 0
              i32.const 96
              i32.add
              global.set $__stack_pointer
              return
            end
            local.get 5
            i32.const 1
            call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
            unreachable
          end
          local.get 0
          i32.const 0
          i32.store offset=72
          local.get 0
          i32.const 8
          i32.add
          local.get 0
          i32.const 72
          i32.add
          call $_ZN4core9panicking13assert_failed17ha4700b3711575da3E
          unreachable
        end
        local.get 0
        i32.const 17
        i32.add
        i64.load16_u align=1
        local.set 3
        local.get 0
        i32.const 19
        i32.add
        i64.load8_u
        local.set 4
        local.get 0
        i64.load32_u offset=13 align=1
        local.set 11
        local.get 0
        i32.const 2
        local.get 2
        local.get 2
        i32.const 255
        i32.and
        i32.const 4
        i32.eq
        local.tee 1
        select
        i32.store8 offset=72
        local.get 0
        i32.const 1054860
        i64.extend_i32_u
        i64.const 24
        i64.shl
        local.get 11
        local.get 3
        local.get 4
        i64.const 16
        i64.shl
        i64.or
        i64.const 32
        i64.shl
        i64.or
        local.get 1
        select
        local.tee 3
        i64.store32 offset=73 align=1
        local.get 0
        local.get 3
        i64.const 48
        i64.shr_u
        i64.store8 offset=79
        local.get 0
        local.get 3
        i64.const 32
        i64.shr_u
        i64.store16 offset=77 align=1
        i32.const 1055836
        i32.const 43
        local.get 0
        i32.const 72
        i32.add
        i32.const 1053076
        i32.const 1053060
        call $_ZN4core6result13unwrap_failed17h1b005d733a1499a6E
        unreachable
      end
      local.get 1
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 6
      i64.extend_i32_u
      i64.or
      local.set 3
    end
    local.get 0
    local.get 3
    i64.store offset=72
    i32.const 1055836
    i32.const 43
    local.get 0
    i32.const 72
    i32.add
    i32.const 1053076
    i32.const 1053044
    call $_ZN4core6result13unwrap_failed17h1b005d733a1499a6E
    unreachable
  )
  (func $_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hd35f5e32e989082cE (type 0) (param i32)
    local.get 0
    call_indirect (type 4)
  )
  (func $_ZN3std4sync4once4Once10call_inner17h06906d6ec9740340E (type 18) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 4
    i32.const 8
    i32.add
    i32.const 2
    i32.or
    local.set 5
    local.get 0
    i32.load
    local.set 6
    loop  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 6
                      local.tee 7
                      br_table 1 (;@8;) 0 (;@9;) 7 (;@2;) 2 (;@7;) 7 (;@2;)
                    end
                    local.get 1
                    i32.eqz
                    br_if 5 (;@3;)
                  end
                  local.get 0
                  i32.const 2
                  local.get 0
                  i32.load
                  local.tee 6
                  local.get 6
                  local.get 7
                  i32.eq
                  local.tee 8
                  select
                  i32.store
                  local.get 8
                  i32.eqz
                  br_if 6 (;@1;)
                  local.get 4
                  local.get 7
                  i32.const 1
                  i32.eq
                  i32.store8 offset=28
                  local.get 4
                  i32.const 3
                  i32.store offset=24
                  local.get 2
                  local.get 4
                  i32.const 24
                  i32.add
                  local.get 3
                  i32.load offset=16
                  call_indirect (type 5)
                  local.get 0
                  i32.load
                  local.set 7
                  local.get 0
                  local.get 4
                  i32.load offset=24
                  i32.store
                  local.get 4
                  local.get 7
                  i32.const 3
                  i32.and
                  local.tee 6
                  i32.store offset=8
                  local.get 6
                  i32.const 2
                  i32.ne
                  br_if 1 (;@6;)
                  local.get 7
                  i32.const -4
                  i32.and
                  local.tee 6
                  i32.eqz
                  br_if 0 (;@7;)
                  loop  ;; label = @8
                    local.get 6
                    i32.load
                    local.set 7
                    local.get 6
                    i32.const 0
                    i32.store
                    local.get 7
                    i32.eqz
                    br_if 3 (;@5;)
                    local.get 6
                    i32.load offset=4
                    local.set 8
                    local.get 6
                    i32.const 1
                    i32.store8 offset=8
                    local.get 7
                    i32.load offset=24
                    local.set 6
                    local.get 7
                    i32.const 2
                    i32.store offset=24
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 6
                          br_table 2 (;@9;) 1 (;@10;) 2 (;@9;) 0 (;@11;)
                        end
                        local.get 4
                        i32.const 44
                        i32.add
                        i32.const 0
                        i32.store
                        local.get 4
                        i32.const 1053116
                        i32.store offset=40
                        local.get 4
                        i64.const 1
                        i64.store offset=28 align=4
                        local.get 4
                        i32.const 1057732
                        i32.store offset=24
                        local.get 4
                        i32.const 24
                        i32.add
                        i32.const 1057740
                        call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
                        unreachable
                      end
                      local.get 7
                      i32.const 24
                      i32.add
                      local.tee 6
                      i32.load8_u offset=4
                      local.set 0
                      local.get 6
                      i32.const 1
                      i32.store8 offset=4
                      local.get 4
                      local.get 0
                      i32.const 1
                      i32.and
                      local.tee 0
                      i32.store8 offset=8
                      local.get 0
                      br_if 5 (;@4;)
                      local.get 6
                      i32.const 4
                      i32.add
                      local.set 0
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                i32.const 0
                                i32.load offset=1059908
                                i32.const 2147483647
                                i32.and
                                i32.eqz
                                br_if 0 (;@14;)
                                call $_ZN3std9panicking11panic_count17is_zero_slow_path17h4ea576c666392881E
                                local.set 5
                                local.get 6
                                i32.const 5
                                i32.add
                                i32.load8_u
                                i32.eqz
                                br_if 2 (;@12;)
                                local.get 5
                                i32.const 1
                                i32.xor
                                local.set 7
                                br 1 (;@13;)
                              end
                              local.get 6
                              i32.const 5
                              i32.add
                              i32.load8_u
                              i32.eqz
                              br_if 2 (;@11;)
                              i32.const 0
                              local.set 7
                            end
                            local.get 4
                            local.get 7
                            i32.store8 offset=28
                            local.get 4
                            local.get 0
                            i32.store offset=24
                            i32.const 1055836
                            i32.const 43
                            local.get 4
                            i32.const 24
                            i32.add
                            i32.const 1055896
                            i32.const 1057756
                            call $_ZN4core6result13unwrap_failed17h1b005d733a1499a6E
                            unreachable
                          end
                          local.get 5
                          i32.eqz
                          br_if 1 (;@10;)
                        end
                        i32.const 0
                        i32.load offset=1059908
                        i32.const 2147483647
                        i32.and
                        i32.eqz
                        br_if 0 (;@10;)
                        call $_ZN3std9panicking11panic_count17is_zero_slow_path17h4ea576c666392881E
                        br_if 0 (;@10;)
                        local.get 0
                        i32.const 1
                        i32.store8 offset=1
                      end
                      local.get 0
                      i32.const 0
                      i32.store8
                    end
                    local.get 7
                    local.get 7
                    i32.load
                    local.tee 6
                    i32.const -1
                    i32.add
                    i32.store
                    block  ;; label = @9
                      local.get 6
                      i32.const 1
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 7
                      call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17ha7cc5cab043eefa7E
                    end
                    local.get 8
                    local.set 6
                    local.get 8
                    br_if 0 (;@8;)
                  end
                end
                local.get 4
                i32.const 48
                i32.add
                global.set $__stack_pointer
                return
              end
              local.get 4
              i32.const 0
              i32.store offset=24
              local.get 4
              i32.const 8
              i32.add
              local.get 4
              i32.const 24
              i32.add
              i32.const 1055384
              call $_ZN4core9panicking13assert_failed17h24e62d98e3800fc3E
              unreachable
            end
            i32.const 1055427
            i32.const 43
            i32.const 1055400
            call $_ZN4core9panicking5panic17h66d99d0b614b1c7cE
            unreachable
          end
          local.get 4
          i32.const 44
          i32.add
          i32.const 0
          i32.store
          local.get 4
          i32.const 40
          i32.add
          i32.const 1053116
          i32.store
          local.get 4
          i64.const 1
          i64.store offset=28 align=4
          local.get 4
          i32.const 1057332
          i32.store offset=24
          local.get 4
          i32.const 8
          i32.add
          local.get 4
          i32.const 24
          i32.add
          call $_ZN4core9panicking13assert_failed17hfcfdb11ed0fde9ebE
          unreachable
        end
        local.get 4
        i32.const 44
        i32.add
        i32.const 0
        i32.store
        local.get 4
        i32.const 1053116
        i32.store offset=40
        local.get 4
        i64.const 1
        i64.store offset=28 align=4
        local.get 4
        i32.const 1055356
        i32.store offset=24
        local.get 4
        i32.const 24
        i32.add
        i32.const 1055364
        call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
        unreachable
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 7
                              i32.const 3
                              i32.and
                              i32.const 2
                              i32.ne
                              br_if 0 (;@13;)
                              loop  ;; label = @14
                                local.get 7
                                local.set 8
                                i32.const 0
                                i32.load offset=1059912
                                br_if 2 (;@12;)
                                i32.const 0
                                i32.const -1
                                i32.store offset=1059912
                                block  ;; label = @15
                                  i32.const 0
                                  i32.load offset=1059916
                                  local.tee 6
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  i32.const 0
                                  local.get 7
                                  call $_ZN3std6thread6Thread3new17hab16636756d5a840E
                                  local.tee 6
                                  i32.store offset=1059916
                                end
                                local.get 6
                                local.get 6
                                i32.load
                                local.tee 7
                                i32.const 1
                                i32.add
                                i32.store
                                local.get 7
                                i32.const -1
                                i32.le_s
                                br_if 3 (;@11;)
                                i32.const 0
                                i32.const 0
                                i32.load offset=1059912
                                i32.const 1
                                i32.add
                                i32.store offset=1059912
                                local.get 6
                                i32.eqz
                                br_if 4 (;@10;)
                                local.get 0
                                local.get 5
                                local.get 0
                                i32.load
                                local.tee 7
                                local.get 7
                                local.get 8
                                i32.eq
                                select
                                i32.store
                                local.get 4
                                i32.const 0
                                i32.store8 offset=16
                                local.get 4
                                local.get 6
                                i32.store offset=8
                                local.get 4
                                local.get 8
                                i32.const -4
                                i32.and
                                i32.store offset=12
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 7
                                    local.get 8
                                    i32.ne
                                    br_if 0 (;@16;)
                                    local.get 4
                                    i32.load8_u offset=16
                                    i32.eqz
                                    br_if 1 (;@15;)
                                    br 13 (;@3;)
                                  end
                                  block  ;; label = @16
                                    local.get 4
                                    i32.load offset=8
                                    local.tee 6
                                    i32.eqz
                                    br_if 0 (;@16;)
                                    local.get 6
                                    local.get 6
                                    i32.load
                                    local.tee 8
                                    i32.const -1
                                    i32.add
                                    i32.store
                                    local.get 8
                                    i32.const 1
                                    i32.ne
                                    br_if 0 (;@16;)
                                    local.get 4
                                    i32.load offset=8
                                    call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17ha7cc5cab043eefa7E
                                  end
                                  local.get 7
                                  i32.const 3
                                  i32.and
                                  i32.const 2
                                  i32.eq
                                  br_if 1 (;@14;)
                                  br 13 (;@2;)
                                end
                              end
                              loop  ;; label = @14
                                i32.const 0
                                i32.load offset=1059912
                                br_if 5 (;@9;)
                                i32.const 0
                                i32.const -1
                                i32.store offset=1059912
                                block  ;; label = @15
                                  i32.const 0
                                  i32.load offset=1059916
                                  local.tee 7
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  i32.const 0
                                  local.get 7
                                  call $_ZN3std6thread6Thread3new17hab16636756d5a840E
                                  local.tee 7
                                  i32.store offset=1059916
                                end
                                local.get 7
                                local.get 7
                                i32.load
                                local.tee 6
                                i32.const 1
                                i32.add
                                i32.store
                                local.get 6
                                i32.const -1
                                i32.le_s
                                br_if 3 (;@11;)
                                i32.const 0
                                i32.const 0
                                i32.load offset=1059912
                                i32.const 1
                                i32.add
                                i32.store offset=1059912
                                local.get 7
                                i32.eqz
                                br_if 4 (;@10;)
                                local.get 7
                                i32.const 0
                                local.get 7
                                i32.load offset=24
                                local.tee 6
                                local.get 6
                                i32.const 2
                                i32.eq
                                local.tee 6
                                select
                                i32.store offset=24
                                block  ;; label = @15
                                  local.get 6
                                  br_if 0 (;@15;)
                                  local.get 7
                                  i32.const 24
                                  i32.add
                                  local.tee 6
                                  i32.load8_u offset=4
                                  local.set 8
                                  local.get 6
                                  i32.const 1
                                  i32.store8 offset=4
                                  local.get 4
                                  local.get 8
                                  i32.const 1
                                  i32.and
                                  local.tee 8
                                  i32.store8 offset=20
                                  local.get 8
                                  br_if 7 (;@8;)
                                  i32.const 0
                                  local.set 9
                                  block  ;; label = @16
                                    i32.const 0
                                    i32.load offset=1059908
                                    i32.const 2147483647
                                    i32.and
                                    i32.eqz
                                    br_if 0 (;@16;)
                                    call $_ZN3std9panicking11panic_count17is_zero_slow_path17h4ea576c666392881E
                                    i32.const 1
                                    i32.xor
                                    local.set 9
                                  end
                                  local.get 6
                                  i32.const 4
                                  i32.add
                                  local.set 10
                                  local.get 6
                                  i32.const 5
                                  i32.add
                                  i32.load8_u
                                  br_if 8 (;@7;)
                                  local.get 6
                                  local.get 6
                                  i32.load
                                  local.tee 8
                                  i32.const 1
                                  local.get 8
                                  select
                                  i32.store
                                  local.get 8
                                  i32.eqz
                                  br_if 11 (;@4;)
                                  local.get 8
                                  i32.const 2
                                  i32.ne
                                  br_if 9 (;@6;)
                                  local.get 6
                                  i32.load
                                  local.set 8
                                  local.get 6
                                  i32.const 0
                                  i32.store
                                  local.get 4
                                  local.get 8
                                  i32.store offset=20
                                  local.get 8
                                  i32.const 2
                                  i32.ne
                                  br_if 10 (;@5;)
                                  block  ;; label = @16
                                    local.get 9
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    i32.load offset=1059908
                                    i32.const 2147483647
                                    i32.and
                                    i32.eqz
                                    br_if 0 (;@16;)
                                    call $_ZN3std9panicking11panic_count17is_zero_slow_path17h4ea576c666392881E
                                    br_if 0 (;@16;)
                                    local.get 6
                                    i32.const 1
                                    i32.store8 offset=5
                                  end
                                  local.get 10
                                  i32.const 0
                                  i32.store8
                                end
                                local.get 7
                                local.get 7
                                i32.load
                                local.tee 6
                                i32.const -1
                                i32.add
                                i32.store
                                block  ;; label = @15
                                  local.get 6
                                  i32.const 1
                                  i32.ne
                                  br_if 0 (;@15;)
                                  local.get 7
                                  call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17ha7cc5cab043eefa7E
                                end
                                local.get 4
                                i32.load8_u offset=16
                                br_if 11 (;@3;)
                                br 0 (;@14;)
                              end
                            end
                            i32.const 1055236
                            i32.const 57
                            i32.const 1055296
                            call $_ZN4core9panicking5panic17h66d99d0b614b1c7cE
                            unreachable
                          end
                          i32.const 1053428
                          i32.const 16
                          local.get 4
                          i32.const 24
                          i32.add
                          i32.const 1055820
                          i32.const 1055740
                          call $_ZN4core6result13unwrap_failed17h1b005d733a1499a6E
                          unreachable
                        end
                        unreachable
                        unreachable
                      end
                      call $_ZN4core6option13expect_failed17h492d2d279a279116E
                      unreachable
                    end
                    i32.const 1053428
                    i32.const 16
                    local.get 4
                    i32.const 24
                    i32.add
                    i32.const 1055820
                    i32.const 1055740
                    call $_ZN4core6result13unwrap_failed17h1b005d733a1499a6E
                    unreachable
                  end
                  local.get 4
                  i32.const 44
                  i32.add
                  i32.const 0
                  i32.store
                  local.get 4
                  i32.const 40
                  i32.add
                  i32.const 1053116
                  i32.store
                  local.get 4
                  i64.const 1
                  i64.store offset=28 align=4
                  local.get 4
                  i32.const 1057332
                  i32.store offset=24
                  local.get 4
                  i32.const 20
                  i32.add
                  local.get 4
                  i32.const 24
                  i32.add
                  call $_ZN4core9panicking13assert_failed17hfcfdb11ed0fde9ebE
                  unreachable
                end
                local.get 4
                local.get 9
                i32.store8 offset=28
                local.get 4
                local.get 10
                i32.store offset=24
                i32.const 1055836
                i32.const 43
                local.get 4
                i32.const 24
                i32.add
                i32.const 1055896
                i32.const 1057584
                call $_ZN4core6result13unwrap_failed17h1b005d733a1499a6E
                unreachable
              end
              local.get 4
              i32.const 44
              i32.add
              i32.const 0
              i32.store
              local.get 4
              i32.const 1053116
              i32.store offset=40
              local.get 4
              i64.const 1
              i64.store offset=28 align=4
              local.get 4
              i32.const 1057624
              i32.store offset=24
              local.get 4
              i32.const 24
              i32.add
              i32.const 1057632
              call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
              unreachable
            end
            local.get 4
            i32.const 44
            i32.add
            i32.const 0
            i32.store
            local.get 4
            i32.const 40
            i32.add
            i32.const 1053116
            i32.store
            local.get 4
            i64.const 1
            i64.store offset=28 align=4
            local.get 4
            i32.const 1057680
            i32.store offset=24
            local.get 4
            i32.const 20
            i32.add
            local.get 4
            i32.const 24
            i32.add
            i32.const 1057688
            call $_ZN4core9panicking13assert_failed17h24e62d98e3800fc3E
            unreachable
          end
          local.get 4
          i32.const 44
          i32.add
          i32.const 0
          i32.store
          local.get 4
          i32.const 1053116
          i32.store offset=40
          local.get 4
          i64.const 1
          i64.store offset=28 align=4
          local.get 4
          i32.const 1057096
          i32.store offset=24
          local.get 4
          i32.const 24
          i32.add
          i32.const 1057156
          call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
          unreachable
        end
        local.get 4
        i32.load offset=8
        local.tee 7
        i32.eqz
        br_if 0 (;@2;)
        local.get 7
        local.get 7
        i32.load
        local.tee 6
        i32.const -1
        i32.add
        i32.store
        local.get 6
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=8
        call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17ha7cc5cab043eefa7E
        local.get 0
        i32.load
        local.set 6
        br 1 (;@1;)
      end
      local.get 0
      i32.load
      local.set 6
      br 0 (;@1;)
    end
  )
  (func $_ZN3std2io5Write9write_fmt17h49c2f05193f87c52E (type 19) (param i32 i32) (result i64)
    (local i32 i64 i64)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 4
    i32.store8 offset=12
    local.get 2
    local.get 0
    i32.store offset=8
    local.get 2
    i32.const 24
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 24
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=24
    local.get 2
    i32.const 8
    i32.add
    i32.const 1054872
    local.get 2
    i32.const 24
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 0
    local.get 2
    i32.load8_u offset=12
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 1
          i32.const 255
          i32.and
          i32.const 4
          i32.ne
          br_if 0 (;@3;)
          i32.const 1054860
          i64.extend_i32_u
          i64.const 32
          i64.shl
          i64.const 8
          i64.shr_u
          local.set 3
          i64.const 2
          local.set 4
          br 2 (;@1;)
        end
        local.get 1
        i64.extend_i32_u
        i64.const 255
        i64.and
        local.set 4
        local.get 2
        i64.load32_u offset=13 align=1
        local.get 2
        i32.const 17
        i32.add
        i64.load16_u align=1
        local.get 2
        i32.const 19
        i32.add
        i64.load8_u
        i64.const 16
        i64.shl
        i64.or
        i64.const 32
        i64.shl
        i64.or
        local.set 3
        br 1 (;@1;)
      end
      i64.const 4
      local.set 4
      block  ;; label = @2
        local.get 1
        i32.const 255
        i32.and
        i32.const 3
        i32.eq
        br_if 0 (;@2;)
        i32.const 1054860
        i64.extend_i32_u
        i64.const 32
        i64.shl
        i64.const 8
        i64.shr_u
        local.set 3
        br 1 (;@1;)
      end
      local.get 2
      i32.const 16
      i32.add
      i32.load
      local.tee 1
      i32.load
      local.get 1
      i32.load offset=4
      i32.load
      call_indirect (type 0)
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        i32.load offset=4
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load
        call $free
      end
      local.get 2
      i32.load offset=16
      call $free
      i32.const 1054860
      i64.extend_i32_u
      i64.const 32
      i64.shl
      i64.const 8
      i64.shr_u
      local.set 3
    end
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 3
    i64.const 8
    i64.shl
    local.get 4
    i64.or
  )
  (func $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17hb77b520a01bfdb66E (type 0) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load8_u
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.const 4
      i32.add
      i32.load
      local.tee 1
      i32.load
      local.get 1
      i32.load offset=4
      i32.load
      call_indirect (type 0)
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        i32.load offset=4
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load
        call $free
      end
      local.get 0
      i32.load offset=4
      call $free
    end
  )
  (func $_ZN3std3sys4wasi14abort_internal17h6f2ef412fa287f15E (type 4)
    call $abort
    unreachable
  )
  (func $_ZN70_$LT$$RF$str$u20$as$u20$std..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17he72994ed41181ff3E (type 11) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 2
                        i32.const 1
                        i32.add
                        local.tee 4
                        local.get 2
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 4
                        i32.const -1
                        i32.le_s
                        br_if 1 (;@9;)
                        local.get 4
                        call $malloc
                        local.tee 5
                        i32.eqz
                        br_if 2 (;@8;)
                        local.get 5
                        local.get 1
                        local.get 2
                        call $memcpy
                        local.set 6
                        block  ;; label = @11
                          local.get 2
                          i32.const 8
                          i32.lt_u
                          br_if 0 (;@11;)
                          block  ;; label = @12
                            local.get 1
                            i32.const 3
                            i32.add
                            i32.const -4
                            i32.and
                            local.get 1
                            i32.sub
                            local.tee 5
                            br_if 0 (;@12;)
                            local.get 2
                            i32.const -8
                            i32.add
                            local.set 7
                            i32.const 0
                            local.set 5
                            br 6 (;@6;)
                          end
                          local.get 2
                          local.get 5
                          local.get 5
                          local.get 2
                          i32.gt_u
                          select
                          local.set 5
                          i32.const 0
                          local.set 8
                          loop  ;; label = @12
                            local.get 1
                            local.get 8
                            i32.add
                            i32.load8_u
                            i32.eqz
                            br_if 10 (;@2;)
                            local.get 5
                            local.get 8
                            i32.const 1
                            i32.add
                            local.tee 8
                            i32.eq
                            br_if 5 (;@7;)
                            br 0 (;@12;)
                          end
                        end
                        local.get 2
                        i32.eqz
                        br_if 6 (;@4;)
                        block  ;; label = @11
                          local.get 1
                          i32.load8_u
                          br_if 0 (;@11;)
                          i32.const 0
                          local.set 8
                          br 9 (;@2;)
                        end
                        i32.const 1
                        local.set 8
                        local.get 2
                        i32.const 1
                        i32.eq
                        br_if 6 (;@4;)
                        local.get 1
                        i32.load8_u offset=1
                        i32.eqz
                        br_if 8 (;@2;)
                        i32.const 2
                        local.set 8
                        local.get 2
                        i32.const 2
                        i32.eq
                        br_if 6 (;@4;)
                        local.get 1
                        i32.load8_u offset=2
                        i32.eqz
                        br_if 8 (;@2;)
                        i32.const 3
                        local.set 8
                        local.get 2
                        i32.const 3
                        i32.eq
                        br_if 6 (;@4;)
                        local.get 1
                        i32.load8_u offset=3
                        i32.eqz
                        br_if 8 (;@2;)
                        i32.const 4
                        local.set 8
                        local.get 2
                        i32.const 4
                        i32.eq
                        br_if 6 (;@4;)
                        local.get 1
                        i32.load8_u offset=4
                        i32.eqz
                        br_if 8 (;@2;)
                        i32.const 5
                        local.set 8
                        local.get 2
                        i32.const 5
                        i32.eq
                        br_if 6 (;@4;)
                        local.get 1
                        i32.load8_u offset=5
                        i32.eqz
                        br_if 8 (;@2;)
                        i32.const 6
                        local.set 8
                        local.get 2
                        i32.const 6
                        i32.eq
                        br_if 6 (;@4;)
                        local.get 1
                        i32.load8_u offset=6
                        i32.eqz
                        br_if 8 (;@2;)
                        br 6 (;@4;)
                      end
                      i32.const 1055427
                      i32.const 43
                      i32.const 1053524
                      call $_ZN4core9panicking5panic17h66d99d0b614b1c7cE
                      unreachable
                    end
                    call $_ZN5alloc7raw_vec17capacity_overflow17h0f57087aa53660d1E
                    unreachable
                  end
                  local.get 4
                  i32.const 1
                  call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
                  unreachable
                end
                local.get 5
                local.get 2
                i32.const -8
                i32.add
                local.tee 7
                i32.gt_u
                br_if 1 (;@5;)
              end
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 1
                  local.get 5
                  i32.add
                  local.tee 8
                  i32.load
                  local.tee 9
                  i32.const -1
                  i32.xor
                  local.get 9
                  i32.const -16843009
                  i32.add
                  i32.and
                  local.get 8
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 8
                  i32.const -1
                  i32.xor
                  local.get 8
                  i32.const -16843009
                  i32.add
                  i32.and
                  i32.or
                  i32.const -2139062144
                  i32.and
                  br_if 1 (;@6;)
                  local.get 5
                  i32.const 8
                  i32.add
                  local.tee 5
                  local.get 7
                  i32.le_u
                  br_if 0 (;@7;)
                end
              end
              local.get 5
              local.get 2
              i32.le_u
              br_if 0 (;@5;)
              local.get 5
              local.get 2
              i32.const 1049776
              call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
              unreachable
            end
            local.get 5
            local.get 2
            i32.eq
            br_if 0 (;@4;)
            local.get 5
            local.get 2
            i32.sub
            local.set 9
            local.get 1
            local.get 5
            i32.add
            local.set 1
            i32.const 0
            local.set 8
            loop  ;; label = @5
              local.get 1
              local.get 8
              i32.add
              i32.load8_u
              i32.eqz
              br_if 2 (;@3;)
              local.get 9
              local.get 8
              i32.const 1
              i32.add
              local.tee 8
              i32.add
              br_if 0 (;@5;)
            end
          end
          local.get 3
          local.get 2
          i32.store offset=24
          local.get 3
          local.get 4
          i32.store offset=20
          local.get 3
          local.get 6
          i32.store offset=16
          local.get 3
          i32.const 8
          i32.add
          local.get 3
          i32.const 16
          i32.add
          call $_ZN3std3ffi5c_str7CString19_from_vec_unchecked17h0219136b109b7902E
          local.get 0
          local.get 3
          i64.load offset=8
          i64.store offset=4 align=4
          i32.const 0
          local.set 5
          br 2 (;@1;)
        end
        local.get 5
        local.get 8
        i32.add
        local.set 8
      end
      local.get 0
      local.get 8
      i32.store offset=4
      local.get 0
      i32.const 16
      i32.add
      local.get 2
      i32.store
      local.get 0
      i32.const 12
      i32.add
      local.get 4
      i32.store
      local.get 0
      i32.const 8
      i32.add
      local.get 6
      i32.store
      i32.const 1
      local.set 5
    end
    local.get 0
    local.get 5
    i32.store
    local.get 3
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17h259a58d95d5c98aeE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load8_u
              br_table 0 (;@5;) 1 (;@4;) 2 (;@3;) 3 (;@2;) 0 (;@5;)
            end
            local.get 2
            local.get 0
            i32.const 4
            i32.add
            i32.load
            local.tee 0
            i32.store offset=4
            local.get 2
            i32.const 8
            i32.add
            local.get 0
            call $_ZN3std3sys4wasi2os12error_string17h4bf3bfd3b66b331fE
            local.get 2
            i32.const 36
            i32.add
            i32.const 13
            i32.store
            local.get 2
            i32.const 14
            i32.store offset=28
            local.get 1
            i32.const 28
            i32.add
            i32.load
            local.set 0
            local.get 2
            local.get 2
            i32.const 4
            i32.add
            i32.store offset=32
            local.get 2
            local.get 2
            i32.const 8
            i32.add
            i32.store offset=24
            local.get 1
            i32.load offset=24
            local.set 1
            local.get 2
            i32.const 60
            i32.add
            i32.const 2
            i32.store
            local.get 2
            i64.const 3
            i64.store offset=44 align=4
            local.get 2
            i32.const 1054620
            i32.store offset=40
            local.get 2
            local.get 2
            i32.const 24
            i32.add
            i32.store offset=56
            local.get 1
            local.get 0
            local.get 2
            i32.const 40
            i32.add
            call $_ZN4core3fmt5write17h8776c655b56f9e02E
            local.set 0
            local.get 2
            i32.load offset=12
            i32.eqz
            br_if 3 (;@1;)
            local.get 2
            i32.load offset=8
            local.tee 1
            i32.eqz
            br_if 3 (;@1;)
            local.get 1
            call $free
            br 3 (;@1;)
          end
          local.get 0
          i32.load8_s offset=1
          local.set 0
          local.get 2
          i32.const 4
          i32.store offset=12
          local.get 2
          local.get 0
          i32.const 2
          i32.shl
          local.tee 0
          i32.const 1057788
          i32.add
          i32.load
          i32.store offset=28
          local.get 2
          local.get 0
          i32.const 1057952
          i32.add
          i32.load
          i32.store offset=24
          local.get 1
          i32.const 28
          i32.add
          i32.load
          local.set 0
          local.get 2
          local.get 2
          i32.const 24
          i32.add
          i32.store offset=8
          local.get 1
          i32.load offset=24
          local.set 1
          local.get 2
          i32.const 60
          i32.add
          i32.const 1
          i32.store
          local.get 2
          i64.const 1
          i64.store offset=44 align=4
          local.get 2
          i32.const 1053444
          i32.store offset=40
          local.get 2
          local.get 2
          i32.const 8
          i32.add
          i32.store offset=56
          local.get 1
          local.get 0
          local.get 2
          i32.const 40
          i32.add
          call $_ZN4core3fmt5write17h8776c655b56f9e02E
          local.set 0
          br 2 (;@1;)
        end
        local.get 1
        local.get 0
        i32.const 4
        i32.add
        i32.load
        local.tee 0
        i32.load
        local.get 0
        i32.load offset=4
        call $_ZN4core3fmt9Formatter3pad17h0b95dbdf7f5bfcd3E
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      i32.const 4
      i32.add
      i32.load
      local.tee 0
      i32.load
      local.get 1
      local.get 0
      i32.load offset=4
      i32.load offset=16
      call_indirect (type 2)
      local.set 0
    end
    local.get 2
    i32.const 64
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN61_$LT$std..ffi..c_str..CString$u20$as$u20$core..fmt..Debug$GT$3fmt17h56771385d9418557E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 1
    i32.const 28
    i32.add
    i32.load
    local.set 3
    local.get 0
    i32.load offset=4
    local.set 4
    local.get 0
    i32.load
    local.set 0
    local.get 1
    i32.load offset=24
    local.set 5
    i32.const 0
    local.set 6
    local.get 2
    i32.const 28
    i32.add
    i32.const 0
    i32.store
    local.get 2
    i32.const 1053116
    i32.store offset=24
    local.get 2
    i64.const 1
    i64.store offset=12 align=4
    local.get 2
    i32.const 1053540
    i32.store offset=8
    i32.const 1
    local.set 1
    block  ;; label = @1
      local.get 5
      local.get 3
      local.get 2
      i32.const 8
      i32.add
      call $_ZN4core3fmt5write17h8776c655b56f9e02E
      br_if 0 (;@1;)
      local.get 2
      i32.const 0
      i32.store8 offset=23
      local.get 2
      i32.const 0
      i32.store8 offset=16
      local.get 2
      local.get 4
      local.get 0
      i32.add
      i32.const -1
      i32.add
      local.tee 7
      i32.store offset=12
      local.get 2
      i32.const 17
      i32.add
      local.set 8
      local.get 2
      i32.const 21
      i32.add
      local.set 9
      loop  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 6
              i32.const 255
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 10
              i32.const 255
              i32.and
              local.tee 1
              local.get 4
              i32.const 255
              i32.and
              i32.ge_u
              br_if 0 (;@5;)
              local.get 2
              local.get 10
              i32.const 1
              i32.add
              local.tee 10
              i32.store8 offset=17
              local.get 1
              i32.const 4
              i32.ge_u
              br_if 1 (;@4;)
              br 2 (;@3;)
            end
            block  ;; label = @5
              local.get 0
              i32.eqz
              br_if 0 (;@5;)
              local.get 7
              local.get 0
              i32.eq
              br_if 0 (;@5;)
              local.get 2
              local.get 0
              i32.const 1
              i32.add
              local.tee 11
              i32.store offset=8
              i32.const 2
              local.set 4
              i32.const 29788
              local.set 1
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 0
                                i32.load8_u
                                local.tee 0
                                i32.const -9
                                i32.add
                                br_table 8 (;@6;) 3 (;@11;) 1 (;@13;) 1 (;@13;) 2 (;@12;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 6 (;@8;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 1 (;@13;) 5 (;@9;) 0 (;@14;)
                              end
                              local.get 0
                              i32.const 92
                              i32.eq
                              br_if 3 (;@10;)
                            end
                            local.get 0
                            i32.const -32
                            i32.add
                            i32.const 255
                            i32.and
                            i32.const 95
                            i32.lt_u
                            br_if 5 (;@7;)
                            i32.const 4
                            local.set 4
                            i32.const 48
                            i32.const 87
                            local.get 0
                            i32.const 15
                            i32.and
                            local.tee 1
                            i32.const 10
                            i32.lt_u
                            select
                            local.get 1
                            i32.add
                            i32.const 24
                            i32.shl
                            i32.const 48
                            i32.const 87
                            local.get 0
                            i32.const 160
                            i32.lt_u
                            select
                            local.get 0
                            i32.const 4
                            i32.shr_u
                            i32.add
                            i32.const 16
                            i32.shl
                            i32.or
                            i32.const 30812
                            i32.or
                            local.set 1
                            br 6 (;@6;)
                          end
                          i32.const 29276
                          local.set 1
                          br 5 (;@6;)
                        end
                        i32.const 28252
                        local.set 1
                        br 4 (;@6;)
                      end
                      i32.const 23644
                      local.set 1
                      br 3 (;@6;)
                    end
                    i32.const 10076
                    local.set 1
                    br 2 (;@6;)
                  end
                  i32.const 8796
                  local.set 1
                  br 1 (;@6;)
                end
                i32.const 1
                local.set 4
                local.get 0
                local.set 1
              end
              local.get 9
              local.get 1
              i32.const 16
              i32.shr_u
              i64.extend_i32_u
              i64.store16 align=1
              local.get 2
              local.get 4
              i32.const 8
              i32.shl
              local.get 1
              i32.const 16
              i32.shl
              i32.or
              i32.store offset=17 align=1
              i32.const 1
              local.set 10
              local.get 2
              i32.const 1
              i32.store8 offset=17
              i32.const 0
              local.set 1
              i32.const 1
              local.set 6
              local.get 11
              local.set 0
              br 2 (;@3;)
            end
            local.get 2
            i32.const 28
            i32.add
            i32.const 0
            i32.store
            local.get 2
            i32.const 1053116
            i32.store offset=24
            local.get 2
            i64.const 1
            i64.store offset=12 align=4
            local.get 2
            i32.const 1053540
            i32.store offset=8
            local.get 5
            local.get 3
            local.get 2
            i32.const 8
            i32.add
            call $_ZN4core3fmt5write17h8776c655b56f9e02E
            local.set 1
            br 3 (;@1;)
          end
          local.get 1
          i32.const 4
          i32.const 1048936
          call $_ZN4core9panicking18panic_bounds_check17h8c564c2b20bece92E
          unreachable
        end
        local.get 5
        local.get 8
        local.get 1
        i32.add
        i32.const 2
        i32.add
        i32.load8_u
        local.get 3
        i32.load offset=16
        call_indirect (type 2)
        i32.eqz
        br_if 0 (;@2;)
      end
      i32.const 1
      local.set 1
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN4core9panicking13assert_failed17ha4700b3711575da3E (type 5) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 1054952
    i32.store offset=4
    local.get 2
    local.get 0
    i32.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    i32.const 1
    local.get 2
    i32.const 1055664
    local.get 2
    i32.const 4
    i32.add
    i32.const 1055664
    local.get 2
    i32.const 8
    i32.add
    i32.const 1054988
    call $_ZN4core9panicking19assert_failed_inner17h8160b73ca348502bE
    unreachable
  )
  (func $_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h212910dea3e9a103E (type 0) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load8_u
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.const 4
      i32.add
      i32.load
      local.tee 1
      i32.load
      local.get 1
      i32.load offset=4
      i32.load
      call_indirect (type 0)
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        i32.load offset=4
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load
        call $free
      end
      local.get 0
      i32.load offset=4
      call $free
    end
  )
  (func $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hef47d23a38b516b0E (type 1) (param i32 i32 i32) (result i32)
    (local i64 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.get 1
      local.get 2
      call $_ZN3std2io5Write9write_all17h80d03cd6417d9f63E
      local.tee 3
      i32.wrap_i64
      local.tee 2
      i32.const 255
      i32.and
      local.tee 1
      i32.const 4
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      i64.const 8
      i64.shr_u
      local.set 3
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=4
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.const 8
        i32.add
        i32.load
        local.tee 4
        i32.load
        local.get 4
        i32.load offset=4
        i32.load
        call_indirect (type 0)
        block  ;; label = @3
          local.get 4
          i32.load offset=4
          i32.load offset=4
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.load
          call $free
        end
        local.get 4
        call $free
      end
      local.get 0
      local.get 2
      i32.store8 offset=4
      local.get 0
      i32.const 11
      i32.add
      local.get 3
      i64.const 48
      i64.shr_u
      i64.store8
      local.get 0
      i32.const 9
      i32.add
      local.get 3
      i64.const 32
      i64.shr_u
      i64.store16 align=1
      local.get 0
      i32.const 5
      i32.add
      local.get 3
      i64.store32 align=1
    end
    local.get 1
    i32.const 4
    i32.ne
  )
  (func $_ZN4core3fmt5Write10write_char17hf3707acfb2f0507dE (type 2) (param i32 i32) (result i32)
    (local i32 i64 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 2 (;@2;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.store8 offset=12
          i32.const 1
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=15
      local.get 2
      local.get 1
      i32.const 18
      i32.shr_u
      i32.const 240
      i32.or
      i32.store8 offset=12
      local.get 2
      local.get 1
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=14
      local.get 2
      local.get 1
      i32.const 12
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=13
      i32.const 4
      local.set 1
    end
    block  ;; label = @1
      local.get 0
      i32.load
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $_ZN3std2io5Write9write_all17h80d03cd6417d9f63E
      local.tee 3
      i32.wrap_i64
      local.tee 4
      i32.const 255
      i32.and
      local.tee 1
      i32.const 4
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      i64.const 8
      i64.shr_u
      local.set 3
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=4
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.const 8
        i32.add
        i32.load
        local.tee 5
        i32.load
        local.get 5
        i32.load offset=4
        i32.load
        call_indirect (type 0)
        block  ;; label = @3
          local.get 5
          i32.load offset=4
          i32.load offset=4
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          i32.load
          call $free
        end
        local.get 5
        call $free
      end
      local.get 0
      local.get 4
      i32.store8 offset=4
      local.get 0
      i32.const 11
      i32.add
      local.get 3
      i64.const 48
      i64.shr_u
      i64.store8
      local.get 0
      i32.const 9
      i32.add
      local.get 3
      i64.const 32
      i64.shr_u
      i64.store16 align=1
      local.get 0
      i32.const 5
      i32.add
      local.get 3
      i64.store32 align=1
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
    i32.const 4
    i32.ne
  )
  (func $_ZN4core3fmt5Write9write_fmt17h32c0c646c526e6d7E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1053092
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h75238bb108307ba7E (type 1) (param i32 i32 i32) (result i32)
    (local i64 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 0
      i32.load
      local.get 1
      local.get 2
      call $_ZN3std2io5Write9write_all17h80d03cd6417d9f63E
      local.tee 3
      i32.wrap_i64
      local.tee 2
      i32.const 255
      i32.and
      local.tee 1
      i32.const 4
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      i64.const 8
      i64.shr_u
      local.set 3
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=4
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.const 8
        i32.add
        i32.load
        local.tee 4
        i32.load
        local.get 4
        i32.load offset=4
        i32.load
        call_indirect (type 0)
        block  ;; label = @3
          local.get 4
          i32.load offset=4
          i32.load offset=4
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.load
          call $free
        end
        local.get 4
        call $free
      end
      local.get 0
      local.get 2
      i32.store8 offset=4
      local.get 0
      i32.const 11
      i32.add
      local.get 3
      i64.const 48
      i64.shr_u
      i64.store8
      local.get 0
      i32.const 9
      i32.add
      local.get 3
      i64.const 32
      i64.shr_u
      i64.store16 align=1
      local.get 0
      i32.const 5
      i32.add
      local.get 3
      i64.store32 align=1
    end
    local.get 1
    i32.const 4
    i32.ne
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17ha5f9b44bf800e420E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN4core3fmt5Write10write_char17hf3707acfb2f0507dE
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h109e2202b03b6abdE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1053092
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN4core9panicking13assert_failed17hfcfdb11ed0fde9ebE (type 5) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 1053548
    i32.store offset=4
    local.get 2
    local.get 0
    i32.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    i32.const 0
    local.get 2
    i32.const 1055680
    local.get 2
    i32.const 4
    i32.add
    i32.const 1055680
    local.get 2
    i32.const 8
    i32.add
    i32.const 1057388
    call $_ZN4core9panicking19assert_failed_inner17h8160b73ca348502bE
    unreachable
  )
  (func $_ZN3std9panicking11panic_count17is_zero_slow_path17h4ea576c666392881E (type 17) (result i32)
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=1059920
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      i32.load offset=1059928
      i32.eqz
      return
    end
    i32.const 0
    i32.const 1
    i32.store8 offset=1059920
    i32.const 0
    i32.const 0
    i32.store offset=1059928
    i32.const 1
  )
  (func $_ZN4core3ptr103drop_in_place$LT$std..sync..poison..PoisonError$LT$std..sync..mutex..MutexGuard$LT$$LP$$RP$$GT$$GT$$GT$17hb4a0226b66dabfd4E (type 0) (param i32)
    (local i32)
    local.get 0
    i32.load
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=4
      br_if 0 (;@1;)
      i32.const 0
      i32.load offset=1059908
      i32.const 2147483647
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      call $_ZN3std9panicking11panic_count17is_zero_slow_path17h4ea576c666392881E
      br_if 0 (;@1;)
      local.get 1
      i32.const 1
      i32.store8 offset=1
    end
    local.get 1
    i32.const 0
    i32.store8
  )
  (func $_ZN76_$LT$std..sync..poison..PoisonError$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd198f92e4a161b03E (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    i32.const 1
    local.set 2
    block  ;; label = @1
      local.get 1
      i32.load offset=24
      i32.const 1055416
      i32.const 11
      local.get 1
      i32.const 28
      i32.add
      local.tee 3
      i32.load
      i32.load offset=12
      call_indirect (type 1)
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=24
      i32.const 1049375
      i32.const 7
      local.get 3
      i32.load
      i32.load offset=12
      call_indirect (type 1)
      local.set 2
    end
    local.get 2
  )
  (func $_ZN4core3ptr100drop_in_place$LT$$RF$mut$u20$std..io..Write..write_fmt..Adapter$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$17h6618fd488f4ae4e0E (type 0) (param i32))
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he06d51f3d3b8d64eE (type 2) (param i32 i32) (result i32)
    block  ;; label = @1
      local.get 0
      i32.load
      i32.load8_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 1049707
      i32.const 5
      call $_ZN4core3fmt9Formatter3pad17h0b95dbdf7f5bfcd3E
      return
    end
    local.get 1
    i32.const 1049703
    i32.const 4
    call $_ZN4core3fmt9Formatter3pad17h0b95dbdf7f5bfcd3E
  )
  (func $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17ha7cc5cab043eefa7E (type 0) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=16
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 0
      i32.store8
      local.get 0
      i32.const 20
      i32.add
      i32.load
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=16
      call $free
    end
    block  ;; label = @1
      local.get 0
      i32.const -1
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 0
      i32.load offset=4
      local.tee 1
      i32.const -1
      i32.add
      i32.store offset=4
      local.get 1
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      call $free
    end
  )
  (func $_ZN4core9panicking13assert_failed17h24e62d98e3800fc3E (type 11) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 1055380
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 3
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 3
    local.get 1
    i64.load align=4
    i64.store offset=8
    i32.const 0
    local.get 3
    i32.const 1055648
    local.get 3
    i32.const 4
    i32.add
    i32.const 1055648
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking19assert_failed_inner17h8160b73ca348502bE
    unreachable
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h65566335057149d7E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load
              local.tee 3
              i32.const 16
              i32.and
              br_if 0 (;@5;)
              local.get 3
              i32.const 32
              i32.and
              br_if 1 (;@4;)
              local.get 0
              i64.load32_u
              i32.const 1
              local.get 1
              call $_ZN4core3fmt3num3imp7fmt_u6417hcb4d540e45bed3d8E
              local.set 0
              br 2 (;@3;)
            end
            local.get 0
            i32.load
            local.set 0
            i32.const 0
            local.set 3
            loop  ;; label = @5
              local.get 2
              local.get 3
              i32.add
              i32.const 127
              i32.add
              i32.const 48
              i32.const 87
              local.get 0
              i32.const 15
              i32.and
              local.tee 4
              i32.const 10
              i32.lt_u
              select
              local.get 4
              i32.add
              i32.store8
              local.get 3
              i32.const -1
              i32.add
              local.set 3
              local.get 0
              i32.const 15
              i32.gt_u
              local.set 4
              local.get 0
              i32.const 4
              i32.shr_u
              local.set 0
              local.get 4
              br_if 0 (;@5;)
            end
            local.get 3
            i32.const 128
            i32.add
            local.tee 0
            i32.const 129
            i32.ge_u
            br_if 2 (;@2;)
            local.get 1
            i32.const 1
            i32.const 1049448
            i32.const 2
            local.get 2
            local.get 3
            i32.add
            i32.const 128
            i32.add
            i32.const 0
            local.get 3
            i32.sub
            call $_ZN4core3fmt9Formatter12pad_integral17h88d3d7a17289fb2aE
            local.set 0
            br 1 (;@3;)
          end
          local.get 0
          i32.load
          local.set 0
          i32.const 0
          local.set 3
          loop  ;; label = @4
            local.get 2
            local.get 3
            i32.add
            i32.const 127
            i32.add
            i32.const 48
            i32.const 55
            local.get 0
            i32.const 15
            i32.and
            local.tee 4
            i32.const 10
            i32.lt_u
            select
            local.get 4
            i32.add
            i32.store8
            local.get 3
            i32.const -1
            i32.add
            local.set 3
            local.get 0
            i32.const 15
            i32.gt_u
            local.set 4
            local.get 0
            i32.const 4
            i32.shr_u
            local.set 0
            local.get 4
            br_if 0 (;@4;)
          end
          local.get 3
          i32.const 128
          i32.add
          local.tee 0
          i32.const 129
          i32.ge_u
          br_if 2 (;@1;)
          local.get 1
          i32.const 1
          i32.const 1049448
          i32.const 2
          local.get 2
          local.get 3
          i32.add
          i32.const 128
          i32.add
          i32.const 0
          local.get 3
          i32.sub
          call $_ZN4core3fmt9Formatter12pad_integral17h88d3d7a17289fb2aE
          local.set 0
        end
        local.get 2
        i32.const 128
        i32.add
        global.set $__stack_pointer
        local.get 0
        return
      end
      local.get 0
      i32.const 128
      i32.const 1049432
      call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
      unreachable
    end
    local.get 0
    i32.const 128
    i32.const 1049432
    call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
    unreachable
  )
  (func $_ZN4core3ptr226drop_in_place$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$GT$17h8db58a1894934150E (type 0) (param i32)
    block  ;; label = @1
      local.get 0
      i32.const 4
      i32.add
      i32.load
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      call $free
    end
  )
  (func $_ZN3std3sys4wasi2os12error_string17h4bf3bfd3b66b331fE (type 5) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 1056
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 0
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          local.get 2
          i32.const 0
          i32.const 1024
          call $memset
          local.tee 4
          i32.const 1024
          call $strerror_r
          i32.const 0
          i32.lt_s
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 4
            i32.load8_u
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            i32.const 1
            i32.add
            local.set 5
            i32.const 0
            local.set 1
            loop  ;; label = @5
              local.get 5
              local.get 1
              i32.add
              local.set 2
              local.get 1
              i32.const 1
              i32.add
              local.tee 3
              local.set 1
              local.get 2
              i32.load8_u
              br_if 0 (;@5;)
            end
          end
          local.get 4
          i32.const 1024
          i32.add
          local.get 4
          local.get 3
          call $_ZN4core3str8converts9from_utf817h5e14e3bada571d6dE
          local.get 4
          i32.load offset=1024
          br_if 1 (;@2;)
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 4
                i32.const 1032
                i32.add
                i32.load
                local.tee 1
                i32.const 0
                i32.lt_s
                br_if 0 (;@6;)
                local.get 4
                i32.load offset=1028
                local.set 2
                local.get 1
                br_if 1 (;@5;)
                i32.const 1
                local.set 3
                br 2 (;@4;)
              end
              call $_ZN5alloc7raw_vec17capacity_overflow17h0f57087aa53660d1E
              unreachable
            end
            local.get 1
            call $malloc
            local.tee 3
            i32.eqz
            br_if 3 (;@1;)
          end
          local.get 3
          local.get 2
          local.get 1
          call $memcpy
          local.set 2
          local.get 0
          local.get 1
          i32.store offset=8
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 0
          local.get 2
          i32.store
          local.get 4
          i32.const 1056
          i32.add
          global.set $__stack_pointer
          return
        end
        local.get 4
        i32.const 1044
        i32.add
        i32.const 0
        i32.store
        local.get 4
        i32.const 1053116
        i32.store offset=1040
        local.get 4
        i64.const 1
        i64.store offset=1028 align=4
        local.get 4
        i32.const 1057424
        i32.store offset=1024
        local.get 4
        i32.const 1024
        i32.add
        i32.const 1057464
        call $_ZN4core9panicking9panic_fmt17h51698eb89a3b5869E
        unreachable
      end
      local.get 4
      local.get 4
      i64.load offset=1028 align=4
      i64.store offset=1048
      i32.const 1055836
      i32.const 43
      local.get 4
      i32.const 1048
      i32.add
      i32.const 1055880
      i32.const 1057480
      call $_ZN4core6result13unwrap_failed17h1b005d733a1499a6E
      unreachable
    end
    local.get 1
    i32.const 1
    call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
    unreachable
  )
  (func $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h775593b8d3a4c26dE (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.const 8
    i32.add
    i32.load
    call $_ZN4core3fmt9Formatter3pad17h0b95dbdf7f5bfcd3E
  )
  (func $_ZN243_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$u20$as$u20$std..error..Error$GT$11description17h16c78575d1f51985E (type 5) (param i32 i32)
    local.get 0
    local.get 1
    i32.const 8
    i32.add
    i32.load
    i32.store offset=4
    local.get 0
    local.get 1
    i32.load
    i32.store
  )
  (func $_ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17h111faae4cf6dfba4E (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.const 8
    i32.add
    i32.load
    call $_ZN4core3fmt9Formatter3pad17h0b95dbdf7f5bfcd3E
  )
  (func $_ZN242_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Debug$GT$3fmt17h9a83e5beffb7b670E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.const 8
    i32.add
    i32.load
    local.get 1
    call $_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17hecac681ab20f31d6E
  )
  (func $_ZN3std5error5Error5cause17h4a924e3a0ab474d1E (type 5) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store
  )
  (func $_ZN3std5error5Error7type_id17h732f71d4b4fa15d2E (type 3) (param i32) (result i64)
    i64.const 2244133503078196034
  )
  (func $_ZN3std5error5Error9backtrace17h46b4ece30348119aE (type 8) (param i32) (result i32)
    i32.const 0
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h3912ee47733b7ba0E (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 3
      i32.const 4
      i32.add
      i32.load
      local.get 3
      i32.const 8
      i32.add
      local.tee 4
      i32.load
      local.tee 0
      i32.sub
      local.get 2
      i32.ge_u
      br_if 0 (;@1;)
      local.get 3
      local.get 0
      local.get 2
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h9e9d16cf4f4f67caE
      local.get 4
      i32.load
      local.set 0
    end
    local.get 3
    i32.load
    local.get 0
    i32.add
    local.get 1
    local.get 2
    call $memcpy
    drop
    local.get 4
    local.get 0
    local.get 2
    i32.add
    i32.store
    i32.const 0
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hccd4d1b29a695c46E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.const 128
              i32.lt_u
              br_if 0 (;@5;)
              local.get 2
              i32.const 0
              i32.store offset=12
              local.get 1
              i32.const 2048
              i32.lt_u
              br_if 1 (;@4;)
              local.get 1
              i32.const 65536
              i32.ge_u
              br_if 2 (;@3;)
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              local.set 1
              br 3 (;@2;)
            end
            block  ;; label = @5
              local.get 0
              i32.load offset=8
              local.tee 3
              local.get 0
              i32.const 4
              i32.add
              i32.load
              i32.ne
              br_if 0 (;@5;)
              local.get 0
              local.get 3
              call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h2033c1963b518374E
              local.get 0
              i32.load offset=8
              local.set 3
            end
            local.get 0
            local.get 3
            i32.const 1
            i32.add
            i32.store offset=8
            local.get 0
            i32.load
            local.get 3
            i32.add
            local.get 1
            i32.store8
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          local.set 1
          br 1 (;@2;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=15
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const 240
        i32.or
        i32.store8 offset=12
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=14
        local.get 2
        local.get 1
        i32.const 12
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        i32.const 4
        local.set 1
      end
      block  ;; label = @2
        local.get 0
        i32.const 4
        i32.add
        i32.load
        local.get 0
        i32.const 8
        i32.add
        local.tee 4
        i32.load
        local.tee 3
        i32.sub
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        local.get 1
        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h9e9d16cf4f4f67caE
        local.get 4
        i32.load
        local.set 3
      end
      local.get 0
      i32.load
      local.get 3
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $memcpy
      drop
      local.get 4
      local.get 3
      local.get 1
      i32.add
      i32.store
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    i32.const 0
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h51b055fb88cf3b7bE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1056452
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h6c7ecce5f4b16f20E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load
              local.tee 3
              i32.const 16
              i32.and
              br_if 0 (;@5;)
              local.get 3
              i32.const 32
              i32.and
              br_if 1 (;@4;)
              local.get 0
              i32.load
              local.tee 0
              i64.extend_i32_u
              local.get 0
              i32.const -1
              i32.xor
              i64.extend_i32_s
              i64.const 1
              i64.add
              local.get 0
              i32.const -1
              i32.gt_s
              local.tee 0
              select
              local.get 0
              local.get 1
              call $_ZN4core3fmt3num3imp7fmt_u6417hcb4d540e45bed3d8E
              local.set 0
              br 2 (;@3;)
            end
            local.get 0
            i32.load
            local.set 0
            i32.const 0
            local.set 3
            loop  ;; label = @5
              local.get 2
              local.get 3
              i32.add
              i32.const 127
              i32.add
              i32.const 48
              i32.const 87
              local.get 0
              i32.const 15
              i32.and
              local.tee 4
              i32.const 10
              i32.lt_u
              select
              local.get 4
              i32.add
              i32.store8
              local.get 3
              i32.const -1
              i32.add
              local.set 3
              local.get 0
              i32.const 15
              i32.gt_u
              local.set 4
              local.get 0
              i32.const 4
              i32.shr_u
              local.set 0
              local.get 4
              br_if 0 (;@5;)
            end
            local.get 3
            i32.const 128
            i32.add
            local.tee 0
            i32.const 129
            i32.ge_u
            br_if 2 (;@2;)
            local.get 1
            i32.const 1
            i32.const 1049448
            i32.const 2
            local.get 2
            local.get 3
            i32.add
            i32.const 128
            i32.add
            i32.const 0
            local.get 3
            i32.sub
            call $_ZN4core3fmt9Formatter12pad_integral17h88d3d7a17289fb2aE
            local.set 0
            br 1 (;@3;)
          end
          local.get 0
          i32.load
          local.set 0
          i32.const 0
          local.set 3
          loop  ;; label = @4
            local.get 2
            local.get 3
            i32.add
            i32.const 127
            i32.add
            i32.const 48
            i32.const 55
            local.get 0
            i32.const 15
            i32.and
            local.tee 4
            i32.const 10
            i32.lt_u
            select
            local.get 4
            i32.add
            i32.store8
            local.get 3
            i32.const -1
            i32.add
            local.set 3
            local.get 0
            i32.const 15
            i32.gt_u
            local.set 4
            local.get 0
            i32.const 4
            i32.shr_u
            local.set 0
            local.get 4
            br_if 0 (;@4;)
          end
          local.get 3
          i32.const 128
          i32.add
          local.tee 0
          i32.const 129
          i32.ge_u
          br_if 2 (;@1;)
          local.get 1
          i32.const 1
          i32.const 1049448
          i32.const 2
          local.get 2
          local.get 3
          i32.add
          i32.const 128
          i32.add
          i32.const 0
          local.get 3
          i32.sub
          call $_ZN4core3fmt9Formatter12pad_integral17h88d3d7a17289fb2aE
          local.set 0
        end
        local.get 2
        i32.const 128
        i32.add
        global.set $__stack_pointer
        local.get 0
        return
      end
      local.get 0
      i32.const 128
      i32.const 1049432
      call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
      unreachable
    end
    local.get 0
    i32.const 128
    i32.const 1049432
    call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
    unreachable
  )
  (func $_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17h7c8b9f25d8b78d37E.100 (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load
              local.tee 3
              i32.const 16
              i32.and
              br_if 0 (;@5;)
              local.get 3
              i32.const 32
              i32.and
              br_if 1 (;@4;)
              local.get 0
              i32.load
              local.tee 0
              i64.extend_i32_u
              local.get 0
              i32.const -1
              i32.xor
              i64.extend_i32_s
              i64.const 1
              i64.add
              local.get 0
              i32.const -1
              i32.gt_s
              local.tee 0
              select
              local.get 0
              local.get 1
              call $_ZN4core3fmt3num3imp7fmt_u6417hcb4d540e45bed3d8E
              local.set 0
              br 4 (;@1;)
            end
            local.get 0
            i32.load
            local.set 0
            i32.const 0
            local.set 3
            loop  ;; label = @5
              local.get 2
              local.get 3
              i32.add
              i32.const 127
              i32.add
              i32.const 48
              i32.const 87
              local.get 0
              i32.const 15
              i32.and
              local.tee 4
              i32.const 10
              i32.lt_u
              select
              local.get 4
              i32.add
              i32.store8
              local.get 3
              i32.const -1
              i32.add
              local.set 3
              local.get 0
              i32.const 15
              i32.gt_u
              local.set 4
              local.get 0
              i32.const 4
              i32.shr_u
              local.set 0
              local.get 4
              br_if 0 (;@5;)
            end
            local.get 3
            i32.const 128
            i32.add
            local.tee 0
            i32.const 129
            i32.ge_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 1
            i32.const 1049448
            i32.const 2
            local.get 2
            local.get 3
            i32.add
            i32.const 128
            i32.add
            i32.const 0
            local.get 3
            i32.sub
            call $_ZN4core3fmt9Formatter12pad_integral17h88d3d7a17289fb2aE
            local.set 0
            br 3 (;@1;)
          end
          local.get 0
          i32.load
          local.set 0
          i32.const 0
          local.set 3
          loop  ;; label = @4
            local.get 2
            local.get 3
            i32.add
            i32.const 127
            i32.add
            i32.const 48
            i32.const 55
            local.get 0
            i32.const 15
            i32.and
            local.tee 4
            i32.const 10
            i32.lt_u
            select
            local.get 4
            i32.add
            i32.store8
            local.get 3
            i32.const -1
            i32.add
            local.set 3
            local.get 0
            i32.const 15
            i32.gt_u
            local.set 4
            local.get 0
            i32.const 4
            i32.shr_u
            local.set 0
            local.get 4
            br_if 0 (;@4;)
          end
          local.get 3
          i32.const 128
          i32.add
          local.tee 0
          i32.const 129
          i32.ge_u
          br_if 1 (;@2;)
          local.get 1
          i32.const 1
          i32.const 1049448
          i32.const 2
          local.get 2
          local.get 3
          i32.add
          i32.const 128
          i32.add
          i32.const 0
          local.get 3
          i32.sub
          call $_ZN4core3fmt9Formatter12pad_integral17h88d3d7a17289fb2aE
          local.set 0
          br 2 (;@1;)
        end
        local.get 0
        i32.const 128
        i32.const 1049432
        call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
        unreachable
      end
      local.get 0
      i32.const 128
      i32.const 1049432
      call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
      unreachable
    end
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h5166ee0e21e95d4aE (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.tee 0
    i32.load
    local.get 1
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 2)
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h0af0cbc9c0cb4d34E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN62_$LT$std..io..error..ErrorKind$u20$as$u20$core..fmt..Debug$GT$3fmt17h47fccc56bfd5655dE
  )
  (func $_ZN62_$LT$std..io..error..ErrorKind$u20$as$u20$core..fmt..Debug$GT$3fmt17h47fccc56bfd5655dE (type 2) (param i32 i32) (result i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          block  ;; label = @20
                                            block  ;; label = @21
                                              block  ;; label = @22
                                                block  ;; label = @23
                                                  block  ;; label = @24
                                                    block  ;; label = @25
                                                      block  ;; label = @26
                                                        block  ;; label = @27
                                                          block  ;; label = @28
                                                            block  ;; label = @29
                                                              block  ;; label = @30
                                                                block  ;; label = @31
                                                                  block  ;; label = @32
                                                                    block  ;; label = @33
                                                                      block  ;; label = @34
                                                                        block  ;; label = @35
                                                                          block  ;; label = @36
                                                                            block  ;; label = @37
                                                                              block  ;; label = @38
                                                                                block  ;; label = @39
                                                                                  block  ;; label = @40
                                                                                    block  ;; label = @41
                                                                                      local.get 0
                                                                                      i32.load8_u
                                                                                      br_table 0 (;@41;) 1 (;@40;) 2 (;@39;) 3 (;@38;) 4 (;@37;) 5 (;@36;) 6 (;@35;) 7 (;@34;) 8 (;@33;) 9 (;@32;) 10 (;@31;) 11 (;@30;) 12 (;@29;) 13 (;@28;) 14 (;@27;) 15 (;@26;) 16 (;@25;) 17 (;@24;) 18 (;@23;) 19 (;@22;) 20 (;@21;) 21 (;@20;) 22 (;@19;) 23 (;@18;) 24 (;@17;) 25 (;@16;) 26 (;@15;) 27 (;@14;) 28 (;@13;) 29 (;@12;) 30 (;@11;) 31 (;@10;) 32 (;@9;) 33 (;@8;) 34 (;@7;) 35 (;@6;) 36 (;@5;) 37 (;@4;) 38 (;@3;) 39 (;@2;) 40 (;@1;) 0 (;@41;)
                                                                                    end
                                                                                    local.get 1
                                                                                    i32.load offset=24
                                                                                    i32.const 1057061
                                                                                    i32.const 8
                                                                                    local.get 1
                                                                                    i32.const 28
                                                                                    i32.add
                                                                                    i32.load
                                                                                    i32.load offset=12
                                                                                    call_indirect (type 1)
                                                                                    return
                                                                                  end
                                                                                  local.get 1
                                                                                  i32.load offset=24
                                                                                  i32.const 1057045
                                                                                  i32.const 16
                                                                                  local.get 1
                                                                                  i32.const 28
                                                                                  i32.add
                                                                                  i32.load
                                                                                  i32.load offset=12
                                                                                  call_indirect (type 1)
                                                                                  return
                                                                                end
                                                                                local.get 1
                                                                                i32.load offset=24
                                                                                i32.const 1057028
                                                                                i32.const 17
                                                                                local.get 1
                                                                                i32.const 28
                                                                                i32.add
                                                                                i32.load
                                                                                i32.load offset=12
                                                                                call_indirect (type 1)
                                                                                return
                                                                              end
                                                                              local.get 1
                                                                              i32.load offset=24
                                                                              i32.const 1057013
                                                                              i32.const 15
                                                                              local.get 1
                                                                              i32.const 28
                                                                              i32.add
                                                                              i32.load
                                                                              i32.load offset=12
                                                                              call_indirect (type 1)
                                                                              return
                                                                            end
                                                                            local.get 1
                                                                            i32.load offset=24
                                                                            i32.const 1056998
                                                                            i32.const 15
                                                                            local.get 1
                                                                            i32.const 28
                                                                            i32.add
                                                                            i32.load
                                                                            i32.load offset=12
                                                                            call_indirect (type 1)
                                                                            return
                                                                          end
                                                                          local.get 1
                                                                          i32.load offset=24
                                                                          i32.const 1056980
                                                                          i32.const 18
                                                                          local.get 1
                                                                          i32.const 28
                                                                          i32.add
                                                                          i32.load
                                                                          i32.load offset=12
                                                                          call_indirect (type 1)
                                                                          return
                                                                        end
                                                                        local.get 1
                                                                        i32.load offset=24
                                                                        i32.const 1056963
                                                                        i32.const 17
                                                                        local.get 1
                                                                        i32.const 28
                                                                        i32.add
                                                                        i32.load
                                                                        i32.load offset=12
                                                                        call_indirect (type 1)
                                                                        return
                                                                      end
                                                                      local.get 1
                                                                      i32.load offset=24
                                                                      i32.const 1056951
                                                                      i32.const 12
                                                                      local.get 1
                                                                      i32.const 28
                                                                      i32.add
                                                                      i32.load
                                                                      i32.load offset=12
                                                                      call_indirect (type 1)
                                                                      return
                                                                    end
                                                                    local.get 1
                                                                    i32.load offset=24
                                                                    i32.const 1056942
                                                                    i32.const 9
                                                                    local.get 1
                                                                    i32.const 28
                                                                    i32.add
                                                                    i32.load
                                                                    i32.load offset=12
                                                                    call_indirect (type 1)
                                                                    return
                                                                  end
                                                                  local.get 1
                                                                  i32.load offset=24
                                                                  i32.const 1056926
                                                                  i32.const 16
                                                                  local.get 1
                                                                  i32.const 28
                                                                  i32.add
                                                                  i32.load
                                                                  i32.load offset=12
                                                                  call_indirect (type 1)
                                                                  return
                                                                end
                                                                local.get 1
                                                                i32.load offset=24
                                                                i32.const 1056915
                                                                i32.const 11
                                                                local.get 1
                                                                i32.const 28
                                                                i32.add
                                                                i32.load
                                                                i32.load offset=12
                                                                call_indirect (type 1)
                                                                return
                                                              end
                                                              local.get 1
                                                              i32.load offset=24
                                                              i32.const 1056905
                                                              i32.const 10
                                                              local.get 1
                                                              i32.const 28
                                                              i32.add
                                                              i32.load
                                                              i32.load offset=12
                                                              call_indirect (type 1)
                                                              return
                                                            end
                                                            local.get 1
                                                            i32.load offset=24
                                                            i32.const 1056892
                                                            i32.const 13
                                                            local.get 1
                                                            i32.const 28
                                                            i32.add
                                                            i32.load
                                                            i32.load offset=12
                                                            call_indirect (type 1)
                                                            return
                                                          end
                                                          local.get 1
                                                          i32.load offset=24
                                                          i32.const 1056882
                                                          i32.const 10
                                                          local.get 1
                                                          i32.const 28
                                                          i32.add
                                                          i32.load
                                                          i32.load offset=12
                                                          call_indirect (type 1)
                                                          return
                                                        end
                                                        local.get 1
                                                        i32.load offset=24
                                                        i32.const 1056869
                                                        i32.const 13
                                                        local.get 1
                                                        i32.const 28
                                                        i32.add
                                                        i32.load
                                                        i32.load offset=12
                                                        call_indirect (type 1)
                                                        return
                                                      end
                                                      local.get 1
                                                      i32.load offset=24
                                                      i32.const 1056857
                                                      i32.const 12
                                                      local.get 1
                                                      i32.const 28
                                                      i32.add
                                                      i32.load
                                                      i32.load offset=12
                                                      call_indirect (type 1)
                                                      return
                                                    end
                                                    local.get 1
                                                    i32.load offset=24
                                                    i32.const 1056840
                                                    i32.const 17
                                                    local.get 1
                                                    i32.const 28
                                                    i32.add
                                                    i32.load
                                                    i32.load offset=12
                                                    call_indirect (type 1)
                                                    return
                                                  end
                                                  local.get 1
                                                  i32.load offset=24
                                                  i32.const 1056822
                                                  i32.const 18
                                                  local.get 1
                                                  i32.const 28
                                                  i32.add
                                                  i32.load
                                                  i32.load offset=12
                                                  call_indirect (type 1)
                                                  return
                                                end
                                                local.get 1
                                                i32.load offset=24
                                                i32.const 1056808
                                                i32.const 14
                                                local.get 1
                                                i32.const 28
                                                i32.add
                                                i32.load
                                                i32.load offset=12
                                                call_indirect (type 1)
                                                return
                                              end
                                              local.get 1
                                              i32.load offset=24
                                              i32.const 1056786
                                              i32.const 22
                                              local.get 1
                                              i32.const 28
                                              i32.add
                                              i32.load
                                              i32.load offset=12
                                              call_indirect (type 1)
                                              return
                                            end
                                            local.get 1
                                            i32.load offset=24
                                            i32.const 1056774
                                            i32.const 12
                                            local.get 1
                                            i32.const 28
                                            i32.add
                                            i32.load
                                            i32.load offset=12
                                            call_indirect (type 1)
                                            return
                                          end
                                          local.get 1
                                          i32.load offset=24
                                          i32.const 1056763
                                          i32.const 11
                                          local.get 1
                                          i32.const 28
                                          i32.add
                                          i32.load
                                          i32.load offset=12
                                          call_indirect (type 1)
                                          return
                                        end
                                        local.get 1
                                        i32.load offset=24
                                        i32.const 1056755
                                        i32.const 8
                                        local.get 1
                                        i32.const 28
                                        i32.add
                                        i32.load
                                        i32.load offset=12
                                        call_indirect (type 1)
                                        return
                                      end
                                      local.get 1
                                      i32.load offset=24
                                      i32.const 1056746
                                      i32.const 9
                                      local.get 1
                                      i32.const 28
                                      i32.add
                                      i32.load
                                      i32.load offset=12
                                      call_indirect (type 1)
                                      return
                                    end
                                    local.get 1
                                    i32.load offset=24
                                    i32.const 1056735
                                    i32.const 11
                                    local.get 1
                                    i32.const 28
                                    i32.add
                                    i32.load
                                    i32.load offset=12
                                    call_indirect (type 1)
                                    return
                                  end
                                  local.get 1
                                  i32.load offset=24
                                  i32.const 1056724
                                  i32.const 11
                                  local.get 1
                                  i32.const 28
                                  i32.add
                                  i32.load
                                  i32.load offset=12
                                  call_indirect (type 1)
                                  return
                                end
                                local.get 1
                                i32.load offset=24
                                i32.const 1056701
                                i32.const 23
                                local.get 1
                                i32.const 28
                                i32.add
                                i32.load
                                i32.load offset=12
                                call_indirect (type 1)
                                return
                              end
                              local.get 1
                              i32.load offset=24
                              i32.const 1056689
                              i32.const 12
                              local.get 1
                              i32.const 28
                              i32.add
                              i32.load
                              i32.load offset=12
                              call_indirect (type 1)
                              return
                            end
                            local.get 1
                            i32.load offset=24
                            i32.const 1056677
                            i32.const 12
                            local.get 1
                            i32.const 28
                            i32.add
                            i32.load
                            i32.load offset=12
                            call_indirect (type 1)
                            return
                          end
                          local.get 1
                          i32.load offset=24
                          i32.const 1056659
                          i32.const 18
                          local.get 1
                          i32.const 28
                          i32.add
                          i32.load
                          i32.load offset=12
                          call_indirect (type 1)
                          return
                        end
                        local.get 1
                        i32.load offset=24
                        i32.const 1056651
                        i32.const 8
                        local.get 1
                        i32.const 28
                        i32.add
                        i32.load
                        i32.load offset=12
                        call_indirect (type 1)
                        return
                      end
                      local.get 1
                      i32.load offset=24
                      i32.const 1056637
                      i32.const 14
                      local.get 1
                      i32.const 28
                      i32.add
                      i32.load
                      i32.load offset=12
                      call_indirect (type 1)
                      return
                    end
                    local.get 1
                    i32.load offset=24
                    i32.const 1056625
                    i32.const 12
                    local.get 1
                    i32.const 28
                    i32.add
                    i32.load
                    i32.load offset=12
                    call_indirect (type 1)
                    return
                  end
                  local.get 1
                  i32.load offset=24
                  i32.const 1056610
                  i32.const 15
                  local.get 1
                  i32.const 28
                  i32.add
                  i32.load
                  i32.load offset=12
                  call_indirect (type 1)
                  return
                end
                local.get 1
                i32.load offset=24
                i32.const 1056591
                i32.const 19
                local.get 1
                i32.const 28
                i32.add
                i32.load
                i32.load offset=12
                call_indirect (type 1)
                return
              end
              local.get 1
              i32.load offset=24
              i32.const 1056580
              i32.const 11
              local.get 1
              i32.const 28
              i32.add
              i32.load
              i32.load offset=12
              call_indirect (type 1)
              return
            end
            local.get 1
            i32.load offset=24
            i32.const 1056500
            i32.const 11
            local.get 1
            i32.const 28
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type 1)
            return
          end
          local.get 1
          i32.load offset=24
          i32.const 1056567
          i32.const 13
          local.get 1
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 1)
          return
        end
        local.get 1
        i32.load offset=24
        i32.const 1056556
        i32.const 11
        local.get 1
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        return
      end
      local.get 1
      i32.load offset=24
      i32.const 1056551
      i32.const 5
      local.get 1
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 1)
      return
    end
    local.get 1
    i32.load offset=24
    i32.const 1056538
    i32.const 13
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 1)
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hd1c21b7c6ab1aac8E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 1
    call $_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17hecac681ab20f31d6E
  )
  (func $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17hee063b8b83848866E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.const 8
    i32.add
    i32.load
    local.get 1
    call $_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17hecac681ab20f31d6E
  )
  (func $_ZN58_$LT$std..io..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h14ce87ca4c0ddcf8E (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load8_u
                br_table 0 (;@6;) 1 (;@5;) 2 (;@4;) 3 (;@3;) 0 (;@6;)
              end
              local.get 2
              local.get 0
              i32.const 4
              i32.add
              i32.load
              i32.store offset=12
              local.get 2
              i64.const 4294967296
              i64.const 0
              local.get 1
              i32.load offset=24
              i32.const 1054568
              i32.const 2
              local.get 1
              i32.const 28
              i32.add
              i32.load
              i32.load offset=12
              call_indirect (type 1)
              select
              local.get 1
              i64.extend_i32_u
              i64.or
              i64.store offset=16
              local.get 2
              i32.const 16
              i32.add
              i32.const 1054570
              i32.const 4
              local.get 2
              i32.const 12
              i32.add
              i32.const 1054576
              call $_ZN4core3fmt8builders11DebugStruct5field17h8a23062b858eb435E
              local.set 0
              i32.const 40
              local.set 1
              block  ;; label = @6
                local.get 2
                i32.load offset=12
                local.tee 3
                i32.const 65535
                i32.gt_u
                br_if 0 (;@6;)
                i32.const 2
                local.set 1
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          block  ;; label = @20
                                            block  ;; label = @21
                                              local.get 3
                                              i32.const -2
                                              i32.add
                                              br_table 2 (;@19;) 7 (;@14;) 6 (;@15;) 0 (;@21;) 13 (;@8;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 5 (;@16;) 15 (;@6;) 1 (;@20;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 12 (;@9;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 9 (;@12;) 10 (;@11;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 8 (;@13;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 14 (;@7;) 4 (;@17;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 2 (;@19;) 3 (;@18;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 0 (;@21;) 11 (;@10;) 0 (;@21;)
                                            end
                                            i32.const 38
                                            i32.const 40
                                            local.get 3
                                            i32.const 48
                                            i32.eq
                                            select
                                            local.set 1
                                            br 14 (;@6;)
                                          end
                                          i32.const 3
                                          local.set 1
                                          br 13 (;@6;)
                                        end
                                        i32.const 1
                                        local.set 1
                                        br 12 (;@6;)
                                      end
                                      i32.const 11
                                      local.set 1
                                      br 11 (;@6;)
                                    end
                                    i32.const 7
                                    local.set 1
                                    br 10 (;@6;)
                                  end
                                  i32.const 6
                                  local.set 1
                                  br 9 (;@6;)
                                end
                                i32.const 9
                                local.set 1
                                br 8 (;@6;)
                              end
                              i32.const 8
                              local.set 1
                              br 7 (;@6;)
                            end
                            i32.const 0
                            local.set 1
                            br 6 (;@6;)
                          end
                          i32.const 35
                          local.set 1
                          br 5 (;@6;)
                        end
                        i32.const 20
                        local.set 1
                        br 4 (;@6;)
                      end
                      i32.const 22
                      local.set 1
                      br 3 (;@6;)
                    end
                    i32.const 12
                    local.set 1
                    br 2 (;@6;)
                  end
                  i32.const 13
                  local.set 1
                  br 1 (;@6;)
                end
                i32.const 36
                local.set 1
              end
              local.get 2
              local.get 1
              i32.store8 offset=31
              local.get 0
              i32.const 1054518
              i32.const 4
              local.get 2
              i32.const 31
              i32.add
              i32.const 1054524
              call $_ZN4core3fmt8builders11DebugStruct5field17h8a23062b858eb435E
              local.set 1
              local.get 2
              i32.const 32
              i32.add
              local.get 2
              i32.load offset=12
              call $_ZN3std3sys4wasi2os12error_string17h4bf3bfd3b66b331fE
              local.get 1
              i32.const 1054540
              i32.const 7
              local.get 2
              i32.const 32
              i32.add
              i32.const 1054592
              call $_ZN4core3fmt8builders11DebugStruct5field17h8a23062b858eb435E
              local.tee 0
              i32.load8_u offset=4
              local.set 1
              block  ;; label = @6
                local.get 0
                i32.load8_u offset=5
                i32.eqz
                br_if 0 (;@6;)
                local.get 1
                i32.const 255
                i32.and
                local.set 3
                i32.const 1
                local.set 1
                block  ;; label = @7
                  local.get 3
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    local.get 2
                    i32.load offset=16
                    local.tee 1
                    i32.load8_u
                    i32.const 4
                    i32.and
                    br_if 0 (;@8;)
                    local.get 1
                    i32.load offset=24
                    i32.const 1049382
                    i32.const 2
                    local.get 1
                    i32.const 28
                    i32.add
                    i32.load
                    i32.load offset=12
                    call_indirect (type 1)
                    local.set 1
                    br 1 (;@7;)
                  end
                  local.get 1
                  i32.load offset=24
                  i32.const 1049374
                  i32.const 1
                  local.get 1
                  i32.const 28
                  i32.add
                  i32.load
                  i32.load offset=12
                  call_indirect (type 1)
                  local.set 1
                end
                local.get 0
                local.get 1
                i32.store8 offset=4
              end
              local.get 2
              i32.load offset=36
              i32.eqz
              br_if 4 (;@1;)
              local.get 2
              i32.load offset=32
              local.tee 0
              i32.eqz
              br_if 4 (;@1;)
              local.get 0
              call $free
              br 4 (;@1;)
            end
            local.get 2
            local.get 0
            i32.load8_u offset=1
            i32.store8 offset=16
            local.get 2
            local.get 1
            i32.load offset=24
            i32.const 1054564
            i32.const 4
            local.get 1
            i32.const 28
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type 1)
            i32.store8 offset=40
            local.get 2
            local.get 1
            i32.store offset=32
            local.get 2
            i32.const 0
            i32.store8 offset=41
            local.get 2
            i32.const 0
            i32.store offset=36
            local.get 2
            i32.const 32
            i32.add
            local.get 2
            i32.const 16
            i32.add
            i32.const 1054524
            call $_ZN4core3fmt8builders10DebugTuple5field17h4b7d21df77126ad7E
            drop
            local.get 2
            i32.load8_u offset=40
            local.set 1
            local.get 2
            i32.load offset=36
            local.tee 3
            i32.eqz
            br_if 3 (;@1;)
            local.get 1
            i32.const 255
            i32.and
            local.set 0
            i32.const 1
            local.set 1
            local.get 0
            br_if 3 (;@1;)
            local.get 2
            i32.load offset=32
            local.set 0
            local.get 3
            i32.const 1
            i32.ne
            br_if 2 (;@2;)
            local.get 2
            i32.load8_u offset=41
            i32.const 255
            i32.and
            i32.eqz
            br_if 2 (;@2;)
            local.get 0
            i32.load8_u
            i32.const 4
            i32.and
            br_if 2 (;@2;)
            i32.const 1
            local.set 1
            local.get 0
            i32.load offset=24
            i32.const 1049387
            i32.const 1
            local.get 0
            i32.const 28
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type 1)
            i32.eqz
            br_if 2 (;@2;)
            br 3 (;@1;)
          end
          local.get 0
          i32.const 4
          i32.add
          i32.load
          local.set 0
          local.get 2
          i64.const 4294967296
          i64.const 0
          local.get 1
          i32.load offset=24
          i32.const 1054513
          i32.const 5
          local.get 1
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 1)
          select
          local.get 1
          i64.extend_i32_u
          i64.or
          i64.store offset=32
          local.get 2
          i32.const 32
          i32.add
          i32.const 1054518
          i32.const 4
          local.get 0
          i32.const 8
          i32.add
          i32.const 1054524
          call $_ZN4core3fmt8builders11DebugStruct5field17h8a23062b858eb435E
          i32.const 1054540
          i32.const 7
          local.get 0
          i32.const 1054548
          call $_ZN4core3fmt8builders11DebugStruct5field17h8a23062b858eb435E
          local.tee 0
          i32.load8_u offset=4
          local.set 1
          local.get 0
          i32.load8_u offset=5
          i32.eqz
          br_if 2 (;@1;)
          local.get 1
          i32.const 255
          i32.and
          local.set 0
          i32.const 1
          local.set 1
          local.get 0
          br_if 2 (;@1;)
          block  ;; label = @4
            local.get 2
            i32.load offset=32
            local.tee 1
            i32.load8_u
            i32.const 4
            i32.and
            br_if 0 (;@4;)
            local.get 1
            i32.load offset=24
            i32.const 1049382
            i32.const 2
            local.get 1
            i32.const 28
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type 1)
            local.set 1
            br 3 (;@1;)
          end
          local.get 1
          i32.load offset=24
          i32.const 1049374
          i32.const 1
          local.get 1
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 1)
          local.set 1
          br 2 (;@1;)
        end
        local.get 0
        i32.const 4
        i32.add
        i32.load
        local.set 0
        local.get 2
        i64.const 4294967296
        i64.const 0
        local.get 1
        i32.load offset=24
        i32.const 1056532
        i32.const 6
        local.get 1
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        select
        local.get 1
        i64.extend_i32_u
        i64.or
        i64.store offset=32
        local.get 2
        local.get 0
        i32.const 8
        i32.add
        i32.store offset=16
        local.get 2
        i32.const 32
        i32.add
        i32.const 1054518
        i32.const 4
        local.get 2
        i32.const 16
        i32.add
        i32.const 1056516
        call $_ZN4core3fmt8builders11DebugStruct5field17h8a23062b858eb435E
        local.set 1
        local.get 2
        local.get 0
        i32.store offset=16
        local.get 1
        i32.const 1056511
        i32.const 5
        local.get 2
        i32.const 16
        i32.add
        i32.const 1057772
        call $_ZN4core3fmt8builders11DebugStruct5field17h8a23062b858eb435E
        local.tee 0
        i32.load8_u offset=4
        local.set 1
        local.get 0
        i32.load8_u offset=5
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        i32.const 255
        i32.and
        local.set 0
        i32.const 1
        local.set 1
        local.get 0
        br_if 1 (;@1;)
        block  ;; label = @3
          local.get 2
          i32.load offset=32
          local.tee 1
          i32.load8_u
          i32.const 4
          i32.and
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=24
          i32.const 1049382
          i32.const 2
          local.get 1
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 1)
          local.set 1
          br 2 (;@1;)
        end
        local.get 1
        i32.load offset=24
        i32.const 1049374
        i32.const 1
        local.get 1
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        local.set 1
        br 1 (;@1;)
      end
      local.get 0
      i32.load offset=24
      i32.const 1054619
      i32.const 1
      local.get 0
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 1)
      local.set 1
    end
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 1
    i32.const 255
    i32.and
    i32.const 0
    i32.ne
  )
  (func $_ZN3std2io5Write9write_all17h3d11cae23b9bba14E (type 19) (param i32 i32) (result i64)
    (local i32 i64 i64 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i64.const 4
    local.set 3
    i64.const 0
    local.set 4
    block  ;; label = @1
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 2
        local.get 1
        i32.store offset=4
        local.get 2
        local.get 0
        i32.store
        block  ;; label = @3
          block  ;; label = @4
            i32.const 1
            local.get 2
            i32.const 1
            local.get 2
            i32.const 12
            i32.add
            call $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h6a67043e9c1785e7E
            local.tee 5
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 2
              i32.load offset=12
              local.tee 5
              br_if 0 (;@5;)
              i32.const 1054672
              i64.extend_i32_u
              i64.const 32
              i64.shl
              local.set 4
              i64.const 2
              local.set 3
              br 4 (;@1;)
            end
            local.get 1
            local.get 5
            i32.ge_u
            br_if 1 (;@3;)
            local.get 5
            local.get 1
            i32.const 1054828
            call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
            unreachable
          end
          block  ;; label = @4
            local.get 5
            i32.const 65535
            i32.and
            local.tee 5
            i32.const 27
            i32.ne
            br_if 0 (;@4;)
            local.get 1
            br_if 2 (;@2;)
            br 3 (;@1;)
          end
          local.get 5
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.set 4
          i64.const 0
          local.set 3
          br 2 (;@1;)
        end
        local.get 0
        local.get 5
        i32.add
        local.set 0
        local.get 1
        local.get 5
        i32.sub
        local.tee 1
        br_if 0 (;@2;)
      end
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 4
    local.get 3
    i64.or
  )
  (func $_ZN3std2io5Write9write_all17hb8c7bdf3b66691e0E (type 19) (param i32 i32) (result i64)
    (local i32 i64 i64 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i64.const 4
    local.set 3
    i64.const 0
    local.set 4
    block  ;; label = @1
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 2
        local.get 1
        i32.store offset=4
        local.get 2
        local.get 0
        i32.store
        block  ;; label = @3
          block  ;; label = @4
            i32.const 2
            local.get 2
            i32.const 1
            local.get 2
            i32.const 12
            i32.add
            call $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h6a67043e9c1785e7E
            local.tee 5
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 2
              i32.load offset=12
              local.tee 5
              br_if 0 (;@5;)
              i32.const 1054672
              i64.extend_i32_u
              i64.const 32
              i64.shl
              local.set 4
              i64.const 2
              local.set 3
              br 4 (;@1;)
            end
            local.get 1
            local.get 5
            i32.ge_u
            br_if 1 (;@3;)
            local.get 5
            local.get 1
            i32.const 1054828
            call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
            unreachable
          end
          block  ;; label = @4
            local.get 5
            i32.const 65535
            i32.and
            local.tee 5
            i32.const 27
            i32.ne
            br_if 0 (;@4;)
            local.get 1
            br_if 2 (;@2;)
            br 3 (;@1;)
          end
          local.get 5
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.set 4
          i64.const 0
          local.set 3
          br 2 (;@1;)
        end
        local.get 0
        local.get 5
        i32.add
        local.set 0
        local.get 1
        local.get 5
        i32.sub
        local.tee 1
        br_if 0 (;@2;)
      end
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 4
    local.get 3
    i64.or
  )
  (func $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h91afc922b67f4252E (type 1) (param i32 i32 i32) (result i32)
    (local i64 i32)
    block  ;; label = @1
      local.get 1
      local.get 2
      call $_ZN3std2io5Write9write_all17hb8c7bdf3b66691e0E
      local.tee 3
      i32.wrap_i64
      local.tee 2
      i32.const 255
      i32.and
      local.tee 1
      i32.const 4
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      i64.const 8
      i64.shr_u
      local.set 3
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=4
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.const 8
        i32.add
        i32.load
        local.tee 4
        i32.load
        local.get 4
        i32.load offset=4
        i32.load
        call_indirect (type 0)
        block  ;; label = @3
          local.get 4
          i32.load offset=4
          i32.load offset=4
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.load
          call $free
        end
        local.get 4
        call $free
      end
      local.get 0
      local.get 2
      i32.store8 offset=4
      local.get 0
      i32.const 11
      i32.add
      local.get 3
      i64.const 48
      i64.shr_u
      i64.store8
      local.get 0
      i32.const 9
      i32.add
      local.get 3
      i64.const 32
      i64.shr_u
      i64.store16 align=1
      local.get 0
      i32.const 5
      i32.add
      local.get 3
      i64.store32 align=1
    end
    local.get 1
    i32.const 4
    i32.ne
  )
  (func $_ZN4core3fmt5Write10write_char17h7a41aa260b58b7adE (type 2) (param i32 i32) (result i32)
    (local i32 i64 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 2 (;@2;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.store8 offset=12
          i32.const 1
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=15
      local.get 2
      local.get 1
      i32.const 18
      i32.shr_u
      i32.const 240
      i32.or
      i32.store8 offset=12
      local.get 2
      local.get 1
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=14
      local.get 2
      local.get 1
      i32.const 12
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=13
      i32.const 4
      local.set 1
    end
    block  ;; label = @1
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $_ZN3std2io5Write9write_all17hb8c7bdf3b66691e0E
      local.tee 3
      i32.wrap_i64
      local.tee 4
      i32.const 255
      i32.and
      local.tee 1
      i32.const 4
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      i64.const 8
      i64.shr_u
      local.set 3
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=4
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.const 8
        i32.add
        i32.load
        local.tee 5
        i32.load
        local.get 5
        i32.load offset=4
        i32.load
        call_indirect (type 0)
        block  ;; label = @3
          local.get 5
          i32.load offset=4
          i32.load offset=4
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          i32.load
          call $free
        end
        local.get 5
        call $free
      end
      local.get 0
      local.get 4
      i32.store8 offset=4
      local.get 0
      i32.const 11
      i32.add
      local.get 3
      i64.const 48
      i64.shr_u
      i64.store8
      local.get 0
      i32.const 9
      i32.add
      local.get 3
      i64.const 32
      i64.shr_u
      i64.store16 align=1
      local.get 0
      i32.const 5
      i32.add
      local.get 3
      i64.store32 align=1
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
    i32.const 4
    i32.ne
  )
  (func $_ZN4core3fmt5Write9write_fmt17h82208e6ac213d2bcE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1056476
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17he59324c441734165E (type 1) (param i32 i32 i32) (result i32)
    (local i64 i32)
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      local.get 1
      local.get 2
      call $_ZN3std2io5Write9write_all17hb8c7bdf3b66691e0E
      local.tee 3
      i32.wrap_i64
      local.tee 2
      i32.const 255
      i32.and
      local.tee 1
      i32.const 4
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      i64.const 8
      i64.shr_u
      local.set 3
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=4
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.const 8
        i32.add
        i32.load
        local.tee 4
        i32.load
        local.get 4
        i32.load offset=4
        i32.load
        call_indirect (type 0)
        block  ;; label = @3
          local.get 4
          i32.load offset=4
          i32.load offset=4
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.load
          call $free
        end
        local.get 4
        call $free
      end
      local.get 0
      local.get 2
      i32.store8 offset=4
      local.get 0
      i32.const 11
      i32.add
      local.get 3
      i64.const 48
      i64.shr_u
      i64.store8
      local.get 0
      i32.const 9
      i32.add
      local.get 3
      i64.const 32
      i64.shr_u
      i64.store16 align=1
      local.get 0
      i32.const 5
      i32.add
      local.get 3
      i64.store32 align=1
    end
    local.get 1
    i32.const 4
    i32.ne
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hf5346a345d2eb246E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN4core3fmt5Write10write_char17h7a41aa260b58b7adE
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h9dbd3b7280334987E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1056476
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hfd65f0ac2fab3244E (type 5) (param i32 i32)
    (local i32)
    local.get 0
    i32.load
    local.tee 2
    i32.load
    local.set 0
    local.get 2
    i32.const 0
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        i32.const 1024
        call $malloc
        local.tee 2
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 0
        i32.store8 offset=16
        local.get 0
        i64.const 1024
        i64.store offset=8 align=4
        local.get 0
        local.get 2
        i32.store offset=4
        local.get 0
        i32.const 0
        i32.store
        return
      end
      i32.const 1055427
      i32.const 43
      i32.const 1055220
      call $_ZN4core9panicking5panic17h66d99d0b614b1c7cE
      unreachable
    end
    i32.const 1024
    i32.const 1
    call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
    unreachable
  )
  (func $_ZN3std4sync4once4Once15call_once_force28_$u7b$$u7b$closure$u7d$$u7d$17h616156d4effed58bE (type 5) (param i32 i32)
    (local i32)
    local.get 0
    i32.load
    local.tee 2
    i32.load
    local.set 0
    local.get 2
    i32.const 0
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        i32.const 1024
        call $malloc
        local.tee 2
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 0
        i32.store8 offset=16
        local.get 0
        i64.const 1024
        i64.store offset=8 align=4
        local.get 0
        local.get 2
        i32.store offset=4
        local.get 0
        i32.const 0
        i32.store
        return
      end
      i32.const 1055427
      i32.const 43
      i32.const 1055220
      call $_ZN4core9panicking5panic17h66d99d0b614b1c7cE
      unreachable
    end
    i32.const 1024
    i32.const 1
    call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
    unreachable
  )
  (func $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$9flush_buf17ha7ea64205195f0ceE (type 3) (param i32) (result i64)
    (local i32 i32 i32 i32 i64 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    i32.const 4
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.const 8
          i32.add
          i32.load
          local.tee 3
          br_if 0 (;@3;)
          br 1 (;@2;)
        end
        local.get 0
        i32.load
        local.set 4
        i32.const 1053632
        i64.extend_i32_u
        i64.const 32
        i64.shl
        i64.const 8
        i64.shr_u
        local.set 5
        i32.const 0
        local.set 6
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              loop  ;; label = @6
                local.get 3
                local.get 6
                i32.lt_u
                br_if 1 (;@5;)
                local.get 1
                local.get 3
                local.get 6
                i32.sub
                local.tee 7
                i32.store offset=20
                local.get 1
                local.get 4
                local.get 6
                i32.add
                local.tee 8
                i32.store offset=16
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 1
                            local.get 1
                            i32.const 16
                            i32.add
                            i32.const 1
                            local.get 1
                            i32.const 28
                            i32.add
                            call $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h6a67043e9c1785e7E
                            local.tee 9
                            br_if 0 (;@12;)
                            local.get 1
                            i32.load offset=28
                            local.set 10
                            br 1 (;@11;)
                          end
                          local.get 7
                          local.set 10
                          local.get 9
                          i32.const 65535
                          i32.and
                          local.tee 9
                          i32.const 8
                          i32.ne
                          br_if 1 (;@10;)
                        end
                        local.get 0
                        i32.const 0
                        i32.store8 offset=12
                        local.get 10
                        br_if 2 (;@8;)
                        i32.const 2
                        local.set 2
                        br 1 (;@9;)
                      end
                      local.get 0
                      i32.const 0
                      i32.store8 offset=12
                      local.get 1
                      i32.const 0
                      i32.store offset=8
                      local.get 1
                      local.get 9
                      i32.store offset=12
                      local.get 9
                      i32.const 27
                      i32.eq
                      br_if 2 (;@7;)
                      local.get 1
                      i64.load32_u offset=9 align=1
                      local.get 1
                      i64.load16_u offset=13 align=1
                      local.get 1
                      i64.load8_u offset=15
                      i64.const 16
                      i64.shl
                      i64.or
                      i64.const 32
                      i64.shl
                      i64.or
                      local.set 5
                      i32.const 0
                      local.set 2
                    end
                    local.get 6
                    i32.eqz
                    br_if 6 (;@2;)
                    local.get 0
                    i32.const 8
                    i32.add
                    i32.const 0
                    i32.store
                    local.get 7
                    i32.eqz
                    br_if 6 (;@2;)
                    local.get 4
                    local.get 8
                    local.get 7
                    call $memmove
                    drop
                    br 5 (;@3;)
                  end
                  local.get 10
                  local.get 6
                  i32.add
                  local.set 6
                end
                local.get 3
                local.get 6
                i32.le_u
                br_if 2 (;@4;)
                br 0 (;@6;)
              end
            end
            local.get 6
            local.get 3
            i32.const 1053684
            call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
            unreachable
          end
          local.get 3
          local.get 6
          i32.lt_u
          br_if 2 (;@1;)
          local.get 0
          i32.const 8
          i32.add
          i32.const 0
          i32.store
          block  ;; label = @4
            local.get 3
            local.get 6
            i32.sub
            local.tee 7
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
          local.get 4
          local.get 4
          local.get 6
          i32.add
          local.get 7
          call $memmove
          drop
          i32.const 4
          local.set 2
        end
        local.get 0
        i32.const 8
        i32.add
        local.get 7
        i32.store
      end
      local.get 1
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get 5
      i64.const 8
      i64.shl
      local.get 2
      i64.extend_i32_u
      i64.or
      return
    end
    local.get 6
    local.get 3
    i32.const 1055120
    call $_ZN4core5slice5index24slice_end_index_len_fail17h53a611cf4b2e1c2bE
    unreachable
  )
  (func $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17hf6bf54e1f5632f2fE (type 10) (param i32 i32 i32) (result i64)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i64 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load
                local.tee 4
                i32.load
                br_if 0 (;@6;)
                local.get 4
                i32.const -1
                i32.store
                local.get 2
                i32.const 0
                local.get 2
                local.get 1
                i32.const 3
                i32.add
                i32.const -4
                i32.and
                local.get 1
                i32.sub
                local.tee 0
                i32.sub
                i32.const 7
                i32.and
                local.get 2
                local.get 0
                i32.lt_u
                local.tee 5
                select
                local.tee 6
                i32.sub
                local.set 7
                local.get 2
                local.get 6
                i32.lt_u
                br_if 1 (;@5;)
                local.get 2
                local.get 0
                local.get 5
                select
                local.set 8
                local.get 4
                i32.const 4
                i32.add
                local.set 9
                block  ;; label = @7
                  local.get 6
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 2
                  i32.add
                  local.tee 0
                  local.get 1
                  local.get 7
                  i32.add
                  local.tee 5
                  i32.sub
                  local.set 6
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.const -1
                      i32.add
                      local.tee 10
                      i32.load8_u
                      i32.const 10
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 6
                      i32.const -1
                      i32.add
                      local.set 0
                      br 1 (;@8;)
                    end
                    local.get 5
                    local.get 10
                    i32.eq
                    br_if 1 (;@7;)
                    block  ;; label = @9
                      local.get 0
                      i32.const -2
                      i32.add
                      local.tee 10
                      i32.load8_u
                      i32.const 10
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 6
                      i32.const -2
                      i32.add
                      local.set 0
                      br 1 (;@8;)
                    end
                    local.get 5
                    local.get 10
                    i32.eq
                    br_if 1 (;@7;)
                    block  ;; label = @9
                      local.get 0
                      i32.const -3
                      i32.add
                      local.tee 10
                      i32.load8_u
                      i32.const 10
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 6
                      i32.const -3
                      i32.add
                      local.set 0
                      br 1 (;@8;)
                    end
                    local.get 5
                    local.get 10
                    i32.eq
                    br_if 1 (;@7;)
                    block  ;; label = @9
                      local.get 0
                      i32.const -4
                      i32.add
                      local.tee 10
                      i32.load8_u
                      i32.const 10
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 6
                      i32.const -4
                      i32.add
                      local.set 0
                      br 1 (;@8;)
                    end
                    local.get 5
                    local.get 10
                    i32.eq
                    br_if 1 (;@7;)
                    block  ;; label = @9
                      local.get 0
                      i32.const -5
                      i32.add
                      local.tee 10
                      i32.load8_u
                      i32.const 10
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 6
                      i32.const -5
                      i32.add
                      local.set 0
                      br 1 (;@8;)
                    end
                    local.get 5
                    local.get 10
                    i32.eq
                    br_if 1 (;@7;)
                    block  ;; label = @9
                      local.get 0
                      i32.const -6
                      i32.add
                      local.tee 10
                      i32.load8_u
                      i32.const 10
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 6
                      i32.const -6
                      i32.add
                      local.set 0
                      br 1 (;@8;)
                    end
                    local.get 5
                    local.get 10
                    i32.eq
                    br_if 1 (;@7;)
                    block  ;; label = @9
                      local.get 0
                      i32.const -7
                      i32.add
                      local.tee 0
                      i32.load8_u
                      i32.const 10
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 6
                      i32.const -7
                      i32.add
                      local.set 0
                      br 1 (;@8;)
                    end
                    local.get 5
                    local.get 0
                    i32.eq
                    br_if 1 (;@7;)
                    local.get 6
                    i32.const -8
                    i32.add
                    local.set 0
                  end
                  local.get 0
                  local.get 7
                  i32.add
                  local.set 6
                  br 4 (;@3;)
                end
                block  ;; label = @7
                  loop  ;; label = @8
                    local.get 7
                    local.tee 0
                    local.get 8
                    i32.le_u
                    br_if 1 (;@7;)
                    local.get 0
                    i32.const -8
                    i32.add
                    local.set 7
                    local.get 1
                    local.get 0
                    i32.add
                    local.tee 6
                    i32.const -8
                    i32.add
                    i32.load
                    local.tee 5
                    i32.const -1
                    i32.xor
                    local.get 5
                    i32.const 168430090
                    i32.xor
                    i32.const -16843009
                    i32.add
                    i32.and
                    local.get 6
                    i32.const -4
                    i32.add
                    i32.load
                    local.tee 6
                    i32.const -1
                    i32.xor
                    local.get 6
                    i32.const 168430090
                    i32.xor
                    i32.const -16843009
                    i32.add
                    i32.and
                    i32.or
                    i32.const -2139062144
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                  end
                end
                local.get 0
                local.get 2
                i32.gt_u
                br_if 2 (;@4;)
                block  ;; label = @7
                  loop  ;; label = @8
                    local.get 0
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 1
                    local.get 0
                    i32.add
                    local.set 7
                    local.get 0
                    i32.const -1
                    i32.add
                    local.tee 6
                    local.set 0
                    local.get 7
                    i32.const -1
                    i32.add
                    i32.load8_u
                    i32.const 10
                    i32.eq
                    br_if 5 (;@3;)
                    br 0 (;@8;)
                  end
                end
                i64.const 4
                local.set 11
                block  ;; label = @7
                  local.get 4
                  i32.const 12
                  i32.add
                  i32.load
                  local.tee 0
                  i32.eqz
                  br_if 0 (;@7;)
                  i64.const 4
                  local.set 11
                  local.get 4
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 7
                  i32.eqz
                  br_if 0 (;@7;)
                  i64.const 4
                  local.set 11
                  local.get 0
                  local.get 7
                  i32.add
                  i32.const -1
                  i32.add
                  i32.load8_u
                  i32.const 10
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 9
                  call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$9flush_buf17ha7ea64205195f0ceE
                  local.set 11
                end
                block  ;; label = @7
                  i64.const 4
                  local.get 11
                  i64.const 255
                  i64.and
                  local.get 11
                  i32.wrap_i64
                  i32.const 255
                  i32.and
                  i32.const 4
                  i32.eq
                  select
                  local.tee 12
                  i64.const 4
                  i64.eq
                  br_if 0 (;@7;)
                  local.get 12
                  local.get 11
                  i64.const -256
                  i64.and
                  i64.or
                  local.set 11
                  br 6 (;@1;)
                end
                block  ;; label = @7
                  local.get 4
                  i32.const 8
                  i32.add
                  i32.load
                  local.get 4
                  i32.const 12
                  i32.add
                  local.tee 7
                  i32.load
                  local.tee 0
                  i32.sub
                  local.get 2
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 9
                  local.get 1
                  local.get 2
                  call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h74edfb6f070a6985E
                  local.set 11
                  br 6 (;@1;)
                end
                local.get 4
                i32.load offset=4
                local.get 0
                i32.add
                local.get 1
                local.get 2
                call $memcpy
                drop
                local.get 7
                local.get 0
                local.get 2
                i32.add
                i32.store
                br 4 (;@2;)
              end
              i32.const 1053428
              i32.const 16
              local.get 3
              i32.const 8
              i32.add
              i32.const 1055820
              i32.const 1054728
              call $_ZN4core6result13unwrap_failed17h1b005d733a1499a6E
              unreachable
            end
            local.get 7
            local.get 2
            i32.const 1049792
            call $_ZN4core5slice5index26slice_start_index_len_fail17hc19496dd39092298E
            unreachable
          end
          local.get 0
          local.get 2
          i32.const 1049808
          call $_ZN4core5slice5index24slice_end_index_len_fail17h53a611cf4b2e1c2bE
          unreachable
        end
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 6
              i32.const 1
              i32.add
              local.tee 0
              local.get 2
              i32.gt_u
              br_if 0 (;@5;)
              block  ;; label = @6
                block  ;; label = @7
                  local.get 4
                  i32.const 12
                  i32.add
                  i32.load
                  local.tee 7
                  i32.eqz
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 4
                      i32.const 8
                      i32.add
                      i32.load
                      local.get 7
                      i32.sub
                      local.get 0
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 9
                      local.get 1
                      local.get 0
                      call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h74edfb6f070a6985E
                      local.set 11
                      br 1 (;@8;)
                    end
                    local.get 4
                    i32.load offset=4
                    local.get 7
                    i32.add
                    local.get 1
                    local.get 0
                    call $memcpy
                    drop
                    local.get 4
                    i32.const 12
                    i32.add
                    local.get 7
                    local.get 0
                    i32.add
                    i32.store
                    i64.const 4
                    local.set 11
                  end
                  i64.const 4
                  local.get 11
                  i64.const 255
                  i64.and
                  local.get 11
                  i32.wrap_i64
                  i32.const 255
                  i32.and
                  i32.const 4
                  i32.eq
                  select
                  local.tee 12
                  i64.const 4
                  i64.eq
                  br_if 1 (;@6;)
                  local.get 12
                  local.get 11
                  i64.const -256
                  i64.and
                  i64.or
                  local.set 11
                  br 6 (;@1;)
                end
                i32.const 0
                local.set 7
                i64.const 4
                i64.const 4
                local.get 1
                local.get 0
                call $_ZN3std2io5Write9write_all17h3d11cae23b9bba14E
                local.tee 11
                local.get 11
                i64.const -4294967041
                i64.and
                i64.const 34359738368
                i64.eq
                select
                local.get 11
                local.get 11
                i32.wrap_i64
                i32.const 255
                i32.and
                i32.const 4
                i32.ne
                select
                local.tee 11
                i64.const 255
                i64.and
                local.get 11
                i32.wrap_i64
                i32.const 255
                i32.and
                i32.const 4
                i32.eq
                select
                local.tee 12
                i64.const 4
                i64.eq
                br_if 3 (;@3;)
                local.get 12
                local.get 11
                i64.const -256
                i64.and
                i64.or
                local.set 11
                br 5 (;@1;)
              end
              i64.const 4
              local.get 9
              call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$9flush_buf17ha7ea64205195f0ceE
              local.tee 11
              i64.const 255
              i64.and
              local.get 11
              i32.wrap_i64
              i32.const 255
              i32.and
              i32.const 4
              i32.eq
              select
              local.tee 12
              i64.const 4
              i64.eq
              br_if 1 (;@4;)
              local.get 12
              local.get 11
              i64.const -256
              i64.and
              i64.or
              local.set 11
              br 4 (;@1;)
            end
            i32.const 1057172
            i32.const 35
            i32.const 1053748
            call $_ZN4core9panicking5panic17h66d99d0b614b1c7cE
            unreachable
          end
          local.get 4
          i32.const 12
          i32.add
          i32.load
          local.set 7
        end
        local.get 1
        local.get 0
        i32.add
        local.set 1
        block  ;; label = @3
          local.get 4
          i32.const 8
          i32.add
          i32.load
          local.get 7
          i32.sub
          local.get 2
          local.get 0
          i32.sub
          local.tee 0
          i32.gt_u
          br_if 0 (;@3;)
          local.get 9
          local.get 1
          local.get 0
          call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h74edfb6f070a6985E
          local.set 11
          br 2 (;@1;)
        end
        local.get 4
        i32.load offset=4
        local.get 7
        i32.add
        local.get 1
        local.get 0
        call $memcpy
        drop
        local.get 4
        i32.const 12
        i32.add
        local.get 7
        local.get 0
        i32.add
        i32.store
      end
      i64.const 4
      local.set 11
    end
    local.get 4
    local.get 4
    i32.load
    i32.const 1
    i32.add
    i32.store
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 11
  )
  (func $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h74edfb6f070a6985E (type 10) (param i32 i32 i32) (result i64)
    (local i32 i64 i64 i64)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 4
        i32.add
        i32.load
        local.tee 3
        local.get 0
        i32.const 8
        i32.add
        i32.load
        i32.sub
        local.get 2
        i32.ge_u
        br_if 0 (;@2;)
        block  ;; label = @3
          i64.const 4
          local.get 0
          call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$9flush_buf17ha7ea64205195f0ceE
          local.tee 4
          i64.const 255
          i64.and
          local.get 4
          i32.wrap_i64
          i32.const 255
          i32.and
          i32.const 4
          i32.eq
          select
          local.tee 5
          i64.const 4
          i64.eq
          br_if 0 (;@3;)
          local.get 5
          local.get 4
          i64.const -256
          i64.and
          local.tee 6
          i64.or
          local.set 4
          br 2 (;@1;)
        end
        local.get 0
        i32.const 4
        i32.add
        i32.load
        local.set 3
      end
      block  ;; label = @2
        local.get 3
        local.get 2
        i32.le_u
        br_if 0 (;@2;)
        local.get 0
        i32.load
        local.get 0
        i32.const 8
        i32.add
        local.tee 0
        i32.load
        local.tee 3
        i32.add
        local.get 1
        local.get 2
        call $memcpy
        drop
        local.get 0
        local.get 3
        local.get 2
        i32.add
        i32.store
        i64.const 4
        local.set 4
        i64.const 0
        local.set 6
        br 1 (;@1;)
      end
      local.get 1
      local.get 2
      call $_ZN3std2io5Write9write_all17h3d11cae23b9bba14E
      local.set 4
      local.get 0
      i32.const 0
      i32.store8 offset=12
      i64.const 4
      local.get 4
      local.get 4
      i64.const -4294967041
      i64.and
      i64.const 34359738368
      i64.eq
      select
      local.get 4
      local.get 4
      i32.wrap_i64
      i32.const 255
      i32.and
      i32.const 4
      i32.ne
      select
      local.tee 4
      i64.const -256
      i64.and
      local.set 6
    end
    local.get 4
    i64.const 255
    i64.and
    local.get 6
    i64.or
  )
  (func $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h05f5b7cc3782462eE (type 1) (param i32 i32 i32) (result i32)
    (local i64 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.get 1
      local.get 2
      call $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17hf6bf54e1f5632f2fE
      local.tee 3
      i32.wrap_i64
      local.tee 2
      i32.const 255
      i32.and
      local.tee 1
      i32.const 4
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      i64.const 8
      i64.shr_u
      local.set 3
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=4
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.const 8
        i32.add
        i32.load
        local.tee 4
        i32.load
        local.get 4
        i32.load offset=4
        i32.load
        call_indirect (type 0)
        block  ;; label = @3
          local.get 4
          i32.load offset=4
          i32.load offset=4
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.load
          call $free
        end
        local.get 4
        call $free
      end
      local.get 0
      local.get 2
      i32.store8 offset=4
      local.get 0
      i32.const 11
      i32.add
      local.get 3
      i64.const 48
      i64.shr_u
      i64.store8
      local.get 0
      i32.const 9
      i32.add
      local.get 3
      i64.const 32
      i64.shr_u
      i64.store16 align=1
      local.get 0
      i32.const 5
      i32.add
      local.get 3
      i64.store32 align=1
    end
    local.get 1
    i32.const 4
    i32.ne
  )
  (func $_ZN4core3fmt5Write10write_char17h907aac247470347aE (type 2) (param i32 i32) (result i32)
    (local i32 i64 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 2 (;@2;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.store8 offset=12
          i32.const 1
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=15
      local.get 2
      local.get 1
      i32.const 18
      i32.shr_u
      i32.const 240
      i32.or
      i32.store8 offset=12
      local.get 2
      local.get 1
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=14
      local.get 2
      local.get 1
      i32.const 12
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=13
      i32.const 4
      local.set 1
    end
    block  ;; label = @1
      local.get 0
      i32.load
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17hf6bf54e1f5632f2fE
      local.tee 3
      i32.wrap_i64
      local.tee 4
      i32.const 255
      i32.and
      local.tee 1
      i32.const 4
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      i64.const 8
      i64.shr_u
      local.set 3
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=4
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.const 8
        i32.add
        i32.load
        local.tee 5
        i32.load
        local.get 5
        i32.load offset=4
        i32.load
        call_indirect (type 0)
        block  ;; label = @3
          local.get 5
          i32.load offset=4
          i32.load offset=4
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          i32.load
          call $free
        end
        local.get 5
        call $free
      end
      local.get 0
      local.get 4
      i32.store8 offset=4
      local.get 0
      i32.const 11
      i32.add
      local.get 3
      i64.const 48
      i64.shr_u
      i64.store8
      local.get 0
      i32.const 9
      i32.add
      local.get 3
      i64.const 32
      i64.shr_u
      i64.store16 align=1
      local.get 0
      i32.const 5
      i32.add
      local.get 3
      i64.store32 align=1
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
    i32.const 4
    i32.ne
  )
  (func $_ZN4core3fmt5Write9write_fmt17h30663a1432e14ffeE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1056428
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17hed18ade96c57a42bE (type 1) (param i32 i32 i32) (result i32)
    (local i64 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 0
      i32.load
      local.get 1
      local.get 2
      call $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17hf6bf54e1f5632f2fE
      local.tee 3
      i32.wrap_i64
      local.tee 2
      i32.const 255
      i32.and
      local.tee 1
      i32.const 4
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      i64.const 8
      i64.shr_u
      local.set 3
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=4
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.const 8
        i32.add
        i32.load
        local.tee 4
        i32.load
        local.get 4
        i32.load offset=4
        i32.load
        call_indirect (type 0)
        block  ;; label = @3
          local.get 4
          i32.load offset=4
          i32.load offset=4
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.load
          call $free
        end
        local.get 4
        call $free
      end
      local.get 0
      local.get 2
      i32.store8 offset=4
      local.get 0
      i32.const 11
      i32.add
      local.get 3
      i64.const 48
      i64.shr_u
      i64.store8
      local.get 0
      i32.const 9
      i32.add
      local.get 3
      i64.const 32
      i64.shr_u
      i64.store16 align=1
      local.get 0
      i32.const 5
      i32.add
      local.get 3
      i64.store32 align=1
    end
    local.get 1
    i32.const 4
    i32.ne
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h5391ccdd75d11cb1E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN4core3fmt5Write10write_char17h907aac247470347aE
  )
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h3ad948c51b49e3feE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1056428
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $rust_panic (type 4)
    unreachable
    unreachable
  )
  (func $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h22e1af2a31434bd2E (type 5) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.load
    i32.store offset=12
    local.get 2
    i32.const 12
    i32.add
    local.get 1
    call $_ZN3std4sync4once4Once9call_once28_$u7b$$u7b$closure$u7d$$u7d$17h23a331af73a3a34fE
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN3std4sync4once4Once9call_once28_$u7b$$u7b$closure$u7d$$u7d$17h23a331af73a3a34fE (type 5) (param i32 i32)
    (local i32 i32 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.tee 0
    i32.load8_u
    local.set 3
    local.get 0
    i32.const 0
    i32.store8
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          i32.const 0
          i32.load offset=1059868
          i32.const 3
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1059872
          br_if 2 (;@1;)
          i32.const 0
          i32.const -1
          i32.store offset=1059872
          block  ;; label = @4
            i32.const 0
            i32.load8_u offset=1059888
            br_if 0 (;@4;)
            i32.const 1059876
            call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$9flush_buf17ha7ea64205195f0ceE
            local.tee 4
            i32.wrap_i64
            i32.const 255
            i32.and
            i32.const 3
            i32.ne
            br_if 0 (;@4;)
            local.get 4
            i64.const 32
            i64.shr_u
            i32.wrap_i64
            local.tee 0
            i32.load
            local.get 0
            i32.load offset=4
            i32.load
            call_indirect (type 0)
            block  ;; label = @5
              local.get 0
              i32.load offset=4
              i32.load offset=4
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              i32.load
              call $free
            end
            local.get 0
            call $free
          end
          block  ;; label = @4
            i32.const 0
            i32.load offset=1059880
            i32.eqz
            br_if 0 (;@4;)
            i32.const 0
            i32.load offset=1059876
            local.tee 0
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            call $free
          end
          i32.const 0
          i64.const 0
          i64.store offset=1059880 align=4
          i32.const 0
          i32.const 1
          i32.store offset=1059876
          i32.const 0
          i32.const 0
          i32.load offset=1059872
          i32.const 1
          i32.add
          i32.store offset=1059872
          i32.const 0
          i32.const 0
          i32.store8 offset=1059888
        end
        local.get 2
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      i32.const 1055427
      i32.const 43
      i32.const 1055184
      call $_ZN4core9panicking5panic17h66d99d0b614b1c7cE
      unreachable
    end
    i32.const 1053428
    i32.const 16
    local.get 2
    i32.const 8
    i32.add
    i32.const 1055820
    i32.const 1054712
    call $_ZN4core6result13unwrap_failed17h1b005d733a1499a6E
    unreachable
  )
  (func $_ZN3std7process5abort17hc143e6d6381800dcE (type 4)
    call $_ZN3std3sys4wasi14abort_internal17h6f2ef412fa287f15E
    unreachable
  )
  (func $_ZN91_$LT$std..sys_common..backtrace.._print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17hcd900b1d582d2005E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load8_u
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                i32.const 512
                call $malloc
                local.tee 4
                i32.eqz
                br_if 0 (;@6;)
                local.get 2
                i32.const 512
                i32.store offset=12
                local.get 2
                local.get 4
                i32.store offset=8
                local.get 4
                i32.const 512
                call $getcwd
                br_if 1 (;@5;)
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 0
                    i32.load offset=1060428
                    local.tee 5
                    i32.const 68
                    i32.ne
                    br_if 0 (;@8;)
                    i32.const 512
                    local.set 0
                    loop  ;; label = @9
                      local.get 2
                      local.get 0
                      i32.store offset=16
                      local.get 2
                      i32.const 8
                      i32.add
                      local.get 0
                      i32.const 1
                      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h9e9d16cf4f4f67caE
                      block  ;; label = @10
                        local.get 2
                        i32.load offset=8
                        local.tee 4
                        local.get 2
                        i32.load offset=12
                        local.tee 0
                        call $getcwd
                        i32.eqz
                        br_if 0 (;@10;)
                        block  ;; label = @11
                          local.get 4
                          i32.load8_u
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 0
                          local.set 5
                          br 7 (;@4;)
                        end
                        local.get 0
                        br_if 8 (;@2;)
                        i32.const 0
                        local.set 5
                        br 9 (;@1;)
                      end
                      i32.const 0
                      i32.load offset=1060428
                      local.tee 5
                      i32.const 68
                      i32.eq
                      br_if 0 (;@9;)
                    end
                    local.get 0
                    i32.eqz
                    br_if 1 (;@7;)
                  end
                  local.get 4
                  call $free
                end
                i32.const 0
                local.set 4
                br 5 (;@1;)
              end
              i32.const 512
              i32.const 1
              call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
              unreachable
            end
            local.get 4
            i32.load8_u
            i32.eqz
            br_if 1 (;@3;)
            i32.const 512
            local.set 5
          end
          i32.const 0
          local.set 0
          loop  ;; label = @4
            local.get 4
            local.get 0
            i32.add
            local.set 6
            local.get 0
            i32.const 1
            i32.add
            local.tee 7
            local.set 0
            local.get 6
            i32.const 1
            i32.add
            i32.load8_u
            br_if 0 (;@4;)
          end
          local.get 5
          local.get 7
          i32.le_u
          br_if 2 (;@1;)
          local.get 7
          i32.eqz
          br_if 1 (;@2;)
          block  ;; label = @4
            local.get 4
            local.get 7
            call $realloc
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 7
            local.set 5
            br 3 (;@1;)
          end
          local.get 7
          i32.const 1
          call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
          unreachable
        end
        local.get 2
        i32.const 0
        i32.store offset=16
      end
      local.get 4
      call $free
      i32.const 1
      local.set 4
      i32.const 0
      local.set 5
    end
    local.get 1
    i32.const 28
    i32.add
    i32.load
    local.set 0
    local.get 1
    i32.load offset=24
    local.set 6
    local.get 2
    i32.const 28
    i32.add
    i32.const 0
    i32.store
    local.get 2
    i32.const 1053116
    i32.store offset=24
    local.get 2
    i64.const 1
    i64.store offset=12 align=4
    local.get 2
    i32.const 1055528
    i32.store offset=8
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 6
          local.get 0
          local.get 2
          i32.const 8
          i32.add
          call $_ZN4core3fmt5write17h8776c655b56f9e02E
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 3
            i32.const 255
            i32.and
            br_if 0 (;@4;)
            local.get 2
            i32.const 28
            i32.add
            i32.const 0
            i32.store
            local.get 2
            i32.const 1053116
            i32.store offset=24
            local.get 2
            i64.const 1
            i64.store offset=12 align=4
            local.get 2
            i32.const 1055624
            i32.store offset=8
            local.get 6
            local.get 0
            local.get 2
            i32.const 8
            i32.add
            call $_ZN4core3fmt5write17h8776c655b56f9e02E
            br_if 1 (;@3;)
          end
          i32.const 0
          local.set 0
          local.get 4
          i32.eqz
          br_if 2 (;@1;)
          local.get 5
          i32.eqz
          br_if 2 (;@1;)
          br 1 (;@2;)
        end
        i32.const 1
        local.set 0
        local.get 4
        i32.eqz
        br_if 1 (;@1;)
        local.get 5
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 4
      call $free
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN3std5alloc24default_alloc_error_hook17ha1b29e04979abe0bE (type 5) (param i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 1
    i32.store offset=4
    local.get 2
    local.get 0
    i32.store offset=12
    local.get 2
    local.get 2
    i32.const 12
    i32.add
    i32.store
    local.get 2
    i32.const 4
    i32.store8 offset=20
    local.get 2
    local.get 2
    i32.const 56
    i32.add
    i32.store offset=16
    local.get 2
    i32.const 52
    i32.add
    i32.const 1
    i32.store
    local.get 2
    i64.const 2
    i64.store offset=36 align=4
    local.get 2
    i32.const 1055948
    i32.store offset=32
    local.get 2
    local.get 2
    i32.store offset=48
    local.get 2
    i32.const 16
    i32.add
    i32.const 1054872
    local.get 2
    i32.const 32
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 0
    local.get 2
    i32.load8_u offset=20
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 255
        i32.and
        local.tee 0
        i32.const 4
        i32.eq
        br_if 1 (;@1;)
        local.get 0
        i32.const 3
        i32.ne
        br_if 1 (;@1;)
        local.get 2
        i32.const 24
        i32.add
        i32.load align=1
        local.tee 0
        i32.load
        local.get 0
        i32.load offset=4
        i32.load
        call_indirect (type 0)
        block  ;; label = @3
          local.get 0
          i32.load offset=4
          i32.load offset=4
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.load
          call $free
        end
        local.get 0
        call $free
        br 1 (;@1;)
      end
      local.get 3
      i32.const 255
      i32.and
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 2
      i32.const 24
      i32.add
      i32.load
      local.tee 0
      i32.load
      local.get 0
      i32.load offset=4
      i32.load
      call_indirect (type 0)
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        i32.load offset=4
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load
        call $free
      end
      local.get 2
      i32.load offset=24
      call $free
    end
    local.get 2
    i32.const 64
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h4f5e4ec6fd0d92cdE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    local.get 2
    i32.const 20
    i32.add
    i32.const 1
    i32.store
    local.get 2
    i32.const 12
    i32.add
    i32.const 1
    i32.store
    local.get 2
    local.get 0
    i32.const 12
    i32.add
    i32.store offset=16
    local.get 2
    local.get 0
    i32.const 8
    i32.add
    i32.store offset=8
    local.get 2
    i32.const 4
    i32.store offset=4
    local.get 2
    local.get 0
    i32.store
    local.get 1
    i32.const 28
    i32.add
    i32.load
    local.set 0
    local.get 1
    i32.load offset=24
    local.set 1
    local.get 2
    i32.const 24
    i32.add
    i32.const 20
    i32.add
    i32.const 3
    i32.store
    local.get 2
    i64.const 3
    i64.store offset=28 align=4
    local.get 2
    i32.const 1048968
    i32.store offset=24
    local.get 2
    local.get 2
    i32.store offset=40
    local.get 1
    local.get 0
    local.get 2
    i32.const 24
    i32.add
    call $_ZN4core3fmt5write17h8776c655b56f9e02E
    local.set 0
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core6option15Option$LT$T$GT$6unwrap17hef65611489e1fddeE (type 8) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 1055427
      i32.const 43
      i32.const 1056160
      call $_ZN4core9panicking5panic17h66d99d0b614b1c7cE
      unreachable
    end
    local.get 0
  )
  (func $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h8e8491797c8758c1E (type 11) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h924d75db94d5ef99E
    unreachable
  )
  (func $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h924d75db94d5ef99E (type 11) (param i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 0
    i32.const 20
    i32.add
    i32.load
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.const 4
            i32.add
            i32.load
            br_table 0 (;@4;) 1 (;@3;) 3 (;@1;)
          end
          local.get 4
          br_if 2 (;@1;)
          i32.const 1053116
          local.set 0
          i32.const 0
          local.set 4
          br 1 (;@2;)
        end
        local.get 4
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.tee 0
        i32.load offset=4
        local.set 4
        local.get 0
        i32.load
        local.set 0
      end
      local.get 3
      local.get 4
      i32.store offset=4
      local.get 3
      local.get 0
      i32.store
      local.get 3
      i32.const 1056228
      local.get 1
      i32.load offset=8
      local.get 2
      local.get 1
      i32.load8_u offset=16
      call $_ZN3std9panicking20rust_panic_with_hook17h80cadb720b17d11aE
      unreachable
    end
    local.get 3
    i32.const 0
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 1056208
    local.get 1
    i32.load offset=8
    local.get 2
    local.get 1
    i32.load8_u offset=16
    call $_ZN3std9panicking20rust_panic_with_hook17h80cadb720b17d11aE
    unreachable
  )
  (func $_ZN3std9panicking20rust_panic_with_hook17h80cadb720b17d11aE (type 12) (param i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 112
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    i32.const 1
    local.set 6
    i32.const 0
    i32.const 0
    i32.load offset=1059908
    local.tee 7
    i32.const 1
    i32.add
    i32.store offset=1059908
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        i32.load8_u offset=1059920
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        i32.load offset=1059928
        i32.const 1
        i32.add
        local.set 6
        br 1 (;@1;)
      end
      i32.const 0
      i32.const 1
      i32.store8 offset=1059920
    end
    i32.const 0
    local.get 6
    i32.store offset=1059928
    block  ;; label = @1
      block  ;; label = @2
        local.get 7
        i32.const 0
        i32.lt_s
        br_if 0 (;@2;)
        local.get 6
        i32.const 2
        i32.gt_u
        br_if 0 (;@2;)
        i32.const 0
        i32.load offset=1059904
        local.tee 7
        i32.const -1
        i32.gt_s
        br_if 1 (;@1;)
        local.get 5
        i32.const 40
        i32.add
        i32.const 20
        i32.add
        i32.const 1
        i32.store
        local.get 5
        i32.const 80
        i32.add
        i32.const 20
        i32.add
        i32.const 0
        i32.store
        local.get 5
        i64.const 2
        i64.store offset=44 align=4
        local.get 5
        i32.const 1055492
        i32.store offset=40
        local.get 5
        i32.const 5
        i32.store offset=68
        local.get 5
        i32.const 1053116
        i32.store offset=96
        local.get 5
        i64.const 1
        i64.store offset=84 align=4
        local.get 5
        i32.const 1057524
        i32.store offset=80
        local.get 5
        local.get 5
        i32.const 64
        i32.add
        i32.store offset=56
        local.get 5
        local.get 5
        i32.const 80
        i32.add
        i32.store offset=64
        local.get 5
        local.get 5
        i32.const 104
        i32.add
        local.get 5
        i32.const 40
        i32.add
        call $_ZN3std2io5Write9write_fmt17h49c2f05193f87c52E
        i64.store offset=32
        local.get 5
        i32.const 32
        i32.add
        call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17hb77b520a01bfdb66E
        call $_ZN3std3sys4wasi14abort_internal17h6f2ef412fa287f15E
        unreachable
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 6
          i32.const 2
          i32.gt_u
          br_if 0 (;@3;)
          local.get 5
          local.get 4
          i32.store8 offset=56
          local.get 5
          local.get 3
          i32.store offset=52
          local.get 5
          local.get 2
          i32.store offset=48
          local.get 5
          i32.const 1055632
          i32.store offset=44
          local.get 5
          i32.const 1053116
          i32.store offset=40
          local.get 5
          i32.const 15
          i32.store offset=36
          local.get 5
          local.get 5
          i32.const 40
          i32.add
          i32.store offset=32
          local.get 5
          i32.const 4
          i32.store8 offset=68
          local.get 5
          local.get 5
          i32.const 104
          i32.add
          i32.store offset=64
          local.get 5
          i32.const 100
          i32.add
          i32.const 1
          i32.store
          local.get 5
          i64.const 2
          i64.store offset=84 align=4
          local.get 5
          i32.const 1056360
          i32.store offset=80
          local.get 5
          local.get 5
          i32.const 32
          i32.add
          i32.store offset=96
          local.get 5
          i32.const 64
          i32.add
          i32.const 1054872
          local.get 5
          i32.const 80
          i32.add
          call $_ZN4core3fmt5write17h8776c655b56f9e02E
          local.set 7
          local.get 5
          i32.load8_u offset=68
          local.set 0
          block  ;; label = @4
            local.get 7
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.const 255
            i32.and
            local.tee 7
            i32.const 4
            i32.eq
            br_if 2 (;@2;)
            local.get 7
            i32.const 3
            i32.ne
            br_if 2 (;@2;)
            local.get 5
            i32.const 72
            i32.add
            i32.load align=1
            local.tee 7
            i32.load
            local.get 7
            i32.load offset=4
            i32.load
            call_indirect (type 0)
            block  ;; label = @5
              local.get 7
              i32.load offset=4
              i32.load offset=4
              i32.eqz
              br_if 0 (;@5;)
              local.get 7
              i32.load
              call $free
            end
            local.get 7
            call $free
            call $_ZN3std3sys4wasi14abort_internal17h6f2ef412fa287f15E
            unreachable
          end
          local.get 0
          i32.const 255
          i32.and
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          local.get 5
          i32.const 72
          i32.add
          i32.load
          local.tee 7
          i32.load
          local.get 7
          i32.load offset=4
          i32.load
          call_indirect (type 0)
          block  ;; label = @4
            local.get 7
            i32.load offset=4
            i32.load offset=4
            i32.eqz
            br_if 0 (;@4;)
            local.get 7
            i32.load
            call $free
          end
          local.get 5
          i32.load offset=72
          call $free
          call $_ZN3std3sys4wasi14abort_internal17h6f2ef412fa287f15E
          unreachable
        end
        local.get 5
        i32.const 4
        i32.store8 offset=44
        local.get 5
        local.get 5
        i32.const 104
        i32.add
        i32.store offset=40
        local.get 5
        i32.const 100
        i32.add
        i32.const 0
        i32.store
        local.get 5
        i32.const 1053116
        i32.store offset=96
        local.get 5
        i64.const 1
        i64.store offset=84 align=4
        local.get 5
        i32.const 1056300
        i32.store offset=80
        local.get 5
        i32.const 40
        i32.add
        i32.const 1054872
        local.get 5
        i32.const 80
        i32.add
        call $_ZN4core3fmt5write17h8776c655b56f9e02E
        local.set 7
        local.get 5
        i32.load8_u offset=44
        local.set 0
        block  ;; label = @3
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.const 255
          i32.and
          local.tee 7
          i32.const 4
          i32.eq
          br_if 1 (;@2;)
          local.get 7
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          local.get 5
          i32.const 48
          i32.add
          i32.load align=1
          local.tee 7
          i32.load
          local.get 7
          i32.load offset=4
          i32.load
          call_indirect (type 0)
          block  ;; label = @4
            local.get 7
            i32.load offset=4
            i32.load offset=4
            i32.eqz
            br_if 0 (;@4;)
            local.get 7
            i32.load
            call $free
          end
          local.get 7
          call $free
          call $_ZN3std3sys4wasi14abort_internal17h6f2ef412fa287f15E
          unreachable
        end
        local.get 0
        i32.const 255
        i32.and
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 5
        i32.const 48
        i32.add
        i32.load
        local.tee 7
        i32.load
        local.get 7
        i32.load offset=4
        i32.load
        call_indirect (type 0)
        block  ;; label = @3
          local.get 7
          i32.load offset=4
          i32.load offset=4
          i32.eqz
          br_if 0 (;@3;)
          local.get 7
          i32.load
          call $free
        end
        local.get 5
        i32.load offset=48
        call $free
      end
      call $_ZN3std3sys4wasi14abort_internal17h6f2ef412fa287f15E
      unreachable
    end
    i32.const 1
    local.set 2
    i32.const 0
    local.get 7
    i32.const 1
    i32.add
    i32.store offset=1059904
    local.get 5
    i32.const 8
    i32.add
    local.get 0
    local.get 1
    i32.load offset=16
    call_indirect (type 5)
    local.get 5
    i32.load offset=12
    local.set 8
    local.get 5
    i32.load offset=8
    local.set 9
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.load8_u offset=1059920
          br_if 0 (;@3;)
          i32.const 0
          i32.const 1
          i32.store8 offset=1059920
          i32.const 0
          i32.const 0
          i32.store offset=1059928
          br 1 (;@2;)
        end
        i32.const 0
        i32.load offset=1059928
        i32.const 1
        i32.gt_u
        br_if 1 (;@1;)
      end
      i32.const 0
      local.set 2
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              i32.const 0
              i32.load offset=1059892
              br_table 3 (;@2;) 4 (;@1;) 1 (;@4;) 2 (;@3;) 0 (;@5;)
            end
            i32.const 1053455
            i32.const 40
            i32.const 1055028
            call $_ZN4core9panicking5panic17h66d99d0b614b1c7cE
            unreachable
          end
          i32.const 1
          local.set 2
          br 2 (;@1;)
        end
        i32.const 2
        local.set 2
        br 1 (;@1;)
      end
      local.get 5
      i32.const 80
      i32.add
      i32.const 1053353
      i32.const 14
      call $_ZN70_$LT$$RF$str$u20$as$u20$std..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17he72994ed41181ff3E
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 5
            i32.load offset=80
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            i32.const 92
            i32.add
            i32.load
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            i32.const 88
            i32.add
            i32.load
            local.tee 7
            i32.eqz
            br_if 1 (;@3;)
            local.get 7
            call $free
            br 1 (;@3;)
          end
          local.get 5
          i32.const 88
          i32.add
          i32.load
          local.set 10
          i32.const 0
          local.set 0
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 5
                i32.load offset=84
                local.tee 11
                call $getenv
                local.tee 12
                i32.eqz
                br_if 0 (;@6;)
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 12
                    i32.load8_u
                    br_if 0 (;@8;)
                    i32.const 1
                    local.set 0
                    i32.const 0
                    local.set 13
                    br 1 (;@7;)
                  end
                  local.get 12
                  i32.const 1
                  i32.add
                  local.set 2
                  i32.const 0
                  local.set 7
                  loop  ;; label = @8
                    local.get 2
                    local.get 7
                    i32.add
                    local.set 0
                    local.get 7
                    i32.const 1
                    i32.add
                    local.tee 1
                    local.set 7
                    local.get 0
                    i32.load8_u
                    br_if 0 (;@8;)
                  end
                  i32.const 0
                  local.set 13
                  local.get 1
                  i32.const 0
                  i32.lt_s
                  br_if 2 (;@5;)
                  i32.const 1
                  local.set 0
                  local.get 1
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 1
                  i32.const 1
                  call $__rust_alloc
                  local.tee 0
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 1
                  local.set 13
                end
                local.get 0
                local.get 12
                local.get 13
                call $memcpy
                drop
              end
              local.get 11
              i32.const 0
              i32.store8
              block  ;; label = @6
                local.get 10
                i32.eqz
                br_if 0 (;@6;)
                local.get 11
                call $free
              end
              local.get 0
              i32.eqz
              br_if 2 (;@3;)
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 13
                    i32.const -1
                    i32.add
                    br_table 0 (;@8;) 1 (;@7;) 1 (;@7;) 2 (;@6;) 1 (;@7;)
                  end
                  local.get 0
                  i32.load8_u
                  local.set 7
                  local.get 0
                  call $free
                  local.get 7
                  i32.const 48
                  i32.eq
                  br_if 4 (;@3;)
                  i32.const 1
                  local.set 7
                  i32.const 0
                  local.set 2
                  br 5 (;@2;)
                end
                i32.const 1
                local.set 7
                i32.const 0
                local.set 2
                local.get 13
                i32.eqz
                br_if 4 (;@2;)
                local.get 0
                call $free
                br 4 (;@2;)
              end
              local.get 0
              i32.load align=1
              local.set 7
              local.get 0
              call $free
              i32.const 2
              i32.const 1
              local.get 7
              i32.const 1819047270
              i32.eq
              local.tee 2
              select
              local.set 7
              br 3 (;@2;)
            end
            call $_ZN5alloc7raw_vec17capacity_overflow17h0f57087aa53660d1E
            unreachable
          end
          local.get 1
          i32.const 1
          call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
          unreachable
        end
        i32.const 3
        local.set 7
        i32.const 2
        local.set 2
      end
      i32.const 0
      local.get 7
      i32.store offset=1059892
    end
    local.get 5
    local.get 3
    i32.store offset=20
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 9
          local.get 8
          i32.load offset=12
          local.tee 7
          call_indirect (type 3)
          i64.const -8867930603987265711
          i64.eq
          br_if 0 (;@3;)
          local.get 9
          local.get 7
          call_indirect (type 3)
          i64.const 92758131779283711
          i64.eq
          br_if 1 (;@2;)
          i32.const 12
          local.set 7
          local.get 5
          i32.const 1055992
          i32.store offset=24
          br 2 (;@1;)
        end
        local.get 5
        local.get 9
        i32.load
        i32.store offset=24
        local.get 9
        i32.load offset=4
        local.set 7
        br 1 (;@1;)
      end
      local.get 9
      i32.const 8
      i32.add
      i32.load
      local.set 7
      local.get 5
      local.get 9
      i32.load
      i32.store offset=24
    end
    local.get 5
    local.get 7
    i32.store offset=28
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  i32.const 0
                  i32.load offset=1059912
                  br_if 0 (;@7;)
                  i32.const 0
                  i32.const -1
                  i32.store offset=1059912
                  block  ;; label = @8
                    i32.const 0
                    i32.load offset=1059916
                    local.tee 7
                    br_if 0 (;@8;)
                    i32.const 0
                    i32.const 0
                    local.get 7
                    call $_ZN3std6thread6Thread3new17hab16636756d5a840E
                    local.tee 7
                    i32.store offset=1059916
                  end
                  local.get 7
                  local.get 7
                  i32.load
                  local.tee 0
                  i32.const 1
                  i32.add
                  i32.store
                  local.get 0
                  i32.const -1
                  i32.le_s
                  br_if 1 (;@6;)
                  i32.const 0
                  local.set 0
                  i32.const 0
                  i32.const 0
                  i32.load offset=1059912
                  i32.const 1
                  i32.add
                  i32.store offset=1059912
                  block  ;; label = @8
                    local.get 7
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 7
                    i32.const 20
                    i32.add
                    i32.load
                    i32.const -1
                    i32.add
                    local.set 1
                    local.get 7
                    i32.load offset=16
                    local.set 0
                  end
                  local.get 5
                  local.get 1
                  i32.const 9
                  local.get 0
                  select
                  i32.store offset=36
                  local.get 5
                  local.get 0
                  i32.const 1056004
                  local.get 0
                  select
                  i32.store offset=32
                  local.get 5
                  i32.const 40
                  i32.add
                  i32.const 20
                  i32.add
                  i32.const 16
                  i32.store
                  local.get 5
                  i32.const 52
                  i32.add
                  i32.const 4
                  i32.store
                  local.get 5
                  i32.const 4
                  i32.store offset=44
                  local.get 5
                  local.get 5
                  i32.const 20
                  i32.add
                  i32.store offset=56
                  local.get 5
                  local.get 5
                  i32.const 24
                  i32.add
                  i32.store offset=48
                  local.get 5
                  local.get 5
                  i32.const 32
                  i32.add
                  i32.store offset=40
                  local.get 5
                  i32.const 4
                  i32.store8 offset=68
                  local.get 5
                  local.get 5
                  i32.const 104
                  i32.add
                  i32.store offset=64
                  local.get 5
                  i32.const 80
                  i32.add
                  i32.const 20
                  i32.add
                  i32.const 3
                  i32.store
                  local.get 5
                  i64.const 4
                  i64.store offset=84 align=4
                  local.get 5
                  i32.const 1056040
                  i32.store offset=80
                  local.get 5
                  local.get 5
                  i32.const 40
                  i32.add
                  i32.store offset=96
                  local.get 5
                  i32.const 64
                  i32.add
                  i32.const 1054872
                  local.get 5
                  i32.const 80
                  i32.add
                  call $_ZN4core3fmt5write17h8776c655b56f9e02E
                  local.set 0
                  local.get 5
                  i32.load8_u offset=68
                  local.set 1
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 1
                      i32.const 255
                      i32.and
                      local.tee 0
                      i32.const 4
                      i32.eq
                      br_if 1 (;@8;)
                      local.get 0
                      i32.const 3
                      i32.ne
                      br_if 1 (;@8;)
                      local.get 5
                      i32.const 72
                      i32.add
                      i32.load align=1
                      local.tee 0
                      i32.load
                      local.get 0
                      i32.load offset=4
                      i32.load
                      call_indirect (type 0)
                      block  ;; label = @10
                        local.get 0
                        i32.load offset=4
                        i32.load offset=4
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 0
                        i32.load
                        call $free
                      end
                      local.get 0
                      call $free
                      br 1 (;@8;)
                    end
                    local.get 1
                    i32.const 255
                    i32.and
                    i32.const 3
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 72
                    i32.add
                    i32.load
                    local.tee 0
                    i32.load
                    local.get 0
                    i32.load offset=4
                    i32.load
                    call_indirect (type 0)
                    block  ;; label = @9
                      local.get 0
                      i32.load offset=4
                      i32.load offset=4
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 0
                      i32.load
                      call $free
                    end
                    local.get 5
                    i32.load offset=72
                    call $free
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 2
                        br_table 0 (;@10;) 1 (;@9;) 2 (;@8;) 0 (;@10;)
                      end
                      i32.const 0
                      i32.load8_u offset=1059900
                      local.set 0
                      i32.const 0
                      i32.const 1
                      i32.store8 offset=1059900
                      local.get 5
                      local.get 0
                      i32.store8 offset=40
                      local.get 0
                      br_if 4 (;@5;)
                      local.get 5
                      i32.const 17
                      i32.store offset=68
                      local.get 5
                      i32.const 0
                      i32.store8 offset=104
                      local.get 5
                      local.get 5
                      i32.const 104
                      i32.add
                      i32.store offset=64
                      local.get 5
                      i32.const 4
                      i32.store8 offset=44
                      local.get 5
                      local.get 5
                      i32.const 104
                      i32.add
                      i32.store offset=40
                      local.get 5
                      i32.const 100
                      i32.add
                      i32.const 1
                      i32.store
                      local.get 5
                      i64.const 1
                      i64.store offset=84 align=4
                      local.get 5
                      i32.const 1053444
                      i32.store offset=80
                      local.get 5
                      local.get 5
                      i32.const 64
                      i32.add
                      i32.store offset=96
                      local.get 5
                      i32.const 40
                      i32.add
                      i32.const 1054872
                      local.get 5
                      i32.const 80
                      i32.add
                      call $_ZN4core3fmt5write17h8776c655b56f9e02E
                      local.set 0
                      local.get 5
                      i32.load8_u offset=44
                      local.set 1
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 0
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 1
                          i32.const 255
                          i32.and
                          local.tee 0
                          i32.const 4
                          i32.eq
                          br_if 1 (;@10;)
                          i32.const 0
                          i32.const 0
                          i32.store8 offset=1059900
                          local.get 0
                          i32.const 3
                          i32.ne
                          br_if 10 (;@1;)
                          local.get 5
                          i32.const 48
                          i32.add
                          i32.load align=1
                          local.tee 0
                          i32.load
                          local.get 0
                          i32.load offset=4
                          i32.load
                          call_indirect (type 0)
                          local.get 0
                          i32.load offset=4
                          i32.load offset=4
                          br_if 8 (;@3;)
                          br 9 (;@2;)
                        end
                        local.get 1
                        i32.const 255
                        i32.and
                        i32.const 3
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 5
                        i32.const 48
                        i32.add
                        i32.load
                        local.tee 0
                        i32.load
                        local.get 0
                        i32.load offset=4
                        i32.load
                        call_indirect (type 0)
                        block  ;; label = @11
                          local.get 0
                          i32.load offset=4
                          i32.load offset=4
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 0
                          i32.load
                          call $free
                        end
                        local.get 5
                        i32.load offset=48
                        call $free
                      end
                      i32.const 0
                      i32.const 0
                      i32.store8 offset=1059900
                      br 8 (;@1;)
                    end
                    i32.const 0
                    i32.load8_u offset=1059900
                    local.set 0
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1059900
                    local.get 5
                    local.get 0
                    i32.store8 offset=40
                    local.get 0
                    br_if 4 (;@4;)
                    local.get 5
                    i32.const 17
                    i32.store offset=68
                    local.get 5
                    i32.const 1
                    i32.store8 offset=104
                    local.get 5
                    local.get 5
                    i32.const 104
                    i32.add
                    i32.store offset=64
                    local.get 5
                    i32.const 4
                    i32.store8 offset=44
                    local.get 5
                    local.get 5
                    i32.const 104
                    i32.add
                    i32.store offset=40
                    local.get 5
                    i32.const 100
                    i32.add
                    i32.const 1
                    i32.store
                    local.get 5
                    i64.const 1
                    i64.store offset=84 align=4
                    local.get 5
                    i32.const 1053444
                    i32.store offset=80
                    local.get 5
                    local.get 5
                    i32.const 64
                    i32.add
                    i32.store offset=96
                    local.get 5
                    i32.const 40
                    i32.add
                    i32.const 1054872
                    local.get 5
                    i32.const 80
                    i32.add
                    call $_ZN4core3fmt5write17h8776c655b56f9e02E
                    local.set 0
                    local.get 5
                    i32.load8_u offset=44
                    local.set 1
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 1
                        i32.const 255
                        i32.and
                        local.tee 0
                        i32.const 4
                        i32.eq
                        br_if 1 (;@9;)
                        i32.const 0
                        i32.const 0
                        i32.store8 offset=1059900
                        local.get 0
                        i32.const 3
                        i32.ne
                        br_if 9 (;@1;)
                        local.get 5
                        i32.const 48
                        i32.add
                        i32.load align=1
                        local.tee 0
                        i32.load
                        local.get 0
                        i32.load offset=4
                        i32.load
                        call_indirect (type 0)
                        local.get 0
                        i32.load offset=4
                        i32.load offset=4
                        br_if 7 (;@3;)
                        br 8 (;@2;)
                      end
                      local.get 1
                      i32.const 255
                      i32.and
                      i32.const 3
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 48
                      i32.add
                      i32.load
                      local.tee 0
                      i32.load
                      local.get 0
                      i32.load offset=4
                      i32.load
                      call_indirect (type 0)
                      block  ;; label = @10
                        local.get 0
                        i32.load offset=4
                        i32.load offset=4
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 0
                        i32.load
                        call $free
                      end
                      local.get 5
                      i32.load offset=48
                      call $free
                    end
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1059900
                    br 7 (;@1;)
                  end
                  i32.const 0
                  i32.load8_u offset=1059848
                  local.set 0
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1059848
                  local.get 0
                  i32.eqz
                  br_if 6 (;@1;)
                  local.get 5
                  i32.const 4
                  i32.store8 offset=44
                  local.get 5
                  local.get 5
                  i32.const 104
                  i32.add
                  i32.store offset=40
                  local.get 5
                  i32.const 100
                  i32.add
                  i32.const 0
                  i32.store
                  local.get 5
                  i32.const 1053116
                  i32.store offset=96
                  local.get 5
                  i64.const 1
                  i64.store offset=84 align=4
                  local.get 5
                  i32.const 1056152
                  i32.store offset=80
                  local.get 5
                  i32.const 40
                  i32.add
                  i32.const 1054872
                  local.get 5
                  i32.const 80
                  i32.add
                  call $_ZN4core3fmt5write17h8776c655b56f9e02E
                  local.set 0
                  local.get 5
                  i32.load8_u offset=44
                  local.set 1
                  block  ;; label = @8
                    local.get 0
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 255
                    i32.and
                    local.tee 0
                    i32.const 4
                    i32.eq
                    br_if 7 (;@1;)
                    local.get 0
                    i32.const 3
                    i32.ne
                    br_if 7 (;@1;)
                    local.get 5
                    i32.const 48
                    i32.add
                    i32.load align=1
                    local.tee 0
                    i32.load
                    local.get 0
                    i32.load offset=4
                    i32.load
                    call_indirect (type 0)
                    local.get 0
                    i32.load offset=4
                    i32.load offset=4
                    br_if 5 (;@3;)
                    br 6 (;@2;)
                  end
                  local.get 1
                  i32.const 255
                  i32.and
                  i32.const 3
                  i32.ne
                  br_if 6 (;@1;)
                  local.get 5
                  i32.const 48
                  i32.add
                  i32.load
                  local.tee 0
                  i32.load
                  local.get 0
                  i32.load offset=4
                  i32.load
                  call_indirect (type 0)
                  block  ;; label = @8
                    local.get 0
                    i32.load offset=4
                    i32.load offset=4
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 0
                    i32.load
                    call $free
                  end
                  local.get 5
                  i32.load offset=48
                  call $free
                  br 6 (;@1;)
                end
                i32.const 1053428
                i32.const 16
                local.get 5
                i32.const 104
                i32.add
                i32.const 1055820
                i32.const 1055740
                call $_ZN4core6result13unwrap_failed17h1b005d733a1499a6E
                unreachable
              end
              unreachable
              unreachable
            end
            local.get 5
            i32.const 100
            i32.add
            i32.const 0
            i32.store
            local.get 5
            i32.const 96
            i32.add
            i32.const 1053116
            i32.store
            local.get 5
            i64.const 1
            i64.store offset=84 align=4
            local.get 5
            i32.const 1057332
            i32.store offset=80
            local.get 5
            i32.const 40
            i32.add
            local.get 5
            i32.const 80
            i32.add
            call $_ZN4core9panicking13assert_failed17hfcfdb11ed0fde9ebE
            unreachable
          end
          local.get 5
          i32.const 100
          i32.add
          i32.const 0
          i32.store
          local.get 5
          i32.const 96
          i32.add
          i32.const 1053116
          i32.store
          local.get 5
          i64.const 1
          i64.store offset=84 align=4
          local.get 5
          i32.const 1057332
          i32.store offset=80
          local.get 5
          i32.const 40
          i32.add
          local.get 5
          i32.const 80
          i32.add
          call $_ZN4core9panicking13assert_failed17hfcfdb11ed0fde9ebE
          unreachable
        end
        local.get 0
        i32.load
        call $free
      end
      local.get 0
      call $free
    end
    block  ;; label = @1
      local.get 7
      i32.eqz
      br_if 0 (;@1;)
      local.get 7
      local.get 7
      i32.load
      local.tee 0
      i32.const -1
      i32.add
      i32.store
      local.get 0
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 7
      call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17ha7cc5cab043eefa7E
    end
    i32.const 0
    i32.const 0
    i32.load offset=1059904
    i32.const -1
    i32.add
    i32.store offset=1059904
    block  ;; label = @1
      local.get 6
      i32.const 1
      i32.gt_u
      br_if 0 (;@1;)
      local.get 4
      i32.eqz
      br_if 0 (;@1;)
      call $rust_panic
      unreachable
    end
    local.get 5
    i32.const 100
    i32.add
    i32.const 0
    i32.store
    local.get 5
    i32.const 1053116
    i32.store offset=96
    local.get 5
    i64.const 1
    i64.store offset=84 align=4
    local.get 5
    i32.const 1056420
    i32.store offset=80
    local.get 5
    local.get 5
    i32.const 104
    i32.add
    local.get 5
    i32.const 80
    i32.add
    call $_ZN3std2io5Write9write_fmt17h49c2f05193f87c52E
    i64.store offset=40
    local.get 5
    i32.const 40
    i32.add
    call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17hb77b520a01bfdb66E
    call $_ZN3std3sys4wasi14abort_internal17h6f2ef412fa287f15E
    unreachable
  )
  (func $_ZN4core3ptr70drop_in_place$LT$std..panicking..begin_panic_handler..PanicPayload$GT$17h03f77c6f266c0a69E (type 0) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 8
      i32.add
      i32.load
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      call $free
    end
  )
  (func $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h57bbe70334078b97E (type 5) (param i32 i32)
    (local i32 i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 1
    i32.const 4
    i32.add
    local.set 3
    block  ;; label = @1
      local.get 1
      i32.load offset=4
      br_if 0 (;@1;)
      local.get 1
      i32.load
      local.set 4
      local.get 2
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      local.tee 5
      i32.const 0
      i32.store
      local.get 2
      i64.const 1
      i64.store offset=8
      local.get 2
      local.get 2
      i32.const 8
      i32.add
      i32.store offset=20
      local.get 2
      i32.const 24
      i32.add
      i32.const 16
      i32.add
      local.get 4
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      i32.const 24
      i32.add
      i32.const 8
      i32.add
      local.get 4
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      local.get 4
      i64.load align=4
      i64.store offset=24
      local.get 2
      i32.const 20
      i32.add
      i32.const 1056452
      local.get 2
      i32.const 24
      i32.add
      call $_ZN4core3fmt5write17h8776c655b56f9e02E
      drop
      local.get 3
      i32.const 8
      i32.add
      local.get 5
      i32.load
      i32.store
      local.get 3
      local.get 2
      i64.load offset=8
      i64.store align=4
    end
    local.get 2
    i32.const 24
    i32.add
    i32.const 8
    i32.add
    local.tee 4
    local.get 3
    i32.const 8
    i32.add
    i32.load
    i32.store
    local.get 1
    i32.const 12
    i32.add
    i32.const 0
    i32.store
    local.get 3
    i64.load align=4
    local.set 6
    local.get 1
    i64.const 1
    i64.store offset=4 align=4
    local.get 2
    local.get 6
    i64.store offset=24
    block  ;; label = @1
      i32.const 12
      call $malloc
      local.tee 1
      br_if 0 (;@1;)
      i32.const 12
      i32.const 4
      call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
      unreachable
    end
    local.get 1
    local.get 2
    i64.load offset=24
    i64.store align=4
    local.get 1
    i32.const 8
    i32.add
    local.get 4
    i32.load
    i32.store
    local.get 0
    i32.const 1056176
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17h04071562b34d7f4eE (type 5) (param i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 1
    i32.const 4
    i32.add
    local.set 3
    block  ;; label = @1
      local.get 1
      i32.load offset=4
      br_if 0 (;@1;)
      local.get 1
      i32.load
      local.set 1
      local.get 2
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      local.tee 4
      i32.const 0
      i32.store
      local.get 2
      i64.const 1
      i64.store offset=8
      local.get 2
      local.get 2
      i32.const 8
      i32.add
      i32.store offset=20
      local.get 2
      i32.const 24
      i32.add
      i32.const 16
      i32.add
      local.get 1
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      i32.const 24
      i32.add
      i32.const 8
      i32.add
      local.get 1
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      local.get 1
      i64.load align=4
      i64.store offset=24
      local.get 2
      i32.const 20
      i32.add
      i32.const 1056452
      local.get 2
      i32.const 24
      i32.add
      call $_ZN4core3fmt5write17h8776c655b56f9e02E
      drop
      local.get 3
      i32.const 8
      i32.add
      local.get 4
      i32.load
      i32.store
      local.get 3
      local.get 2
      i64.load offset=8
      i64.store align=4
    end
    local.get 0
    i32.const 1056176
    i32.store offset=4
    local.get 0
    local.get 3
    i32.store
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hf6be660204d18b5cE (type 3) (param i32) (result i64)
    i64.const 92758131779283711
  )
  (func $_ZN93_$LT$std..panicking..begin_panic_handler..StrPanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h9be14b2fbf7fec1bE (type 5) (param i32 i32)
    (local i32 i32)
    local.get 1
    i32.load offset=4
    local.set 2
    local.get 1
    i32.load
    local.set 3
    block  ;; label = @1
      i32.const 8
      call $malloc
      local.tee 1
      br_if 0 (;@1;)
      i32.const 8
      i32.const 4
      call $_ZN5alloc5alloc18handle_alloc_error17hc1d2868d8bd0f014E
      unreachable
    end
    local.get 1
    local.get 2
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store
    local.get 0
    i32.const 1056192
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func $_ZN93_$LT$std..panicking..begin_panic_handler..StrPanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17h4dd4073ca94e5808E (type 5) (param i32 i32)
    local.get 0
    i32.const 1056192
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hff14a3216932950dE (type 3) (param i32) (result i64)
    i64.const -8867930603987265711
  )
  (func $main (type 2) (param i32 i32) (result i32)
    call $__original_main
  )
  (func $malloc (type 8) (param i32) (result i32)
    local.get 0
    call $dlmalloc
  )
  (func $dlmalloc (type 8) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block  ;; label = @1
      i32.const 0
      i32.load offset=1059956
      br_if 0 (;@1;)
      i32.const 0
      call $sbrk
      i32.const 1060480
      i32.sub
      local.tee 2
      i32.const 89
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 0
      local.set 3
      block  ;; label = @2
        i32.const 0
        i32.load offset=1060404
        local.tee 4
        br_if 0 (;@2;)
        i32.const 0
        i64.const -1
        i64.store offset=1060416 align=4
        i32.const 0
        i64.const 281474976776192
        i64.store offset=1060408 align=4
        i32.const 0
        local.get 1
        i32.const 8
        i32.add
        i32.const -16
        i32.and
        i32.const 1431655768
        i32.xor
        local.tee 4
        i32.store offset=1060404
        i32.const 0
        i32.const 0
        i32.store offset=1060424
        i32.const 0
        i32.const 0
        i32.store offset=1060376
      end
      i32.const 0
      local.get 2
      i32.store offset=1060384
      i32.const 0
      i32.const 1060480
      i32.store offset=1060380
      i32.const 0
      i32.const 1060480
      i32.store offset=1059948
      i32.const 0
      local.get 4
      i32.store offset=1059968
      i32.const 0
      i32.const -1
      i32.store offset=1059964
      loop  ;; label = @2
        local.get 3
        i32.const 1059980
        i32.add
        local.get 3
        i32.const 1059972
        i32.add
        local.tee 4
        i32.store
        local.get 3
        i32.const 1059984
        i32.add
        local.get 4
        i32.store
        local.get 3
        i32.const 8
        i32.add
        local.tee 3
        i32.const 256
        i32.ne
        br_if 0 (;@2;)
      end
      i32.const 0
      i32.const 1060488
      i32.sub
      i32.const 15
      i32.and
      i32.const 0
      i32.const 1060488
      i32.const 15
      i32.and
      select
      local.tee 3
      i32.const 1060484
      i32.add
      local.get 2
      local.get 3
      i32.sub
      i32.const -56
      i32.add
      local.tee 4
      i32.const 1
      i32.or
      i32.store
      i32.const 0
      i32.const 0
      i32.load offset=1060420
      i32.store offset=1059960
      i32.const 0
      local.get 3
      i32.const 1060480
      i32.add
      i32.store offset=1059956
      i32.const 0
      local.get 4
      i32.store offset=1059944
      local.get 2
      i32.const 1060428
      i32.add
      i32.const 56
      i32.store
    end
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 0
                            i32.const 236
                            i32.gt_u
                            br_if 0 (;@12;)
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=1059932
                              local.tee 5
                              i32.const 16
                              local.get 0
                              i32.const 19
                              i32.add
                              i32.const -16
                              i32.and
                              local.get 0
                              i32.const 11
                              i32.lt_u
                              select
                              local.tee 2
                              i32.const 3
                              i32.shr_u
                              local.tee 4
                              i32.shr_u
                              local.tee 3
                              i32.const 3
                              i32.and
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 3
                              i32.const 1
                              i32.and
                              local.get 4
                              i32.or
                              i32.const 1
                              i32.xor
                              local.tee 2
                              i32.const 3
                              i32.shl
                              local.tee 6
                              i32.const 1059980
                              i32.add
                              i32.load
                              local.tee 4
                              i32.const 8
                              i32.add
                              local.set 3
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 4
                                  i32.load offset=8
                                  local.tee 0
                                  local.get 6
                                  i32.const 1059972
                                  i32.add
                                  local.tee 6
                                  i32.ne
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.get 5
                                  i32.const -2
                                  local.get 2
                                  i32.rotl
                                  i32.and
                                  i32.store offset=1059932
                                  br 1 (;@14;)
                                end
                                i32.const 0
                                i32.load offset=1059948
                                local.get 0
                                i32.gt_u
                                drop
                                local.get 6
                                local.get 0
                                i32.store offset=8
                                local.get 0
                                local.get 6
                                i32.store offset=12
                              end
                              local.get 4
                              local.get 2
                              i32.const 3
                              i32.shl
                              local.tee 0
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 4
                              local.get 0
                              i32.add
                              local.tee 4
                              local.get 4
                              i32.load offset=4
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              br 12 (;@1;)
                            end
                            local.get 2
                            i32.const 0
                            i32.load offset=1059940
                            local.tee 7
                            i32.le_u
                            br_if 1 (;@11;)
                            block  ;; label = @13
                              local.get 3
                              i32.eqz
                              br_if 0 (;@13;)
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 3
                                  local.get 4
                                  i32.shl
                                  i32.const 2
                                  local.get 4
                                  i32.shl
                                  local.tee 3
                                  i32.const 0
                                  local.get 3
                                  i32.sub
                                  i32.or
                                  i32.and
                                  local.tee 3
                                  i32.const 0
                                  local.get 3
                                  i32.sub
                                  i32.and
                                  i32.const -1
                                  i32.add
                                  local.tee 3
                                  local.get 3
                                  i32.const 12
                                  i32.shr_u
                                  i32.const 16
                                  i32.and
                                  local.tee 3
                                  i32.shr_u
                                  local.tee 4
                                  i32.const 5
                                  i32.shr_u
                                  i32.const 8
                                  i32.and
                                  local.tee 0
                                  local.get 3
                                  i32.or
                                  local.get 4
                                  local.get 0
                                  i32.shr_u
                                  local.tee 3
                                  i32.const 2
                                  i32.shr_u
                                  i32.const 4
                                  i32.and
                                  local.tee 4
                                  i32.or
                                  local.get 3
                                  local.get 4
                                  i32.shr_u
                                  local.tee 3
                                  i32.const 1
                                  i32.shr_u
                                  i32.const 2
                                  i32.and
                                  local.tee 4
                                  i32.or
                                  local.get 3
                                  local.get 4
                                  i32.shr_u
                                  local.tee 3
                                  i32.const 1
                                  i32.shr_u
                                  i32.const 1
                                  i32.and
                                  local.tee 4
                                  i32.or
                                  local.get 3
                                  local.get 4
                                  i32.shr_u
                                  i32.add
                                  local.tee 0
                                  i32.const 3
                                  i32.shl
                                  local.tee 6
                                  i32.const 1059980
                                  i32.add
                                  i32.load
                                  local.tee 4
                                  i32.load offset=8
                                  local.tee 3
                                  local.get 6
                                  i32.const 1059972
                                  i32.add
                                  local.tee 6
                                  i32.ne
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.get 5
                                  i32.const -2
                                  local.get 0
                                  i32.rotl
                                  i32.and
                                  local.tee 5
                                  i32.store offset=1059932
                                  br 1 (;@14;)
                                end
                                i32.const 0
                                i32.load offset=1059948
                                local.get 3
                                i32.gt_u
                                drop
                                local.get 6
                                local.get 3
                                i32.store offset=8
                                local.get 3
                                local.get 6
                                i32.store offset=12
                              end
                              local.get 4
                              i32.const 8
                              i32.add
                              local.set 3
                              local.get 4
                              local.get 2
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 4
                              local.get 0
                              i32.const 3
                              i32.shl
                              local.tee 0
                              i32.add
                              local.get 0
                              local.get 2
                              i32.sub
                              local.tee 0
                              i32.store
                              local.get 4
                              local.get 2
                              i32.add
                              local.tee 6
                              local.get 0
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              block  ;; label = @14
                                local.get 7
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 7
                                i32.const 3
                                i32.shr_u
                                local.tee 8
                                i32.const 3
                                i32.shl
                                i32.const 1059972
                                i32.add
                                local.set 2
                                i32.const 0
                                i32.load offset=1059952
                                local.set 4
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 5
                                    i32.const 1
                                    local.get 8
                                    i32.shl
                                    local.tee 8
                                    i32.and
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.get 5
                                    local.get 8
                                    i32.or
                                    i32.store offset=1059932
                                    local.get 2
                                    local.set 8
                                    br 1 (;@15;)
                                  end
                                  local.get 2
                                  i32.load offset=8
                                  local.set 8
                                end
                                local.get 8
                                local.get 4
                                i32.store offset=12
                                local.get 2
                                local.get 4
                                i32.store offset=8
                                local.get 4
                                local.get 2
                                i32.store offset=12
                                local.get 4
                                local.get 8
                                i32.store offset=8
                              end
                              i32.const 0
                              local.get 6
                              i32.store offset=1059952
                              i32.const 0
                              local.get 0
                              i32.store offset=1059940
                              br 12 (;@1;)
                            end
                            i32.const 0
                            i32.load offset=1059936
                            local.tee 9
                            i32.eqz
                            br_if 1 (;@11;)
                            local.get 9
                            i32.const 0
                            local.get 9
                            i32.sub
                            i32.and
                            i32.const -1
                            i32.add
                            local.tee 3
                            local.get 3
                            i32.const 12
                            i32.shr_u
                            i32.const 16
                            i32.and
                            local.tee 3
                            i32.shr_u
                            local.tee 4
                            i32.const 5
                            i32.shr_u
                            i32.const 8
                            i32.and
                            local.tee 0
                            local.get 3
                            i32.or
                            local.get 4
                            local.get 0
                            i32.shr_u
                            local.tee 3
                            i32.const 2
                            i32.shr_u
                            i32.const 4
                            i32.and
                            local.tee 4
                            i32.or
                            local.get 3
                            local.get 4
                            i32.shr_u
                            local.tee 3
                            i32.const 1
                            i32.shr_u
                            i32.const 2
                            i32.and
                            local.tee 4
                            i32.or
                            local.get 3
                            local.get 4
                            i32.shr_u
                            local.tee 3
                            i32.const 1
                            i32.shr_u
                            i32.const 1
                            i32.and
                            local.tee 4
                            i32.or
                            local.get 3
                            local.get 4
                            i32.shr_u
                            i32.add
                            i32.const 2
                            i32.shl
                            i32.const 1060236
                            i32.add
                            i32.load
                            local.tee 6
                            i32.load offset=4
                            i32.const -8
                            i32.and
                            local.get 2
                            i32.sub
                            local.set 4
                            local.get 6
                            local.set 0
                            block  ;; label = @13
                              loop  ;; label = @14
                                block  ;; label = @15
                                  local.get 0
                                  i32.load offset=16
                                  local.tee 3
                                  br_if 0 (;@15;)
                                  local.get 0
                                  i32.const 20
                                  i32.add
                                  i32.load
                                  local.tee 3
                                  i32.eqz
                                  br_if 2 (;@13;)
                                end
                                local.get 3
                                i32.load offset=4
                                i32.const -8
                                i32.and
                                local.get 2
                                i32.sub
                                local.tee 0
                                local.get 4
                                local.get 0
                                local.get 4
                                i32.lt_u
                                local.tee 0
                                select
                                local.set 4
                                local.get 3
                                local.get 6
                                local.get 0
                                select
                                local.set 6
                                local.get 3
                                local.set 0
                                br 0 (;@14;)
                              end
                            end
                            local.get 6
                            i32.load offset=24
                            local.set 10
                            block  ;; label = @13
                              local.get 6
                              i32.load offset=12
                              local.tee 8
                              local.get 6
                              i32.eq
                              br_if 0 (;@13;)
                              block  ;; label = @14
                                i32.const 0
                                i32.load offset=1059948
                                local.get 6
                                i32.load offset=8
                                local.tee 3
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 3
                                i32.load offset=12
                                local.get 6
                                i32.ne
                                drop
                              end
                              local.get 8
                              local.get 3
                              i32.store offset=8
                              local.get 3
                              local.get 8
                              i32.store offset=12
                              br 11 (;@2;)
                            end
                            block  ;; label = @13
                              local.get 6
                              i32.const 20
                              i32.add
                              local.tee 0
                              i32.load
                              local.tee 3
                              br_if 0 (;@13;)
                              local.get 6
                              i32.load offset=16
                              local.tee 3
                              i32.eqz
                              br_if 3 (;@10;)
                              local.get 6
                              i32.const 16
                              i32.add
                              local.set 0
                            end
                            loop  ;; label = @13
                              local.get 0
                              local.set 11
                              local.get 3
                              local.tee 8
                              i32.const 20
                              i32.add
                              local.tee 0
                              i32.load
                              local.tee 3
                              br_if 0 (;@13;)
                              local.get 8
                              i32.const 16
                              i32.add
                              local.set 0
                              local.get 8
                              i32.load offset=16
                              local.tee 3
                              br_if 0 (;@13;)
                            end
                            local.get 11
                            i32.const 0
                            i32.store
                            br 10 (;@2;)
                          end
                          i32.const -1
                          local.set 2
                          local.get 0
                          i32.const -65
                          i32.gt_u
                          br_if 0 (;@11;)
                          local.get 0
                          i32.const 19
                          i32.add
                          local.tee 3
                          i32.const -16
                          i32.and
                          local.set 2
                          i32.const 0
                          i32.load offset=1059936
                          local.tee 7
                          i32.eqz
                          br_if 0 (;@11;)
                          i32.const 0
                          local.set 11
                          block  ;; label = @12
                            local.get 3
                            i32.const 8
                            i32.shr_u
                            local.tee 3
                            i32.eqz
                            br_if 0 (;@12;)
                            i32.const 31
                            local.set 11
                            local.get 2
                            i32.const 16777215
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 3
                            local.get 3
                            i32.const 1048320
                            i32.add
                            i32.const 16
                            i32.shr_u
                            i32.const 8
                            i32.and
                            local.tee 4
                            i32.shl
                            local.tee 3
                            local.get 3
                            i32.const 520192
                            i32.add
                            i32.const 16
                            i32.shr_u
                            i32.const 4
                            i32.and
                            local.tee 3
                            i32.shl
                            local.tee 0
                            local.get 0
                            i32.const 245760
                            i32.add
                            i32.const 16
                            i32.shr_u
                            i32.const 2
                            i32.and
                            local.tee 0
                            i32.shl
                            i32.const 15
                            i32.shr_u
                            local.get 3
                            local.get 4
                            i32.or
                            local.get 0
                            i32.or
                            i32.sub
                            local.tee 3
                            i32.const 1
                            i32.shl
                            local.get 2
                            local.get 3
                            i32.const 21
                            i32.add
                            i32.shr_u
                            i32.const 1
                            i32.and
                            i32.or
                            i32.const 28
                            i32.add
                            local.set 11
                          end
                          i32.const 0
                          local.get 2
                          i32.sub
                          local.set 0
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 11
                                  i32.const 2
                                  i32.shl
                                  i32.const 1060236
                                  i32.add
                                  i32.load
                                  local.tee 4
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.set 3
                                  i32.const 0
                                  local.set 8
                                  br 1 (;@14;)
                                end
                                local.get 2
                                i32.const 0
                                i32.const 25
                                local.get 11
                                i32.const 1
                                i32.shr_u
                                i32.sub
                                local.get 11
                                i32.const 31
                                i32.eq
                                select
                                i32.shl
                                local.set 6
                                i32.const 0
                                local.set 3
                                i32.const 0
                                local.set 8
                                loop  ;; label = @15
                                  block  ;; label = @16
                                    local.get 4
                                    i32.load offset=4
                                    i32.const -8
                                    i32.and
                                    local.get 2
                                    i32.sub
                                    local.tee 5
                                    local.get 0
                                    i32.ge_u
                                    br_if 0 (;@16;)
                                    local.get 5
                                    local.set 0
                                    local.get 4
                                    local.set 8
                                    local.get 5
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.set 0
                                    local.get 4
                                    local.set 8
                                    local.get 4
                                    local.set 3
                                    br 3 (;@13;)
                                  end
                                  local.get 3
                                  local.get 4
                                  i32.const 20
                                  i32.add
                                  i32.load
                                  local.tee 5
                                  local.get 5
                                  local.get 4
                                  local.get 6
                                  i32.const 29
                                  i32.shr_u
                                  i32.const 4
                                  i32.and
                                  i32.add
                                  i32.const 16
                                  i32.add
                                  i32.load
                                  local.tee 4
                                  i32.eq
                                  select
                                  local.get 3
                                  local.get 5
                                  select
                                  local.set 3
                                  local.get 6
                                  local.get 4
                                  i32.const 0
                                  i32.ne
                                  i32.shl
                                  local.set 6
                                  local.get 4
                                  br_if 0 (;@15;)
                                end
                              end
                              block  ;; label = @14
                                local.get 3
                                local.get 8
                                i32.or
                                br_if 0 (;@14;)
                                i32.const 2
                                local.get 11
                                i32.shl
                                local.tee 3
                                i32.const 0
                                local.get 3
                                i32.sub
                                i32.or
                                local.get 7
                                i32.and
                                local.tee 3
                                i32.eqz
                                br_if 3 (;@11;)
                                local.get 3
                                i32.const 0
                                local.get 3
                                i32.sub
                                i32.and
                                i32.const -1
                                i32.add
                                local.tee 3
                                local.get 3
                                i32.const 12
                                i32.shr_u
                                i32.const 16
                                i32.and
                                local.tee 3
                                i32.shr_u
                                local.tee 4
                                i32.const 5
                                i32.shr_u
                                i32.const 8
                                i32.and
                                local.tee 6
                                local.get 3
                                i32.or
                                local.get 4
                                local.get 6
                                i32.shr_u
                                local.tee 3
                                i32.const 2
                                i32.shr_u
                                i32.const 4
                                i32.and
                                local.tee 4
                                i32.or
                                local.get 3
                                local.get 4
                                i32.shr_u
                                local.tee 3
                                i32.const 1
                                i32.shr_u
                                i32.const 2
                                i32.and
                                local.tee 4
                                i32.or
                                local.get 3
                                local.get 4
                                i32.shr_u
                                local.tee 3
                                i32.const 1
                                i32.shr_u
                                i32.const 1
                                i32.and
                                local.tee 4
                                i32.or
                                local.get 3
                                local.get 4
                                i32.shr_u
                                i32.add
                                i32.const 2
                                i32.shl
                                i32.const 1060236
                                i32.add
                                i32.load
                                local.set 3
                              end
                              local.get 3
                              i32.eqz
                              br_if 1 (;@12;)
                            end
                            loop  ;; label = @13
                              local.get 3
                              i32.load offset=4
                              i32.const -8
                              i32.and
                              local.get 2
                              i32.sub
                              local.tee 5
                              local.get 0
                              i32.lt_u
                              local.set 6
                              block  ;; label = @14
                                local.get 3
                                i32.load offset=16
                                local.tee 4
                                br_if 0 (;@14;)
                                local.get 3
                                i32.const 20
                                i32.add
                                i32.load
                                local.set 4
                              end
                              local.get 5
                              local.get 0
                              local.get 6
                              select
                              local.set 0
                              local.get 3
                              local.get 8
                              local.get 6
                              select
                              local.set 8
                              local.get 4
                              local.set 3
                              local.get 4
                              br_if 0 (;@13;)
                            end
                          end
                          local.get 8
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 0
                          i32.const 0
                          i32.load offset=1059940
                          local.get 2
                          i32.sub
                          i32.ge_u
                          br_if 0 (;@11;)
                          local.get 8
                          i32.load offset=24
                          local.set 11
                          block  ;; label = @12
                            local.get 8
                            i32.load offset=12
                            local.tee 6
                            local.get 8
                            i32.eq
                            br_if 0 (;@12;)
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=1059948
                              local.get 8
                              i32.load offset=8
                              local.tee 3
                              i32.gt_u
                              br_if 0 (;@13;)
                              local.get 3
                              i32.load offset=12
                              local.get 8
                              i32.ne
                              drop
                            end
                            local.get 6
                            local.get 3
                            i32.store offset=8
                            local.get 3
                            local.get 6
                            i32.store offset=12
                            br 9 (;@3;)
                          end
                          block  ;; label = @12
                            local.get 8
                            i32.const 20
                            i32.add
                            local.tee 4
                            i32.load
                            local.tee 3
                            br_if 0 (;@12;)
                            local.get 8
                            i32.load offset=16
                            local.tee 3
                            i32.eqz
                            br_if 3 (;@9;)
                            local.get 8
                            i32.const 16
                            i32.add
                            local.set 4
                          end
                          loop  ;; label = @12
                            local.get 4
                            local.set 5
                            local.get 3
                            local.tee 6
                            i32.const 20
                            i32.add
                            local.tee 4
                            i32.load
                            local.tee 3
                            br_if 0 (;@12;)
                            local.get 6
                            i32.const 16
                            i32.add
                            local.set 4
                            local.get 6
                            i32.load offset=16
                            local.tee 3
                            br_if 0 (;@12;)
                          end
                          local.get 5
                          i32.const 0
                          i32.store
                          br 8 (;@3;)
                        end
                        block  ;; label = @11
                          i32.const 0
                          i32.load offset=1059940
                          local.tee 3
                          local.get 2
                          i32.lt_u
                          br_if 0 (;@11;)
                          i32.const 0
                          i32.load offset=1059952
                          local.set 4
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 3
                              local.get 2
                              i32.sub
                              local.tee 0
                              i32.const 16
                              i32.lt_u
                              br_if 0 (;@13;)
                              local.get 4
                              local.get 2
                              i32.add
                              local.tee 6
                              local.get 0
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              i32.const 0
                              local.get 0
                              i32.store offset=1059940
                              i32.const 0
                              local.get 6
                              i32.store offset=1059952
                              local.get 4
                              local.get 3
                              i32.add
                              local.get 0
                              i32.store
                              local.get 4
                              local.get 2
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              br 1 (;@12;)
                            end
                            local.get 4
                            local.get 3
                            i32.const 3
                            i32.or
                            i32.store offset=4
                            local.get 4
                            local.get 3
                            i32.add
                            local.tee 3
                            local.get 3
                            i32.load offset=4
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            i32.const 0
                            i32.const 0
                            i32.store offset=1059952
                            i32.const 0
                            i32.const 0
                            i32.store offset=1059940
                          end
                          local.get 4
                          i32.const 8
                          i32.add
                          local.set 3
                          br 10 (;@1;)
                        end
                        block  ;; label = @11
                          i32.const 0
                          i32.load offset=1059944
                          local.tee 6
                          local.get 2
                          i32.le_u
                          br_if 0 (;@11;)
                          i32.const 0
                          i32.load offset=1059956
                          local.tee 3
                          local.get 2
                          i32.add
                          local.tee 4
                          local.get 6
                          local.get 2
                          i32.sub
                          local.tee 0
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          i32.const 0
                          local.get 0
                          i32.store offset=1059944
                          i32.const 0
                          local.get 4
                          i32.store offset=1059956
                          local.get 3
                          local.get 2
                          i32.const 3
                          i32.or
                          i32.store offset=4
                          local.get 3
                          i32.const 8
                          i32.add
                          local.set 3
                          br 10 (;@1;)
                        end
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=1060404
                            i32.eqz
                            br_if 0 (;@12;)
                            i32.const 0
                            i32.load offset=1060412
                            local.set 4
                            br 1 (;@11;)
                          end
                          i32.const 0
                          i64.const -1
                          i64.store offset=1060416 align=4
                          i32.const 0
                          i64.const 281474976776192
                          i64.store offset=1060408 align=4
                          i32.const 0
                          local.get 1
                          i32.const 12
                          i32.add
                          i32.const -16
                          i32.and
                          i32.const 1431655768
                          i32.xor
                          i32.store offset=1060404
                          i32.const 0
                          i32.const 0
                          i32.store offset=1060424
                          i32.const 0
                          i32.const 0
                          i32.store offset=1060376
                          i32.const 65536
                          local.set 4
                        end
                        i32.const 0
                        local.set 3
                        block  ;; label = @11
                          local.get 4
                          local.get 2
                          i32.const 71
                          i32.add
                          local.tee 7
                          i32.add
                          local.tee 5
                          i32.const 0
                          local.get 4
                          i32.sub
                          local.tee 11
                          i32.and
                          local.tee 8
                          local.get 2
                          i32.gt_u
                          br_if 0 (;@11;)
                          i32.const 0
                          i32.const 48
                          i32.store offset=1060428
                          br 10 (;@1;)
                        end
                        block  ;; label = @11
                          i32.const 0
                          i32.load offset=1060372
                          local.tee 3
                          i32.eqz
                          br_if 0 (;@11;)
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=1060364
                            local.tee 4
                            local.get 8
                            i32.add
                            local.tee 0
                            local.get 4
                            i32.le_u
                            br_if 0 (;@12;)
                            local.get 0
                            local.get 3
                            i32.le_u
                            br_if 1 (;@11;)
                          end
                          i32.const 0
                          local.set 3
                          i32.const 0
                          i32.const 48
                          i32.store offset=1060428
                          br 10 (;@1;)
                        end
                        i32.const 0
                        i32.load8_u offset=1060376
                        i32.const 4
                        i32.and
                        br_if 4 (;@6;)
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=1059956
                              local.tee 4
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 1060380
                              local.set 3
                              loop  ;; label = @14
                                block  ;; label = @15
                                  local.get 3
                                  i32.load
                                  local.tee 0
                                  local.get 4
                                  i32.gt_u
                                  br_if 0 (;@15;)
                                  local.get 0
                                  local.get 3
                                  i32.load offset=4
                                  i32.add
                                  local.get 4
                                  i32.gt_u
                                  br_if 3 (;@12;)
                                end
                                local.get 3
                                i32.load offset=8
                                local.tee 3
                                br_if 0 (;@14;)
                              end
                            end
                            i32.const 0
                            call $sbrk
                            local.tee 6
                            i32.const -1
                            i32.eq
                            br_if 5 (;@7;)
                            local.get 8
                            local.set 5
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=1060408
                              local.tee 3
                              i32.const -1
                              i32.add
                              local.tee 4
                              local.get 6
                              i32.and
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 8
                              local.get 6
                              i32.sub
                              local.get 4
                              local.get 6
                              i32.add
                              i32.const 0
                              local.get 3
                              i32.sub
                              i32.and
                              i32.add
                              local.set 5
                            end
                            local.get 5
                            local.get 2
                            i32.le_u
                            br_if 5 (;@7;)
                            local.get 5
                            i32.const 2147483646
                            i32.gt_u
                            br_if 5 (;@7;)
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=1060372
                              local.tee 3
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 0
                              i32.load offset=1060364
                              local.tee 4
                              local.get 5
                              i32.add
                              local.tee 0
                              local.get 4
                              i32.le_u
                              br_if 6 (;@7;)
                              local.get 0
                              local.get 3
                              i32.gt_u
                              br_if 6 (;@7;)
                            end
                            local.get 5
                            call $sbrk
                            local.tee 3
                            local.get 6
                            i32.ne
                            br_if 1 (;@11;)
                            br 7 (;@5;)
                          end
                          local.get 5
                          local.get 6
                          i32.sub
                          local.get 11
                          i32.and
                          local.tee 5
                          i32.const 2147483646
                          i32.gt_u
                          br_if 4 (;@7;)
                          local.get 5
                          call $sbrk
                          local.tee 6
                          local.get 3
                          i32.load
                          local.get 3
                          i32.load offset=4
                          i32.add
                          i32.eq
                          br_if 3 (;@8;)
                          local.get 6
                          local.set 3
                        end
                        block  ;; label = @11
                          local.get 2
                          i32.const 72
                          i32.add
                          local.get 5
                          i32.le_u
                          br_if 0 (;@11;)
                          local.get 3
                          i32.const -1
                          i32.eq
                          br_if 0 (;@11;)
                          block  ;; label = @12
                            local.get 7
                            local.get 5
                            i32.sub
                            i32.const 0
                            i32.load offset=1060412
                            local.tee 4
                            i32.add
                            i32.const 0
                            local.get 4
                            i32.sub
                            i32.and
                            local.tee 4
                            i32.const 2147483646
                            i32.le_u
                            br_if 0 (;@12;)
                            local.get 3
                            local.set 6
                            br 7 (;@5;)
                          end
                          block  ;; label = @12
                            local.get 4
                            call $sbrk
                            i32.const -1
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 4
                            local.get 5
                            i32.add
                            local.set 5
                            local.get 3
                            local.set 6
                            br 7 (;@5;)
                          end
                          i32.const 0
                          local.get 5
                          i32.sub
                          call $sbrk
                          drop
                          br 4 (;@7;)
                        end
                        local.get 3
                        local.set 6
                        local.get 3
                        i32.const -1
                        i32.ne
                        br_if 5 (;@5;)
                        br 3 (;@7;)
                      end
                      i32.const 0
                      local.set 8
                      br 7 (;@2;)
                    end
                    i32.const 0
                    local.set 6
                    br 5 (;@3;)
                  end
                  local.get 6
                  i32.const -1
                  i32.ne
                  br_if 2 (;@5;)
                end
                i32.const 0
                i32.const 0
                i32.load offset=1060376
                i32.const 4
                i32.or
                i32.store offset=1060376
              end
              local.get 8
              i32.const 2147483646
              i32.gt_u
              br_if 1 (;@4;)
              local.get 8
              call $sbrk
              local.tee 6
              i32.const 0
              call $sbrk
              local.tee 3
              i32.ge_u
              br_if 1 (;@4;)
              local.get 6
              i32.const -1
              i32.eq
              br_if 1 (;@4;)
              local.get 3
              i32.const -1
              i32.eq
              br_if 1 (;@4;)
              local.get 3
              local.get 6
              i32.sub
              local.tee 5
              local.get 2
              i32.const 56
              i32.add
              i32.le_u
              br_if 1 (;@4;)
            end
            i32.const 0
            i32.const 0
            i32.load offset=1060364
            local.get 5
            i32.add
            local.tee 3
            i32.store offset=1060364
            block  ;; label = @5
              local.get 3
              i32.const 0
              i32.load offset=1060368
              i32.le_u
              br_if 0 (;@5;)
              i32.const 0
              local.get 3
              i32.store offset=1060368
            end
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 0
                    i32.load offset=1059956
                    local.tee 4
                    i32.eqz
                    br_if 0 (;@8;)
                    i32.const 1060380
                    local.set 3
                    loop  ;; label = @9
                      local.get 6
                      local.get 3
                      i32.load
                      local.tee 0
                      local.get 3
                      i32.load offset=4
                      local.tee 8
                      i32.add
                      i32.eq
                      br_if 2 (;@7;)
                      local.get 3
                      i32.load offset=8
                      local.tee 3
                      br_if 0 (;@9;)
                      br 3 (;@6;)
                    end
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      i32.const 0
                      i32.load offset=1059948
                      local.tee 3
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 6
                      local.get 3
                      i32.ge_u
                      br_if 1 (;@8;)
                    end
                    i32.const 0
                    local.get 6
                    i32.store offset=1059948
                  end
                  i32.const 0
                  local.set 3
                  i32.const 0
                  local.get 5
                  i32.store offset=1060384
                  i32.const 0
                  local.get 6
                  i32.store offset=1060380
                  i32.const 0
                  i32.const -1
                  i32.store offset=1059964
                  i32.const 0
                  i32.const 0
                  i32.load offset=1060404
                  i32.store offset=1059968
                  i32.const 0
                  i32.const 0
                  i32.store offset=1060392
                  loop  ;; label = @8
                    local.get 3
                    i32.const 1059980
                    i32.add
                    local.get 3
                    i32.const 1059972
                    i32.add
                    local.tee 4
                    i32.store
                    local.get 3
                    i32.const 1059984
                    i32.add
                    local.get 4
                    i32.store
                    local.get 3
                    i32.const 8
                    i32.add
                    local.tee 3
                    i32.const 256
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  local.get 6
                  i32.const -8
                  local.get 6
                  i32.sub
                  i32.const 15
                  i32.and
                  i32.const 0
                  local.get 6
                  i32.const 8
                  i32.add
                  i32.const 15
                  i32.and
                  select
                  local.tee 3
                  i32.add
                  local.tee 4
                  local.get 5
                  i32.const -56
                  i32.add
                  local.tee 0
                  local.get 3
                  i32.sub
                  local.tee 3
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=1060420
                  i32.store offset=1059960
                  i32.const 0
                  local.get 3
                  i32.store offset=1059944
                  i32.const 0
                  local.get 4
                  i32.store offset=1059956
                  local.get 6
                  local.get 0
                  i32.add
                  i32.const 56
                  i32.store offset=4
                  br 2 (;@5;)
                end
                local.get 3
                i32.load8_u offset=12
                i32.const 8
                i32.and
                br_if 0 (;@6;)
                local.get 6
                local.get 4
                i32.le_u
                br_if 0 (;@6;)
                local.get 0
                local.get 4
                i32.gt_u
                br_if 0 (;@6;)
                local.get 4
                i32.const -8
                local.get 4
                i32.sub
                i32.const 15
                i32.and
                i32.const 0
                local.get 4
                i32.const 8
                i32.add
                i32.const 15
                i32.and
                select
                local.tee 0
                i32.add
                local.tee 6
                i32.const 0
                i32.load offset=1059944
                local.get 5
                i32.add
                local.tee 11
                local.get 0
                i32.sub
                local.tee 0
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 3
                local.get 8
                local.get 5
                i32.add
                i32.store offset=4
                i32.const 0
                i32.const 0
                i32.load offset=1060420
                i32.store offset=1059960
                i32.const 0
                local.get 0
                i32.store offset=1059944
                i32.const 0
                local.get 6
                i32.store offset=1059956
                local.get 4
                local.get 11
                i32.add
                i32.const 56
                i32.store offset=4
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 6
                i32.const 0
                i32.load offset=1059948
                local.tee 8
                i32.ge_u
                br_if 0 (;@6;)
                i32.const 0
                local.get 6
                i32.store offset=1059948
                local.get 6
                local.set 8
              end
              local.get 6
              local.get 5
              i32.add
              local.set 0
              i32.const 1060380
              local.set 3
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            loop  ;; label = @13
                              local.get 3
                              i32.load
                              local.get 0
                              i32.eq
                              br_if 1 (;@12;)
                              local.get 3
                              i32.load offset=8
                              local.tee 3
                              br_if 0 (;@13;)
                              br 2 (;@11;)
                            end
                          end
                          local.get 3
                          i32.load8_u offset=12
                          i32.const 8
                          i32.and
                          i32.eqz
                          br_if 1 (;@10;)
                        end
                        i32.const 1060380
                        local.set 3
                        loop  ;; label = @11
                          block  ;; label = @12
                            local.get 3
                            i32.load
                            local.tee 0
                            local.get 4
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 0
                            local.get 3
                            i32.load offset=4
                            i32.add
                            local.tee 0
                            local.get 4
                            i32.gt_u
                            br_if 3 (;@9;)
                          end
                          local.get 3
                          i32.load offset=8
                          local.set 3
                          br 0 (;@11;)
                        end
                      end
                      local.get 3
                      local.get 6
                      i32.store
                      local.get 3
                      local.get 3
                      i32.load offset=4
                      local.get 5
                      i32.add
                      i32.store offset=4
                      local.get 6
                      i32.const -8
                      local.get 6
                      i32.sub
                      i32.const 15
                      i32.and
                      i32.const 0
                      local.get 6
                      i32.const 8
                      i32.add
                      i32.const 15
                      i32.and
                      select
                      i32.add
                      local.tee 11
                      local.get 2
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 0
                      i32.const -8
                      local.get 0
                      i32.sub
                      i32.const 15
                      i32.and
                      i32.const 0
                      local.get 0
                      i32.const 8
                      i32.add
                      i32.const 15
                      i32.and
                      select
                      i32.add
                      local.tee 6
                      local.get 11
                      i32.sub
                      local.get 2
                      i32.sub
                      local.set 3
                      local.get 11
                      local.get 2
                      i32.add
                      local.set 0
                      block  ;; label = @10
                        local.get 4
                        local.get 6
                        i32.ne
                        br_if 0 (;@10;)
                        i32.const 0
                        local.get 0
                        i32.store offset=1059956
                        i32.const 0
                        i32.const 0
                        i32.load offset=1059944
                        local.get 3
                        i32.add
                        local.tee 3
                        i32.store offset=1059944
                        local.get 0
                        local.get 3
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        br 3 (;@7;)
                      end
                      block  ;; label = @10
                        i32.const 0
                        i32.load offset=1059952
                        local.get 6
                        i32.ne
                        br_if 0 (;@10;)
                        i32.const 0
                        local.get 0
                        i32.store offset=1059952
                        i32.const 0
                        i32.const 0
                        i32.load offset=1059940
                        local.get 3
                        i32.add
                        local.tee 3
                        i32.store offset=1059940
                        local.get 0
                        local.get 3
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 0
                        local.get 3
                        i32.add
                        local.get 3
                        i32.store
                        br 3 (;@7;)
                      end
                      block  ;; label = @10
                        local.get 6
                        i32.load offset=4
                        local.tee 4
                        i32.const 3
                        i32.and
                        i32.const 1
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 4
                        i32.const -8
                        i32.and
                        local.set 7
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 4
                            i32.const 255
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 6
                            i32.load offset=12
                            local.set 2
                            block  ;; label = @13
                              local.get 6
                              i32.load offset=8
                              local.tee 5
                              local.get 4
                              i32.const 3
                              i32.shr_u
                              local.tee 9
                              i32.const 3
                              i32.shl
                              i32.const 1059972
                              i32.add
                              local.tee 4
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 8
                              local.get 5
                              i32.gt_u
                              drop
                            end
                            block  ;; label = @13
                              local.get 2
                              local.get 5
                              i32.ne
                              br_if 0 (;@13;)
                              i32.const 0
                              i32.const 0
                              i32.load offset=1059932
                              i32.const -2
                              local.get 9
                              i32.rotl
                              i32.and
                              i32.store offset=1059932
                              br 2 (;@11;)
                            end
                            block  ;; label = @13
                              local.get 2
                              local.get 4
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 8
                              local.get 2
                              i32.gt_u
                              drop
                            end
                            local.get 2
                            local.get 5
                            i32.store offset=8
                            local.get 5
                            local.get 2
                            i32.store offset=12
                            br 1 (;@11;)
                          end
                          local.get 6
                          i32.load offset=24
                          local.set 9
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 6
                              i32.load offset=12
                              local.tee 5
                              local.get 6
                              i32.eq
                              br_if 0 (;@13;)
                              block  ;; label = @14
                                local.get 8
                                local.get 6
                                i32.load offset=8
                                local.tee 4
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 4
                                i32.load offset=12
                                local.get 6
                                i32.ne
                                drop
                              end
                              local.get 5
                              local.get 4
                              i32.store offset=8
                              local.get 4
                              local.get 5
                              i32.store offset=12
                              br 1 (;@12;)
                            end
                            block  ;; label = @13
                              local.get 6
                              i32.const 20
                              i32.add
                              local.tee 4
                              i32.load
                              local.tee 2
                              br_if 0 (;@13;)
                              local.get 6
                              i32.const 16
                              i32.add
                              local.tee 4
                              i32.load
                              local.tee 2
                              br_if 0 (;@13;)
                              i32.const 0
                              local.set 5
                              br 1 (;@12;)
                            end
                            loop  ;; label = @13
                              local.get 4
                              local.set 8
                              local.get 2
                              local.tee 5
                              i32.const 20
                              i32.add
                              local.tee 4
                              i32.load
                              local.tee 2
                              br_if 0 (;@13;)
                              local.get 5
                              i32.const 16
                              i32.add
                              local.set 4
                              local.get 5
                              i32.load offset=16
                              local.tee 2
                              br_if 0 (;@13;)
                            end
                            local.get 8
                            i32.const 0
                            i32.store
                          end
                          local.get 9
                          i32.eqz
                          br_if 0 (;@11;)
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 6
                              i32.load offset=28
                              local.tee 2
                              i32.const 2
                              i32.shl
                              i32.const 1060236
                              i32.add
                              local.tee 4
                              i32.load
                              local.get 6
                              i32.ne
                              br_if 0 (;@13;)
                              local.get 4
                              local.get 5
                              i32.store
                              local.get 5
                              br_if 1 (;@12;)
                              i32.const 0
                              i32.const 0
                              i32.load offset=1059936
                              i32.const -2
                              local.get 2
                              i32.rotl
                              i32.and
                              i32.store offset=1059936
                              br 2 (;@11;)
                            end
                            local.get 9
                            i32.const 16
                            i32.const 20
                            local.get 9
                            i32.load offset=16
                            local.get 6
                            i32.eq
                            select
                            i32.add
                            local.get 5
                            i32.store
                            local.get 5
                            i32.eqz
                            br_if 1 (;@11;)
                          end
                          local.get 5
                          local.get 9
                          i32.store offset=24
                          block  ;; label = @12
                            local.get 6
                            i32.load offset=16
                            local.tee 4
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 5
                            local.get 4
                            i32.store offset=16
                            local.get 4
                            local.get 5
                            i32.store offset=24
                          end
                          local.get 6
                          i32.load offset=20
                          local.tee 4
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 5
                          i32.const 20
                          i32.add
                          local.get 4
                          i32.store
                          local.get 4
                          local.get 5
                          i32.store offset=24
                        end
                        local.get 7
                        local.get 3
                        i32.add
                        local.set 3
                        local.get 6
                        local.get 7
                        i32.add
                        local.set 6
                      end
                      local.get 6
                      local.get 6
                      i32.load offset=4
                      i32.const -2
                      i32.and
                      i32.store offset=4
                      local.get 0
                      local.get 3
                      i32.add
                      local.get 3
                      i32.store
                      local.get 0
                      local.get 3
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      block  ;; label = @10
                        local.get 3
                        i32.const 255
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 3
                        i32.const 3
                        i32.shr_u
                        local.tee 4
                        i32.const 3
                        i32.shl
                        i32.const 1059972
                        i32.add
                        local.set 3
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=1059932
                            local.tee 2
                            i32.const 1
                            local.get 4
                            i32.shl
                            local.tee 4
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 0
                            local.get 2
                            local.get 4
                            i32.or
                            i32.store offset=1059932
                            local.get 3
                            local.set 4
                            br 1 (;@11;)
                          end
                          local.get 3
                          i32.load offset=8
                          local.set 4
                        end
                        local.get 4
                        local.get 0
                        i32.store offset=12
                        local.get 3
                        local.get 0
                        i32.store offset=8
                        local.get 0
                        local.get 3
                        i32.store offset=12
                        local.get 0
                        local.get 4
                        i32.store offset=8
                        br 3 (;@7;)
                      end
                      i32.const 0
                      local.set 4
                      block  ;; label = @10
                        local.get 3
                        i32.const 8
                        i32.shr_u
                        local.tee 2
                        i32.eqz
                        br_if 0 (;@10;)
                        i32.const 31
                        local.set 4
                        local.get 3
                        i32.const 16777215
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 2
                        local.get 2
                        i32.const 1048320
                        i32.add
                        i32.const 16
                        i32.shr_u
                        i32.const 8
                        i32.and
                        local.tee 4
                        i32.shl
                        local.tee 2
                        local.get 2
                        i32.const 520192
                        i32.add
                        i32.const 16
                        i32.shr_u
                        i32.const 4
                        i32.and
                        local.tee 2
                        i32.shl
                        local.tee 6
                        local.get 6
                        i32.const 245760
                        i32.add
                        i32.const 16
                        i32.shr_u
                        i32.const 2
                        i32.and
                        local.tee 6
                        i32.shl
                        i32.const 15
                        i32.shr_u
                        local.get 2
                        local.get 4
                        i32.or
                        local.get 6
                        i32.or
                        i32.sub
                        local.tee 4
                        i32.const 1
                        i32.shl
                        local.get 3
                        local.get 4
                        i32.const 21
                        i32.add
                        i32.shr_u
                        i32.const 1
                        i32.and
                        i32.or
                        i32.const 28
                        i32.add
                        local.set 4
                      end
                      local.get 0
                      local.get 4
                      i32.store offset=28
                      local.get 0
                      i64.const 0
                      i64.store offset=16 align=4
                      local.get 4
                      i32.const 2
                      i32.shl
                      i32.const 1060236
                      i32.add
                      local.set 2
                      block  ;; label = @10
                        i32.const 0
                        i32.load offset=1059936
                        local.tee 6
                        i32.const 1
                        local.get 4
                        i32.shl
                        local.tee 8
                        i32.and
                        br_if 0 (;@10;)
                        local.get 2
                        local.get 0
                        i32.store
                        i32.const 0
                        local.get 6
                        local.get 8
                        i32.or
                        i32.store offset=1059936
                        local.get 0
                        local.get 2
                        i32.store offset=24
                        local.get 0
                        local.get 0
                        i32.store offset=8
                        local.get 0
                        local.get 0
                        i32.store offset=12
                        br 3 (;@7;)
                      end
                      local.get 3
                      i32.const 0
                      i32.const 25
                      local.get 4
                      i32.const 1
                      i32.shr_u
                      i32.sub
                      local.get 4
                      i32.const 31
                      i32.eq
                      select
                      i32.shl
                      local.set 4
                      local.get 2
                      i32.load
                      local.set 6
                      loop  ;; label = @10
                        local.get 6
                        local.tee 2
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.get 3
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 4
                        i32.const 29
                        i32.shr_u
                        local.set 6
                        local.get 4
                        i32.const 1
                        i32.shl
                        local.set 4
                        local.get 2
                        local.get 6
                        i32.const 4
                        i32.and
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee 8
                        i32.load
                        local.tee 6
                        br_if 0 (;@10;)
                      end
                      local.get 8
                      local.get 0
                      i32.store
                      local.get 0
                      local.get 2
                      i32.store offset=24
                      local.get 0
                      local.get 0
                      i32.store offset=12
                      local.get 0
                      local.get 0
                      i32.store offset=8
                      br 2 (;@7;)
                    end
                    local.get 6
                    i32.const -8
                    local.get 6
                    i32.sub
                    i32.const 15
                    i32.and
                    i32.const 0
                    local.get 6
                    i32.const 8
                    i32.add
                    i32.const 15
                    i32.and
                    select
                    local.tee 3
                    i32.add
                    local.tee 11
                    local.get 5
                    i32.const -56
                    i32.add
                    local.tee 8
                    local.get 3
                    i32.sub
                    local.tee 3
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 6
                    local.get 8
                    i32.add
                    i32.const 56
                    i32.store offset=4
                    local.get 4
                    local.get 0
                    i32.const 55
                    local.get 0
                    i32.sub
                    i32.const 15
                    i32.and
                    i32.const 0
                    local.get 0
                    i32.const -55
                    i32.add
                    i32.const 15
                    i32.and
                    select
                    i32.add
                    i32.const -63
                    i32.add
                    local.tee 8
                    local.get 8
                    local.get 4
                    i32.const 16
                    i32.add
                    i32.lt_u
                    select
                    local.tee 8
                    i32.const 35
                    i32.store offset=4
                    i32.const 0
                    i32.const 0
                    i32.load offset=1060420
                    i32.store offset=1059960
                    i32.const 0
                    local.get 3
                    i32.store offset=1059944
                    i32.const 0
                    local.get 11
                    i32.store offset=1059956
                    local.get 8
                    i32.const 16
                    i32.add
                    i32.const 0
                    i64.load offset=1060388 align=4
                    i64.store align=4
                    local.get 8
                    i32.const 0
                    i64.load offset=1060380 align=4
                    i64.store offset=8 align=4
                    i32.const 0
                    local.get 8
                    i32.const 8
                    i32.add
                    i32.store offset=1060388
                    i32.const 0
                    local.get 5
                    i32.store offset=1060384
                    i32.const 0
                    local.get 6
                    i32.store offset=1060380
                    i32.const 0
                    i32.const 0
                    i32.store offset=1060392
                    local.get 8
                    i32.const 36
                    i32.add
                    local.set 3
                    loop  ;; label = @9
                      local.get 3
                      i32.const 7
                      i32.store
                      local.get 0
                      local.get 3
                      i32.const 4
                      i32.add
                      local.tee 3
                      i32.gt_u
                      br_if 0 (;@9;)
                    end
                    local.get 8
                    local.get 4
                    i32.eq
                    br_if 3 (;@5;)
                    local.get 8
                    local.get 8
                    i32.load offset=4
                    i32.const -2
                    i32.and
                    i32.store offset=4
                    local.get 8
                    local.get 8
                    local.get 4
                    i32.sub
                    local.tee 5
                    i32.store
                    local.get 4
                    local.get 5
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    block  ;; label = @9
                      local.get 5
                      i32.const 255
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 3
                      i32.shr_u
                      local.tee 0
                      i32.const 3
                      i32.shl
                      i32.const 1059972
                      i32.add
                      local.set 3
                      block  ;; label = @10
                        block  ;; label = @11
                          i32.const 0
                          i32.load offset=1059932
                          local.tee 6
                          i32.const 1
                          local.get 0
                          i32.shl
                          local.tee 0
                          i32.and
                          br_if 0 (;@11;)
                          i32.const 0
                          local.get 6
                          local.get 0
                          i32.or
                          i32.store offset=1059932
                          local.get 3
                          local.set 0
                          br 1 (;@10;)
                        end
                        local.get 3
                        i32.load offset=8
                        local.set 0
                      end
                      local.get 0
                      local.get 4
                      i32.store offset=12
                      local.get 3
                      local.get 4
                      i32.store offset=8
                      local.get 4
                      local.get 3
                      i32.store offset=12
                      local.get 4
                      local.get 0
                      i32.store offset=8
                      br 4 (;@5;)
                    end
                    i32.const 0
                    local.set 3
                    block  ;; label = @9
                      local.get 5
                      i32.const 8
                      i32.shr_u
                      local.tee 0
                      i32.eqz
                      br_if 0 (;@9;)
                      i32.const 31
                      local.set 3
                      local.get 5
                      i32.const 16777215
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 0
                      i32.const 1048320
                      i32.add
                      i32.const 16
                      i32.shr_u
                      i32.const 8
                      i32.and
                      local.tee 3
                      i32.shl
                      local.tee 0
                      local.get 0
                      i32.const 520192
                      i32.add
                      i32.const 16
                      i32.shr_u
                      i32.const 4
                      i32.and
                      local.tee 0
                      i32.shl
                      local.tee 6
                      local.get 6
                      i32.const 245760
                      i32.add
                      i32.const 16
                      i32.shr_u
                      i32.const 2
                      i32.and
                      local.tee 6
                      i32.shl
                      i32.const 15
                      i32.shr_u
                      local.get 0
                      local.get 3
                      i32.or
                      local.get 6
                      i32.or
                      i32.sub
                      local.tee 3
                      i32.const 1
                      i32.shl
                      local.get 5
                      local.get 3
                      i32.const 21
                      i32.add
                      i32.shr_u
                      i32.const 1
                      i32.and
                      i32.or
                      i32.const 28
                      i32.add
                      local.set 3
                    end
                    local.get 4
                    i64.const 0
                    i64.store offset=16 align=4
                    local.get 4
                    i32.const 28
                    i32.add
                    local.get 3
                    i32.store
                    local.get 3
                    i32.const 2
                    i32.shl
                    i32.const 1060236
                    i32.add
                    local.set 0
                    block  ;; label = @9
                      i32.const 0
                      i32.load offset=1059936
                      local.tee 6
                      i32.const 1
                      local.get 3
                      i32.shl
                      local.tee 8
                      i32.and
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 4
                      i32.store
                      i32.const 0
                      local.get 6
                      local.get 8
                      i32.or
                      i32.store offset=1059936
                      local.get 4
                      i32.const 24
                      i32.add
                      local.get 0
                      i32.store
                      local.get 4
                      local.get 4
                      i32.store offset=8
                      local.get 4
                      local.get 4
                      i32.store offset=12
                      br 4 (;@5;)
                    end
                    local.get 5
                    i32.const 0
                    i32.const 25
                    local.get 3
                    i32.const 1
                    i32.shr_u
                    i32.sub
                    local.get 3
                    i32.const 31
                    i32.eq
                    select
                    i32.shl
                    local.set 3
                    local.get 0
                    i32.load
                    local.set 6
                    loop  ;; label = @9
                      local.get 6
                      local.tee 0
                      i32.load offset=4
                      i32.const -8
                      i32.and
                      local.get 5
                      i32.eq
                      br_if 3 (;@6;)
                      local.get 3
                      i32.const 29
                      i32.shr_u
                      local.set 6
                      local.get 3
                      i32.const 1
                      i32.shl
                      local.set 3
                      local.get 0
                      local.get 6
                      i32.const 4
                      i32.and
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 8
                      i32.load
                      local.tee 6
                      br_if 0 (;@9;)
                    end
                    local.get 8
                    local.get 4
                    i32.store
                    local.get 4
                    i32.const 24
                    i32.add
                    local.get 0
                    i32.store
                    local.get 4
                    local.get 4
                    i32.store offset=12
                    local.get 4
                    local.get 4
                    i32.store offset=8
                    br 3 (;@5;)
                  end
                  local.get 2
                  i32.load offset=8
                  local.set 3
                  local.get 2
                  local.get 0
                  i32.store offset=8
                  local.get 3
                  local.get 0
                  i32.store offset=12
                  local.get 0
                  i32.const 0
                  i32.store offset=24
                  local.get 0
                  local.get 3
                  i32.store offset=8
                  local.get 0
                  local.get 2
                  i32.store offset=12
                end
                local.get 11
                i32.const 8
                i32.add
                local.set 3
                br 5 (;@1;)
              end
              local.get 0
              i32.load offset=8
              local.set 3
              local.get 0
              local.get 4
              i32.store offset=8
              local.get 3
              local.get 4
              i32.store offset=12
              local.get 4
              i32.const 24
              i32.add
              i32.const 0
              i32.store
              local.get 4
              local.get 3
              i32.store offset=8
              local.get 4
              local.get 0
              i32.store offset=12
            end
            i32.const 0
            i32.load offset=1059944
            local.tee 3
            local.get 2
            i32.le_u
            br_if 0 (;@4;)
            i32.const 0
            i32.load offset=1059956
            local.tee 4
            local.get 2
            i32.add
            local.tee 0
            local.get 3
            local.get 2
            i32.sub
            local.tee 3
            i32.const 1
            i32.or
            i32.store offset=4
            i32.const 0
            local.get 3
            i32.store offset=1059944
            i32.const 0
            local.get 0
            i32.store offset=1059956
            local.get 4
            local.get 2
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 4
            i32.const 8
            i32.add
            local.set 3
            br 3 (;@1;)
          end
          i32.const 0
          local.set 3
          i32.const 0
          i32.const 48
          i32.store offset=1060428
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 11
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 8
              local.get 8
              i32.load offset=28
              local.tee 4
              i32.const 2
              i32.shl
              i32.const 1060236
              i32.add
              local.tee 3
              i32.load
              i32.ne
              br_if 0 (;@5;)
              local.get 3
              local.get 6
              i32.store
              local.get 6
              br_if 1 (;@4;)
              i32.const 0
              local.get 7
              i32.const -2
              local.get 4
              i32.rotl
              i32.and
              local.tee 7
              i32.store offset=1059936
              br 2 (;@3;)
            end
            local.get 11
            i32.const 16
            i32.const 20
            local.get 11
            i32.load offset=16
            local.get 8
            i32.eq
            select
            i32.add
            local.get 6
            i32.store
            local.get 6
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 6
          local.get 11
          i32.store offset=24
          block  ;; label = @4
            local.get 8
            i32.load offset=16
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            local.get 3
            i32.store offset=16
            local.get 3
            local.get 6
            i32.store offset=24
          end
          local.get 8
          i32.const 20
          i32.add
          i32.load
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          i32.const 20
          i32.add
          local.get 3
          i32.store
          local.get 3
          local.get 6
          i32.store offset=24
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.const 15
            i32.gt_u
            br_if 0 (;@4;)
            local.get 8
            local.get 0
            local.get 2
            i32.add
            local.tee 3
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 8
            local.get 3
            i32.add
            local.tee 3
            local.get 3
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            br 1 (;@3;)
          end
          local.get 8
          local.get 2
          i32.add
          local.tee 6
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 8
          local.get 2
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 6
          local.get 0
          i32.add
          local.get 0
          i32.store
          block  ;; label = @4
            local.get 0
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            local.get 0
            i32.const 3
            i32.shr_u
            local.tee 4
            i32.const 3
            i32.shl
            i32.const 1059972
            i32.add
            local.set 3
            block  ;; label = @5
              block  ;; label = @6
                i32.const 0
                i32.load offset=1059932
                local.tee 0
                i32.const 1
                local.get 4
                i32.shl
                local.tee 4
                i32.and
                br_if 0 (;@6;)
                i32.const 0
                local.get 0
                local.get 4
                i32.or
                i32.store offset=1059932
                local.get 3
                local.set 4
                br 1 (;@5;)
              end
              local.get 3
              i32.load offset=8
              local.set 4
            end
            local.get 4
            local.get 6
            i32.store offset=12
            local.get 3
            local.get 6
            i32.store offset=8
            local.get 6
            local.get 3
            i32.store offset=12
            local.get 6
            local.get 4
            i32.store offset=8
            br 1 (;@3;)
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.const 8
              i32.shr_u
              local.tee 4
              br_if 0 (;@5;)
              i32.const 0
              local.set 3
              br 1 (;@4;)
            end
            i32.const 31
            local.set 3
            local.get 0
            i32.const 16777215
            i32.gt_u
            br_if 0 (;@4;)
            local.get 4
            local.get 4
            i32.const 1048320
            i32.add
            i32.const 16
            i32.shr_u
            i32.const 8
            i32.and
            local.tee 3
            i32.shl
            local.tee 4
            local.get 4
            i32.const 520192
            i32.add
            i32.const 16
            i32.shr_u
            i32.const 4
            i32.and
            local.tee 4
            i32.shl
            local.tee 2
            local.get 2
            i32.const 245760
            i32.add
            i32.const 16
            i32.shr_u
            i32.const 2
            i32.and
            local.tee 2
            i32.shl
            i32.const 15
            i32.shr_u
            local.get 4
            local.get 3
            i32.or
            local.get 2
            i32.or
            i32.sub
            local.tee 3
            i32.const 1
            i32.shl
            local.get 0
            local.get 3
            i32.const 21
            i32.add
            i32.shr_u
            i32.const 1
            i32.and
            i32.or
            i32.const 28
            i32.add
            local.set 3
          end
          local.get 6
          local.get 3
          i32.store offset=28
          local.get 6
          i64.const 0
          i64.store offset=16 align=4
          local.get 3
          i32.const 2
          i32.shl
          i32.const 1060236
          i32.add
          local.set 4
          block  ;; label = @4
            local.get 7
            i32.const 1
            local.get 3
            i32.shl
            local.tee 2
            i32.and
            br_if 0 (;@4;)
            local.get 4
            local.get 6
            i32.store
            i32.const 0
            local.get 7
            local.get 2
            i32.or
            i32.store offset=1059936
            local.get 6
            local.get 4
            i32.store offset=24
            local.get 6
            local.get 6
            i32.store offset=8
            local.get 6
            local.get 6
            i32.store offset=12
            br 1 (;@3;)
          end
          local.get 0
          i32.const 0
          i32.const 25
          local.get 3
          i32.const 1
          i32.shr_u
          i32.sub
          local.get 3
          i32.const 31
          i32.eq
          select
          i32.shl
          local.set 3
          local.get 4
          i32.load
          local.set 2
          block  ;; label = @4
            loop  ;; label = @5
              local.get 2
              local.tee 4
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 0
              i32.eq
              br_if 1 (;@4;)
              local.get 3
              i32.const 29
              i32.shr_u
              local.set 2
              local.get 3
              i32.const 1
              i32.shl
              local.set 3
              local.get 4
              local.get 2
              i32.const 4
              i32.and
              i32.add
              i32.const 16
              i32.add
              local.tee 5
              i32.load
              local.tee 2
              br_if 0 (;@5;)
            end
            local.get 5
            local.get 6
            i32.store
            local.get 6
            local.get 4
            i32.store offset=24
            local.get 6
            local.get 6
            i32.store offset=12
            local.get 6
            local.get 6
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 4
          i32.load offset=8
          local.set 3
          local.get 4
          local.get 6
          i32.store offset=8
          local.get 3
          local.get 6
          i32.store offset=12
          local.get 6
          i32.const 0
          i32.store offset=24
          local.get 6
          local.get 3
          i32.store offset=8
          local.get 6
          local.get 4
          i32.store offset=12
        end
        local.get 8
        i32.const 8
        i32.add
        local.set 3
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 10
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 6
            local.get 6
            i32.load offset=28
            local.tee 0
            i32.const 2
            i32.shl
            i32.const 1060236
            i32.add
            local.tee 3
            i32.load
            i32.ne
            br_if 0 (;@4;)
            local.get 3
            local.get 8
            i32.store
            local.get 8
            br_if 1 (;@3;)
            i32.const 0
            local.get 9
            i32.const -2
            local.get 0
            i32.rotl
            i32.and
            i32.store offset=1059936
            br 2 (;@2;)
          end
          local.get 10
          i32.const 16
          i32.const 20
          local.get 10
          i32.load offset=16
          local.get 6
          i32.eq
          select
          i32.add
          local.get 8
          i32.store
          local.get 8
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 8
        local.get 10
        i32.store offset=24
        block  ;; label = @3
          local.get 6
          i32.load offset=16
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          local.get 3
          i32.store offset=16
          local.get 3
          local.get 8
          i32.store offset=24
        end
        local.get 6
        i32.const 20
        i32.add
        i32.load
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 8
        i32.const 20
        i32.add
        local.get 3
        i32.store
        local.get 3
        local.get 8
        i32.store offset=24
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 4
          i32.const 15
          i32.gt_u
          br_if 0 (;@3;)
          local.get 6
          local.get 4
          local.get 2
          i32.add
          local.tee 3
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 6
          local.get 3
          i32.add
          local.tee 3
          local.get 3
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 6
        local.get 2
        i32.add
        local.tee 0
        local.get 4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 6
        local.get 2
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 0
        local.get 4
        i32.add
        local.get 4
        i32.store
        block  ;; label = @3
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          local.get 7
          i32.const 3
          i32.shr_u
          local.tee 8
          i32.const 3
          i32.shl
          i32.const 1059972
          i32.add
          local.set 2
          i32.const 0
          i32.load offset=1059952
          local.set 3
          block  ;; label = @4
            block  ;; label = @5
              i32.const 1
              local.get 8
              i32.shl
              local.tee 8
              local.get 5
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              local.get 8
              local.get 5
              i32.or
              i32.store offset=1059932
              local.get 2
              local.set 8
              br 1 (;@4;)
            end
            local.get 2
            i32.load offset=8
            local.set 8
          end
          local.get 8
          local.get 3
          i32.store offset=12
          local.get 2
          local.get 3
          i32.store offset=8
          local.get 3
          local.get 2
          i32.store offset=12
          local.get 3
          local.get 8
          i32.store offset=8
        end
        i32.const 0
        local.get 0
        i32.store offset=1059952
        i32.const 0
        local.get 4
        i32.store offset=1059940
      end
      local.get 6
      i32.const 8
      i32.add
      local.set 3
    end
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 3
  )
  (func $free (type 0) (param i32)
    local.get 0
    call $dlfree
  )
  (func $dlfree (type 0) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const -8
      i32.add
      local.tee 1
      local.get 0
      i32.const -4
      i32.add
      i32.load
      local.tee 2
      i32.const -8
      i32.and
      local.tee 0
      i32.add
      local.set 3
      block  ;; label = @2
        local.get 2
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 2
        i32.const 3
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 1
        i32.load
        local.tee 2
        i32.sub
        local.tee 1
        i32.const 0
        i32.load offset=1059948
        local.tee 4
        i32.lt_u
        br_if 1 (;@1;)
        local.get 2
        local.get 0
        i32.add
        local.set 0
        block  ;; label = @3
          i32.const 0
          i32.load offset=1059952
          local.get 1
          i32.eq
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 2
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            local.get 1
            i32.load offset=12
            local.set 5
            block  ;; label = @5
              local.get 1
              i32.load offset=8
              local.tee 6
              local.get 2
              i32.const 3
              i32.shr_u
              local.tee 7
              i32.const 3
              i32.shl
              i32.const 1059972
              i32.add
              local.tee 2
              i32.eq
              br_if 0 (;@5;)
              local.get 4
              local.get 6
              i32.gt_u
              drop
            end
            block  ;; label = @5
              local.get 5
              local.get 6
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1059932
              i32.const -2
              local.get 7
              i32.rotl
              i32.and
              i32.store offset=1059932
              br 3 (;@2;)
            end
            block  ;; label = @5
              local.get 5
              local.get 2
              i32.eq
              br_if 0 (;@5;)
              local.get 4
              local.get 5
              i32.gt_u
              drop
            end
            local.get 5
            local.get 6
            i32.store offset=8
            local.get 6
            local.get 5
            i32.store offset=12
            br 2 (;@2;)
          end
          local.get 1
          i32.load offset=24
          local.set 7
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load offset=12
              local.tee 5
              local.get 1
              i32.eq
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 4
                local.get 1
                i32.load offset=8
                local.tee 2
                i32.gt_u
                br_if 0 (;@6;)
                local.get 2
                i32.load offset=12
                local.get 1
                i32.ne
                drop
              end
              local.get 5
              local.get 2
              i32.store offset=8
              local.get 2
              local.get 5
              i32.store offset=12
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 1
              i32.const 20
              i32.add
              local.tee 2
              i32.load
              local.tee 4
              br_if 0 (;@5;)
              local.get 1
              i32.const 16
              i32.add
              local.tee 2
              i32.load
              local.tee 4
              br_if 0 (;@5;)
              i32.const 0
              local.set 5
              br 1 (;@4;)
            end
            loop  ;; label = @5
              local.get 2
              local.set 6
              local.get 4
              local.tee 5
              i32.const 20
              i32.add
              local.tee 2
              i32.load
              local.tee 4
              br_if 0 (;@5;)
              local.get 5
              i32.const 16
              i32.add
              local.set 2
              local.get 5
              i32.load offset=16
              local.tee 4
              br_if 0 (;@5;)
            end
            local.get 6
            i32.const 0
            i32.store
          end
          local.get 7
          i32.eqz
          br_if 1 (;@2;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load offset=28
              local.tee 4
              i32.const 2
              i32.shl
              i32.const 1060236
              i32.add
              local.tee 2
              i32.load
              local.get 1
              i32.ne
              br_if 0 (;@5;)
              local.get 2
              local.get 5
              i32.store
              local.get 5
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=1059936
              i32.const -2
              local.get 4
              i32.rotl
              i32.and
              i32.store offset=1059936
              br 3 (;@2;)
            end
            local.get 7
            i32.const 16
            i32.const 20
            local.get 7
            i32.load offset=16
            local.get 1
            i32.eq
            select
            i32.add
            local.get 5
            i32.store
            local.get 5
            i32.eqz
            br_if 2 (;@2;)
          end
          local.get 5
          local.get 7
          i32.store offset=24
          block  ;; label = @4
            local.get 1
            i32.load offset=16
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            local.get 2
            i32.store offset=16
            local.get 2
            local.get 5
            i32.store offset=24
          end
          local.get 1
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 1 (;@2;)
          local.get 5
          i32.const 20
          i32.add
          local.get 2
          i32.store
          local.get 2
          local.get 5
          i32.store offset=24
          br 1 (;@2;)
        end
        local.get 3
        i32.load offset=4
        local.tee 2
        i32.const 3
        i32.and
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 3
        local.get 2
        i32.const -2
        i32.and
        i32.store offset=4
        i32.const 0
        local.get 0
        i32.store offset=1059940
        local.get 1
        local.get 0
        i32.add
        local.get 0
        i32.store
        local.get 1
        local.get 0
        i32.const 1
        i32.or
        i32.store offset=4
        return
      end
      local.get 3
      local.get 1
      i32.le_u
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=4
      local.tee 2
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const 2
          i32.and
          br_if 0 (;@3;)
          block  ;; label = @4
            i32.const 0
            i32.load offset=1059956
            local.get 3
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 1
            i32.store offset=1059956
            i32.const 0
            i32.const 0
            i32.load offset=1059944
            local.get 0
            i32.add
            local.tee 0
            i32.store offset=1059944
            local.get 1
            local.get 0
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            i32.const 0
            i32.load offset=1059952
            i32.ne
            br_if 3 (;@1;)
            i32.const 0
            i32.const 0
            i32.store offset=1059940
            i32.const 0
            i32.const 0
            i32.store offset=1059952
            return
          end
          block  ;; label = @4
            i32.const 0
            i32.load offset=1059952
            local.get 3
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 1
            i32.store offset=1059952
            i32.const 0
            i32.const 0
            i32.load offset=1059940
            local.get 0
            i32.add
            local.tee 0
            i32.store offset=1059940
            local.get 1
            local.get 0
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            local.get 0
            i32.add
            local.get 0
            i32.store
            return
          end
          local.get 2
          i32.const -8
          i32.and
          local.get 0
          i32.add
          local.set 0
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.const 255
              i32.gt_u
              br_if 0 (;@5;)
              local.get 3
              i32.load offset=12
              local.set 4
              block  ;; label = @6
                local.get 3
                i32.load offset=8
                local.tee 5
                local.get 2
                i32.const 3
                i32.shr_u
                local.tee 3
                i32.const 3
                i32.shl
                i32.const 1059972
                i32.add
                local.tee 2
                i32.eq
                br_if 0 (;@6;)
                i32.const 0
                i32.load offset=1059948
                local.get 5
                i32.gt_u
                drop
              end
              block  ;; label = @6
                local.get 4
                local.get 5
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                i32.const 0
                i32.load offset=1059932
                i32.const -2
                local.get 3
                i32.rotl
                i32.and
                i32.store offset=1059932
                br 2 (;@4;)
              end
              block  ;; label = @6
                local.get 4
                local.get 2
                i32.eq
                br_if 0 (;@6;)
                i32.const 0
                i32.load offset=1059948
                local.get 4
                i32.gt_u
                drop
              end
              local.get 4
              local.get 5
              i32.store offset=8
              local.get 5
              local.get 4
              i32.store offset=12
              br 1 (;@4;)
            end
            local.get 3
            i32.load offset=24
            local.set 7
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.load offset=12
                local.tee 5
                local.get 3
                i32.eq
                br_if 0 (;@6;)
                block  ;; label = @7
                  i32.const 0
                  i32.load offset=1059948
                  local.get 3
                  i32.load offset=8
                  local.tee 2
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 2
                  i32.load offset=12
                  local.get 3
                  i32.ne
                  drop
                end
                local.get 5
                local.get 2
                i32.store offset=8
                local.get 2
                local.get 5
                i32.store offset=12
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 3
                i32.const 20
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                local.get 3
                i32.const 16
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                i32.const 0
                local.set 5
                br 1 (;@5;)
              end
              loop  ;; label = @6
                local.get 2
                local.set 6
                local.get 4
                local.tee 5
                i32.const 20
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                local.get 5
                i32.const 16
                i32.add
                local.set 2
                local.get 5
                i32.load offset=16
                local.tee 4
                br_if 0 (;@6;)
              end
              local.get 6
              i32.const 0
              i32.store
            end
            local.get 7
            i32.eqz
            br_if 0 (;@4;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.load offset=28
                local.tee 4
                i32.const 2
                i32.shl
                i32.const 1060236
                i32.add
                local.tee 2
                i32.load
                local.get 3
                i32.ne
                br_if 0 (;@6;)
                local.get 2
                local.get 5
                i32.store
                local.get 5
                br_if 1 (;@5;)
                i32.const 0
                i32.const 0
                i32.load offset=1059936
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store offset=1059936
                br 2 (;@4;)
              end
              local.get 7
              i32.const 16
              i32.const 20
              local.get 7
              i32.load offset=16
              local.get 3
              i32.eq
              select
              i32.add
              local.get 5
              i32.store
              local.get 5
              i32.eqz
              br_if 1 (;@4;)
            end
            local.get 5
            local.get 7
            i32.store offset=24
            block  ;; label = @5
              local.get 3
              i32.load offset=16
              local.tee 2
              i32.eqz
              br_if 0 (;@5;)
              local.get 5
              local.get 2
              i32.store offset=16
              local.get 2
              local.get 5
              i32.store offset=24
            end
            local.get 3
            i32.load offset=20
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            i32.const 20
            i32.add
            local.get 2
            i32.store
            local.get 2
            local.get 5
            i32.store offset=24
          end
          local.get 1
          local.get 0
          i32.add
          local.get 0
          i32.store
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          i32.const 0
          i32.load offset=1059952
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 0
          i32.store offset=1059940
          return
        end
        local.get 3
        local.get 2
        i32.const -2
        i32.and
        i32.store offset=4
        local.get 1
        local.get 0
        i32.add
        local.get 0
        i32.store
        local.get 1
        local.get 0
        i32.const 1
        i32.or
        i32.store offset=4
      end
      block  ;; label = @2
        local.get 0
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 3
        i32.shr_u
        local.tee 2
        i32.const 3
        i32.shl
        i32.const 1059972
        i32.add
        local.set 0
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=1059932
            local.tee 4
            i32.const 1
            local.get 2
            i32.shl
            local.tee 2
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 4
            local.get 2
            i32.or
            i32.store offset=1059932
            local.get 0
            local.set 2
            br 1 (;@3;)
          end
          local.get 0
          i32.load offset=8
          local.set 2
        end
        local.get 2
        local.get 1
        i32.store offset=12
        local.get 0
        local.get 1
        i32.store offset=8
        local.get 1
        local.get 0
        i32.store offset=12
        local.get 1
        local.get 2
        i32.store offset=8
        return
      end
      i32.const 0
      local.set 2
      block  ;; label = @2
        local.get 0
        i32.const 8
        i32.shr_u
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        i32.const 31
        local.set 2
        local.get 0
        i32.const 16777215
        i32.gt_u
        br_if 0 (;@2;)
        local.get 4
        local.get 4
        i32.const 1048320
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 8
        i32.and
        local.tee 2
        i32.shl
        local.tee 4
        local.get 4
        i32.const 520192
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 4
        i32.and
        local.tee 4
        i32.shl
        local.tee 5
        local.get 5
        i32.const 245760
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 2
        i32.and
        local.tee 5
        i32.shl
        i32.const 15
        i32.shr_u
        local.get 4
        local.get 2
        i32.or
        local.get 5
        i32.or
        i32.sub
        local.tee 2
        i32.const 1
        i32.shl
        local.get 0
        local.get 2
        i32.const 21
        i32.add
        i32.shr_u
        i32.const 1
        i32.and
        i32.or
        i32.const 28
        i32.add
        local.set 2
      end
      local.get 1
      i64.const 0
      i64.store offset=16 align=4
      local.get 1
      i32.const 28
      i32.add
      local.get 2
      i32.store
      local.get 2
      i32.const 2
      i32.shl
      i32.const 1060236
      i32.add
      local.set 4
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.load offset=1059936
          local.tee 5
          i32.const 1
          local.get 2
          i32.shl
          local.tee 3
          i32.and
          br_if 0 (;@3;)
          local.get 4
          local.get 1
          i32.store
          i32.const 0
          local.get 5
          local.get 3
          i32.or
          i32.store offset=1059936
          local.get 1
          i32.const 24
          i32.add
          local.get 4
          i32.store
          local.get 1
          local.get 1
          i32.store offset=8
          local.get 1
          local.get 1
          i32.store offset=12
          br 1 (;@2;)
        end
        local.get 0
        i32.const 0
        i32.const 25
        local.get 2
        i32.const 1
        i32.shr_u
        i32.sub
        local.get 2
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set 2
        local.get 4
        i32.load
        local.set 5
        block  ;; label = @3
          loop  ;; label = @4
            local.get 5
            local.tee 4
            i32.load offset=4
            i32.const -8
            i32.and
            local.get 0
            i32.eq
            br_if 1 (;@3;)
            local.get 2
            i32.const 29
            i32.shr_u
            local.set 5
            local.get 2
            i32.const 1
            i32.shl
            local.set 2
            local.get 4
            local.get 5
            i32.const 4
            i32.and
            i32.add
            i32.const 16
            i32.add
            local.tee 3
            i32.load
            local.tee 5
            br_if 0 (;@4;)
          end
          local.get 3
          local.get 1
          i32.store
          local.get 1
          i32.const 24
          i32.add
          local.get 4
          i32.store
          local.get 1
          local.get 1
          i32.store offset=12
          local.get 1
          local.get 1
          i32.store offset=8
          br 1 (;@2;)
        end
        local.get 4
        i32.load offset=8
        local.set 0
        local.get 4
        local.get 1
        i32.store offset=8
        local.get 0
        local.get 1
        i32.store offset=12
        local.get 1
        i32.const 24
        i32.add
        i32.const 0
        i32.store
        local.get 1
        local.get 0
        i32.store offset=8
        local.get 1
        local.get 4
        i32.store offset=12
      end
      i32.const 0
      i32.const 0
      i32.load offset=1059964
      i32.const -1
      i32.add
      local.tee 1
      i32.store offset=1059964
      local.get 1
      br_if 0 (;@1;)
      i32.const 1060388
      local.set 1
      loop  ;; label = @2
        local.get 1
        i32.load
        local.tee 0
        i32.const 8
        i32.add
        local.set 1
        local.get 0
        br_if 0 (;@2;)
      end
      i32.const 0
      i32.const -1
      i32.store offset=1059964
    end
  )
  (func $calloc (type 2) (param i32 i32) (result i32)
    (local i32 i64)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      local.get 0
      i64.extend_i32_u
      local.get 1
      i64.extend_i32_u
      i64.mul
      local.tee 3
      i32.wrap_i64
      local.set 2
      local.get 1
      local.get 0
      i32.or
      i32.const 65536
      i32.lt_u
      br_if 0 (;@1;)
      i32.const -1
      local.get 2
      local.get 3
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      i32.const 0
      i32.ne
      select
      local.set 2
    end
    block  ;; label = @1
      local.get 2
      call $dlmalloc
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const -4
      i32.add
      i32.load8_u
      i32.const 3
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      local.get 2
      call $memset
      drop
    end
    local.get 0
  )
  (func $realloc (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      local.get 1
      call $dlmalloc
      return
    end
    block  ;; label = @1
      local.get 1
      i32.const -64
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 48
      i32.store offset=1060428
      i32.const 0
      return
    end
    local.get 1
    i32.const 11
    i32.lt_u
    local.set 2
    local.get 1
    i32.const 19
    i32.add
    i32.const -16
    i32.and
    local.set 3
    local.get 0
    i32.const -8
    i32.add
    local.set 4
    local.get 0
    i32.const -4
    i32.add
    local.tee 5
    i32.load
    local.tee 6
    i32.const 3
    i32.and
    local.set 7
    i32.const 0
    i32.load offset=1059948
    local.set 8
    block  ;; label = @1
      local.get 6
      i32.const -8
      i32.and
      local.tee 9
      i32.const 1
      i32.lt_s
      br_if 0 (;@1;)
      local.get 8
      local.get 4
      i32.gt_u
      br_if 0 (;@1;)
      local.get 7
      i32.const 1
      i32.eq
      drop
    end
    i32.const 16
    local.get 3
    local.get 2
    select
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 7
          br_if 0 (;@3;)
          local.get 2
          i32.const 256
          i32.lt_u
          br_if 1 (;@2;)
          local.get 9
          local.get 2
          i32.const 4
          i32.or
          i32.lt_u
          br_if 1 (;@2;)
          local.get 9
          local.get 2
          i32.sub
          i32.const 0
          i32.load offset=1060412
          i32.const 1
          i32.shl
          i32.le_u
          br_if 2 (;@1;)
          br 1 (;@2;)
        end
        local.get 4
        local.get 9
        i32.add
        local.set 7
        block  ;; label = @3
          local.get 9
          local.get 2
          i32.lt_u
          br_if 0 (;@3;)
          local.get 9
          local.get 2
          i32.sub
          local.tee 1
          i32.const 16
          i32.lt_u
          br_if 2 (;@1;)
          local.get 5
          local.get 2
          local.get 6
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 4
          local.get 2
          i32.add
          local.tee 2
          local.get 1
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 7
          local.get 7
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 2
          local.get 1
          call $dispose_chunk
          local.get 0
          return
        end
        block  ;; label = @3
          i32.const 0
          i32.load offset=1059956
          local.get 7
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1059944
          local.get 9
          i32.add
          local.tee 9
          local.get 2
          i32.le_u
          br_if 1 (;@2;)
          local.get 5
          local.get 2
          local.get 6
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          i32.const 0
          local.get 4
          local.get 2
          i32.add
          local.tee 1
          i32.store offset=1059956
          i32.const 0
          local.get 9
          local.get 2
          i32.sub
          local.tee 2
          i32.store offset=1059944
          local.get 1
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          return
        end
        block  ;; label = @3
          i32.const 0
          i32.load offset=1059952
          local.get 7
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1059940
          local.get 9
          i32.add
          local.tee 9
          local.get 2
          i32.lt_u
          br_if 1 (;@2;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 9
              local.get 2
              i32.sub
              local.tee 1
              i32.const 16
              i32.lt_u
              br_if 0 (;@5;)
              local.get 5
              local.get 2
              local.get 6
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 4
              local.get 2
              i32.add
              local.tee 2
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 4
              local.get 9
              i32.add
              local.tee 9
              local.get 1
              i32.store
              local.get 9
              local.get 9
              i32.load offset=4
              i32.const -2
              i32.and
              i32.store offset=4
              br 1 (;@4;)
            end
            local.get 5
            local.get 6
            i32.const 1
            i32.and
            local.get 9
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 4
            local.get 9
            i32.add
            local.tee 1
            local.get 1
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            i32.const 0
            local.set 1
            i32.const 0
            local.set 2
          end
          i32.const 0
          local.get 2
          i32.store offset=1059952
          i32.const 0
          local.get 1
          i32.store offset=1059940
          local.get 0
          return
        end
        local.get 7
        i32.load offset=4
        local.tee 3
        i32.const 2
        i32.and
        br_if 0 (;@2;)
        local.get 3
        i32.const -8
        i32.and
        local.get 9
        i32.add
        local.tee 10
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
        local.get 10
        local.get 2
        i32.sub
        local.set 11
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            local.get 7
            i32.load offset=12
            local.set 1
            block  ;; label = @5
              local.get 7
              i32.load offset=8
              local.tee 9
              local.get 3
              i32.const 3
              i32.shr_u
              local.tee 3
              i32.const 3
              i32.shl
              i32.const 1059972
              i32.add
              local.tee 7
              i32.eq
              br_if 0 (;@5;)
              local.get 8
              local.get 9
              i32.gt_u
              drop
            end
            block  ;; label = @5
              local.get 1
              local.get 9
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1059932
              i32.const -2
              local.get 3
              i32.rotl
              i32.and
              i32.store offset=1059932
              br 2 (;@3;)
            end
            block  ;; label = @5
              local.get 1
              local.get 7
              i32.eq
              br_if 0 (;@5;)
              local.get 8
              local.get 1
              i32.gt_u
              drop
            end
            local.get 1
            local.get 9
            i32.store offset=8
            local.get 9
            local.get 1
            i32.store offset=12
            br 1 (;@3;)
          end
          local.get 7
          i32.load offset=24
          local.set 12
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              i32.load offset=12
              local.tee 3
              local.get 7
              i32.eq
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 8
                local.get 7
                i32.load offset=8
                local.tee 1
                i32.gt_u
                br_if 0 (;@6;)
                local.get 1
                i32.load offset=12
                local.get 7
                i32.ne
                drop
              end
              local.get 3
              local.get 1
              i32.store offset=8
              local.get 1
              local.get 3
              i32.store offset=12
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 7
              i32.const 20
              i32.add
              local.tee 1
              i32.load
              local.tee 9
              br_if 0 (;@5;)
              local.get 7
              i32.const 16
              i32.add
              local.tee 1
              i32.load
              local.tee 9
              br_if 0 (;@5;)
              i32.const 0
              local.set 3
              br 1 (;@4;)
            end
            loop  ;; label = @5
              local.get 1
              local.set 8
              local.get 9
              local.tee 3
              i32.const 20
              i32.add
              local.tee 1
              i32.load
              local.tee 9
              br_if 0 (;@5;)
              local.get 3
              i32.const 16
              i32.add
              local.set 1
              local.get 3
              i32.load offset=16
              local.tee 9
              br_if 0 (;@5;)
            end
            local.get 8
            i32.const 0
            i32.store
          end
          local.get 12
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              i32.load offset=28
              local.tee 9
              i32.const 2
              i32.shl
              i32.const 1060236
              i32.add
              local.tee 1
              i32.load
              local.get 7
              i32.ne
              br_if 0 (;@5;)
              local.get 1
              local.get 3
              i32.store
              local.get 3
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=1059936
              i32.const -2
              local.get 9
              i32.rotl
              i32.and
              i32.store offset=1059936
              br 2 (;@3;)
            end
            local.get 12
            i32.const 16
            i32.const 20
            local.get 12
            i32.load offset=16
            local.get 7
            i32.eq
            select
            i32.add
            local.get 3
            i32.store
            local.get 3
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 3
          local.get 12
          i32.store offset=24
          block  ;; label = @4
            local.get 7
            i32.load offset=16
            local.tee 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            local.get 1
            i32.store offset=16
            local.get 1
            local.get 3
            i32.store offset=24
          end
          local.get 7
          i32.load offset=20
          local.tee 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          i32.const 20
          i32.add
          local.get 1
          i32.store
          local.get 1
          local.get 3
          i32.store offset=24
        end
        block  ;; label = @3
          local.get 11
          i32.const 15
          i32.gt_u
          br_if 0 (;@3;)
          local.get 5
          local.get 6
          i32.const 1
          i32.and
          local.get 10
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 4
          local.get 10
          i32.add
          local.tee 1
          local.get 1
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          return
        end
        local.get 5
        local.get 2
        local.get 6
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store
        local.get 4
        local.get 2
        i32.add
        local.tee 1
        local.get 11
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 4
        local.get 10
        i32.add
        local.tee 2
        local.get 2
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 1
        local.get 11
        call $dispose_chunk
        local.get 0
        return
      end
      block  ;; label = @2
        local.get 1
        call $dlmalloc
        local.tee 2
        br_if 0 (;@2;)
        i32.const 0
        return
      end
      local.get 2
      local.get 0
      i32.const -4
      i32.const -8
      local.get 5
      i32.load
      local.tee 9
      i32.const 3
      i32.and
      select
      local.get 9
      i32.const -8
      i32.and
      i32.add
      local.tee 9
      local.get 1
      local.get 9
      local.get 1
      i32.lt_u
      select
      call $memcpy
      local.set 1
      local.get 0
      call $dlfree
      local.get 1
      local.set 0
    end
    local.get 0
  )
  (func $dispose_chunk (type 5) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 0
    local.get 1
    i32.add
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 3
        i32.const 3
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.tee 3
        local.get 1
        i32.add
        local.set 1
        block  ;; label = @3
          i32.const 0
          i32.load offset=1059952
          local.get 0
          local.get 3
          i32.sub
          local.tee 0
          i32.eq
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1059948
          local.set 4
          block  ;; label = @4
            local.get 3
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            local.get 0
            i32.load offset=12
            local.set 5
            block  ;; label = @5
              local.get 0
              i32.load offset=8
              local.tee 6
              local.get 3
              i32.const 3
              i32.shr_u
              local.tee 7
              i32.const 3
              i32.shl
              i32.const 1059972
              i32.add
              local.tee 3
              i32.eq
              br_if 0 (;@5;)
              local.get 4
              local.get 6
              i32.gt_u
              drop
            end
            block  ;; label = @5
              local.get 5
              local.get 6
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1059932
              i32.const -2
              local.get 7
              i32.rotl
              i32.and
              i32.store offset=1059932
              br 3 (;@2;)
            end
            block  ;; label = @5
              local.get 5
              local.get 3
              i32.eq
              br_if 0 (;@5;)
              local.get 4
              local.get 5
              i32.gt_u
              drop
            end
            local.get 5
            local.get 6
            i32.store offset=8
            local.get 6
            local.get 5
            i32.store offset=12
            br 2 (;@2;)
          end
          local.get 0
          i32.load offset=24
          local.set 7
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load offset=12
              local.tee 6
              local.get 0
              i32.eq
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 4
                local.get 0
                i32.load offset=8
                local.tee 3
                i32.gt_u
                br_if 0 (;@6;)
                local.get 3
                i32.load offset=12
                local.get 0
                i32.ne
                drop
              end
              local.get 6
              local.get 3
              i32.store offset=8
              local.get 3
              local.get 6
              i32.store offset=12
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 0
              i32.const 20
              i32.add
              local.tee 3
              i32.load
              local.tee 5
              br_if 0 (;@5;)
              local.get 0
              i32.const 16
              i32.add
              local.tee 3
              i32.load
              local.tee 5
              br_if 0 (;@5;)
              i32.const 0
              local.set 6
              br 1 (;@4;)
            end
            loop  ;; label = @5
              local.get 3
              local.set 4
              local.get 5
              local.tee 6
              i32.const 20
              i32.add
              local.tee 3
              i32.load
              local.tee 5
              br_if 0 (;@5;)
              local.get 6
              i32.const 16
              i32.add
              local.set 3
              local.get 6
              i32.load offset=16
              local.tee 5
              br_if 0 (;@5;)
            end
            local.get 4
            i32.const 0
            i32.store
          end
          local.get 7
          i32.eqz
          br_if 1 (;@2;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load offset=28
              local.tee 5
              i32.const 2
              i32.shl
              i32.const 1060236
              i32.add
              local.tee 3
              i32.load
              local.get 0
              i32.ne
              br_if 0 (;@5;)
              local.get 3
              local.get 6
              i32.store
              local.get 6
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=1059936
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1059936
              br 3 (;@2;)
            end
            local.get 7
            i32.const 16
            i32.const 20
            local.get 7
            i32.load offset=16
            local.get 0
            i32.eq
            select
            i32.add
            local.get 6
            i32.store
            local.get 6
            i32.eqz
            br_if 2 (;@2;)
          end
          local.get 6
          local.get 7
          i32.store offset=24
          block  ;; label = @4
            local.get 0
            i32.load offset=16
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            local.get 3
            i32.store offset=16
            local.get 3
            local.get 6
            i32.store offset=24
          end
          local.get 0
          i32.load offset=20
          local.tee 3
          i32.eqz
          br_if 1 (;@2;)
          local.get 6
          i32.const 20
          i32.add
          local.get 3
          i32.store
          local.get 3
          local.get 6
          i32.store offset=24
          br 1 (;@2;)
        end
        local.get 2
        i32.load offset=4
        local.tee 3
        i32.const 3
        i32.and
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 2
        local.get 3
        i32.const -2
        i32.and
        i32.store offset=4
        i32.const 0
        local.get 1
        i32.store offset=1059940
        local.get 2
        local.get 1
        i32.store
        local.get 0
        local.get 1
        i32.const 1
        i32.or
        i32.store offset=4
        return
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.load offset=4
          local.tee 3
          i32.const 2
          i32.and
          br_if 0 (;@3;)
          block  ;; label = @4
            i32.const 0
            i32.load offset=1059956
            local.get 2
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 0
            i32.store offset=1059956
            i32.const 0
            i32.const 0
            i32.load offset=1059944
            local.get 1
            i32.add
            local.tee 1
            i32.store offset=1059944
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            i32.const 0
            i32.load offset=1059952
            i32.ne
            br_if 3 (;@1;)
            i32.const 0
            i32.const 0
            i32.store offset=1059940
            i32.const 0
            i32.const 0
            i32.store offset=1059952
            return
          end
          block  ;; label = @4
            i32.const 0
            i32.load offset=1059952
            local.get 2
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 0
            i32.store offset=1059952
            i32.const 0
            i32.const 0
            i32.load offset=1059940
            local.get 1
            i32.add
            local.tee 1
            i32.store offset=1059940
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            local.get 1
            i32.add
            local.get 1
            i32.store
            return
          end
          i32.const 0
          i32.load offset=1059948
          local.set 4
          local.get 3
          i32.const -8
          i32.and
          local.get 1
          i32.add
          local.set 1
          block  ;; label = @4
            block  ;; label = @5
              local.get 3
              i32.const 255
              i32.gt_u
              br_if 0 (;@5;)
              local.get 2
              i32.load offset=12
              local.set 5
              block  ;; label = @6
                local.get 2
                i32.load offset=8
                local.tee 6
                local.get 3
                i32.const 3
                i32.shr_u
                local.tee 2
                i32.const 3
                i32.shl
                i32.const 1059972
                i32.add
                local.tee 3
                i32.eq
                br_if 0 (;@6;)
                local.get 4
                local.get 6
                i32.gt_u
                drop
              end
              block  ;; label = @6
                local.get 5
                local.get 6
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                i32.const 0
                i32.load offset=1059932
                i32.const -2
                local.get 2
                i32.rotl
                i32.and
                i32.store offset=1059932
                br 2 (;@4;)
              end
              block  ;; label = @6
                local.get 5
                local.get 3
                i32.eq
                br_if 0 (;@6;)
                local.get 4
                local.get 5
                i32.gt_u
                drop
              end
              local.get 5
              local.get 6
              i32.store offset=8
              local.get 6
              local.get 5
              i32.store offset=12
              br 1 (;@4;)
            end
            local.get 2
            i32.load offset=24
            local.set 7
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.load offset=12
                local.tee 6
                local.get 2
                i32.eq
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 4
                  local.get 2
                  i32.load offset=8
                  local.tee 3
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 3
                  i32.load offset=12
                  local.get 2
                  i32.ne
                  drop
                end
                local.get 6
                local.get 3
                i32.store offset=8
                local.get 3
                local.get 6
                i32.store offset=12
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 2
                i32.const 20
                i32.add
                local.tee 3
                i32.load
                local.tee 5
                br_if 0 (;@6;)
                local.get 2
                i32.const 16
                i32.add
                local.tee 3
                i32.load
                local.tee 5
                br_if 0 (;@6;)
                i32.const 0
                local.set 6
                br 1 (;@5;)
              end
              loop  ;; label = @6
                local.get 3
                local.set 4
                local.get 5
                local.tee 6
                i32.const 20
                i32.add
                local.tee 3
                i32.load
                local.tee 5
                br_if 0 (;@6;)
                local.get 6
                i32.const 16
                i32.add
                local.set 3
                local.get 6
                i32.load offset=16
                local.tee 5
                br_if 0 (;@6;)
              end
              local.get 4
              i32.const 0
              i32.store
            end
            local.get 7
            i32.eqz
            br_if 0 (;@4;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.load offset=28
                local.tee 5
                i32.const 2
                i32.shl
                i32.const 1060236
                i32.add
                local.tee 3
                i32.load
                local.get 2
                i32.ne
                br_if 0 (;@6;)
                local.get 3
                local.get 6
                i32.store
                local.get 6
                br_if 1 (;@5;)
                i32.const 0
                i32.const 0
                i32.load offset=1059936
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1059936
                br 2 (;@4;)
              end
              local.get 7
              i32.const 16
              i32.const 20
              local.get 7
              i32.load offset=16
              local.get 2
              i32.eq
              select
              i32.add
              local.get 6
              i32.store
              local.get 6
              i32.eqz
              br_if 1 (;@4;)
            end
            local.get 6
            local.get 7
            i32.store offset=24
            block  ;; label = @5
              local.get 2
              i32.load offset=16
              local.tee 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 6
              local.get 3
              i32.store offset=16
              local.get 3
              local.get 6
              i32.store offset=24
            end
            local.get 2
            i32.load offset=20
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 20
            i32.add
            local.get 3
            i32.store
            local.get 3
            local.get 6
            i32.store offset=24
          end
          local.get 0
          local.get 1
          i32.add
          local.get 1
          i32.store
          local.get 0
          local.get 1
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          i32.const 0
          i32.load offset=1059952
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 1
          i32.store offset=1059940
          return
        end
        local.get 2
        local.get 3
        i32.const -2
        i32.and
        i32.store offset=4
        local.get 0
        local.get 1
        i32.add
        local.get 1
        i32.store
        local.get 0
        local.get 1
        i32.const 1
        i32.or
        i32.store offset=4
      end
      block  ;; label = @2
        local.get 1
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 3
        i32.shr_u
        local.tee 3
        i32.const 3
        i32.shl
        i32.const 1059972
        i32.add
        local.set 1
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=1059932
            local.tee 5
            i32.const 1
            local.get 3
            i32.shl
            local.tee 3
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 5
            local.get 3
            i32.or
            i32.store offset=1059932
            local.get 1
            local.set 3
            br 1 (;@3;)
          end
          local.get 1
          i32.load offset=8
          local.set 3
        end
        local.get 3
        local.get 0
        i32.store offset=12
        local.get 1
        local.get 0
        i32.store offset=8
        local.get 0
        local.get 1
        i32.store offset=12
        local.get 0
        local.get 3
        i32.store offset=8
        return
      end
      i32.const 0
      local.set 3
      block  ;; label = @2
        local.get 1
        i32.const 8
        i32.shr_u
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        i32.const 31
        local.set 3
        local.get 1
        i32.const 16777215
        i32.gt_u
        br_if 0 (;@2;)
        local.get 5
        local.get 5
        i32.const 1048320
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 8
        i32.and
        local.tee 3
        i32.shl
        local.tee 5
        local.get 5
        i32.const 520192
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 4
        i32.and
        local.tee 5
        i32.shl
        local.tee 6
        local.get 6
        i32.const 245760
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 2
        i32.and
        local.tee 6
        i32.shl
        i32.const 15
        i32.shr_u
        local.get 5
        local.get 3
        i32.or
        local.get 6
        i32.or
        i32.sub
        local.tee 3
        i32.const 1
        i32.shl
        local.get 1
        local.get 3
        i32.const 21
        i32.add
        i32.shr_u
        i32.const 1
        i32.and
        i32.or
        i32.const 28
        i32.add
        local.set 3
      end
      local.get 0
      i64.const 0
      i64.store offset=16 align=4
      local.get 0
      i32.const 28
      i32.add
      local.get 3
      i32.store
      local.get 3
      i32.const 2
      i32.shl
      i32.const 1060236
      i32.add
      local.set 5
      block  ;; label = @2
        i32.const 0
        i32.load offset=1059936
        local.tee 6
        i32.const 1
        local.get 3
        i32.shl
        local.tee 2
        i32.and
        br_if 0 (;@2;)
        local.get 5
        local.get 0
        i32.store
        i32.const 0
        local.get 6
        local.get 2
        i32.or
        i32.store offset=1059936
        local.get 0
        i32.const 24
        i32.add
        local.get 5
        i32.store
        local.get 0
        local.get 0
        i32.store offset=8
        local.get 0
        local.get 0
        i32.store offset=12
        return
      end
      local.get 1
      i32.const 0
      i32.const 25
      local.get 3
      i32.const 1
      i32.shr_u
      i32.sub
      local.get 3
      i32.const 31
      i32.eq
      select
      i32.shl
      local.set 3
      local.get 5
      i32.load
      local.set 6
      block  ;; label = @2
        loop  ;; label = @3
          local.get 6
          local.tee 5
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 1
          i32.eq
          br_if 1 (;@2;)
          local.get 3
          i32.const 29
          i32.shr_u
          local.set 6
          local.get 3
          i32.const 1
          i32.shl
          local.set 3
          local.get 5
          local.get 6
          i32.const 4
          i32.and
          i32.add
          i32.const 16
          i32.add
          local.tee 2
          i32.load
          local.tee 6
          br_if 0 (;@3;)
        end
        local.get 2
        local.get 0
        i32.store
        local.get 0
        i32.const 24
        i32.add
        local.get 5
        i32.store
        local.get 0
        local.get 0
        i32.store offset=12
        local.get 0
        local.get 0
        i32.store offset=8
        return
      end
      local.get 5
      i32.load offset=8
      local.set 1
      local.get 5
      local.get 0
      i32.store offset=8
      local.get 1
      local.get 0
      i32.store offset=12
      local.get 0
      i32.const 24
      i32.add
      i32.const 0
      i32.store
      local.get 0
      local.get 1
      i32.store offset=8
      local.get 0
      local.get 5
      i32.store offset=12
    end
  )
  (func $internal_memalign (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        local.get 0
        i32.const -1
        i32.add
        i32.and
        br_if 0 (;@2;)
        local.get 0
        local.set 2
        br 1 (;@1;)
      end
      i32.const 32
      local.set 3
      loop  ;; label = @2
        local.get 3
        local.tee 2
        i32.const 1
        i32.shl
        local.set 3
        local.get 2
        local.get 0
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      i32.const -64
      local.get 2
      i32.sub
      local.get 1
      i32.gt_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 48
      i32.store offset=1060428
      i32.const 0
      return
    end
    block  ;; label = @1
      i32.const 16
      local.get 1
      i32.const 19
      i32.add
      i32.const -16
      i32.and
      local.get 1
      i32.const 11
      i32.lt_u
      select
      local.tee 1
      i32.const 12
      i32.or
      local.get 2
      i32.add
      call $dlmalloc
      local.tee 3
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    local.get 3
    i32.const -8
    i32.add
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const -1
        i32.add
        local.get 3
        i32.and
        br_if 0 (;@2;)
        local.get 0
        local.set 2
        br 1 (;@1;)
      end
      local.get 3
      i32.const -4
      i32.add
      local.tee 4
      i32.load
      local.tee 5
      i32.const -8
      i32.and
      local.get 3
      local.get 2
      i32.add
      i32.const -1
      i32.add
      i32.const 0
      local.get 2
      i32.sub
      i32.and
      i32.const -8
      i32.add
      local.tee 3
      local.get 3
      local.get 2
      i32.add
      local.get 3
      local.get 0
      i32.sub
      i32.const 15
      i32.gt_u
      select
      local.tee 2
      local.get 0
      i32.sub
      local.tee 3
      i32.sub
      local.set 6
      block  ;; label = @2
        local.get 5
        i32.const 3
        i32.and
        br_if 0 (;@2;)
        local.get 2
        local.get 6
        i32.store offset=4
        local.get 2
        local.get 0
        i32.load
        local.get 3
        i32.add
        i32.store
        br 1 (;@1;)
      end
      local.get 2
      local.get 6
      local.get 2
      i32.load offset=4
      i32.const 1
      i32.and
      i32.or
      i32.const 2
      i32.or
      i32.store offset=4
      local.get 2
      local.get 6
      i32.add
      local.tee 6
      local.get 6
      i32.load offset=4
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 4
      local.get 3
      local.get 4
      i32.load
      i32.const 1
      i32.and
      i32.or
      i32.const 2
      i32.or
      i32.store
      local.get 2
      local.get 2
      i32.load offset=4
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 0
      local.get 3
      call $dispose_chunk
    end
    block  ;; label = @1
      local.get 2
      i32.load offset=4
      local.tee 3
      i32.const 3
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const -8
      i32.and
      local.tee 0
      local.get 1
      i32.const 16
      i32.add
      i32.le_u
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      local.get 3
      i32.const 1
      i32.and
      i32.or
      i32.const 2
      i32.or
      i32.store offset=4
      local.get 2
      local.get 1
      i32.add
      local.tee 3
      local.get 0
      local.get 1
      i32.sub
      local.tee 1
      i32.const 3
      i32.or
      i32.store offset=4
      local.get 2
      local.get 0
      i32.add
      local.tee 0
      local.get 0
      i32.load offset=4
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 3
      local.get 1
      call $dispose_chunk
    end
    local.get 2
    i32.const 8
    i32.add
  )
  (func $aligned_alloc (type 2) (param i32 i32) (result i32)
    block  ;; label = @1
      local.get 0
      i32.const 16
      i32.gt_u
      br_if 0 (;@1;)
      local.get 1
      call $dlmalloc
      return
    end
    local.get 0
    local.get 1
    call $internal_memalign
  )
  (func $close (type 8) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      call $__wasi_fd_close
      local.tee 0
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    i32.const 0
    local.get 0
    i32.store offset=1060428
    i32.const -1
  )
  (func $abort (type 4)
    unreachable
    unreachable
  )
  (func $_Exit (type 0) (param i32)
    local.get 0
    call $__wasi_proc_exit
    unreachable
  )
  (func $internal_register_preopened_fd (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 2
        i32.add
        br_table 1 (;@1;) 1 (;@1;) 0 (;@2;)
      end
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        i32.const 0
        i32.load offset=1060432
        local.tee 2
        i32.const 0
        i32.load offset=1060440
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        i32.load offset=1060436
        local.set 3
        block  ;; label = @3
          i32.const 8
          local.get 2
          i32.const 1
          i32.shl
          i32.const 4
          local.get 2
          select
          local.tee 4
          call $calloc
          local.tee 5
          br_if 0 (;@3;)
          i32.const -1
          return
        end
        local.get 5
        local.get 3
        local.get 2
        i32.const 3
        i32.shl
        call $memcpy
        local.set 2
        i32.const 0
        local.get 4
        i32.store offset=1060440
        i32.const 0
        local.get 2
        i32.store offset=1060436
        local.get 3
        call $free
      end
      block  ;; label = @2
        loop  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              local.tee 2
              i32.load8_u
              i32.const -46
              i32.add
              br_table 1 (;@4;) 0 (;@5;) 3 (;@2;)
            end
            local.get 2
            i32.const 1
            i32.add
            local.set 1
            br 1 (;@3;)
          end
          local.get 2
          i32.const 1
          i32.add
          local.set 1
          local.get 2
          i32.load8_u offset=1
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          i32.const 47
          i32.ne
          br_if 1 (;@2;)
          local.get 2
          i32.const 2
          i32.add
          local.set 1
          br 0 (;@3;)
        end
      end
      block  ;; label = @2
        local.get 2
        call $strdup
        local.tee 2
        br_if 0 (;@2;)
        i32.const -1
        return
      end
      i32.const 0
      i32.const 0
      i32.load offset=1060432
      local.tee 1
      i32.const 1
      i32.add
      i32.store offset=1060432
      i32.const 0
      i32.load offset=1060436
      local.get 1
      i32.const 3
      i32.shl
      i32.add
      local.tee 1
      local.get 0
      i32.store offset=4
      local.get 1
      local.get 2
      i32.store
      i32.const 0
      return
    end
    call $abort
    unreachable
  )
  (func $__wasilibc_find_relpath (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 4
    local.get 3
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        local.get 2
        local.get 4
        i32.const 12
        i32.add
        i32.const 0
        call $undefined_weak:__wasilibc_find_relpath_alloc
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      local.get 1
      local.get 2
      call $__wasilibc_find_abspath
      local.set 0
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $__wasilibc_find_abspath (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    loop  ;; label = @1
      local.get 0
      local.tee 3
      i32.const 1
      i32.add
      local.set 0
      local.get 3
      i32.load8_u
      i32.const 47
      i32.eq
      br_if 0 (;@1;)
    end
    i32.const 0
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        i32.load offset=1060432
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        i32.load offset=1060436
        local.set 6
        i32.const -1
        local.set 7
        loop  ;; label = @3
          local.get 6
          local.get 5
          i32.const -1
          i32.add
          local.tee 5
          i32.const 3
          i32.shl
          i32.add
          local.tee 8
          i32.load
          local.tee 9
          call $strlen
          local.set 10
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              i32.const -1
              i32.eq
              br_if 0 (;@5;)
              local.get 10
              local.get 4
              i32.le_u
              br_if 1 (;@4;)
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 10
                br_if 0 (;@6;)
                local.get 3
                i32.load8_u
                i32.const 255
                i32.and
                i32.const 47
                i32.ne
                br_if 1 (;@5;)
              end
              local.get 3
              local.get 9
              local.get 10
              call $memcmp
              br_if 1 (;@4;)
              local.get 10
              i32.const -1
              i32.add
              local.set 11
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 11
                  local.tee 12
                  i32.const -1
                  i32.eq
                  br_if 1 (;@6;)
                  local.get 12
                  i32.const -1
                  i32.add
                  local.set 11
                  local.get 9
                  local.get 12
                  i32.add
                  i32.load8_u
                  i32.const 47
                  i32.eq
                  br_if 0 (;@7;)
                end
              end
              local.get 0
              local.get 12
              i32.add
              i32.load8_u
              local.tee 12
              i32.const 47
              i32.eq
              br_if 0 (;@5;)
              local.get 12
              br_if 1 (;@4;)
            end
            local.get 1
            local.get 9
            i32.store
            local.get 8
            i32.load offset=4
            local.set 7
            local.get 10
            local.set 4
          end
          local.get 5
          br_if 0 (;@3;)
        end
        local.get 7
        i32.const -1
        i32.ne
        br_if 1 (;@1;)
      end
      i32.const 0
      i32.const 44
      i32.store offset=1060428
      i32.const -1
      return
    end
    local.get 3
    local.get 4
    i32.add
    local.set 12
    loop (result i32)  ;; label = @1
      block  ;; label = @2
        local.get 12
        i32.load8_u
        local.tee 3
        i32.const 47
        i32.eq
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 3
          br_if 0 (;@3;)
          i32.const 1058116
          local.set 12
        end
        local.get 2
        local.get 12
        i32.store
        local.get 7
        return
      end
      local.get 12
      i32.const 1
      i32.add
      local.set 12
      br 0 (;@1;)
    end
  )
  (func $__wasilibc_populate_preopens (type 4)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    i32.const 3
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              local.get 1
              local.get 0
              i32.const 8
              i32.add
              call $__wasi_fd_prestat_get
              br_table 0 (;@5;) 3 (;@2;) 3 (;@2;) 3 (;@2;) 3 (;@2;) 3 (;@2;) 3 (;@2;) 3 (;@2;) 2 (;@3;) 3 (;@2;)
            end
            block  ;; label = @5
              local.get 0
              i32.load8_u offset=8
              br_if 0 (;@5;)
              local.get 0
              i32.load offset=12
              local.tee 2
              i32.const 1
              i32.add
              call $malloc
              local.tee 3
              i32.eqz
              br_if 4 (;@1;)
              local.get 1
              local.get 3
              local.get 2
              call $__wasi_fd_prestat_dir_name
              br_if 3 (;@2;)
              local.get 3
              local.get 0
              i32.load offset=12
              i32.add
              i32.const 0
              i32.store8
              local.get 1
              local.get 3
              call $internal_register_preopened_fd
              br_if 4 (;@1;)
              local.get 3
              call $free
            end
            local.get 1
            i32.const 1
            i32.add
            local.set 1
            br 0 (;@4;)
          end
        end
        local.get 0
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      i32.const 71
      call $_Exit
      unreachable
    end
    i32.const 70
    call $_Exit
    unreachable
  )
  (func $__wasi_environ_get (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_environ_get
    i32.const 65535
    i32.and
  )
  (func $__wasi_environ_sizes_get (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_environ_sizes_get
    i32.const 65535
    i32.and
  )
  (func $__wasi_fd_close (type 8) (param i32) (result i32)
    local.get 0
    call $__imported_wasi_snapshot_preview1_fd_close
    i32.const 65535
    i32.and
  )
  (func $__wasi_fd_prestat_get (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_fd_prestat_get
    i32.const 65535
    i32.and
  )
  (func $__wasi_fd_prestat_dir_name (type 1) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $__imported_wasi_snapshot_preview1_fd_prestat_dir_name
    i32.const 65535
    i32.and
  )
  (func $__wasi_proc_exit (type 0) (param i32)
    local.get 0
    call $__imported_wasi_snapshot_preview1_proc_exit
    unreachable
  )
  (func $sbrk (type 8) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      memory.size
      i32.const 16
      i32.shl
      return
    end
    block  ;; label = @1
      local.get 0
      i32.const 65535
      i32.and
      br_if 0 (;@1;)
      local.get 0
      i32.const -1
      i32.le_s
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 0
        i32.const 16
        i32.shr_u
        memory.grow
        local.tee 0
        i32.const -1
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        i32.const 48
        i32.store offset=1060428
        i32.const -1
        return
      end
      local.get 0
      i32.const 16
      i32.shl
      return
    end
    call $abort
    unreachable
  )
  (func $getcwd (type 2) (param i32 i32) (result i32)
    (local i32)
    i32.const 0
    i32.load offset=1059852
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        br_if 0 (;@2;)
        local.get 2
        call $strdup
        local.tee 0
        br_if 1 (;@1;)
        i32.const 0
        i32.const 48
        i32.store offset=1060428
        i32.const 0
        return
      end
      block  ;; label = @2
        local.get 2
        call $strlen
        i32.const 1
        i32.add
        local.get 1
        i32.le_u
        br_if 0 (;@2;)
        i32.const 0
        i32.const 68
        i32.store offset=1060428
        i32.const 0
        return
      end
      local.get 0
      local.get 2
      call $strcpy
      local.set 0
    end
    local.get 0
  )
  (func $dummy (type 4))
  (func $__wasm_call_dtors (type 4)
    call $dummy
    call $dummy
  )
  (func $exit (type 0) (param i32)
    call $dummy
    call $dummy
    local.get 0
    call $_Exit
    unreachable
  )
  (func $__wasilibc_ensure_environ (type 4)
    block  ;; label = @1
      i32.const 0
      i32.load offset=1059856
      i32.const -1
      i32.ne
      br_if 0 (;@1;)
      call $__wasilibc_initialize_environ
    end
  )
  (func $__wasilibc_initialize_environ (type 4)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.const 12
          i32.add
          local.get 0
          i32.const 8
          i32.add
          call $__wasi_environ_sizes_get
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 0
            i32.load offset=12
            local.tee 1
            br_if 0 (;@4;)
            i32.const 0
            i32.const 1060444
            i32.store offset=1059856
            br 3 (;@1;)
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.const 1
              i32.add
              local.tee 2
              local.get 1
              i32.lt_u
              br_if 0 (;@5;)
              local.get 0
              i32.load offset=8
              call $malloc
              local.tee 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              i32.const 4
              call $calloc
              local.tee 1
              br_if 1 (;@4;)
              local.get 3
              call $free
            end
            i32.const 70
            call $_Exit
            unreachable
          end
          local.get 1
          local.get 3
          call $__wasi_environ_get
          i32.eqz
          br_if 1 (;@2;)
          local.get 3
          call $free
          local.get 1
          call $free
        end
        i32.const 71
        call $_Exit
        unreachable
      end
      i32.const 0
      local.get 1
      i32.store offset=1059856
    end
    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $getenv (type 8) (param i32) (result i32)
    (local i32 i32 i32 i32)
    call $__wasilibc_ensure_environ
    i32.const 0
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.const 61
      call $__strchrnul
      local.tee 2
      local.get 0
      i32.sub
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.load8_u
      br_if 0 (;@1;)
      i32.const 0
      i32.load offset=1059856
      local.tee 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      i32.load
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      i32.const 4
      i32.add
      local.set 4
      block  ;; label = @2
        loop  ;; label = @3
          block  ;; label = @4
            local.get 0
            local.get 2
            local.get 3
            call $strncmp
            br_if 0 (;@4;)
            local.get 2
            local.get 3
            i32.add
            local.tee 2
            i32.load8_u
            i32.const 61
            i32.eq
            br_if 2 (;@2;)
          end
          local.get 4
          i32.load
          local.set 2
          local.get 4
          i32.const 4
          i32.add
          local.set 4
          local.get 2
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      local.get 2
      i32.const 1
      i32.add
      local.set 1
    end
    local.get 1
  )
  (func $memcmp (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 3
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        loop  ;; label = @3
          local.get 0
          i32.load8_u
          local.tee 4
          local.get 1
          i32.load8_u
          local.tee 5
          i32.ne
          br_if 1 (;@2;)
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 2
          i32.const -1
          i32.add
          local.tee 2
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      local.get 4
      local.get 5
      i32.sub
      local.set 3
    end
    local.get 3
  )
  (func $strdup (type 8) (param i32) (result i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      call $strlen
      i32.const 1
      i32.add
      local.tee 1
      call $malloc
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 0
      local.get 1
      call $memcpy
      drop
    end
    local.get 2
  )
  (func $strlen (type 8) (param i32) (result i32)
    (local i32 i32 i32)
    local.get 0
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 0
            i32.load8_u
            br_if 0 (;@4;)
            local.get 0
            local.get 0
            i32.sub
            return
          end
          local.get 0
          i32.const 1
          i32.add
          local.set 1
          loop  ;; label = @4
            local.get 1
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 1
            i32.load8_u
            local.set 2
            local.get 1
            i32.const 1
            i32.add
            local.tee 3
            local.set 1
            local.get 2
            i32.eqz
            br_if 2 (;@2;)
            br 0 (;@4;)
          end
        end
        local.get 1
        i32.const -4
        i32.add
        local.set 1
        loop  ;; label = @3
          local.get 1
          i32.const 4
          i32.add
          local.tee 1
          i32.load
          local.tee 2
          i32.const -1
          i32.xor
          local.get 2
          i32.const -16843009
          i32.add
          i32.and
          i32.const -2139062144
          i32.and
          i32.eqz
          br_if 0 (;@3;)
        end
        block  ;; label = @3
          local.get 2
          i32.const 255
          i32.and
          br_if 0 (;@3;)
          local.get 1
          local.get 0
          i32.sub
          return
        end
        loop  ;; label = @3
          local.get 1
          i32.load8_u offset=1
          local.set 2
          local.get 1
          i32.const 1
          i32.add
          local.tee 3
          local.set 1
          local.get 2
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      local.get 3
      i32.const -1
      i32.add
      local.set 3
    end
    local.get 3
    local.get 0
    i32.sub
  )
  (func $strerror (type 8) (param i32) (result i32)
    (local i32)
    block  ;; label = @1
      i32.const 0
      i32.load offset=1060472
      local.tee 1
      br_if 0 (;@1;)
      i32.const 1060448
      local.set 1
      i32.const 0
      i32.const 1060448
      i32.store offset=1060472
    end
    i32.const 0
    local.get 0
    local.get 0
    i32.const 76
    i32.gt_u
    select
    i32.const 1
    i32.shl
    i32.const 1059680
    i32.add
    i32.load16_u
    i32.const 1058120
    i32.add
    local.get 1
    i32.load offset=20
    call $__lctrans
  )
  (func $strerror_r (type 1) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        call $strerror
        local.tee 0
        call $strlen
        local.tee 3
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
        i32.const 68
        local.set 3
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 0
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        call $memcpy
        local.get 2
        i32.add
        i32.const 0
        i32.store8
        i32.const 68
        return
      end
      local.get 1
      local.get 0
      local.get 3
      i32.const 1
      i32.add
      call $memcpy
      drop
      i32.const 0
      local.set 3
    end
    local.get 3
  )
  (func $__stpcpy (type 2) (param i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.get 0
        i32.xor
        i32.const 3
        i32.and
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 1
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 0
            local.get 1
            i32.load8_u
            local.tee 2
            i32.store8
            local.get 2
            i32.eqz
            br_if 3 (;@1;)
            local.get 0
            i32.const 1
            i32.add
            local.set 0
            local.get 1
            i32.const 1
            i32.add
            local.tee 1
            i32.const 3
            i32.and
            br_if 0 (;@4;)
          end
        end
        local.get 1
        i32.load
        local.tee 2
        i32.const -1
        i32.xor
        local.get 2
        i32.const -16843009
        i32.add
        i32.and
        i32.const -2139062144
        i32.and
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 0
          local.get 2
          i32.store
          local.get 1
          i32.load offset=4
          local.set 2
          local.get 0
          i32.const 4
          i32.add
          local.set 0
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 2
          i32.const -1
          i32.xor
          local.get 2
          i32.const -16843009
          i32.add
          i32.and
          i32.const -2139062144
          i32.and
          i32.eqz
          br_if 0 (;@3;)
        end
      end
      local.get 0
      local.get 1
      i32.load8_u
      local.tee 2
      i32.store8
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 1
      i32.add
      local.set 1
      loop  ;; label = @2
        local.get 0
        local.get 1
        i32.load8_u
        local.tee 2
        i32.store8 offset=1
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 2
        br_if 0 (;@2;)
      end
    end
    local.get 0
  )
  (func $strcpy (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__stpcpy
    drop
    local.get 0
  )
  (func $memcpy (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        loop  ;; label = @3
          local.get 3
          local.get 1
          i32.load8_u
          i32.store8
          local.get 2
          i32.const -1
          i32.add
          local.set 4
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 2
          i32.const 1
          i32.eq
          br_if 2 (;@1;)
          local.get 4
          local.set 2
          local.get 1
          i32.const 3
          i32.and
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      local.get 2
      local.set 4
      local.get 0
      local.set 3
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.const 3
        i32.and
        local.tee 2
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 4
          i32.const 16
          i32.lt_u
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 3
            local.get 1
            i32.load
            i32.store
            local.get 3
            i32.const 4
            i32.add
            local.get 1
            i32.const 4
            i32.add
            i32.load
            i32.store
            local.get 3
            i32.const 8
            i32.add
            local.get 1
            i32.const 8
            i32.add
            i32.load
            i32.store
            local.get 3
            i32.const 12
            i32.add
            local.get 1
            i32.const 12
            i32.add
            i32.load
            i32.store
            local.get 3
            i32.const 16
            i32.add
            local.set 3
            local.get 1
            i32.const 16
            i32.add
            local.set 1
            local.get 4
            i32.const -16
            i32.add
            local.tee 4
            i32.const 15
            i32.gt_u
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 4
          i32.const 8
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 1
          i64.load align=4
          i64.store align=4
          local.get 1
          i32.const 8
          i32.add
          local.set 1
          local.get 3
          i32.const 8
          i32.add
          local.set 3
        end
        block  ;; label = @3
          local.get 4
          i32.const 4
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 1
          i32.load
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 3
          i32.const 4
          i32.add
          local.set 3
        end
        block  ;; label = @3
          local.get 4
          i32.const 2
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 1
          i32.load8_u
          i32.store8
          local.get 3
          local.get 1
          i32.load8_u offset=1
          i32.store8 offset=1
          local.get 3
          i32.const 2
          i32.add
          local.set 3
          local.get 1
          i32.const 2
          i32.add
          local.set 1
        end
        local.get 4
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 0
        return
      end
      block  ;; label = @2
        local.get 4
        i32.const 32
        i32.lt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.const -1
              i32.add
              br_table 0 (;@5;) 1 (;@4;) 2 (;@3;) 3 (;@2;)
            end
            local.get 3
            local.get 1
            i32.load8_u offset=1
            i32.store8 offset=1
            local.get 3
            local.get 1
            i32.load
            local.tee 5
            i32.store8
            local.get 3
            local.get 1
            i32.load8_u offset=2
            i32.store8 offset=2
            local.get 4
            i32.const -3
            i32.add
            local.set 4
            local.get 3
            i32.const 3
            i32.add
            local.set 6
            i32.const 0
            local.set 2
            loop  ;; label = @5
              local.get 6
              local.get 2
              i32.add
              local.tee 3
              local.get 1
              local.get 2
              i32.add
              local.tee 7
              i32.const 4
              i32.add
              i32.load
              local.tee 8
              i32.const 8
              i32.shl
              local.get 5
              i32.const 24
              i32.shr_u
              i32.or
              i32.store
              local.get 3
              i32.const 4
              i32.add
              local.get 7
              i32.const 8
              i32.add
              i32.load
              local.tee 5
              i32.const 8
              i32.shl
              local.get 8
              i32.const 24
              i32.shr_u
              i32.or
              i32.store
              local.get 3
              i32.const 8
              i32.add
              local.get 7
              i32.const 12
              i32.add
              i32.load
              local.tee 8
              i32.const 8
              i32.shl
              local.get 5
              i32.const 24
              i32.shr_u
              i32.or
              i32.store
              local.get 3
              i32.const 12
              i32.add
              local.get 7
              i32.const 16
              i32.add
              i32.load
              local.tee 5
              i32.const 8
              i32.shl
              local.get 8
              i32.const 24
              i32.shr_u
              i32.or
              i32.store
              local.get 2
              i32.const 16
              i32.add
              local.set 2
              local.get 4
              i32.const -16
              i32.add
              local.tee 4
              i32.const 16
              i32.gt_u
              br_if 0 (;@5;)
            end
            local.get 6
            local.get 2
            i32.add
            local.set 3
            local.get 1
            local.get 2
            i32.add
            i32.const 3
            i32.add
            local.set 1
            br 2 (;@2;)
          end
          local.get 3
          local.get 1
          i32.load
          local.tee 5
          i32.store8
          local.get 3
          local.get 1
          i32.load8_u offset=1
          i32.store8 offset=1
          local.get 4
          i32.const -2
          i32.add
          local.set 4
          local.get 3
          i32.const 2
          i32.add
          local.set 6
          i32.const 0
          local.set 2
          loop  ;; label = @4
            local.get 6
            local.get 2
            i32.add
            local.tee 3
            local.get 1
            local.get 2
            i32.add
            local.tee 7
            i32.const 4
            i32.add
            i32.load
            local.tee 8
            i32.const 16
            i32.shl
            local.get 5
            i32.const 16
            i32.shr_u
            i32.or
            i32.store
            local.get 3
            i32.const 4
            i32.add
            local.get 7
            i32.const 8
            i32.add
            i32.load
            local.tee 5
            i32.const 16
            i32.shl
            local.get 8
            i32.const 16
            i32.shr_u
            i32.or
            i32.store
            local.get 3
            i32.const 8
            i32.add
            local.get 7
            i32.const 12
            i32.add
            i32.load
            local.tee 8
            i32.const 16
            i32.shl
            local.get 5
            i32.const 16
            i32.shr_u
            i32.or
            i32.store
            local.get 3
            i32.const 12
            i32.add
            local.get 7
            i32.const 16
            i32.add
            i32.load
            local.tee 5
            i32.const 16
            i32.shl
            local.get 8
            i32.const 16
            i32.shr_u
            i32.or
            i32.store
            local.get 2
            i32.const 16
            i32.add
            local.set 2
            local.get 4
            i32.const -16
            i32.add
            local.tee 4
            i32.const 17
            i32.gt_u
            br_if 0 (;@4;)
          end
          local.get 6
          local.get 2
          i32.add
          local.set 3
          local.get 1
          local.get 2
          i32.add
          i32.const 2
          i32.add
          local.set 1
          br 1 (;@2;)
        end
        local.get 3
        local.get 1
        i32.load
        local.tee 5
        i32.store8
        local.get 4
        i32.const -1
        i32.add
        local.set 4
        local.get 3
        i32.const 1
        i32.add
        local.set 6
        i32.const 0
        local.set 2
        loop  ;; label = @3
          local.get 6
          local.get 2
          i32.add
          local.tee 3
          local.get 1
          local.get 2
          i32.add
          local.tee 7
          i32.const 4
          i32.add
          i32.load
          local.tee 8
          i32.const 24
          i32.shl
          local.get 5
          i32.const 8
          i32.shr_u
          i32.or
          i32.store
          local.get 3
          i32.const 4
          i32.add
          local.get 7
          i32.const 8
          i32.add
          i32.load
          local.tee 5
          i32.const 24
          i32.shl
          local.get 8
          i32.const 8
          i32.shr_u
          i32.or
          i32.store
          local.get 3
          i32.const 8
          i32.add
          local.get 7
          i32.const 12
          i32.add
          i32.load
          local.tee 8
          i32.const 24
          i32.shl
          local.get 5
          i32.const 8
          i32.shr_u
          i32.or
          i32.store
          local.get 3
          i32.const 12
          i32.add
          local.get 7
          i32.const 16
          i32.add
          i32.load
          local.tee 5
          i32.const 24
          i32.shl
          local.get 8
          i32.const 8
          i32.shr_u
          i32.or
          i32.store
          local.get 2
          i32.const 16
          i32.add
          local.set 2
          local.get 4
          i32.const -16
          i32.add
          local.tee 4
          i32.const 18
          i32.gt_u
          br_if 0 (;@3;)
        end
        local.get 6
        local.get 2
        i32.add
        local.set 3
        local.get 1
        local.get 2
        i32.add
        i32.const 1
        i32.add
        local.set 1
      end
      block  ;; label = @2
        local.get 4
        i32.const 16
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 1
        i32.load16_u align=1
        i32.store16 align=1
        local.get 3
        local.get 1
        i32.load8_u offset=2
        i32.store8 offset=2
        local.get 3
        local.get 1
        i32.load8_u offset=3
        i32.store8 offset=3
        local.get 3
        local.get 1
        i32.load8_u offset=4
        i32.store8 offset=4
        local.get 3
        local.get 1
        i32.load8_u offset=5
        i32.store8 offset=5
        local.get 3
        local.get 1
        i32.load8_u offset=6
        i32.store8 offset=6
        local.get 3
        local.get 1
        i32.load8_u offset=7
        i32.store8 offset=7
        local.get 3
        local.get 1
        i32.load8_u offset=8
        i32.store8 offset=8
        local.get 3
        local.get 1
        i32.load8_u offset=9
        i32.store8 offset=9
        local.get 3
        local.get 1
        i32.load8_u offset=10
        i32.store8 offset=10
        local.get 3
        local.get 1
        i32.load8_u offset=11
        i32.store8 offset=11
        local.get 3
        local.get 1
        i32.load8_u offset=12
        i32.store8 offset=12
        local.get 3
        local.get 1
        i32.load8_u offset=13
        i32.store8 offset=13
        local.get 3
        local.get 1
        i32.load8_u offset=14
        i32.store8 offset=14
        local.get 3
        local.get 1
        i32.load8_u offset=15
        i32.store8 offset=15
        local.get 3
        i32.const 16
        i32.add
        local.set 3
        local.get 1
        i32.const 16
        i32.add
        local.set 1
      end
      block  ;; label = @2
        local.get 4
        i32.const 8
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 3
        local.get 1
        i32.load8_u offset=1
        i32.store8 offset=1
        local.get 3
        local.get 1
        i32.load8_u offset=2
        i32.store8 offset=2
        local.get 3
        local.get 1
        i32.load8_u offset=3
        i32.store8 offset=3
        local.get 3
        local.get 1
        i32.load8_u offset=4
        i32.store8 offset=4
        local.get 3
        local.get 1
        i32.load8_u offset=5
        i32.store8 offset=5
        local.get 3
        local.get 1
        i32.load8_u offset=6
        i32.store8 offset=6
        local.get 3
        local.get 1
        i32.load8_u offset=7
        i32.store8 offset=7
        local.get 3
        i32.const 8
        i32.add
        local.set 3
        local.get 1
        i32.const 8
        i32.add
        local.set 1
      end
      block  ;; label = @2
        local.get 4
        i32.const 4
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 3
        local.get 1
        i32.load8_u offset=1
        i32.store8 offset=1
        local.get 3
        local.get 1
        i32.load8_u offset=2
        i32.store8 offset=2
        local.get 3
        local.get 1
        i32.load8_u offset=3
        i32.store8 offset=3
        local.get 3
        i32.const 4
        i32.add
        local.set 3
        local.get 1
        i32.const 4
        i32.add
        local.set 1
      end
      block  ;; label = @2
        local.get 4
        i32.const 2
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 3
        local.get 1
        i32.load8_u offset=1
        i32.store8 offset=1
        local.get 3
        i32.const 2
        i32.add
        local.set 3
        local.get 1
        i32.const 2
        i32.add
        local.set 1
      end
      local.get 4
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      local.get 1
      i32.load8_u
      i32.store8
    end
    local.get 0
  )
  (func $__strchrnul (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 1
      i32.const 255
      i32.and
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 0
            i32.load8_u
            local.tee 3
            i32.eqz
            br_if 2 (;@2;)
            local.get 3
            local.get 1
            i32.const 255
            i32.and
            i32.eq
            br_if 2 (;@2;)
            local.get 0
            i32.const 1
            i32.add
            local.tee 0
            i32.const 3
            i32.and
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 0
          i32.load
          local.tee 3
          i32.const -1
          i32.xor
          local.get 3
          i32.const -16843009
          i32.add
          i32.and
          i32.const -2139062144
          i32.and
          br_if 0 (;@3;)
          local.get 2
          i32.const 16843009
          i32.mul
          local.set 2
          loop  ;; label = @4
            local.get 3
            local.get 2
            i32.xor
            local.tee 3
            i32.const -1
            i32.xor
            local.get 3
            i32.const -16843009
            i32.add
            i32.and
            i32.const -2139062144
            i32.and
            br_if 1 (;@3;)
            local.get 0
            i32.load offset=4
            local.set 3
            local.get 0
            i32.const 4
            i32.add
            local.set 0
            local.get 3
            i32.const -1
            i32.xor
            local.get 3
            i32.const -16843009
            i32.add
            i32.and
            i32.const -2139062144
            i32.and
            i32.eqz
            br_if 0 (;@4;)
          end
        end
        local.get 0
        i32.const -1
        i32.add
        local.set 0
        loop  ;; label = @3
          local.get 0
          i32.const 1
          i32.add
          local.tee 0
          i32.load8_u
          local.tee 3
          i32.eqz
          br_if 1 (;@2;)
          local.get 3
          local.get 1
          i32.const 255
          i32.and
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 0
      return
    end
    local.get 0
    local.get 0
    call $strlen
    i32.add
  )
  (func $memset (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i64)
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store8
      local.get 2
      local.get 0
      i32.add
      local.tee 3
      i32.const -1
      i32.add
      local.get 1
      i32.store8
      local.get 2
      i32.const 3
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store8 offset=2
      local.get 0
      local.get 1
      i32.store8 offset=1
      local.get 3
      i32.const -3
      i32.add
      local.get 1
      i32.store8
      local.get 3
      i32.const -2
      i32.add
      local.get 1
      i32.store8
      local.get 2
      i32.const 7
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store8 offset=3
      local.get 3
      i32.const -4
      i32.add
      local.get 1
      i32.store8
      local.get 2
      i32.const 9
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 4
      i32.add
      local.tee 3
      local.get 1
      i32.const 255
      i32.and
      i32.const 16843009
      i32.mul
      local.tee 1
      i32.store
      local.get 3
      local.get 2
      local.get 4
      i32.sub
      i32.const -4
      i32.and
      local.tee 4
      i32.add
      local.tee 2
      i32.const -4
      i32.add
      local.get 1
      i32.store
      local.get 4
      i32.const 9
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      local.get 1
      i32.store offset=8
      local.get 3
      local.get 1
      i32.store offset=4
      local.get 2
      i32.const -8
      i32.add
      local.get 1
      i32.store
      local.get 2
      i32.const -12
      i32.add
      local.get 1
      i32.store
      local.get 4
      i32.const 25
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      local.get 1
      i32.store offset=24
      local.get 3
      local.get 1
      i32.store offset=20
      local.get 3
      local.get 1
      i32.store offset=16
      local.get 3
      local.get 1
      i32.store offset=12
      local.get 2
      i32.const -16
      i32.add
      local.get 1
      i32.store
      local.get 2
      i32.const -20
      i32.add
      local.get 1
      i32.store
      local.get 2
      i32.const -24
      i32.add
      local.get 1
      i32.store
      local.get 2
      i32.const -28
      i32.add
      local.get 1
      i32.store
      local.get 4
      local.get 3
      i32.const 4
      i32.and
      i32.const 24
      i32.or
      local.tee 5
      i32.sub
      local.tee 2
      i32.const 32
      i32.lt_u
      br_if 0 (;@1;)
      local.get 1
      i64.extend_i32_u
      local.tee 6
      i64.const 32
      i64.shl
      local.get 6
      i64.or
      local.set 6
      local.get 3
      local.get 5
      i32.add
      local.set 1
      loop  ;; label = @2
        local.get 1
        local.get 6
        i64.store
        local.get 1
        i32.const 24
        i32.add
        local.get 6
        i64.store
        local.get 1
        i32.const 16
        i32.add
        local.get 6
        i64.store
        local.get 1
        i32.const 8
        i32.add
        local.get 6
        i64.store
        local.get 1
        i32.const 32
        i32.add
        local.set 1
        local.get 2
        i32.const -32
        i32.add
        local.tee 2
        i32.const 31
        i32.gt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0
  )
  (func $strncmp (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    block  ;; label = @1
      local.get 2
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    i32.const 0
    local.set 3
    block  ;; label = @1
      local.get 0
      i32.load8_u
      local.tee 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 1
      i32.add
      local.set 0
      local.get 2
      i32.const -1
      i32.add
      local.set 2
      loop  ;; label = @2
        block  ;; label = @3
          local.get 4
          i32.const 255
          i32.and
          local.get 1
          i32.load8_u
          local.tee 5
          i32.eq
          br_if 0 (;@3;)
          local.get 4
          local.set 3
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 2
          br_if 0 (;@3;)
          local.get 4
          local.set 3
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 5
          br_if 0 (;@3;)
          local.get 4
          local.set 3
          br 2 (;@1;)
        end
        local.get 2
        i32.const -1
        i32.add
        local.set 2
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 0
        i32.load8_u
        local.set 4
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 4
        br_if 0 (;@2;)
      end
    end
    local.get 3
    i32.const 255
    i32.and
    local.get 1
    i32.load8_u
    i32.sub
  )
  (func $memmove (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      local.get 0
      local.get 1
      i32.eq
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 1
        local.get 0
        i32.sub
        local.get 2
        i32.sub
        i32.const 0
        local.get 2
        i32.const 1
        i32.shl
        i32.sub
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        local.get 2
        call $memcpy
        drop
        br 1 (;@1;)
      end
      local.get 1
      local.get 0
      i32.xor
      i32.const 3
      i32.and
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            local.get 1
            i32.ge_u
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              local.set 3
              br 3 (;@2;)
            end
            block  ;; label = @5
              local.get 0
              i32.const 3
              i32.and
              br_if 0 (;@5;)
              local.get 0
              local.set 3
              br 2 (;@3;)
            end
            local.get 0
            local.set 3
            loop  ;; label = @5
              local.get 2
              i32.eqz
              br_if 4 (;@1;)
              local.get 3
              local.get 1
              i32.load8_u
              i32.store8
              local.get 1
              i32.const 1
              i32.add
              local.set 1
              local.get 2
              i32.const -1
              i32.add
              local.set 2
              local.get 3
              i32.const 1
              i32.add
              local.tee 3
              i32.const 3
              i32.and
              i32.eqz
              br_if 2 (;@3;)
              br 0 (;@5;)
            end
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              local.set 3
              br 1 (;@4;)
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                local.get 2
                i32.add
                i32.const 3
                i32.and
                br_if 0 (;@6;)
                local.get 2
                local.set 3
                br 1 (;@5;)
              end
              local.get 1
              i32.const -1
              i32.add
              local.set 4
              local.get 0
              i32.const -1
              i32.add
              local.set 5
              loop  ;; label = @6
                local.get 2
                i32.eqz
                br_if 5 (;@1;)
                local.get 5
                local.get 2
                i32.add
                local.tee 6
                local.get 4
                local.get 2
                i32.add
                i32.load8_u
                i32.store8
                local.get 2
                i32.const -1
                i32.add
                local.tee 3
                local.set 2
                local.get 6
                i32.const 3
                i32.and
                br_if 0 (;@6;)
              end
            end
            local.get 3
            i32.const 4
            i32.lt_u
            br_if 0 (;@4;)
            local.get 0
            i32.const -4
            i32.add
            local.set 2
            local.get 1
            i32.const -4
            i32.add
            local.set 6
            loop  ;; label = @5
              local.get 2
              local.get 3
              i32.add
              local.get 6
              local.get 3
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const -4
              i32.add
              local.tee 3
              i32.const 3
              i32.gt_u
              br_if 0 (;@5;)
            end
          end
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 1
          i32.const -1
          i32.add
          local.set 1
          local.get 0
          i32.const -1
          i32.add
          local.set 2
          loop  ;; label = @4
            local.get 2
            local.get 3
            i32.add
            local.get 1
            local.get 3
            i32.add
            i32.load8_u
            i32.store8
            local.get 3
            i32.const -1
            i32.add
            local.tee 3
            br_if 0 (;@4;)
            br 3 (;@1;)
          end
        end
        local.get 2
        i32.const 4
        i32.lt_u
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 3
          local.get 1
          i32.load
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 3
          i32.const 4
          i32.add
          local.set 3
          local.get 2
          i32.const -4
          i32.add
          local.tee 2
          i32.const 3
          i32.gt_u
          br_if 0 (;@3;)
        end
      end
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        br_if 0 (;@2;)
      end
    end
    local.get 0
  )
  (func $#func205<dummy> (@name "dummy") (type 2) (param i32 i32) (result i32)
    local.get 0
  )
  (func $__lctrans (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $#func205<dummy>
  )
  (func $_start.command_export (type 4)
    call $__wasm_call_ctors
    call $_start
    call $__wasm_call_dtors
  )
  (func $main.command_export (type 2) (param i32 i32) (result i32)
    call $__wasm_call_ctors
    local.get 0
    local.get 1
    call $main
    call $__wasm_call_dtors
  )
  (table (;0;) 87 87 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1060480)
  (global (;2;) i32 i32.const 1060476)
  (export "memory" (memory 0))
  (export "__heap_base" (global 1))
  (export "__data_end" (global 2))
  (export "_start" (func $_start.command_export))
  (export "main" (func $main.command_export))
  (elem (;0;) (i32.const 1) func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h110f0df6209a13b2E $_ZN4core3ops8function6FnOnce9call_once17h0d605c8afa86665cE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h3c603a185463c3edE $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h0f2edd5418c91d47E $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17h26ce3e7a201dd52aE $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h7fb80c02ed96c242E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h174d90cd314a320fE $_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hdb42396925a5638aE $_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17h6c3bcede51e1d38fE $_ZN10hello_rust4main17hb681533656a5f6b2E $_ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17h259a58d95d5c98aeE $_ZN61_$LT$std..ffi..c_str..CString$u20$as$u20$core..fmt..Debug$GT$3fmt17h56771385d9418557E $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h8cd11e9fef8d604aE $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h775593b8d3a4c26dE $_ZN73_$LT$core..panic..panic_info..PanicInfo$u20$as$u20$core..fmt..Display$GT$3fmt17hf6025793d8663473E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h4f5e4ec6fd0d92cdE $_ZN91_$LT$std..sys_common..backtrace.._print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17hcd900b1d582d2005E $_ZN4core3ptr76drop_in_place$LT$std..io..Write..write_fmt..Adapter$LT$std..fs..File$GT$$GT$17hd00642ff593fcf18E $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hef47d23a38b516b0E $_ZN4core3fmt5Write10write_char17hf3707acfb2f0507dE $_ZN4core3fmt5Write9write_fmt17h32c0c646c526e6d7E $_ZN4core3ptr100drop_in_place$LT$$RF$mut$u20$std..io..Write..write_fmt..Adapter$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$17h6618fd488f4ae4e0E $_ZN53_$LT$core..fmt..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h20b05970f39fae96E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h3912ee47733b7ba0E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h6d66c53f3322ec2aE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17he7114d91aaeb75baE $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h3f4d0c27f5d87265E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he9943eac91fbe3f2E $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17hb1584ad756e22b8fE $_ZN4core3fmt5Write10write_char17h48f1f74822f1d37aE $_ZN4core3fmt5Write9write_fmt17hd301836d8c71db60E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hbf33bc403fdf05b1E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h8d24c0da07505a80E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hd434d93e5b266e4aE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h12d461925b966368E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha1290dd7e4804cd6E $_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h212910dea3e9a103E $_ZN58_$LT$std..io..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h14ce87ca4c0ddcf8E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h75238bb108307ba7E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17ha5f9b44bf800e420E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h109e2202b03b6abdE $_ZN4core3ptr226drop_in_place$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$GT$17h8db58a1894934150E $_ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17h111faae4cf6dfba4E $_ZN242_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Debug$GT$3fmt17h9a83e5beffb7b670E $_ZN3std5error5Error5cause17h4a924e3a0ab474d1E $_ZN3std5error5Error7type_id17h732f71d4b4fa15d2E $_ZN3std5error5Error9backtrace17h46b4ece30348119aE $_ZN243_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$u20$as$u20$std..error..Error$GT$11description17h16c78575d1f51985E $_ZN62_$LT$std..io..error..ErrorKind$u20$as$u20$core..fmt..Debug$GT$3fmt17h47fccc56bfd5655dE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hd1c21b7c6ab1aac8E $_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17h7c8b9f25d8b78d37E.100 $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17hee063b8b83848866E $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h91afc922b67f4252E $_ZN4core3fmt5Write10write_char17h7a41aa260b58b7adE $_ZN4core3fmt5Write9write_fmt17h82208e6ac213d2bcE $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h05f5b7cc3782462eE $_ZN4core3fmt5Write10write_char17h907aac247470347aE $_ZN4core3fmt5Write9write_fmt17h30663a1432e14ffeE $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h22e1af2a31434bd2E $_ZN3std4sync4once4Once9call_once28_$u7b$$u7b$closure$u7d$$u7d$17h23a331af73a3a34fE $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hfd65f0ac2fab3244E $_ZN3std4sync4once4Once15call_once_force28_$u7b$$u7b$closure$u7d$$u7d$17h616156d4effed58bE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h65566335057149d7E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h6c7ecce5f4b16f20E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he06d51f3d3b8d64eE $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17h6605a725a9142927E $_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h1654d1d6387babd5E $_ZN4core3ptr103drop_in_place$LT$std..sync..poison..PoisonError$LT$std..sync..mutex..MutexGuard$LT$$LP$$RP$$GT$$GT$$GT$17hb4a0226b66dabfd4E $_ZN76_$LT$std..sync..poison..PoisonError$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd198f92e4a161b03E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hf6be660204d18b5cE $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hff14a3216932950dE $_ZN4core3ptr70drop_in_place$LT$std..panicking..begin_panic_handler..PanicPayload$GT$17h03f77c6f266c0a69E $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h57bbe70334078b97E $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17h04071562b34d7f4eE $_ZN93_$LT$std..panicking..begin_panic_handler..StrPanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h9be14b2fbf7fec1bE $_ZN93_$LT$std..panicking..begin_panic_handler..StrPanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17h4dd4073ca94e5808E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17hed18ade96c57a42bE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h5391ccdd75d11cb1E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h3ad948c51b49e3feE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hccd4d1b29a695c46E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h51b055fb88cf3b7bE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17he59324c441734165E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hf5346a345d2eb246E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h9dbd3b7280334987E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h0af0cbc9c0cb4d34E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h5166ee0e21e95d4aE)
  (data $.rodata (i32.const 1048576) "/rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/std/src/io/mod.rs\00\00\00\00\00\10\00I\00\00\00\f4\05\00\00!\00\00\00\12\00\00\00\0c\00\00\00\04\00\00\00\13\00\00\00\14\00\00\00\15\00\00\00library/alloc/src/raw_vec.rscapacity overflow\00\00\00\90\00\10\00\11\00\00\00t\00\10\00\1c\00\00\00\06\02\00\00\05\00\00\00a formatting trait implementation returned an errorlibrary/alloc/src/fmt.rs\00\ef\00\10\00\18\00\00\00U\02\00\00\1c\00\00\00\16\00\00\00\00\00\00\00\01\00\00\00\17\00\00\00\16\00\00\00\04\00\00\00\04\00\00\00\18\00\00\00\19\00\00\00\1a\00\00\00..\00\00@\01\10\00\02\00\00\00library/core/src/ascii.rs\00\00\00L\01\10\00\19\00\00\00v\00\00\00#\00\00\00BorrowMutError:\00\bc\11\10\00\00\00\00\00\86\01\10\00\01\00\00\00\86\01\10\00\01\00\00\00\16\00\00\00\00\00\00\00\01\00\00\00\1b\00\00\00panicked at '\00\00\00\bc\01\10\00\01\00\00\00$\1d\10\00\03\00\00\00index out of bounds: the len is  but the index is \00\00\d0\01\10\00 \00\00\00\f0\01\10\00\12\00\00\00\16\00\00\00\04\00\00\00\04\00\00\00\1c\00\00\00!===assertion failed: `(left  right)`\0a  left: ``,\0a right: ``: \00\00(\02\10\00\19\00\00\00A\02\10\00\12\00\00\00S\02\10\00\0c\00\00\00_\02\10\00\03\00\00\00`\00\00\00(\02\10\00\19\00\00\00A\02\10\00\12\00\00\00S\02\10\00\0c\00\00\00\84\02\10\00\01\00\00\00\bc\11\10\00\00\00\00\00\0c\13\10\00\02\00\00\00\16\00\00\00\0c\00\00\00\04\00\00\00\1d\00\00\00\1e\00\00\00\1f\00\00\00    library/core/src/fmt/builders.rs\d4\02\10\00 \00\00\00/\00\00\00!\00\00\00\d4\02\10\00 \00\00\000\00\00\00\12\00\00\00 {\0a,\0a,  { } { .. } }(\0a(,\16\00\00\00\04\00\00\00\04\00\00\00 \00\00\00library/core/src/fmt/num.rs\00<\03\10\00\1b\00\00\00e\00\00\00\14\00\00\000x00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\00\00\16\00\00\00\04\00\00\00\04\00\00\00!\00\00\00\22\00\00\00#\00\00\00library/core/src/fmt/mod.rstruefalseL\04\10\00\1b\00\00\00\86\08\00\00\1e\00\00\00L\04\10\00\1b\00\00\00\8d\08\00\00\16\00\00\00library/core/src/slice/memchr.rs\90\04\10\00 \00\00\00[\00\00\00\05\00\00\00\90\04\10\00 \00\00\00u\00\00\00\1a\00\00\00\90\04\10\00 \00\00\00\91\00\00\00\05\00\00\00range start index  out of range for slice of length \e0\04\10\00\12\00\00\00\f2\04\10\00\22\00\00\00range end index $\05\10\00\10\00\00\00\f2\04\10\00\22\00\00\00slice index starts at  but ends at \00D\05\10\00\16\00\00\00Z\05\10\00\0d\00\00\00\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\04\04\04\04\04\00\00\00\00\00\00\00\00\00\00\00[...]byte index  is out of bounds of `\00\00}\06\10\00\0b\00\00\00\88\06\10\00\16\00\00\00\84\02\10\00\01\00\00\00begin <= end ( <= ) when slicing `\00\00\b8\06\10\00\0e\00\00\00\c6\06\10\00\04\00\00\00\ca\06\10\00\10\00\00\00\84\02\10\00\01\00\00\00 is not a char boundary; it is inside  (bytes ) of `}\06\10\00\0b\00\00\00\fc\06\10\00&\00\00\00\22\07\10\00\08\00\00\00*\07\10\00\06\00\00\00\84\02\10\00\01\00\00\00library/core/src/str/mod.rs\00X\07\10\00\1b\00\00\00\f5\00\00\00\1d\00\00\00library/core/src/unicode/printable.rs\00\00\00\84\07\10\00%\00\00\00\0a\00\00\00\1c\00\00\00\84\07\10\00%\00\00\00\1a\00\00\006\00\00\00\00\01\03\05\05\06\06\02\07\06\08\07\09\11\0a\1c\0b\19\0c\1a\0d\10\0e\0d\0f\04\10\03\12\12\13\09\16\01\17\04\18\01\19\03\1a\07\1b\01\1c\02\1f\16 \03+\03-\0b.\010\031\022\01\a7\02\a9\02\aa\04\ab\08\fa\02\fb\05\fd\02\fe\03\ff\09\adxy\8b\8d\a20WX\8b\8c\90\1c\dd\0e\0fKL\fb\fc./?\5c]_\e2\84\8d\8e\91\92\a9\b1\ba\bb\c5\c6\c9\ca\de\e4\e5\ff\00\04\11\12)147:;=IJ]\84\8e\92\a9\b1\b4\ba\bb\c6\ca\ce\cf\e4\e5\00\04\0d\0e\11\12)14:;EFIJ^de\84\91\9b\9d\c9\ce\cf\0d\11):;EIW[\5c^_de\8d\91\a9\b4\ba\bb\c5\c9\df\e4\e5\f0\0d\11EIde\80\84\b2\bc\be\bf\d5\d7\f0\f1\83\85\8b\a4\a6\be\bf\c5\c7\ce\cf\da\dbH\98\bd\cd\c6\ce\cfINOWY^_\89\8e\8f\b1\b6\b7\bf\c1\c6\c7\d7\11\16\17[\5c\f6\f7\fe\ff\80mq\de\df\0e\1fno\1c\1d_}~\ae\af\7f\bb\bc\16\17\1e\1fFGNOXZ\5c^~\7f\b5\c5\d4\d5\dc\f0\f1\f5rs\8ftu\96&./\a7\af\b7\bf\c7\cf\d7\df\9a@\97\980\8f\1f\d2\d4\ce\ffNOZ[\07\08\0f\10'/\ee\efno7=?BE\90\91Sgu\c8\c9\d0\d1\d8\d9\e7\fe\ff\00 _\22\82\df\04\82D\08\1b\04\06\11\81\ac\0e\80\ab\05\1f\09\81\1b\03\19\08\01\04/\044\04\07\03\01\07\06\07\11\0aP\0f\12\07U\07\03\04\1c\0a\09\03\08\03\07\03\02\03\03\03\0c\04\05\03\0b\06\01\0e\15\05N\07\1b\07W\07\02\06\16\0dP\04C\03-\03\01\04\11\06\0f\0c:\04\1d%_ m\04j%\80\c8\05\82\b0\03\1a\06\82\fd\03Y\07\16\09\18\09\14\0c\14\0cj\06\0a\06\1a\06Y\07+\05F\0a,\04\0c\04\01\031\0b,\04\1a\06\0b\03\80\ac\06\0a\06/1M\03\80\a4\08<\03\0f\03<\078\08+\05\82\ff\11\18\08/\11-\03!\0f!\0f\80\8c\04\82\97\19\0b\15\88\94\05/\05;\07\02\0e\18\09\80\be\22t\0c\80\d6\1a\0c\05\80\ff\05\80\df\0c\f2\9d\037\09\81\5c\14\80\b8\08\80\cb\05\0a\18;\03\0a\068\08F\08\0c\06t\0b\1e\03Z\04Y\09\80\83\18\1c\0a\16\09L\04\80\8a\06\ab\a4\0c\17\041\a1\04\81\da&\07\0c\05\05\80\a6\10\81\f5\07\01 *\06L\04\80\8d\04\80\be\03\1b\03\0f\0d\00\06\01\01\03\01\04\02\05\07\07\02\08\08\09\02\0a\05\0b\02\0e\04\10\01\11\02\12\05\13\11\14\01\15\02\17\02\19\0d\1c\05\1d\08$\01j\04k\02\af\03\bc\02\cf\02\d1\02\d4\0c\d5\09\d6\02\d7\02\da\01\e0\05\e1\02\e7\04\e8\02\ee \f0\04\f8\02\fa\02\fb\01\0c';>NO\8f\9e\9e\9f{\8b\93\96\a2\b2\ba\86\b1\06\07\096=>V\f3\d0\d1\04\14\1867VW\7f\aa\ae\af\bd5\e0\12\87\89\8e\9e\04\0d\0e\11\12)14:EFIJNOde\5c\b6\b7\1b\1c\07\08\0a\0b\14\1769:\a8\a9\d8\d9\097\90\91\a8\07\0a;>fi\8f\92o_\bf\ee\efZb\f4\fc\ff\9a\9b./'(U\9d\a0\a1\a3\a4\a7\a8\ad\ba\bc\c4\06\0b\0c\15\1d:?EQ\a6\a7\cc\cd\a0\07\19\1a\22%>?\e7\ec\ef\ff\c5\c6\04 #%&(38:HJLPSUVXZ\5c^`cefksx}\7f\8a\a4\aa\af\b0\c0\d0\ae\afno\93^\22{\05\03\04-\03f\03\01/.\80\82\1d\031\0f\1c\04$\09\1e\05+\05D\04\0e*\80\aa\06$\04$\04(\084\0bNC\817\09\16\0a\08\18;E9\03c\08\090\16\05!\03\1b\05\01@8\04K\05/\04\0a\07\09\07@ '\04\0c\096\03:\05\1a\07\04\0c\07PI73\0d3\07.\08\0a\81&RN(\08*\16\1a&\1c\14\17\09N\04$\09D\0d\19\07\0a\06H\08'\09u\0b?A*\06;\05\0a\06Q\06\01\05\10\03\05\80\8bb\1eH\08\0a\80\a6^\22E\0b\0a\06\0d\13:\06\0a6,\04\17\80\b9<dS\0cH\09\0aFE\1bH\08S\0dI\81\07F\0a\1d\03GI7\03\0e\08\0a\069\07\0a\816\19\80\b7\01\0f2\0d\83\9bfu\0b\80\c4\8aLc\0d\84/\8f\d1\82G\a1\b9\829\07*\04\5c\06&\0aF\0a(\05\13\82\b0[eK\049\07\11@\05\0b\02\0e\97\f8\08\84\d6*\09\a2\e7\813-\03\11\04\08\81\8c\89\04k\05\0d\03\09\07\10\92`G\09t<\80\f6\0as\08p\15F\80\9a\14\0cW\09\19\80\87\81G\03\85B\0f\15\84P\1f\80\e1+\80\d5-\03\1a\04\02\81@\1f\11:\05\01\84\e0\80\f7)L\04\0a\04\02\83\11DL=\80\c2<\06\01\04U\05\1b4\02\81\0e,\04d\0cV\0a\80\ae8\1d\0d,\04\09\07\02\0e\06\80\9a\83\d8\05\10\03\0d\03t\0cY\07\0c\04\01\0f\0c\048\08\0a\06(\08\22N\81T\0c\15\03\05\03\07\09\1d\03\0b\05\06\0a\0a\06\08\08\07\09\80\cb%\0a\84\06library/core/src/unicode/unicode_data.rs\00\00\005\0d\10\00(\00\00\00K\00\00\00(\00\00\005\0d\10\00(\00\00\00W\00\00\00\16\00\00\005\0d\10\00(\00\00\00R\00\00\00>\00\00\00SomeNoneUtf8Errorvalid_up_toerror_len\00\00\00\16\00\00\00\04\00\00\00\04\00\00\00$\00\00\00\00\03\00\00\83\04 \00\91\05`\00]\13\a0\00\12\17 \1f\0c `\1f\ef,\a0+*0 ,o\a6\e0,\02\a8`-\1e\fb`.\00\fe 6\9e\ff`6\fd\01\e16\01\0a!7$\0d\e17\ab\0ea9/\18\a190\1c\e1G\f3\1e!L\f0j\e1OOo!P\9d\bc\a1P\00\cfaQe\d1\a1Q\00\da!R\00\e0\e1S0\e1aU\ae\e2\a1V\d0\e8\e1V \00nW\f0\01\ffW\00p\00\07\00-\01\01\01\02\01\02\01\01H\0b0\15\10\01e\07\02\06\02\02\01\04#\01\1e\1b[\0b:\09\09\01\18\04\01\09\01\03\01\05+\03<\08*\18\01 7\01\01\01\04\08\04\01\03\07\0a\02\1d\01:\01\01\01\02\04\08\01\09\01\0a\02\1a\01\02\029\01\04\02\04\02\02\03\03\01\1e\02\03\01\0b\029\01\04\05\01\02\04\01\14\02\16\06\01\01:\01\01\02\01\04\08\01\07\03\0a\02\1e\01;\01\01\01\0c\01\09\01(\01\03\017\01\01\03\05\03\01\04\07\02\0b\02\1d\01:\01\02\01\02\01\03\01\05\02\07\02\0b\02\1c\029\02\01\01\02\04\08\01\09\01\0a\02\1d\01H\01\04\01\02\03\01\01\08\01Q\01\02\07\0c\08b\01\02\09\0b\06J\02\1b\01\01\01\01\017\0e\01\05\01\02\05\0b\01$\09\01f\04\01\06\01\02\02\02\19\02\04\03\10\04\0d\01\02\02\06\01\0f\01\00\03\00\03\1d\02\1e\02\1e\02@\02\01\07\08\01\02\0b\09\01-\03\01\01u\02\22\01v\03\04\02\09\01\06\03\db\02\02\01:\01\01\07\01\01\01\01\02\08\06\0a\02\010\1f1\040\07\01\01\05\01(\09\0c\02 \04\02\02\01\038\01\01\02\03\01\01\03:\08\02\02\98\03\01\0d\01\07\04\01\06\01\03\02\c6@\00\01\c3!\00\03\8d\01` \00\06i\02\00\04\01\0a \02P\02\00\01\03\01\04\01\19\02\05\01\97\02\1a\12\0d\01&\08\19\0b.\030\01\02\04\02\02'\01C\06\02\02\02\02\0c\01\08\01/\013\01\01\03\02\02\05\02\01\01*\02\08\01\ee\01\02\01\04\01\00\01\00\10\10\10\00\02\00\01\e2\01\95\05\00\03\01\02\05\04(\03\04\01\a5\02\00\04\00\02\99\0b1\04{\016\0f)\01\02\02\0a\031\04\02\02\07\01=\03$\05\01\08>\01\0c\024\09\0a\04\02\01_\03\02\01\01\02\06\01\a0\01\03\08\15\029\02\01\01\01\01\16\01\0e\07\03\05\c3\08\02\03\01\01\17\01Q\01\02\06\01\01\02\01\01\02\01\02\eb\01\02\04\06\02\01\02\1b\02U\08\02\01\01\02j\01\01\01\02\06\01\01e\03\02\04\01\05\00\09\01\02\f5\01\0a\02\01\01\04\01\90\04\02\02\04\01 \0a(\06\02\04\08\01\09\06\02\03.\0d\01\02\00\07\01\06\01\01R\16\02\07\01\02\01\02z\06\03\01\01\02\01\07\01\01H\02\03\01\01\01\00\02\00\05;\07\00\01?\04Q\01\00\02\00.\02\17\00\01\01\03\04\05\08\08\02\07\1e\04\94\03\007\042\08\01\0e\01\16\05\01\0f\00\07\01\11\02\07\01\02\01\05\00\07\00\01=\04\00\07m\07\00`\80\f0\00Hello world!\0a\0b\11\10\00\0d\00\00\00/var/test.txt/home/yang/projects/xiaoxuan-vm/crates/native/resources/hello-rust.rs\00\00-\11\10\00E\00\00\00\10\00\00\006\00\00\00-\11\10\00E\00\00\00\11\00\00\00$\00\00\00%\00\00\00\08\00\00\00\04\00\00\00&\00\00\00\16\00\00\00\04\00\00\00\04\00\00\00'\00\00\00(\00\00\00)\00\00\00use of std::thread::current() is not possible after the thread's local data has been destroyedlibrary/std/src/thread/mod.rs\00\1a\12\10\00\1d\00\00\00\a2\02\00\00#\00\00\00failed to generate unique thread ID: bitspace exhausted\00H\12\10\007\00\00\00\1a\12\10\00\1d\00\00\00\10\04\00\00\11\00\00\00\1a\12\10\00\1d\00\00\00\16\04\00\00*\00\00\00\22RUST_BACKTRACE\00*\00\00\00\0c\00\00\00\04\00\00\00+\00\00\00*\00\00\00\0c\00\00\00\04\00\00\00,\00\00\00+\00\00\00\b8\12\10\00-\00\00\00.\00\00\00/\00\00\000\00\00\00-\00\00\00already borrowed\bc\11\10\00\00\00\00\00: \0ainternal error: entered unreachable codelibrary/std/src/ffi/c_str.rs\007\13\10\00\1c\00\00\00\8d\01\00\007\00\00\00\a8\12\10\00\01\00\00\00\00data provided contains a nul byte\00\00m\13\10\00!\00\00\00\14\00\00\00failed to write the buffered data\00\00\00\9c\13\10\00!\00\00\00\17\00\00\00library/std/src/io/buffered/bufwriter.rs\cc\13\10\00(\00\00\00\8d\00\00\00\12\00\00\00library/std/src/io/buffered/linewritershim.rs\00\00\00\04\14\10\00-\00\00\00\01\01\00\00)\00\00\00uncategorized errorother errorout of memoryunexpected end of fileunsupportedoperation interruptedargument list too longinvalid filenametoo many linkscross-device link or renamedeadlockexecutable file busyresource busyfile too largefilesystem quota exceededseek on unseekable fileno storage spacewrite zerotimed outinvalid datainvalid input parameterstale network file handlefilesystem loop or indirection limit (e.g. symlink loop)read-only filesystem or storage mediumdirectory not emptyis a directorynot a directoryoperation would blockentity already existsbroken pipenetwork downaddress not availableaddress in usenot connectedconnection abortednetwork unreachablehost unreachableconnection resetconnection refusedpermission deniedentity not foundErrorkind\00\00\16\00\00\00\01\00\00\00\01\00\00\001\00\00\00message\00\16\00\00\00\08\00\00\00\04\00\00\002\00\00\00KindOscode\00\00\16\00\00\00\04\00\00\00\04\00\00\003\00\00\00*\00\00\00\0c\00\00\00\04\00\00\004\00\00\00 (os error )\bc\11\10\00\00\00\00\00\90\17\10\00\0b\00\00\00\9b\17\10\00\01\00\00\00failed to write whole buffer\b4\17\10\00\1c\00\00\00\17\00\00\00library/std/src/io/stdio.rs\00\dc\17\10\00\1b\00\00\00\b9\02\00\00\13\00\00\00\dc\17\10\00\1b\00\00\00I\03\00\00\14\00\00\00failed printing to \00\18\18\10\00\13\00\00\00\0c\13\10\00\02\00\00\00\dc\17\10\00\1b\00\00\00\8c\04\00\00\09\00\00\00stdoutlibrary/std/src/io/mod.rs\00R\18\10\00\19\00\00\00\f4\05\00\00!\00\00\00formatter error\00|\18\10\00\0f\00\00\00(\00\00\00\12\00\00\00\0c\00\00\00\04\00\00\005\00\00\006\00\00\007\00\00\00\12\00\00\00\0c\00\00\00\04\00\00\008\00\00\009\00\00\00:\00\00\00input must be utf-8\00\c8\18\10\00\13\00\00\00(\00\00\00\ff\ff\ff\fflibrary/std/src/os/fd/owned.rs\00\00\ec\18\10\00\1e\00\00\00\8b\00\00\00\09\00\00\00library/std/src/panic.rs\1c\19\10\00\18\00\00\00\f0\00\00\00\12\00\00\00/rustc/7737e0b5c4103216d6fd8cf941b7ab9bdbaace7c/library/alloc/src/vec/mod.rsD\19\10\00L\00\00\008\07\00\00$\00\00\00\16\00\00\00\04\00\00\00\04\00\00\00;\00\00\00<\00\00\00library/std/src/sync/once.rs\b4\19\10\00\1c\00\00\00\0d\01\00\002\00\00\00\16\00\00\00\04\00\00\00\04\00\00\00=\00\00\00>\00\00\00\b4\19\10\00\1c\00\00\00G\01\00\001\00\00\00assertion failed: state_and_queue & STATE_MASK == RUNNING\00\00\00\b4\19\10\00\1c\00\00\00\b1\01\00\00\15\00\00\00Once instance has previously been poisoned\00\00P\1a\10\00*\00\00\00\b4\19\10\00\1c\00\00\00\90\01\00\00\15\00\00\00\02\00\00\00\b4\19\10\00\1c\00\00\00\f7\01\00\00\09\00\00\00\b4\19\10\00\1c\00\00\00\03\02\00\005\00\00\00PoisonErrorcalled `Option::unwrap()` on a `None` valuefatal runtime error: \00\ee\1a\10\00\15\00\00\00\0e\13\10\00\01\00\00\00stack backtrace:\0a\00\00\00\14\1b\10\00\11\00\00\00note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.\0a0\1b\10\00X\00\00\00\16\00\00\00\00\00\00\00\01\00\00\00\1b\00\00\00\16\00\00\00\04\00\00\00\04\00\00\00?\00\00\00\16\00\00\00\04\00\00\00\04\00\00\00@\00\00\00\16\00\00\00\04\00\00\00\04\00\00\00A\00\00\00library/std/src/sys_common/thread_info.rs\00\00\00\d0\1b\10\00)\00\00\00\16\00\00\003\00\00\00\d0\1b\10\00)\00\00\00+\00\00\00+\00\00\00assertion failed: thread_info.is_none()\00\1c\1c\10\00'\00\00\00\16\00\00\00\00\00\00\00\01\00\00\00B\00\00\00called `Result::unwrap()` on an `Err` value\00\16\00\00\00\08\00\00\00\04\00\00\00C\00\00\00D\00\00\00\08\00\00\00\04\00\00\00E\00\00\00memory allocation of  bytes failed\0a\00\a8\1c\10\00\15\00\00\00\bd\1c\10\00\0e\00\00\00library/std/src/panicking.rsBox<dyn Any><unnamed>thread '' panicked at '', \00\0d\1d\10\00\08\00\00\00\15\1d\10\00\0f\00\00\00$\1d\10\00\03\00\00\00\0e\13\10\00\01\00\00\00note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\0a\00\00H\1d\10\00N\00\00\00\dc\1c\10\00\1c\00\00\00G\02\00\00\1e\00\00\00*\00\00\00\0c\00\00\00\04\00\00\00F\00\00\00\16\00\00\00\08\00\00\00\04\00\00\00G\00\00\00H\00\00\00\10\00\00\00\04\00\00\00I\00\00\00J\00\00\00\16\00\00\00\08\00\00\00\04\00\00\00K\00\00\00L\00\00\00thread panicked while processing panic. aborting.\0a\00\00\f8\1d\10\002\00\00\00\0apanicked after panic::always_abort(), aborting.\0a\00\00\00\bc\11\10\00\00\00\00\004\1e\10\001\00\00\00thread panicked while panicking. aborting.\0a\00x\1e\10\00+\00\00\00\16\00\00\00\04\00\00\00\04\00\00\00M\00\00\00N\00\00\00O\00\00\00\16\00\00\00\04\00\00\00\04\00\00\00\18\00\00\00P\00\00\00Q\00\00\00\16\00\00\00\04\00\00\00\04\00\00\00R\00\00\00S\00\00\00T\00\00\00Unsupportederror\16\00\00\00\04\00\00\00\04\00\00\00U\00\00\00CustomUncategorizedOtherOutOfMemoryUnexpectedEofInterruptedArgumentListTooLongInvalidFilenameTooManyLinksCrossesDevicesDeadlockExecutableFileBusyResourceBusyFileTooLargeFilesystemQuotaExceededNotSeekableStorageFullWriteZeroTimedOutInvalidDataInvalidInputStaleNetworkFileHandleFilesystemLoopReadOnlyFilesystemDirectoryNotEmptyIsADirectoryNotADirectoryWouldBlockAlreadyExistsBrokenPipeNetworkDownAddrNotAvailableAddrInUseNotConnectedConnectionAbortedNetworkUnreachableHostUnreachableConnectionResetConnectionRefusedPermissionDeniedNotFoundcondvar wait not supported\00-!\10\00\1a\00\00\00library/std/src/sys/wasi/../unsupported/condvar.rs\00\00P!\10\002\00\00\00\17\00\00\00\09\00\00\00assertion failed: mid <= self.len()failed to find a pre-opened file descriptor through which  could be opened\00\00\00\b7!\10\00:\00\00\00\f1!\10\00\10\00\00\00cannot recursively acquire mutex\14\22\10\00 \00\00\00library/std/src/sys/wasi/../unsupported/mutex.rs<\22\10\000\00\00\00\17\00\00\00\09\00\00\00strerror_r failure\00\00|\22\10\00\12\00\00\00library/std/src/sys/wasi/os.rs\00\00\98\22\10\00\1e\00\00\00/\00\00\00\0d\00\00\00\98\22\10\00\1e\00\00\001\00\00\006\00\00\00rwlock locked for writing\00\00\00\d8\22\10\00\19\00\00\00library/std/src/sys_common/thread_parker/generic.rs\00\fc\22\10\003\00\00\00!\00\00\00&\00\00\00inconsistent park state\00@#\10\00\17\00\00\00\fc\22\10\003\00\00\00/\00\00\00\17\00\00\00park state changed unexpectedly\00p#\10\00\1f\00\00\00\fc\22\10\003\00\00\00,\00\00\00\11\00\00\00inconsistent state in unpark\a8#\10\00\1c\00\00\00\fc\22\10\003\00\00\00f\00\00\00\12\00\00\00\fc\22\10\003\00\00\00t\00\00\00\1f\00\00\00\16\00\00\00\04\00\00\00\04\00\00\00V\00\00\00\10\00\00\00\11\00\00\00\12\00\00\00\10\00\00\00\10\00\00\00\13\00\00\00\12\00\00\00\0d\00\00\00\0e\00\00\00\15\00\00\00\0c\00\00\00\0b\00\00\00\15\00\00\00\15\00\00\00\0f\00\00\00\0e\00\00\00\13\00\00\00&\00\00\008\00\00\00\19\00\00\00\17\00\00\00\0c\00\00\00\09\00\00\00\0a\00\00\00\10\00\00\00\17\00\00\00\19\00\00\00\0e\00\00\00\0d\00\00\00\14\00\00\00\08\00\00\00\1b\00\00\00\0e\00\00\00\10\00\00\00\16\00\00\00\15\00\00\00\0b\00\00\00\16\00\00\00\0d\00\00\00\0b\00\00\00\13\00\00\00!\17\10\00\10\17\10\00\fe\16\10\00\ee\16\10\00\de\16\10\00\cb\16\10\00\b9\16\10\00\ac\16\10\00\9e\16\10\00\89\16\10\00}\16\10\00r\16\10\00]\16\10\00H\16\10\009\16\10\00+\16\10\00\18\16\10\00\f2\15\10\00\ba\15\10\00\a1\15\10\00\8a\15\10\00~\15\10\00u\15\10\00k\15\10\00[\15\10\00D\15\10\00+\15\10\00\1d\15\10\00\10\15\10\00\fc\14\10\00\f4\14\10\00\d9\14\10\00\cb\14\10\00\bb\14\10\00\a5\14\10\00\90\14\10\00\85\14\10\00o\14\10\00b\14\10\00W\14\10\00D\14\10\00.\00/\00Success\00Illegal byte sequence\00Domain error\00Result not representable\00Not a tty\00Permission denied\00Operation not permitted\00No such file or directory\00No such process\00File exists\00Value too large for data type\00No space left on device\00Out of memory\00Resource busy\00Interrupted system call\00Resource temporarily unavailable\00Invalid seek\00Cross-device link\00Read-only file system\00Directory not empty\00Connection reset by peer\00Operation timed out\00Connection refused\00Host is unreachable\00Address in use\00Broken pipe\00I/O error\00No such device or address\00No such device\00Not a directory\00Is a directory\00Text file busy\00Exec format error\00Invalid argument\00Argument list too long\00Symbolic link loop\00Filename too long\00Too many open files in system\00No file descriptors available\00Bad file descriptor\00No child process\00Bad address\00File too large\00Too many links\00No locks available\00Resource deadlock would occur\00State not recoverable\00Previous owner died\00Operation canceled\00Function not implemented\00No message of desired type\00Identifier removed\00Link has been severed\00Protocol error\00Bad message\00Not a socket\00Destination address required\00Message too large\00Protocol wrong type for socket\00Protocol not available\00Protocol not supported\00Not supported\00Address family not supported by protocol\00Address not available\00Network is down\00Network unreachable\00Connection reset by network\00Connection aborted\00No buffer space available\00Socket is connected\00Socket not connected\00Operation already in progress\00Operation in progress\00Stale file handle\00Quota exceeded\00Multihop attempted\00Capabilities insufficient\00\00\00\00\00\00\00\00\00\00\00u\02N\00\d6\01\e2\04\b9\04\18\01\8e\05\ed\02\16\04\f2\00\97\03\01\038\05\af\01\82\01O\03/\04\1e\00\d4\05\a2\00\12\03\1e\03\c2\01\de\03\08\00\ac\05\00\01d\02\f1\01e\054\02\8c\02\cf\02-\03L\04\e3\05\9f\02\f8\04\1c\05\08\05\b1\02K\05\15\02x\00R\02<\03\f1\03\e4\00\c3\03}\04\cc\00\aa\03y\05$\02n\01m\03\22\04\ab\04D\00\fb\01\ae\00\83\03`\00\e5\01\07\04\94\04^\04+\00X\019\01\92\00\c2\05\9b\01C\02F\01\f6\05")
  (data $.data (i32.const 1059840) "\01\00\00\00\00\00\00\00\01\00\00\00F%\10\00\ff\ff\ff\ff")
)
