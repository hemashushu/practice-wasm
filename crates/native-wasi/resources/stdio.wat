(module
  (type (;0;) (func (param i32 i32 i32) (result i32)))
  (type (;1;) (func (param i32 i64 i32) (result i64)))
  (type (;2;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;3;) (func (param i32) (result i32)))
  (type (;4;) (func (param i32 i32) (result i32)))
  (type (;5;) (func (param i32 i64 i32 i32) (result i32)))
  (type (;6;) (func (param i32)))
  (type (;7;) (func (result i32)))
  (type (;8;) (func))
  (import "wasi_snapshot_preview1" "fd_close" (func (;0;) (type 3)))
  (import "wasi_snapshot_preview1" "fd_fdstat_get" (func (;1;) (type 4)))
  (import "wasi_snapshot_preview1" "fd_read" (func (;2;) (type 2)))
  (import "wasi_snapshot_preview1" "fd_seek" (func (;3;) (type 5)))
  (import "wasi_snapshot_preview1" "fd_write" (func (;4;) (type 2)))
  (func (;5;) (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 2
    i32.store offset=12
    local.get 3
    local.get 1
    i32.store offset=8
    block (result i32)  ;; label = @1
      local.get 0
      local.get 3
      i32.const 8
      i32.add
      i32.const 1
      local.get 3
      i32.const 4
      i32.add
      call 6
      local.tee 0
      if  ;; label = @2
        i32.const 1360
        i32.const 8
        local.get 0
        local.get 0
        i32.const 76
        i32.eq
        select
        i32.store
        i32.const -1
        br 1 (;@1;)
      end
      local.get 3
      i32.load offset=4
    end
    local.set 0
    local.get 3
    i32.const 16
    i32.add
    global.set 0
    local.get 0
  )
  (func (;6;) (type 2) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call 2
    i32.const 65535
    i32.and
  )
  (func (;7;) (type 2) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call 4
    i32.const 65535
    i32.and
  )
  (func (;8;) (type 6) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    i32.const -1
    i32.const 0
    block (result i32)  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          local.tee 2
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          i32.load8_u
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          i32.const 1
          i32.add
          local.tee 0
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.load8_u
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          i32.const 2
          i32.add
          local.tee 0
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.load8_u
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          i32.const 3
          i32.add
          local.tee 0
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.load8_u
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          i32.const 4
          i32.add
          local.set 0
        end
        local.get 0
        i32.const 4
        i32.sub
        local.set 0
        loop  ;; label = @3
          local.get 0
          i32.const 4
          i32.add
          local.tee 0
          i32.load
          local.tee 1
          i32.const -1
          i32.xor
          local.get 1
          i32.const 16843009
          i32.sub
          i32.and
          i32.const -2139062144
          i32.and
          i32.eqz
          br_if 0 (;@3;)
        end
        local.get 1
        i32.const 255
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 0
          i32.const 1
          i32.add
          local.tee 0
          i32.load8_u
          br_if 0 (;@3;)
        end
      end
      local.get 0
      local.get 2
      i32.sub
      local.set 0
      block  ;; label = @2
        local.get 0
        i32.const 1256
        i32.load
        local.tee 1
        if (result i32)  ;; label = @3
          local.get 1
        else
          i32.const 0
          local.set 1
          call 15
          br_if 1 (;@2;)
          i32.const 1256
          i32.load
        end
        i32.const 1260
        i32.load
        local.tee 5
        i32.sub
        i32.gt_u
        if  ;; label = @3
          i32.const 1240
          local.get 2
          local.get 0
          i32.const 1272
          i32.load
          call_indirect (type 0)
          local.set 1
          br 1 (;@2;)
        end
        block (result i32)  ;; label = @3
          local.get 0
          i32.const 1304
          i32.load
          i32.const 0
          i32.lt_s
          br_if 0 (;@3;)
          drop
          local.get 0
          local.get 2
          i32.add
          local.set 6
          i32.const 0
          local.set 1
          loop  ;; label = @4
            local.get 0
            local.get 0
            local.get 1
            i32.add
            i32.eqz
            br_if 1 (;@3;)
            drop
            local.get 1
            local.get 6
            i32.add
            local.set 7
            local.get 1
            i32.const 1
            i32.sub
            local.tee 4
            local.set 1
            local.get 7
            i32.const 1
            i32.sub
            i32.load8_u
            i32.const 10
            i32.ne
            br_if 0 (;@4;)
          end
          i32.const 1240
          local.get 2
          local.get 0
          local.get 4
          i32.add
          i32.const 1
          i32.add
          local.tee 3
          i32.const 1272
          i32.load
          call_indirect (type 0)
          local.tee 1
          local.get 3
          i32.lt_u
          br_if 1 (;@2;)
          local.get 4
          local.get 6
          i32.add
          i32.const 1
          i32.add
          local.set 2
          i32.const 1260
          i32.load
          local.set 5
          local.get 4
          i32.const -1
          i32.xor
        end
        local.set 1
        local.get 5
        local.get 2
        local.get 1
        call 16
        drop
        i32.const 1260
        i32.const 1260
        i32.load
        local.get 1
        i32.add
        i32.store
        local.get 1
        local.get 3
        i32.add
        local.set 1
      end
      local.get 0
      local.get 0
      local.get 1
      i32.eq
      br_if 0 (;@1;)
      drop
      local.get 1
    end
    local.get 0
    i32.ne
    select
    i32.const 0
    i32.lt_s
    if  ;; label = @1
      return
    end
    block  ;; label = @1
      i32.const 1304
      i32.load
      i32.const 10
      i32.eq
      br_if 0 (;@1;)
      i32.const 1260
      i32.load
      local.tee 0
      i32.const 1256
      i32.load
      i32.eq
      br_if 0 (;@1;)
      i32.const 1260
      local.get 0
      i32.const 1
      i32.add
      i32.store
      local.get 0
      i32.const 10
      i32.store8
      return
    end
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 10
    i32.store8 offset=15
    block  ;; label = @1
      block  ;; label = @2
        i32.const 1256
        i32.load
        local.tee 1
        if (result i32)  ;; label = @3
          local.get 1
        else
          call 15
          br_if 2 (;@1;)
          i32.const 1256
          i32.load
        end
        i32.const 1260
        i32.load
        local.tee 1
        i32.eq
        br_if 0 (;@2;)
        i32.const 1304
        i32.load
        i32.const 10
        i32.eq
        br_if 0 (;@2;)
        i32.const 1260
        local.get 1
        i32.const 1
        i32.add
        i32.store
        local.get 1
        i32.const 10
        i32.store8
        br 1 (;@1;)
      end
      i32.const 1240
      local.get 0
      i32.const 15
      i32.add
      i32.const 1
      i32.const 1272
      i32.load
      call_indirect (type 0)
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.load8_u offset=15
      drop
    end
    local.get 0
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;9;) (type 3) (param i32) (result i32)
    local.get 0
    i32.load offset=56
    call 0
    i32.const 65535
    i32.and
    local.tee 0
    if (result i32)  ;; label = @1
      i32.const 1360
      local.get 0
      i32.store
      i32.const -1
    else
      i32.const 0
    end
  )
  (func (;10;) (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 5
    global.set 0
    local.get 5
    local.get 1
    i32.store
    local.get 5
    local.get 0
    i32.load offset=44
    local.tee 4
    i32.store offset=12
    local.get 5
    local.get 0
    i32.load offset=40
    local.tee 6
    i32.store offset=8
    local.get 5
    local.get 2
    local.get 4
    i32.const 0
    i32.ne
    i32.sub
    local.tee 3
    i32.store offset=4
    block (result i32)  ;; label = @1
      local.get 3
      if  ;; label = @2
        local.get 0
        i32.load offset=56
        local.set 3
        global.get 0
        i32.const 16
        i32.sub
        local.tee 4
        global.set 0
        i32.const -1
        local.set 6
        block  ;; label = @3
          local.get 3
          local.get 5
          i32.const 2
          local.get 4
          i32.const 12
          i32.add
          call 6
          local.tee 3
          if  ;; label = @4
            i32.const 1360
            local.get 3
            i32.store
            br 1 (;@3;)
          end
          local.get 4
          i32.load offset=12
          local.set 6
        end
        local.get 4
        i32.const 16
        i32.add
        global.set 0
        local.get 6
        br 1 (;@1;)
      end
      local.get 0
      i32.load offset=56
      local.get 6
      local.get 4
      call 5
    end
    local.set 4
    i32.const 0
    local.set 3
    block  ;; label = @1
      local.get 4
      i32.const 0
      i32.le_s
      if  ;; label = @2
        local.get 0
        local.get 0
        i32.load
        i32.const 32
        i32.const 16
        local.get 4
        select
        i32.or
        i32.store
        br 1 (;@1;)
      end
      local.get 5
      i32.load offset=4
      local.tee 6
      local.get 4
      i32.ge_u
      if  ;; label = @2
        local.get 4
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      local.get 0
      i32.load offset=40
      local.tee 3
      i32.store offset=4
      local.get 0
      local.get 3
      local.get 4
      local.get 6
      i32.sub
      i32.add
      i32.store offset=8
      local.get 0
      i32.load offset=44
      if  ;; label = @2
        local.get 0
        local.get 3
        i32.const 1
        i32.add
        i32.store offset=4
        local.get 1
        local.get 2
        i32.add
        i32.const 1
        i32.sub
        local.get 3
        i32.load8_u
        i32.store8
      end
      local.get 2
      local.set 3
    end
    local.get 5
    i32.const 16
    i32.add
    global.set 0
    local.get 3
  )
  (func (;11;) (type 1) (param i32 i64 i32) (result i64)
    (local i32)
    local.get 0
    i32.load offset=56
    local.set 0
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    block (result i64)  ;; label = @1
      local.get 0
      local.get 1
      local.get 2
      i32.const 255
      i32.and
      local.get 3
      i32.const 8
      i32.add
      call 3
      i32.const 65535
      i32.and
      local.tee 0
      if  ;; label = @2
        i32.const 1360
        i32.const 70
        local.get 0
        local.get 0
        i32.const 76
        i32.eq
        select
        i32.store
        i64.const -1
        br 1 (;@1;)
      end
      local.get 3
      i64.load offset=8
    end
    local.set 1
    local.get 3
    i32.const 16
    i32.add
    global.set 0
    local.get 1
  )
  (func (;12;) (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    i32.const -1
    local.set 4
    block  ;; label = @1
      local.get 2
      i32.const 0
      i32.lt_s
      if  ;; label = @2
        i32.const 1360
        i32.const 28
        i32.store
        br 1 (;@1;)
      end
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      i32.const 12
      i32.add
      call 7
      local.tee 2
      if  ;; label = @2
        i32.const 1360
        local.get 2
        i32.store
        br 1 (;@1;)
      end
      local.get 3
      i32.load offset=12
      local.set 4
    end
    local.get 3
    i32.const 16
    i32.add
    global.set 0
    local.get 4
  )
  (func (;13;) (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 2
    i32.store offset=12
    local.get 3
    local.get 1
    i32.store offset=8
    local.get 3
    local.get 0
    i32.load offset=24
    local.tee 1
    i32.store
    local.get 3
    local.get 0
    i32.load offset=20
    local.get 1
    i32.sub
    local.tee 1
    i32.store offset=4
    i32.const 2
    local.set 6
    block (result i32)  ;; label = @1
      local.get 1
      local.get 2
      i32.add
      local.tee 9
      local.get 0
      i32.load offset=56
      local.get 3
      i32.const 2
      call 12
      local.tee 4
      i32.ne
      if  ;; label = @2
        local.get 3
        local.set 1
        loop  ;; label = @3
          local.get 4
          i32.const 0
          i32.lt_s
          if  ;; label = @4
            local.get 0
            i32.const 0
            i32.store offset=24
            local.get 0
            i64.const 0
            i64.store offset=16
            local.get 0
            local.get 0
            i32.load
            i32.const 32
            i32.or
            i32.store
            i32.const 0
            local.get 6
            i32.const 2
            i32.eq
            br_if 3 (;@1;)
            drop
            local.get 2
            local.get 1
            i32.load offset=4
            i32.sub
            br 3 (;@1;)
          end
          local.get 1
          local.get 4
          local.get 1
          i32.load offset=4
          local.tee 7
          i32.gt_u
          local.tee 5
          i32.const 3
          i32.shl
          i32.add
          local.tee 8
          local.get 4
          local.get 7
          i32.const 0
          local.get 5
          select
          i32.sub
          local.tee 7
          local.get 8
          i32.load
          i32.add
          i32.store
          local.get 1
          i32.const 12
          i32.const 4
          local.get 5
          select
          i32.add
          local.tee 8
          local.get 8
          i32.load
          local.get 7
          i32.sub
          i32.store
          local.get 9
          local.get 4
          i32.sub
          local.tee 9
          local.get 0
          i32.load offset=56
          local.get 1
          i32.const 8
          i32.add
          local.get 1
          local.get 5
          select
          local.tee 1
          local.get 6
          local.get 5
          i32.sub
          local.tee 6
          call 12
          local.tee 4
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 0
      local.get 0
      i32.load offset=40
      local.tee 1
      i32.store offset=24
      local.get 0
      local.get 1
      i32.store offset=20
      local.get 0
      local.get 1
      local.get 0
      i32.load offset=44
      i32.add
      i32.store offset=16
      local.get 2
    end
    local.set 4
    local.get 3
    i32.const 16
    i32.add
    global.set 0
    local.get 4
  )
  (func (;14;) (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32)
    local.get 0
    i32.const 5
    i32.store offset=32
    block  ;; label = @1
      local.get 0
      i32.load8_u
      i32.const 64
      i32.and
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=56
      local.set 3
      global.get 0
      i32.const 32
      i32.sub
      local.tee 4
      global.set 0
      block (result i32)  ;; label = @2
        block  ;; label = @3
          local.get 3
          local.get 4
          i32.const 8
          i32.add
          call 1
          i32.const 65535
          i32.and
          local.tee 3
          br_if 0 (;@3;)
          i32.const 59
          local.set 3
          local.get 4
          i32.load8_u offset=8
          i32.const 2
          i32.ne
          br_if 0 (;@3;)
          local.get 4
          i32.load8_u offset=16
          i32.const 36
          i32.and
          br_if 0 (;@3;)
          i32.const 1
          br 1 (;@2;)
        end
        i32.const 1360
        local.get 3
        i32.store
        i32.const 0
      end
      local.set 3
      local.get 4
      i32.const 32
      i32.add
      global.set 0
      local.get 3
      br_if 0 (;@1;)
      local.get 0
      i32.const -1
      i32.store offset=64
    end
    local.get 0
    local.get 1
    local.get 2
    call 13
  )
  (func (;15;) (type 7) (result i32)
    (local i32)
    i32.const 1300
    i32.const 1300
    i32.load
    local.tee 0
    i32.const 1
    i32.sub
    local.get 0
    i32.or
    i32.store
    i32.const 1240
    i32.load
    local.tee 0
    i32.const 8
    i32.and
    if  ;; label = @1
      i32.const 1240
      local.get 0
      i32.const 32
      i32.or
      i32.store
      i32.const -1
      return
    end
    i32.const 1244
    i64.const 0
    i64.store align=4
    i32.const 1264
    i32.const 1280
    i32.load
    local.tee 0
    i32.store
    i32.const 1260
    local.get 0
    i32.store
    i32.const 1256
    local.get 0
    i32.const 1284
    i32.load
    i32.add
    i32.store
    i32.const 0
  )
  (func (;16;) (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        i32.load8_u
        i32.store8
        local.get 2
        i32.const 1
        i32.sub
        local.set 5
        local.get 0
        i32.const 1
        i32.add
        local.set 3
        local.get 1
        i32.const 1
        i32.add
        local.tee 4
        i32.const 3
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 5
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        i32.load8_u offset=1
        i32.store8 offset=1
        local.get 2
        i32.const 2
        i32.sub
        local.set 5
        local.get 0
        i32.const 2
        i32.add
        local.set 3
        local.get 1
        i32.const 2
        i32.add
        local.tee 4
        i32.const 3
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 5
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        i32.load8_u offset=2
        i32.store8 offset=2
        local.get 2
        i32.const 3
        i32.sub
        local.set 5
        local.get 0
        i32.const 3
        i32.add
        local.set 3
        local.get 1
        i32.const 3
        i32.add
        local.tee 4
        i32.const 3
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 5
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        i32.load8_u offset=3
        i32.store8 offset=3
        local.get 2
        i32.const 4
        i32.sub
        local.set 5
        local.get 0
        i32.const 4
        i32.add
        local.set 3
        local.get 1
        i32.const 4
        i32.add
        local.set 4
        br 1 (;@1;)
      end
      local.get 2
      local.set 5
      local.get 0
      local.set 3
      local.get 1
      local.set 4
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.const 3
        i32.and
        local.tee 1
        i32.eqz
        if  ;; label = @3
          block  ;; label = @4
            local.get 5
            i32.const 16
            i32.ge_u
            if  ;; label = @5
              local.get 5
              i32.const 16
              i32.sub
              local.tee 1
              i32.const 16
              i32.and
              i32.eqz
              if  ;; label = @6
                local.get 3
                local.get 4
                i64.load align=4
                i64.store align=4
                local.get 3
                local.get 4
                i64.load offset=8 align=4
                i64.store offset=8 align=4
                local.get 3
                i32.const 16
                i32.add
                local.set 3
                local.get 4
                i32.const 16
                i32.add
                local.set 4
                local.get 1
                local.set 5
              end
              local.get 1
              i32.const 16
              i32.lt_u
              br_if 1 (;@4;)
              loop  ;; label = @6
                local.get 3
                local.get 4
                i64.load align=4
                i64.store align=4
                local.get 3
                i32.const 8
                i32.add
                local.get 4
                i32.const 8
                i32.add
                i64.load align=4
                i64.store align=4
                local.get 3
                i32.const 16
                i32.add
                local.get 4
                i32.const 16
                i32.add
                i64.load align=4
                i64.store align=4
                local.get 3
                i32.const 24
                i32.add
                local.get 4
                i32.const 24
                i32.add
                i64.load align=4
                i64.store align=4
                local.get 3
                i32.const 32
                i32.add
                local.set 3
                local.get 4
                i32.const 32
                i32.add
                local.set 4
                local.get 5
                i32.const 32
                i32.sub
                local.tee 5
                i32.const 15
                i32.gt_u
                br_if 0 (;@6;)
              end
            end
            local.get 5
            local.set 1
          end
          local.get 1
          i32.const 8
          i32.and
          if  ;; label = @4
            local.get 3
            local.get 4
            i64.load align=4
            i64.store align=4
            local.get 4
            i32.const 8
            i32.add
            local.set 4
            local.get 3
            i32.const 8
            i32.add
            local.set 3
          end
          local.get 1
          i32.const 4
          i32.and
          if  ;; label = @4
            local.get 3
            local.get 4
            i32.load
            i32.store
            local.get 4
            i32.const 4
            i32.add
            local.set 4
            local.get 3
            i32.const 4
            i32.add
            local.set 3
          end
          local.get 1
          i32.const 2
          i32.and
          if  ;; label = @4
            local.get 3
            local.get 4
            i32.load16_u align=1
            i32.store16 align=1
            local.get 4
            i32.const 2
            i32.add
            local.set 4
            local.get 3
            i32.const 2
            i32.add
            local.set 3
          end
          local.get 1
          i32.const 1
          i32.and
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 5
          i32.const 32
          i32.lt_u
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                i32.const 1
                i32.sub
                br_table 0 (;@6;) 1 (;@5;) 2 (;@4;) 3 (;@3;)
              end
              local.get 3
              local.get 4
              i32.load
              local.tee 6
              i32.store8
              local.get 3
              local.get 6
              i32.const 16
              i32.shr_u
              i32.store8 offset=2
              local.get 3
              local.get 6
              i32.const 8
              i32.shr_u
              i32.store8 offset=1
              local.get 5
              i32.const 3
              i32.sub
              local.set 5
              local.get 3
              i32.const 3
              i32.add
              local.set 8
              i32.const 0
              local.set 1
              loop  ;; label = @6
                local.get 1
                local.get 8
                i32.add
                local.tee 3
                local.get 1
                local.get 4
                i32.add
                local.tee 2
                i32.const 4
                i32.add
                i32.load
                local.tee 7
                i32.const 8
                i32.shl
                local.get 6
                i32.const 24
                i32.shr_u
                i32.or
                i32.store
                local.get 3
                i32.const 4
                i32.add
                local.get 2
                i32.const 8
                i32.add
                i32.load
                local.tee 6
                i32.const 8
                i32.shl
                local.get 7
                i32.const 24
                i32.shr_u
                i32.or
                i32.store
                local.get 3
                i32.const 8
                i32.add
                local.get 2
                i32.const 12
                i32.add
                i32.load
                local.tee 7
                i32.const 8
                i32.shl
                local.get 6
                i32.const 24
                i32.shr_u
                i32.or
                i32.store
                local.get 3
                i32.const 12
                i32.add
                local.get 2
                i32.const 16
                i32.add
                i32.load
                local.tee 6
                i32.const 8
                i32.shl
                local.get 7
                i32.const 24
                i32.shr_u
                i32.or
                i32.store
                local.get 1
                i32.const 16
                i32.add
                local.set 1
                local.get 5
                i32.const 16
                i32.sub
                local.tee 5
                i32.const 16
                i32.gt_u
                br_if 0 (;@6;)
              end
              local.get 1
              local.get 8
              i32.add
              local.set 3
              local.get 1
              local.get 4
              i32.add
              i32.const 3
              i32.add
              local.set 4
              br 2 (;@3;)
            end
            local.get 3
            local.get 4
            i32.load
            local.tee 6
            i32.store16 align=1
            local.get 5
            i32.const 2
            i32.sub
            local.set 5
            local.get 3
            i32.const 2
            i32.add
            local.set 8
            i32.const 0
            local.set 1
            loop  ;; label = @5
              local.get 1
              local.get 8
              i32.add
              local.tee 3
              local.get 1
              local.get 4
              i32.add
              local.tee 2
              i32.const 4
              i32.add
              i32.load
              local.tee 7
              i32.const 16
              i32.shl
              local.get 6
              i32.const 16
              i32.shr_u
              i32.or
              i32.store
              local.get 3
              i32.const 4
              i32.add
              local.get 2
              i32.const 8
              i32.add
              i32.load
              local.tee 6
              i32.const 16
              i32.shl
              local.get 7
              i32.const 16
              i32.shr_u
              i32.or
              i32.store
              local.get 3
              i32.const 8
              i32.add
              local.get 2
              i32.const 12
              i32.add
              i32.load
              local.tee 7
              i32.const 16
              i32.shl
              local.get 6
              i32.const 16
              i32.shr_u
              i32.or
              i32.store
              local.get 3
              i32.const 12
              i32.add
              local.get 2
              i32.const 16
              i32.add
              i32.load
              local.tee 6
              i32.const 16
              i32.shl
              local.get 7
              i32.const 16
              i32.shr_u
              i32.or
              i32.store
              local.get 1
              i32.const 16
              i32.add
              local.set 1
              local.get 5
              i32.const 16
              i32.sub
              local.tee 5
              i32.const 17
              i32.gt_u
              br_if 0 (;@5;)
            end
            local.get 1
            local.get 8
            i32.add
            local.set 3
            local.get 1
            local.get 4
            i32.add
            i32.const 2
            i32.add
            local.set 4
            br 1 (;@3;)
          end
          local.get 3
          local.get 4
          i32.load
          local.tee 6
          i32.store8
          local.get 5
          i32.const 1
          i32.sub
          local.set 5
          local.get 3
          i32.const 1
          i32.add
          local.set 8
          i32.const 0
          local.set 1
          loop  ;; label = @4
            local.get 1
            local.get 8
            i32.add
            local.tee 3
            local.get 1
            local.get 4
            i32.add
            local.tee 2
            i32.const 4
            i32.add
            i32.load
            local.tee 7
            i32.const 24
            i32.shl
            local.get 6
            i32.const 8
            i32.shr_u
            i32.or
            i32.store
            local.get 3
            i32.const 4
            i32.add
            local.get 2
            i32.const 8
            i32.add
            i32.load
            local.tee 6
            i32.const 24
            i32.shl
            local.get 7
            i32.const 8
            i32.shr_u
            i32.or
            i32.store
            local.get 3
            i32.const 8
            i32.add
            local.get 2
            i32.const 12
            i32.add
            i32.load
            local.tee 7
            i32.const 24
            i32.shl
            local.get 6
            i32.const 8
            i32.shr_u
            i32.or
            i32.store
            local.get 3
            i32.const 12
            i32.add
            local.get 2
            i32.const 16
            i32.add
            i32.load
            local.tee 6
            i32.const 24
            i32.shl
            local.get 7
            i32.const 8
            i32.shr_u
            i32.or
            i32.store
            local.get 1
            i32.const 16
            i32.add
            local.set 1
            local.get 5
            i32.const 16
            i32.sub
            local.tee 5
            i32.const 18
            i32.gt_u
            br_if 0 (;@4;)
          end
          local.get 1
          local.get 8
          i32.add
          local.set 3
          local.get 1
          local.get 4
          i32.add
          i32.const 1
          i32.add
          local.set 4
        end
        local.get 5
        i32.const 16
        i32.and
        if  ;; label = @3
          local.get 3
          local.get 4
          i32.load8_u
          i32.store8
          local.get 3
          local.get 4
          i32.load offset=1 align=1
          i32.store offset=1 align=1
          local.get 3
          local.get 4
          i64.load offset=5 align=1
          i64.store offset=5 align=1
          local.get 3
          local.get 4
          i32.load16_u offset=13 align=1
          i32.store16 offset=13 align=1
          local.get 3
          local.get 4
          i32.load8_u offset=15
          i32.store8 offset=15
          local.get 4
          i32.const 16
          i32.add
          local.set 4
          local.get 3
          i32.const 16
          i32.add
          local.set 3
        end
        local.get 5
        i32.const 8
        i32.and
        if  ;; label = @3
          local.get 3
          local.get 4
          i64.load align=1
          i64.store align=1
          local.get 4
          i32.const 8
          i32.add
          local.set 4
          local.get 3
          i32.const 8
          i32.add
          local.set 3
        end
        local.get 5
        i32.const 4
        i32.and
        if  ;; label = @3
          local.get 3
          local.get 4
          i32.load align=1
          i32.store align=1
          local.get 4
          i32.const 4
          i32.add
          local.set 4
          local.get 3
          i32.const 4
          i32.add
          local.set 3
        end
        local.get 5
        i32.const 2
        i32.and
        if  ;; label = @3
          local.get 3
          local.get 4
          i32.load16_u align=1
          i32.store16 align=1
          local.get 4
          i32.const 2
          i32.add
          local.set 4
          local.get 3
          i32.const 2
          i32.add
          local.set 3
        end
        local.get 5
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 3
      local.get 4
      i32.load8_u
      i32.store8
    end
    local.get 0
  )
  (func (;17;) (type 8)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 4112
    i32.sub
    local.tee 8
    global.set 0
    i32.const 1061
    call 8
    i32.const 1098
    call 8
    i32.const 1112
    i32.load
    local.set 4
    i32.const 9
    local.set 7
    local.get 8
    i32.const 4102
    i32.add
    local.tee 10
    local.set 1
    block  ;; label = @1
      loop  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              i32.load offset=4
              local.tee 3
              local.get 4
              i32.load offset=8
              local.tee 6
              i32.eq
              br_if 0 (;@5;)
              local.get 1
              local.get 3
              block (result i32)  ;; label = @6
                local.get 6
                local.get 3
                i32.sub
                local.tee 6
                local.tee 2
                i32.const 0
                i32.ne
                local.set 9
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 3
                        local.tee 1
                        i32.const 3
                        i32.and
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 2
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 1
                        i32.load8_u
                        i32.const 10
                        i32.eq
                        if  ;; label = @11
                          local.get 1
                          local.set 5
                          local.get 2
                          local.set 0
                          br 3 (;@8;)
                        end
                        local.get 2
                        i32.const 1
                        i32.sub
                        local.tee 0
                        i32.const 0
                        i32.ne
                        local.set 9
                        local.get 1
                        i32.const 1
                        i32.add
                        local.tee 5
                        i32.const 3
                        i32.and
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 0
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 5
                        i32.load8_u
                        i32.const 10
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 2
                        i32.const 2
                        i32.sub
                        local.tee 0
                        i32.const 0
                        i32.ne
                        local.set 9
                        local.get 1
                        i32.const 2
                        i32.add
                        local.tee 5
                        i32.const 3
                        i32.and
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 0
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 5
                        i32.load8_u
                        i32.const 10
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 2
                        i32.const 3
                        i32.sub
                        local.tee 0
                        i32.const 0
                        i32.ne
                        local.set 9
                        local.get 1
                        i32.const 3
                        i32.add
                        local.tee 5
                        i32.const 3
                        i32.and
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 0
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 5
                        i32.load8_u
                        i32.const 10
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 1
                        i32.const 4
                        i32.add
                        local.set 5
                        local.get 2
                        i32.const 4
                        i32.sub
                        local.tee 0
                        i32.const 0
                        i32.ne
                        local.set 9
                        br 1 (;@9;)
                      end
                      local.get 2
                      local.set 0
                      local.get 1
                      local.set 5
                    end
                    local.get 9
                    i32.eqz
                    br_if 1 (;@7;)
                  end
                  block  ;; label = @8
                    local.get 5
                    i32.load8_u
                    i32.const 10
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 0
                    i32.const 4
                    i32.lt_u
                    br_if 0 (;@8;)
                    loop  ;; label = @9
                      local.get 5
                      i32.load
                      i32.const 168430090
                      i32.xor
                      local.tee 2
                      i32.const -1
                      i32.xor
                      local.get 2
                      i32.const 16843009
                      i32.sub
                      i32.and
                      i32.const -2139062144
                      i32.and
                      br_if 1 (;@8;)
                      local.get 5
                      i32.const 4
                      i32.add
                      local.set 5
                      local.get 0
                      i32.const 4
                      i32.sub
                      local.tee 0
                      i32.const 3
                      i32.gt_u
                      br_if 0 (;@9;)
                    end
                  end
                  local.get 0
                  i32.eqz
                  br_if 0 (;@7;)
                  loop  ;; label = @8
                    local.get 5
                    local.get 5
                    i32.load8_u
                    i32.const 10
                    i32.eq
                    br_if 2 (;@6;)
                    drop
                    local.get 5
                    i32.const 1
                    i32.add
                    local.set 5
                    local.get 0
                    i32.const 1
                    i32.sub
                    local.tee 0
                    br_if 0 (;@8;)
                  end
                end
                i32.const 0
              end
              local.tee 2
              local.get 3
              i32.sub
              i32.const 1
              i32.add
              local.get 6
              local.get 2
              select
              local.tee 6
              local.get 7
              local.get 6
              local.get 7
              i32.lt_u
              select
              local.tee 6
              call 16
              local.set 1
              local.get 4
              local.get 4
              i32.load offset=4
              local.get 6
              i32.add
              local.tee 3
              i32.store offset=4
              local.get 1
              local.get 6
              i32.add
              local.set 1
              local.get 2
              br_if 2 (;@3;)
              local.get 7
              local.get 6
              i32.sub
              local.tee 7
              i32.eqz
              br_if 2 (;@3;)
              local.get 3
              local.get 4
              i32.load offset=8
              i32.eq
              br_if 0 (;@5;)
              local.get 4
              local.get 3
              i32.const 1
              i32.add
              i32.store offset=4
              local.get 3
              i32.load8_u
              local.set 3
              br 1 (;@4;)
            end
            global.get 0
            i32.const 16
            i32.sub
            local.tee 0
            global.set 0
            i32.const -1
            local.set 2
            block  ;; label = @5
              block (result i32)  ;; label = @6
                local.get 4
                local.get 4
                i32.load offset=60
                local.tee 3
                i32.const 1
                i32.sub
                local.get 3
                i32.or
                i32.store offset=60
                local.get 4
                i32.load offset=20
                local.get 4
                i32.load offset=24
                i32.ne
                if  ;; label = @7
                  local.get 4
                  i32.const 0
                  i32.const 0
                  local.get 4
                  i32.load offset=32
                  call_indirect (type 0)
                  drop
                end
                local.get 4
                i32.const 0
                i32.store offset=24
                local.get 4
                i64.const 0
                i64.store offset=16
                local.get 4
                i32.load
                local.tee 3
                i32.const 4
                i32.and
                if  ;; label = @7
                  local.get 4
                  local.get 3
                  i32.const 32
                  i32.or
                  i32.store
                  i32.const -1
                  br 1 (;@6;)
                end
                local.get 4
                local.get 4
                i32.load offset=40
                local.get 4
                i32.load offset=44
                i32.add
                local.tee 5
                i32.store offset=8
                local.get 4
                local.get 5
                i32.store offset=4
                local.get 3
                i32.const 27
                i32.shl
                i32.const 31
                i32.shr_s
              end
              br_if 0 (;@5;)
              local.get 4
              local.get 0
              i32.const 15
              i32.add
              i32.const 1
              local.get 4
              i32.load offset=28
              call_indirect (type 0)
              i32.const 1
              i32.ne
              br_if 0 (;@5;)
              local.get 0
              i32.load8_u offset=15
              local.set 2
            end
            local.get 0
            i32.const 16
            i32.add
            global.set 0
            local.get 2
            local.tee 3
            i32.const 0
            i32.ge_s
            br_if 0 (;@4;)
            local.get 1
            local.get 10
            i32.eq
            br_if 3 (;@1;)
            local.get 4
            i32.load8_u
            i32.const 16
            i32.and
            i32.eqz
            br_if 3 (;@1;)
            br 1 (;@3;)
          end
          local.get 1
          local.get 3
          i32.store8
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 3
          i32.const 255
          i32.and
          i32.const 10
          i32.eq
          br_if 0 (;@3;)
          local.get 7
          i32.const 1
          i32.sub
          local.tee 7
          br_if 1 (;@2;)
        end
      end
      local.get 10
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 0
      i32.store8
    end
    local.get 8
    i32.const 4102
    i32.add
    call 8
    i32.const 1024
    call 8
    block  ;; label = @1
      block (result i32)  ;; label = @2
        block  ;; label = @3
          loop  ;; label = @4
            i32.const 0
            local.get 8
            i32.const 4096
            call 5
            local.tee 1
            i32.const 0
            i32.le_s
            br_if 1 (;@3;)
            global.get 0
            i32.const 16
            i32.sub
            local.tee 3
            global.set 0
            local.get 3
            local.get 1
            i32.store offset=12
            local.get 3
            local.get 8
            i32.store offset=8
            block (result i32)  ;; label = @5
              i32.const 1
              local.get 3
              i32.const 8
              i32.add
              i32.const 1
              local.get 3
              i32.const 4
              i32.add
              call 7
              local.tee 2
              if  ;; label = @6
                i32.const 1360
                i32.const 8
                local.get 2
                local.get 2
                i32.const 76
                i32.eq
                select
                i32.store
                i32.const -1
                br 1 (;@5;)
              end
              local.get 3
              i32.load offset=4
            end
            local.set 2
            local.get 3
            i32.const 16
            i32.add
            global.set 0
            local.get 1
            local.get 2
            i32.eq
            br_if 0 (;@4;)
          end
          i32.const 1075
          br 1 (;@2;)
        end
        local.get 1
        i32.const 0
        i32.ge_s
        br_if 1 (;@1;)
        i32.const 1087
      end
      call 8
    end
    local.get 8
    i32.const 4112
    i32.add
    global.set 0
    i32.const 3448
    i32.load
    local.tee 0
    if  ;; label = @1
      loop  ;; label = @2
        local.get 0
        i32.load offset=20
        local.get 0
        i32.load offset=24
        i32.ne
        if  ;; label = @3
          local.get 0
          i32.const 0
          i32.const 0
          local.get 0
          i32.load offset=32
          call_indirect (type 0)
          drop
        end
        local.get 0
        i32.load offset=4
        local.tee 2
        local.get 0
        i32.load offset=8
        local.tee 1
        i32.ne
        if  ;; label = @3
          local.get 0
          local.get 2
          local.get 1
          i32.sub
          i64.extend_i32_s
          i32.const 1
          local.get 0
          i32.load offset=36
          call_indirect (type 1)
          drop
        end
        local.get 0
        i32.load offset=52
        local.tee 0
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      i32.const 1232
      i32.load
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=20
      local.get 0
      i32.load offset=24
      i32.ne
      if  ;; label = @2
        local.get 0
        i32.const 0
        i32.const 0
        local.get 0
        i32.load offset=32
        call_indirect (type 0)
        drop
      end
      local.get 0
      i32.load offset=4
      local.tee 2
      local.get 0
      i32.load offset=8
      local.tee 1
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      local.get 1
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 0
      i32.load offset=36
      call_indirect (type 1)
      drop
    end
    block  ;; label = @1
      i32.const 1352
      i32.load
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=20
      local.get 0
      i32.load offset=24
      i32.ne
      if  ;; label = @2
        local.get 0
        i32.const 0
        i32.const 0
        local.get 0
        i32.load offset=32
        call_indirect (type 0)
        drop
      end
      local.get 0
      i32.load offset=4
      local.tee 2
      local.get 0
      i32.load offset=8
      local.tee 1
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      local.get 1
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 0
      i32.load offset=36
      call_indirect (type 1)
      drop
    end
    block  ;; label = @1
      i32.const 3452
      i32.load
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=20
      local.get 0
      i32.load offset=24
      i32.ne
      if  ;; label = @2
        local.get 0
        i32.const 0
        i32.const 0
        local.get 0
        i32.load offset=32
        call_indirect (type 0)
        drop
      end
      local.get 0
      i32.load offset=4
      local.tee 2
      local.get 0
      i32.load offset=8
      local.tee 1
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      local.get 1
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 0
      i32.load offset=36
      call_indirect (type 1)
      drop
    end
  )
  (table (;0;) 6 6 funcref)
  (memory (;0;) 2)
  (global (;0;) (mut i32) i32.const 68992)
  (export "memory" (memory 0))
  (export "_start" (func 17))
  (elem (;0;) (i32.const 1) func 9 10 11 14 13)
  (data (;0;) (i32.const 1024) "read and write, press Ctrl+D to exit\00gets and puts\00write error\00read error\00input a word:\00`\04")
  (data (;1;) (i32.const 1120) "\09")
  (data (;2;) (i32.const 1132) "\01")
  (data (;3;) (i32.const 1148) "\02\00\00\00\00\00\00\00\03\00\00\00h\05\00\00\00\04")
  (data (;4;) (i32.const 1232) "`\04\00\00\00\00\00\00\05")
  (data (;5;) (i32.const 1252) "\01")
  (data (;6;) (i32.const 1272) "\04\00\00\00\03\00\00\00x\09\00\00\00\04")
  (data (;7;) (i32.const 1296) "\01\00\00\00\00\00\00\00\0a")
  (data (;8;) (i32.const 1352) "\d8\04")
)
