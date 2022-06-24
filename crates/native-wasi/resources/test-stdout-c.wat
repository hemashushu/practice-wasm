(module
  (type (;0;) (func (param i32 i32 i32) (result i32)))
  (type (;1;) (func (param i32 i64 i32) (result i64)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32 i32) (result i32)))
  (type (;4;) (func (param i32 i64 i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;6;) (func))
  (import "wasi_snapshot_preview1" "fd_close" (func (;0;) (type 2)))
  (import "wasi_snapshot_preview1" "fd_fdstat_get" (func (;1;) (type 3)))
  (import "wasi_snapshot_preview1" "fd_seek" (func (;2;) (type 4)))
  (import "wasi_snapshot_preview1" "fd_write" (func (;3;) (type 5)))
  (func (;4;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=56
    call 0
    i32.const 65535
    i32.and
    local.tee 0
    if (result i32)  ;; label = @1
      i32.const 2200
      local.get 0
      i32.store
      i32.const -1
    else
      i32.const 0
    end
  )
  (func (;5;) (type 1) (param i32 i64 i32) (result i64)
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
      call 2
      i32.const 65535
      i32.and
      local.tee 2
      if  ;; label = @2
        i32.const 2200
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
  (func (;6;) (type 0) (param i32 i32 i32) (result i32)
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
        i32.const 2200
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
      call 3
      i32.const 65535
      i32.and
      local.tee 0
      if  ;; label = @2
        i32.const 2200
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
  (func (;7;) (type 0) (param i32 i32 i32) (result i32)
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
      call 6
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
          call 6
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
  (func (;8;) (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32)
    local.get 0
    i32.const 4
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
        i32.const 2200
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
    call 7
  )
  (func (;9;) (type 6)
    (local i32 i32 i32 i32 i32 i32 i32)
    i32.const 1024
    local.set 4
    block  ;; label = @1
      i32.const 1040
      i32.load
      local.tee 5
      i32.load offset=16
      local.tee 2
      if (result i32)  ;; label = @2
        local.get 2
      else
        block (result i32)  ;; label = @3
          local.get 5
          local.get 5
          i32.load offset=60
          local.tee 2
          i32.const 1
          i32.sub
          local.get 2
          i32.or
          i32.store offset=60
          local.get 5
          i32.load
          local.tee 2
          i32.const 8
          i32.and
          if  ;; label = @4
            local.get 5
            local.get 2
            i32.const 32
            i32.or
            i32.store
            i32.const -1
            br 1 (;@3;)
          end
          local.get 5
          i64.const 0
          i64.store offset=4 align=4
          local.get 5
          local.get 5
          i32.load offset=40
          local.tee 2
          i32.store offset=24
          local.get 5
          local.get 2
          i32.store offset=20
          local.get 5
          local.get 2
          local.get 5
          i32.load offset=44
          i32.add
          i32.store offset=16
          i32.const 0
        end
        br_if 1 (;@1;)
        local.get 5
        i32.load offset=16
      end
      local.get 5
      i32.load offset=20
      local.tee 6
      i32.sub
      i32.const 12
      i32.lt_u
      if  ;; label = @2
        local.get 5
        i32.const 1024
        i32.const 12
        local.get 5
        i32.load offset=32
        call_indirect (type 0)
        drop
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block (result i32)  ;; label = @5
              i32.const 12
              local.get 5
              i32.load offset=64
              i32.const 0
              i32.lt_s
              br_if 0 (;@5;)
              drop
              i32.const 0
              local.set 2
              loop  ;; label = @6
                i32.const 12
                local.get 2
                i32.const -12
                i32.eq
                br_if 1 (;@5;)
                drop
                local.get 2
                i32.const 1036
                i32.add
                local.get 2
                i32.const 1
                i32.sub
                local.set 2
                i32.const 1
                i32.sub
                i32.load8_u
                i32.const 10
                i32.ne
                br_if 0 (;@6;)
              end
              local.get 5
              i32.const 1024
              local.get 2
              i32.const 13
              i32.add
              local.tee 4
              local.get 5
              i32.load offset=32
              call_indirect (type 0)
              local.get 4
              i32.lt_u
              br_if 4 (;@1;)
              local.get 2
              i32.const 1037
              i32.add
              local.set 4
              local.get 5
              i32.load offset=20
              local.set 6
              local.get 2
              i32.const -1
              i32.xor
            end
            local.tee 3
            i32.const 32
            i32.le_u
            if  ;; label = @5
              local.get 4
              i32.const 3
              i32.and
              i32.eqz
              br_if 1 (;@4;)
              local.get 3
              i32.eqz
              br_if 1 (;@4;)
              local.get 6
              local.get 4
              i32.load8_u
              i32.store8
              local.get 3
              i32.const 1
              i32.sub
              local.set 2
              local.get 6
              i32.const 1
              i32.add
              local.set 0
              local.get 4
              i32.const 1
              i32.add
              local.tee 1
              i32.const 3
              i32.and
              i32.eqz
              br_if 2 (;@3;)
              local.get 2
              i32.eqz
              br_if 2 (;@3;)
              local.get 6
              local.get 4
              i32.load8_u offset=1
              i32.store8 offset=1
              local.get 3
              i32.const 2
              i32.sub
              local.set 2
              local.get 6
              i32.const 2
              i32.add
              local.set 0
              local.get 4
              i32.const 2
              i32.add
              local.tee 1
              i32.const 3
              i32.and
              i32.eqz
              br_if 2 (;@3;)
              local.get 2
              i32.eqz
              br_if 2 (;@3;)
              local.get 6
              local.get 4
              i32.load8_u offset=2
              i32.store8 offset=2
              local.get 3
              i32.const 3
              i32.sub
              local.set 2
              local.get 6
              i32.const 3
              i32.add
              local.set 0
              local.get 4
              i32.const 3
              i32.add
              local.tee 1
              i32.const 3
              i32.and
              i32.eqz
              br_if 2 (;@3;)
              local.get 2
              i32.eqz
              br_if 2 (;@3;)
              local.get 6
              local.get 4
              i32.load8_u offset=3
              i32.store8 offset=3
              local.get 3
              i32.const 4
              i32.sub
              local.set 2
              local.get 6
              i32.const 4
              i32.add
              local.set 0
              local.get 4
              i32.const 4
              i32.add
              local.set 1
              br 2 (;@3;)
            end
            local.get 6
            local.get 4
            local.get 3
            memory.copy
            br 2 (;@2;)
          end
          local.get 3
          local.set 2
          local.get 6
          local.set 0
          local.get 4
          local.set 1
        end
        block  ;; label = @3
          local.get 0
          i32.const 3
          i32.and
          local.tee 4
          i32.eqz
          if  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.const 16
              i32.ge_u
              if  ;; label = @6
                local.get 2
                i32.const 16
                i32.sub
                local.tee 4
                i32.const 16
                i32.and
                i32.eqz
                if  ;; label = @7
                  local.get 0
                  local.get 1
                  i64.load align=4
                  i64.store align=4
                  local.get 0
                  local.get 1
                  i64.load offset=8 align=4
                  i64.store offset=8 align=4
                  local.get 0
                  i32.const 16
                  i32.add
                  local.set 0
                  local.get 1
                  i32.const 16
                  i32.add
                  local.set 1
                  local.get 4
                  local.set 2
                end
                local.get 4
                i32.const 16
                i32.lt_u
                br_if 1 (;@5;)
                loop  ;; label = @7
                  local.get 0
                  local.get 1
                  i64.load align=4
                  i64.store align=4
                  local.get 0
                  i32.const 8
                  i32.add
                  local.get 1
                  i32.const 8
                  i32.add
                  i64.load align=4
                  i64.store align=4
                  local.get 0
                  i32.const 16
                  i32.add
                  local.get 1
                  i32.const 16
                  i32.add
                  i64.load align=4
                  i64.store align=4
                  local.get 0
                  i32.const 24
                  i32.add
                  local.get 1
                  i32.const 24
                  i32.add
                  i64.load align=4
                  i64.store align=4
                  local.get 0
                  i32.const 32
                  i32.add
                  local.set 0
                  local.get 1
                  i32.const 32
                  i32.add
                  local.set 1
                  local.get 2
                  i32.const 32
                  i32.sub
                  local.tee 2
                  i32.const 15
                  i32.gt_u
                  br_if 0 (;@7;)
                end
              end
              local.get 2
              local.set 4
            end
            local.get 4
            i32.const 8
            i32.and
            if  ;; label = @5
              local.get 0
              local.get 1
              i64.load align=4
              i64.store align=4
              local.get 1
              i32.const 8
              i32.add
              local.set 1
              local.get 0
              i32.const 8
              i32.add
              local.set 0
            end
            local.get 4
            i32.const 4
            i32.and
            if  ;; label = @5
              local.get 0
              local.get 1
              i32.load
              i32.store
              local.get 1
              i32.const 4
              i32.add
              local.set 1
              local.get 0
              i32.const 4
              i32.add
              local.set 0
            end
            local.get 4
            i32.const 2
            i32.and
            if  ;; label = @5
              local.get 0
              local.get 1
              i32.load16_u align=1
              i32.store16 align=1
              local.get 1
              i32.const 2
              i32.add
              local.set 1
              local.get 0
              i32.const 2
              i32.add
              local.set 0
            end
            local.get 4
            i32.const 1
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 0
            local.get 1
            i32.load8_u
            i32.store8
            br 2 (;@2;)
          end
          block  ;; label = @4
            block (result i32)  ;; label = @5
              block (result i32)  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  i32.const 32
                  i32.ge_u
                  if  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 4
                        i32.const 1
                        i32.sub
                        br_table 3 (;@7;) 0 (;@10;) 1 (;@9;) 7 (;@3;)
                      end
                      local.get 0
                      local.get 1
                      i32.load
                      i32.store16 align=1
                      local.get 0
                      local.get 1
                      i32.const 2
                      i32.add
                      i32.load align=2
                      i32.store offset=2
                      local.get 0
                      local.get 1
                      i32.const 6
                      i32.add
                      i64.load align=2
                      i64.store offset=6 align=4
                      local.get 1
                      i32.const 14
                      i32.add
                      i32.load align=2
                      local.set 6
                      i32.const 18
                      local.set 4
                      i32.const 14
                      local.set 2
                      i32.const 14
                      br 3 (;@6;)
                    end
                    local.get 0
                    local.get 1
                    i32.load
                    i32.store8
                    local.get 0
                    local.get 1
                    i32.const 1
                    i32.add
                    i32.load align=1
                    i32.store offset=1
                    local.get 0
                    local.get 1
                    i32.const 5
                    i32.add
                    i64.load align=1
                    i64.store offset=5 align=4
                    local.get 1
                    i32.const 13
                    i32.add
                    i32.load align=1
                    local.set 6
                    i32.const 15
                    local.set 2
                    i32.const 17
                    local.set 4
                    i32.const 13
                    br 2 (;@6;)
                  end
                  local.get 2
                  i32.const 16
                  i32.and
                  if  ;; label = @8
                    local.get 0
                    local.get 1
                    i32.load8_u
                    i32.store8
                    local.get 0
                    local.get 1
                    i32.load offset=1 align=1
                    i32.store offset=1 align=1
                    local.get 0
                    local.get 1
                    i64.load offset=5 align=1
                    i64.store offset=5 align=1
                    local.get 0
                    local.get 1
                    i32.load16_u offset=13 align=1
                    i32.store16 offset=13 align=1
                    local.get 0
                    local.get 1
                    i32.load8_u offset=15
                    i32.store8 offset=15
                    local.get 1
                    i32.const 16
                    i32.add
                    local.set 1
                    local.get 0
                    i32.const 16
                    i32.add
                    local.set 0
                  end
                  local.get 0
                  local.get 2
                  i32.const 8
                  i32.and
                  br_if 2 (;@5;)
                  drop
                  br 3 (;@4;)
                end
                local.get 0
                local.get 1
                i32.load
                local.tee 4
                i32.store8
                local.get 0
                local.get 4
                i32.const 16
                i32.shr_u
                i32.store8 offset=2
                local.get 0
                local.get 4
                i32.const 8
                i32.shr_u
                i32.store8 offset=1
                local.get 0
                local.get 1
                i32.const 3
                i32.add
                i32.load align=1
                i32.store offset=3
                local.get 0
                local.get 1
                i32.const 7
                i32.add
                i64.load align=1
                i64.store offset=7 align=4
                local.get 1
                i32.const 15
                i32.add
                i32.load align=1
                local.set 6
                i32.const 13
                local.set 2
                i32.const 19
                local.set 4
                i32.const 15
              end
              local.get 0
              i32.add
              local.get 6
              i32.store
              local.get 1
              local.get 4
              i32.add
              local.set 1
              local.get 0
              local.get 4
              i32.add
            end
            local.tee 4
            local.get 1
            i64.load align=1
            i64.store align=1
            local.get 4
            i32.const 8
            i32.add
            local.set 0
            local.get 1
            i32.const 8
            i32.add
            local.set 1
          end
          local.get 2
          i32.const 4
          i32.and
          if  ;; label = @4
            local.get 0
            local.get 1
            i32.load align=1
            i32.store align=1
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 0
            i32.const 4
            i32.add
            local.set 0
          end
          local.get 2
          i32.const 2
          i32.and
          if  ;; label = @4
            local.get 0
            local.get 1
            i32.load16_u align=1
            i32.store16 align=1
            local.get 1
            i32.const 2
            i32.add
            local.set 1
            local.get 0
            i32.const 2
            i32.add
            local.set 0
          end
          local.get 2
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          i32.load8_u
          i32.store8
        end
      end
      local.get 5
      local.get 5
      i32.load offset=20
      local.get 3
      i32.add
      i32.store offset=20
    end
    i32.const 2208
    i32.load
    local.tee 3
    if  ;; label = @1
      loop  ;; label = @2
        local.get 3
        i32.load offset=20
        local.get 3
        i32.load offset=24
        i32.ne
        if  ;; label = @3
          local.get 3
          i32.const 0
          i32.const 0
          local.get 3
          i32.load offset=32
          call_indirect (type 0)
          drop
        end
        local.get 3
        i32.load offset=4
        local.tee 2
        local.get 3
        i32.load offset=8
        local.tee 4
        i32.ne
        if  ;; label = @3
          local.get 3
          local.get 2
          local.get 4
          i32.sub
          i64.extend_i32_s
          i32.const 1
          local.get 3
          i32.load offset=36
          call_indirect (type 1)
          drop
        end
        local.get 3
        i32.load offset=52
        local.tee 3
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      i32.const 2204
      i32.load
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=20
      local.get 3
      i32.load offset=24
      i32.ne
      if  ;; label = @2
        local.get 3
        i32.const 0
        i32.const 0
        local.get 3
        i32.load offset=32
        call_indirect (type 0)
        drop
      end
      local.get 3
      i32.load offset=4
      local.tee 2
      local.get 3
      i32.load offset=8
      local.tee 4
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      local.get 2
      local.get 4
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 3
      i32.load offset=36
      call_indirect (type 1)
      drop
    end
    block  ;; label = @1
      i32.const 1160
      i32.load
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=20
      local.get 3
      i32.load offset=24
      i32.ne
      if  ;; label = @2
        local.get 3
        i32.const 0
        i32.const 0
        local.get 3
        i32.load offset=32
        call_indirect (type 0)
        drop
      end
      local.get 3
      i32.load offset=4
      local.tee 2
      local.get 3
      i32.load offset=8
      local.tee 4
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      local.get 2
      local.get 4
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 3
      i32.load offset=36
      call_indirect (type 1)
      drop
    end
    block  ;; label = @1
      i32.const 2204
      i32.load
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=20
      local.get 3
      i32.load offset=24
      i32.ne
      if  ;; label = @2
        local.get 3
        i32.const 0
        i32.const 0
        local.get 3
        i32.load offset=32
        call_indirect (type 0)
        drop
      end
      local.get 3
      i32.load offset=4
      local.tee 2
      local.get 3
      i32.load offset=8
      local.tee 4
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      local.get 2
      local.get 4
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 3
      i32.load offset=36
      call_indirect (type 1)
      drop
    end
  )
  (table (;0;) 5 5 funcref)
  (memory (;0;) 2)
  (global (;0;) (mut i32) i32.const 67760)
  (export "memory" (memory 0))
  (export "_start" (func 9))
  (elem (;0;) (i32.const 1) func 4 8 5 7)
  (data (;0;) (i32.const 1024) "Hello world!\00\00\00\00\18\04")
  (data (;1;) (i32.const 1048) "\05")
  (data (;2;) (i32.const 1060) "\01")
  (data (;3;) (i32.const 1080) "\02\00\00\00\03\00\00\00\98\04\00\00\00\04")
  (data (;4;) (i32.const 1104) "\01\00\00\00\00\00\00\00\0a")
  (data (;5;) (i32.const 1160) "\18\04")
)
