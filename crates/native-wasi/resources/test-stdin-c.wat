(module
  (type (;0;) (func (param i32 i32 i32) (result i32)))
  (type (;1;) (func (param i32 i64 i32) (result i64)))
  (type (;2;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;3;) (func (param i32) (result i32)))
  (type (;4;) (func (result i32)))
  (type (;5;) (func))
  (type (;6;) (func (param i32 i32) (result i32)))
  (type (;7;) (func (param i32 i64 i32 i32) (result i32)))
  (type (;8;) (func (param i32)))
  (type (;9;) (func (param i32 i32 i32)))
  (import "wasi_snapshot_preview1" "fd_close" (func (;0;) (type 3)))
  (import "wasi_snapshot_preview1" "fd_fdstat_get" (func (;1;) (type 6)))
  (import "wasi_snapshot_preview1" "fd_read" (func (;2;) (type 2)))
  (import "wasi_snapshot_preview1" "fd_seek" (func (;3;) (type 7)))
  (import "wasi_snapshot_preview1" "fd_write" (func (;4;) (type 2)))
  (import "wasi_snapshot_preview1" "proc_exit" (func (;5;) (type 8)))
  (func (;6;) (type 4) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    i32.const 1052
    i32.load
    local.set 1
    i32.const 1056
    i32.load
    local.set 0
    block  ;; label = @1
      loop  ;; label = @2
        block (result i32)  ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 2
          local.get 0
          i32.load offset=8
          i32.ne
          if  ;; label = @4
            local.get 0
            local.get 2
            i32.const 1
            i32.add
            i32.store offset=4
            local.get 2
            i32.load8_u
            br 1 (;@3;)
          end
          global.get 0
          i32.const 16
          i32.sub
          local.tee 5
          global.set 0
          i32.const -1
          local.set 3
          block  ;; label = @4
            block (result i32)  ;; label = @5
              local.get 0
              local.get 0
              i32.load offset=60
              local.tee 2
              i32.const 1
              i32.sub
              local.get 2
              i32.or
              i32.store offset=60
              local.get 0
              i32.load offset=20
              local.get 0
              i32.load offset=24
              i32.ne
              if  ;; label = @6
                local.get 0
                i32.const 0
                i32.const 0
                local.get 0
                i32.load offset=32
                call_indirect (type 0)
                drop
              end
              local.get 0
              i32.const 0
              i32.store offset=24
              local.get 0
              i64.const 0
              i64.store offset=16
              local.get 0
              i32.load
              local.tee 4
              i32.const 4
              i32.and
              if  ;; label = @6
                local.get 0
                local.get 4
                i32.const 32
                i32.or
                i32.store
                i32.const -1
                br 1 (;@5;)
              end
              local.get 0
              local.get 0
              i32.load offset=40
              local.get 0
              i32.load offset=44
              i32.add
              local.tee 2
              i32.store offset=8
              local.get 0
              local.get 2
              i32.store offset=4
              local.get 4
              i32.const 27
              i32.shl
              i32.const 31
              i32.shr_s
            end
            br_if 0 (;@4;)
            local.get 0
            local.get 5
            i32.const 15
            i32.add
            i32.const 1
            local.get 0
            i32.load offset=28
            call_indirect (type 0)
            i32.const 1
            i32.ne
            br_if 0 (;@4;)
            local.get 5
            i32.load8_u offset=15
            local.set 3
          end
          local.get 5
          i32.const 16
          i32.add
          global.set 0
          local.get 3
        end
        local.tee 3
        i32.const -1
        i32.eq
        br_if 1 (;@1;)
        block (result i32)  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 255
            i32.and
            local.tee 2
            local.get 1
            i32.load offset=64
            i32.eq
            br_if 0 (;@4;)
            local.get 1
            i32.load offset=20
            local.tee 4
            local.get 1
            i32.load offset=16
            i32.eq
            br_if 0 (;@4;)
            local.get 1
            local.get 4
            i32.const 1
            i32.add
            i32.store offset=20
            local.get 4
            local.get 3
            i32.store8
            local.get 2
            br 1 (;@3;)
          end
          global.get 0
          i32.const 16
          i32.sub
          local.tee 5
          global.set 0
          local.get 5
          local.get 2
          i32.store8 offset=15
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load offset=16
              local.tee 4
              if (result i32)  ;; label = @6
                local.get 4
              else
                i32.const -1
                local.set 3
                local.get 1
                call 14
                br_if 2 (;@4;)
                local.get 1
                i32.load offset=16
              end
              local.get 1
              i32.load offset=20
              local.tee 4
              i32.eq
              br_if 0 (;@5;)
              local.get 2
              i32.const 255
              i32.and
              local.tee 3
              local.get 1
              i32.load offset=64
              i32.eq
              br_if 0 (;@5;)
              local.get 1
              local.get 4
              i32.const 1
              i32.add
              i32.store offset=20
              local.get 4
              local.get 2
              i32.store8
              br 1 (;@4;)
            end
            i32.const -1
            local.set 3
            local.get 1
            local.get 5
            i32.const 15
            i32.add
            i32.const 1
            local.get 1
            i32.load offset=32
            call_indirect (type 0)
            i32.const 1
            i32.ne
            br_if 0 (;@4;)
            local.get 5
            i32.load8_u offset=15
            local.set 3
          end
          local.get 5
          i32.const 16
          i32.add
          global.set 0
          local.get 3
        end
        i32.const -1
        i32.ne
        br_if 0 (;@2;)
      end
      i32.const 1024
      i32.const 11
      i32.const 1048
      i32.load
      call 10
      i32.const 1
      return
    end
    local.get 0
    i32.load
    i32.const 5
    i32.shr_u
    i32.const 1
    i32.and
    if (result i32)  ;; label = @1
      i32.const 1036
      i32.const 10
      i32.const 1048
      i32.load
      call 10
      i32.const 1
    else
      i32.const 0
    end
  )
  (func (;7;) (type 0) (param i32 i32 i32) (result i32)
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
      call 8
      local.tee 0
      if  ;; label = @2
        i32.const 1424
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
    local.get 3
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;8;) (type 2) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call 2
    i32.const 65535
    i32.and
  )
  (func (;9;) (type 2) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call 4
    i32.const 65535
    i32.and
  )
  (func (;10;) (type 9) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 1
      local.get 2
      i32.load offset=16
      local.tee 5
      if (result i32)  ;; label = @2
        local.get 5
      else
        i32.const 0
        local.set 5
        local.get 2
        call 14
        br_if 1 (;@1;)
        local.get 2
        i32.load offset=16
      end
      local.get 2
      i32.load offset=20
      local.tee 6
      i32.sub
      i32.gt_u
      if  ;; label = @2
        local.get 2
        local.get 0
        local.get 1
        local.get 2
        i32.load offset=32
        call_indirect (type 0)
        local.set 5
        br 1 (;@1;)
      end
      block (result i32)  ;; label = @2
        local.get 1
        local.get 2
        i32.load offset=64
        i32.const 0
        i32.lt_s
        br_if 0 (;@2;)
        drop
        local.get 0
        local.get 1
        i32.add
        local.set 3
        i32.const 0
        local.set 5
        loop  ;; label = @3
          local.get 1
          local.get 1
          local.get 5
          i32.add
          i32.eqz
          br_if 1 (;@2;)
          drop
          local.get 3
          local.get 5
          i32.add
          local.get 5
          i32.const 1
          i32.sub
          local.tee 7
          local.set 5
          i32.const 1
          i32.sub
          i32.load8_u
          i32.const 10
          i32.ne
          br_if 0 (;@3;)
        end
        local.get 2
        local.get 0
        local.get 1
        local.get 7
        i32.add
        i32.const 1
        i32.add
        local.tee 8
        local.get 2
        i32.load offset=32
        call_indirect (type 0)
        local.tee 5
        local.get 8
        i32.lt_u
        br_if 1 (;@1;)
        local.get 3
        local.get 7
        i32.add
        i32.const 1
        i32.add
        local.set 0
        local.get 2
        i32.load offset=20
        local.set 6
        local.get 7
        i32.const -1
        i32.xor
      end
      local.set 7
      local.get 0
      local.set 5
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 7
            i32.const 32
            i32.le_u
            if  ;; label = @5
              local.get 5
              i32.const 3
              i32.and
              i32.eqz
              br_if 1 (;@4;)
              local.get 7
              i32.eqz
              br_if 1 (;@4;)
              local.get 6
              local.get 5
              i32.load8_u
              i32.store8
              local.get 7
              i32.const 1
              i32.sub
              local.set 0
              local.get 6
              i32.const 1
              i32.add
              local.set 4
              local.get 5
              i32.const 1
              i32.add
              local.tee 3
              i32.const 3
              i32.and
              i32.eqz
              br_if 2 (;@3;)
              local.get 0
              i32.eqz
              br_if 2 (;@3;)
              local.get 6
              local.get 5
              i32.load8_u offset=1
              i32.store8 offset=1
              local.get 7
              i32.const 2
              i32.sub
              local.set 0
              local.get 6
              i32.const 2
              i32.add
              local.set 4
              local.get 5
              i32.const 2
              i32.add
              local.tee 3
              i32.const 3
              i32.and
              i32.eqz
              br_if 2 (;@3;)
              local.get 0
              i32.eqz
              br_if 2 (;@3;)
              local.get 6
              local.get 5
              i32.load8_u offset=2
              i32.store8 offset=2
              local.get 7
              i32.const 3
              i32.sub
              local.set 0
              local.get 6
              i32.const 3
              i32.add
              local.set 4
              local.get 5
              i32.const 3
              i32.add
              local.tee 3
              i32.const 3
              i32.and
              i32.eqz
              br_if 2 (;@3;)
              local.get 0
              i32.eqz
              br_if 2 (;@3;)
              local.get 6
              local.get 5
              i32.load8_u offset=3
              i32.store8 offset=3
              local.get 7
              i32.const 4
              i32.sub
              local.set 0
              local.get 6
              i32.const 4
              i32.add
              local.set 4
              local.get 5
              i32.const 4
              i32.add
              local.set 3
              br 2 (;@3;)
            end
            local.get 6
            local.get 5
            local.get 7
            memory.copy
            br 2 (;@2;)
          end
          local.get 7
          local.set 0
          local.get 6
          local.set 4
          local.get 5
          local.set 3
        end
        block  ;; label = @3
          local.get 4
          i32.const 3
          i32.and
          local.tee 5
          i32.eqz
          if  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.const 16
              i32.ge_u
              if  ;; label = @6
                local.get 0
                i32.const 16
                i32.sub
                local.tee 5
                i32.const 16
                i32.and
                i32.eqz
                if  ;; label = @7
                  local.get 4
                  local.get 3
                  i64.load align=4
                  i64.store align=4
                  local.get 4
                  local.get 3
                  i64.load offset=8 align=4
                  i64.store offset=8 align=4
                  local.get 4
                  i32.const 16
                  i32.add
                  local.set 4
                  local.get 3
                  i32.const 16
                  i32.add
                  local.set 3
                  local.get 5
                  local.set 0
                end
                local.get 5
                i32.const 16
                i32.lt_u
                br_if 1 (;@5;)
                loop  ;; label = @7
                  local.get 4
                  local.get 3
                  i64.load align=4
                  i64.store align=4
                  local.get 4
                  i32.const 8
                  i32.add
                  local.get 3
                  i32.const 8
                  i32.add
                  i64.load align=4
                  i64.store align=4
                  local.get 4
                  i32.const 16
                  i32.add
                  local.get 3
                  i32.const 16
                  i32.add
                  i64.load align=4
                  i64.store align=4
                  local.get 4
                  i32.const 24
                  i32.add
                  local.get 3
                  i32.const 24
                  i32.add
                  i64.load align=4
                  i64.store align=4
                  local.get 4
                  i32.const 32
                  i32.add
                  local.set 4
                  local.get 3
                  i32.const 32
                  i32.add
                  local.set 3
                  local.get 0
                  i32.const 32
                  i32.sub
                  local.tee 0
                  i32.const 15
                  i32.gt_u
                  br_if 0 (;@7;)
                end
              end
              local.get 0
              local.set 5
            end
            local.get 5
            i32.const 8
            i32.and
            if  ;; label = @5
              local.get 4
              local.get 3
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
            local.get 5
            i32.const 4
            i32.and
            if  ;; label = @5
              local.get 4
              local.get 3
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
            local.get 5
            i32.const 2
            i32.and
            if  ;; label = @5
              local.get 4
              local.get 3
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
            br_if 1 (;@3;)
            local.get 4
            local.get 3
            i32.load8_u
            i32.store8
            br 2 (;@2;)
          end
          block  ;; label = @4
            block (result i32)  ;; label = @5
              block (result i32)  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.const 32
                  i32.ge_u
                  if  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 5
                        i32.const 1
                        i32.sub
                        br_table 3 (;@7;) 0 (;@10;) 1 (;@9;) 7 (;@3;)
                      end
                      local.get 4
                      local.get 3
                      i32.load
                      i32.store16 align=1
                      local.get 4
                      local.get 3
                      i32.const 2
                      i32.add
                      i32.load align=2
                      i32.store offset=2
                      local.get 4
                      local.get 3
                      i32.const 6
                      i32.add
                      i64.load align=2
                      i64.store offset=6 align=4
                      local.get 3
                      i32.const 14
                      i32.add
                      i32.load align=2
                      local.set 6
                      i32.const 18
                      local.set 5
                      i32.const 14
                      local.set 0
                      i32.const 14
                      br 3 (;@6;)
                    end
                    local.get 4
                    local.get 3
                    i32.load
                    i32.store8
                    local.get 4
                    local.get 3
                    i32.const 1
                    i32.add
                    i32.load align=1
                    i32.store offset=1
                    local.get 4
                    local.get 3
                    i32.const 5
                    i32.add
                    i64.load align=1
                    i64.store offset=5 align=4
                    local.get 3
                    i32.const 13
                    i32.add
                    i32.load align=1
                    local.set 6
                    i32.const 15
                    local.set 0
                    i32.const 17
                    local.set 5
                    i32.const 13
                    br 2 (;@6;)
                  end
                  local.get 0
                  i32.const 16
                  i32.and
                  if  ;; label = @8
                    local.get 4
                    local.get 3
                    i32.load8_u
                    i32.store8
                    local.get 4
                    local.get 3
                    i32.load offset=1 align=1
                    i32.store offset=1 align=1
                    local.get 4
                    local.get 3
                    i64.load offset=5 align=1
                    i64.store offset=5 align=1
                    local.get 4
                    local.get 3
                    i32.load16_u offset=13 align=1
                    i32.store16 offset=13 align=1
                    local.get 4
                    local.get 3
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
                  local.get 4
                  local.get 0
                  i32.const 8
                  i32.and
                  br_if 2 (;@5;)
                  drop
                  br 3 (;@4;)
                end
                local.get 4
                local.get 3
                i32.load
                local.tee 0
                i32.store8
                local.get 4
                local.get 0
                i32.const 16
                i32.shr_u
                i32.store8 offset=2
                local.get 4
                local.get 0
                i32.const 8
                i32.shr_u
                i32.store8 offset=1
                local.get 4
                local.get 3
                i32.const 3
                i32.add
                i32.load align=1
                i32.store offset=3
                local.get 4
                local.get 3
                i32.const 7
                i32.add
                i64.load align=1
                i64.store offset=7 align=4
                local.get 3
                i32.const 15
                i32.add
                i32.load align=1
                local.set 6
                i32.const 13
                local.set 0
                i32.const 19
                local.set 5
                i32.const 15
              end
              local.get 4
              i32.add
              local.get 6
              i32.store
              local.get 3
              local.get 5
              i32.add
              local.set 3
              local.get 4
              local.get 5
              i32.add
            end
            local.tee 5
            local.get 3
            i64.load align=1
            i64.store align=1
            local.get 5
            i32.const 8
            i32.add
            local.set 4
            local.get 3
            i32.const 8
            i32.add
            local.set 3
          end
          local.get 0
          i32.const 4
          i32.and
          if  ;; label = @4
            local.get 4
            local.get 3
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
          local.get 0
          i32.const 2
          i32.and
          if  ;; label = @4
            local.get 4
            local.get 3
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
          local.get 0
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 3
          i32.load8_u
          i32.store8
        end
      end
      local.get 2
      local.get 2
      i32.load offset=20
      local.get 7
      i32.add
      i32.store offset=20
      local.get 7
      local.get 8
      i32.add
      local.set 5
    end
    local.get 1
    local.get 5
    i32.eq
    if  ;; label = @1
      return
    end
    local.get 5
    local.get 1
    i32.div_u
    drop
  )
  (func (;11;) (type 0) (param i32 i32 i32) (result i32)
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
        i32.const 1424
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
      call 9
      local.tee 0
      if  ;; label = @2
        i32.const 1424
        local.get 0
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
  (func (;12;) (type 0) (param i32 i32 i32) (result i32)
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
      call 11
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
          call 11
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
    local.get 3
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;13;) (type 3) (param i32) (result i32)
    local.get 0
    i32.load offset=56
    call 0
    i32.const 65535
    i32.and
    local.tee 0
    if (result i32)  ;; label = @1
      i32.const 1424
      local.get 0
      i32.store
      i32.const -1
    else
      i32.const 0
    end
  )
  (func (;14;) (type 3) (param i32) (result i32)
    (local i32)
    local.get 0
    local.get 0
    i32.load offset=60
    local.tee 1
    i32.const 1
    i32.sub
    local.get 1
    i32.or
    i32.store offset=60
    local.get 0
    i32.load
    local.tee 1
    i32.const 8
    i32.and
    if  ;; label = @1
      local.get 0
      local.get 1
      i32.const 32
      i32.or
      i32.store
      i32.const -1
      return
    end
    local.get 0
    i64.const 0
    i64.store offset=4 align=4
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
    i32.const 0
  )
  (func (;15;) (type 5)
    (local i32 i32 i32)
    i32.const 3512
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
        local.tee 1
        local.get 0
        i32.load offset=8
        local.tee 2
        i32.ne
        if  ;; label = @3
          local.get 0
          local.get 1
          local.get 2
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
      i32.const 1416
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
      local.tee 1
      local.get 0
      i32.load offset=8
      local.tee 2
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 2
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 0
      i32.load offset=36
      call_indirect (type 1)
      drop
    end
    block  ;; label = @1
      i32.const 1296
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
      local.tee 1
      local.get 0
      i32.load offset=8
      local.tee 2
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 2
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 0
      i32.load offset=36
      call_indirect (type 1)
      drop
    end
    block  ;; label = @1
      i32.const 1176
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
      local.tee 1
      local.get 0
      i32.load offset=8
      local.tee 2
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 2
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 0
      i32.load offset=36
      call_indirect (type 1)
      drop
    end
  )
  (func (;16;) (type 1) (param i32 i64 i32) (result i64)
    (local i32)
    local.get 0
    i32.load offset=56
    local.set 3
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    block (result i64)  ;; label = @1
      local.get 3
      local.get 1
      local.get 2
      i32.const 255
      i32.and
      local.get 0
      i32.const 8
      i32.add
      call 3
      i32.const 65535
      i32.and
      local.tee 2
      if  ;; label = @2
        i32.const 1424
        i32.const 70
        local.get 2
        local.get 2
        i32.const 76
        i32.eq
        select
        i32.store
        i64.const -1
        br 1 (;@1;)
      end
      local.get 0
      i64.load offset=8
    end
    local.get 0
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;17;) (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 1
    i32.store
    local.get 4
    local.get 0
    i32.load offset=44
    local.tee 3
    i32.store offset=12
    local.get 4
    local.get 0
    i32.load offset=40
    local.tee 6
    i32.store offset=8
    local.get 4
    local.get 2
    local.get 3
    i32.const 0
    i32.ne
    i32.sub
    local.tee 7
    i32.store offset=4
    block  ;; label = @1
      block (result i32)  ;; label = @2
        local.get 7
        if  ;; label = @3
          local.get 0
          i32.load offset=56
          local.set 7
          global.get 0
          i32.const 16
          i32.sub
          local.tee 3
          global.set 0
          i32.const -1
          local.set 6
          block  ;; label = @4
            local.get 7
            local.get 4
            i32.const 2
            local.get 3
            i32.const 12
            i32.add
            call 8
            local.tee 7
            if  ;; label = @5
              i32.const 1424
              local.get 7
              i32.store
              br 1 (;@4;)
            end
            local.get 3
            i32.load offset=12
            local.set 6
          end
          local.get 3
          i32.const 16
          i32.add
          global.set 0
          local.get 6
          br 1 (;@2;)
        end
        local.get 0
        i32.load offset=56
        local.get 6
        local.get 3
        call 7
      end
      local.tee 3
      i32.const 0
      i32.le_s
      if  ;; label = @2
        local.get 0
        local.get 0
        i32.load
        i32.const 32
        i32.const 16
        local.get 3
        select
        i32.or
        i32.store
        br 1 (;@1;)
      end
      local.get 4
      i32.load offset=4
      local.tee 6
      local.get 3
      i32.ge_u
      if  ;; label = @2
        local.get 3
        local.set 5
        br 1 (;@1;)
      end
      local.get 0
      local.get 0
      i32.load offset=40
      local.tee 5
      i32.store offset=4
      local.get 0
      local.get 5
      local.get 3
      local.get 6
      i32.sub
      i32.add
      i32.store offset=8
      local.get 0
      i32.load offset=44
      if  ;; label = @2
        local.get 0
        local.get 5
        i32.const 1
        i32.add
        i32.store offset=4
        local.get 1
        local.get 2
        i32.add
        i32.const 1
        i32.sub
        local.get 5
        i32.load8_u
        i32.store8
      end
      local.get 2
      local.set 5
    end
    local.get 4
    i32.const 16
    i32.add
    global.set 0
    local.get 5
  )
  (func (;18;) (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32)
    local.get 0
    i32.const 2
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
        i32.const 1424
        local.get 3
        i32.store
        i32.const 0
      end
      local.get 4
      i32.const 32
      i32.add
      global.set 0
      br_if 0 (;@1;)
      local.get 0
      i32.const -1
      i32.store offset=64
    end
    local.get 0
    local.get 1
    local.get 2
    call 12
  )
  (func (;19;) (type 5)
    (local i32)
    call 6
    local.tee 0
    if  ;; label = @1
      call 15
      local.get 0
      call 5
      unreachable
    end
    call 15
  )
  (func (;20;) (type 4) (result i32)
    (local i32)
    call 6
    call 15
  )
  (func (;21;) (type 4) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 4096
    i32.sub
    local.tee 2
    global.set 0
    block (result i32)  ;; label = @1
      block  ;; label = @2
        loop  ;; label = @3
          i32.const 0
          local.get 2
          i32.const 4096
          call 7
          local.tee 3
          i32.const 0
          i32.le_s
          br_if 1 (;@2;)
          global.get 0
          i32.const 16
          i32.sub
          local.tee 0
          global.set 0
          local.get 0
          local.get 3
          i32.store offset=12
          local.get 0
          local.get 2
          i32.store offset=8
          block (result i32)  ;; label = @4
            i32.const 1
            local.get 0
            i32.const 8
            i32.add
            i32.const 1
            local.get 0
            i32.const 4
            i32.add
            call 9
            local.tee 1
            if  ;; label = @5
              i32.const 1424
              i32.const 8
              local.get 1
              local.get 1
              i32.const 76
              i32.eq
              select
              i32.store
              i32.const -1
              br 1 (;@4;)
            end
            local.get 0
            i32.load offset=4
          end
          local.get 0
          i32.const 16
          i32.add
          global.set 0
          local.get 3
          i32.eq
          br_if 0 (;@3;)
        end
        i32.const 1024
        i32.const 11
        i32.const 1048
        i32.load
        call 10
        i32.const 1
        br 1 (;@1;)
      end
      i32.const 0
      local.get 3
      i32.const 0
      i32.ge_s
      br_if 0 (;@1;)
      drop
      i32.const 1036
      i32.const 10
      i32.const 1048
      i32.load
      call 10
      i32.const 1
    end
    local.get 2
    i32.const 4096
    i32.add
    global.set 0
    call 15
  )
  (table (;0;) 6 6 funcref)
  (memory (;0;) 2)
  (global (;0;) (mut i32) i32.const 69056)
  (export "memory" (memory 0))
  (export "_start" (func 19))
  (export "echo_by_std" (func 20))
  (export "echo_by_syscall" (func 21))
  (elem (;0;) (i32.const 1) func 13 12 16 18 17)
  (data (;0;) (i32.const 1024) "write error\00read error\00\00(\04\00\00\a0\04\00\00\18\05")
  (data (;1;) (i32.const 1064) "\05")
  (data (;2;) (i32.const 1076) "\01")
  (data (;3;) (i32.const 1096) "\02\00\00\00\03\00\00\00\9c\05")
  (data (;4;) (i32.const 1120) "\02\00\00\00\00\00\00\00\ff\ff\ff\ff")
  (data (;5;) (i32.const 1176) "(\04\00\00\00\00\00\00\05")
  (data (;6;) (i32.const 1196) "\01")
  (data (;7;) (i32.const 1216) "\04\00\00\00\03\00\00\00\a8\05\00\00\00\04")
  (data (;8;) (i32.const 1240) "\01\00\00\00\00\00\00\00\0a")
  (data (;9;) (i32.const 1296) "\a0\04\00\00\00\00\00\00\09")
  (data (;10;) (i32.const 1316) "\01")
  (data (;11;) (i32.const 1332) "\05\00\00\00\00\00\00\00\03\00\00\00\b8\09\00\00\00\04")
  (data (;12;) (i32.const 1416) "\18\05")
)
