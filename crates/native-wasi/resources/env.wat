(module
  (type (;0;) (func (param i32 i32 i32) (result i32)))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32 i64 i32) (result i64)))
  (type (;4;) (func (param i32)))
  (type (;5;) (func))
  (type (;6;) (func (param i32 i64 i32 i32) (result i32)))
  (type (;7;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;8;) (func (result i32)))
  (import "wasi_snapshot_preview1" "args_get" (func (;0;) (type 1)))
  (import "wasi_snapshot_preview1" "args_sizes_get" (func (;1;) (type 1)))
  (import "wasi_snapshot_preview1" "environ_get" (func (;2;) (type 1)))
  (import "wasi_snapshot_preview1" "environ_sizes_get" (func (;3;) (type 1)))
  (import "wasi_snapshot_preview1" "fd_close" (func (;4;) (type 2)))
  (import "wasi_snapshot_preview1" "fd_fdstat_get" (func (;5;) (type 1)))
  (import "wasi_snapshot_preview1" "fd_seek" (func (;6;) (type 6)))
  (import "wasi_snapshot_preview1" "fd_write" (func (;7;) (type 7)))
  (import "wasi_snapshot_preview1" "proc_exit" (func (;8;) (type 4)))
  (func (;9;) (type 2) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 11
    global.set 0
    block  ;; label = @1
      i32.const 1196
      i32.load
      br_if 0 (;@1;)
      i32.const 0
      call 13
      i32.const 68256
      i32.sub
      local.tee 5
      i32.const 89
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 1644
      i32.load
      local.tee 2
      i32.eqz
      if  ;; label = @2
        i32.const 1656
        i64.const -1
        i64.store align=4
        i32.const 1648
        i64.const 281474976776192
        i64.store align=4
        i32.const 1644
        local.get 11
        i32.const 8
        i32.add
        i32.const -16
        i32.and
        i32.const 1431655768
        i32.xor
        local.tee 2
        i32.store
        i32.const 1664
        i32.const 0
        i32.store
        i32.const 1616
        i32.const 0
        i32.store
      end
      i32.const 1624
      local.get 5
      i32.store
      i32.const 1620
      i32.const 68256
      i32.store
      i32.const 1188
      i32.const 68256
      i32.store
      i32.const 1208
      local.get 2
      i32.store
      i32.const 1204
      i32.const -1
      i32.store
      loop  ;; label = @2
        local.get 1
        i32.const 1232
        i32.add
        local.get 1
        i32.const 1220
        i32.add
        local.tee 2
        i32.store
        local.get 2
        local.get 1
        i32.const 1212
        i32.add
        local.tee 3
        i32.store
        local.get 1
        i32.const 1224
        i32.add
        local.get 3
        i32.store
        local.get 1
        i32.const 1240
        i32.add
        local.get 1
        i32.const 1228
        i32.add
        local.tee 3
        i32.store
        local.get 3
        local.get 2
        i32.store
        local.get 1
        i32.const 1248
        i32.add
        local.get 1
        i32.const 1236
        i32.add
        local.tee 2
        i32.store
        local.get 2
        local.get 3
        i32.store
        local.get 1
        i32.const 1244
        i32.add
        local.get 2
        i32.store
        local.get 1
        i32.const 32
        i32.add
        local.tee 1
        i32.const 256
        i32.ne
        br_if 0 (;@2;)
      end
      i32.const 8
      local.tee 1
      i32.const 68256
      i32.add
      local.tee 2
      i32.const 4
      i32.add
      local.get 5
      local.get 1
      i32.sub
      i32.const 56
      i32.sub
      local.tee 1
      i32.const 1
      i32.or
      i32.store
      i32.const 1200
      i32.const 1660
      i32.load
      i32.store
      i32.const 1196
      local.get 2
      i32.store
      i32.const 1184
      local.get 1
      i32.store
      local.get 5
      i32.const 68204
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
                          local.get 0
                          i32.const 236
                          i32.le_u
                          if  ;; label = @12
                            i32.const 1172
                            i32.load
                            local.tee 6
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
                            local.tee 5
                            i32.const 3
                            i32.shr_u
                            local.tee 2
                            i32.shr_u
                            local.tee 1
                            i32.const 3
                            i32.and
                            if  ;; label = @13
                              local.get 1
                              i32.const 1
                              i32.and
                              local.get 2
                              i32.or
                              i32.const 1
                              i32.xor
                              local.tee 3
                              i32.const 3
                              i32.shl
                              local.tee 0
                              i32.const 1220
                              i32.add
                              i32.load
                              local.tee 2
                              i32.const 8
                              i32.add
                              local.set 1
                              block  ;; label = @14
                                local.get 2
                                i32.load offset=8
                                local.tee 5
                                local.get 0
                                i32.const 1212
                                i32.add
                                local.tee 0
                                i32.eq
                                if  ;; label = @15
                                  i32.const 1172
                                  local.get 6
                                  i32.const -2
                                  local.get 3
                                  i32.rotl
                                  i32.and
                                  i32.store
                                  br 1 (;@14;)
                                end
                                local.get 0
                                local.get 5
                                i32.store offset=8
                                local.get 5
                                local.get 0
                                i32.store offset=12
                              end
                              local.get 2
                              local.get 3
                              i32.const 3
                              i32.shl
                              local.tee 3
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 2
                              local.get 3
                              i32.add
                              i32.const 4
                              i32.add
                              local.tee 2
                              local.get 2
                              i32.load
                              i32.const 1
                              i32.or
                              i32.store
                              br 12 (;@1;)
                            end
                            local.get 5
                            i32.const 1180
                            i32.load
                            local.tee 8
                            i32.le_u
                            br_if 1 (;@11;)
                            local.get 1
                            if  ;; label = @13
                              block  ;; label = @14
                                local.get 1
                                local.get 2
                                i32.shl
                                i32.const 2
                                local.get 2
                                i32.shl
                                local.tee 1
                                i32.const 0
                                local.get 1
                                i32.sub
                                i32.or
                                i32.and
                                local.tee 1
                                i32.const 0
                                local.get 1
                                i32.sub
                                i32.and
                                i32.const 1
                                i32.sub
                                local.tee 1
                                local.get 1
                                i32.const 12
                                i32.shr_u
                                i32.const 16
                                i32.and
                                local.tee 1
                                i32.shr_u
                                local.tee 2
                                i32.const 5
                                i32.shr_u
                                i32.const 8
                                i32.and
                                local.tee 3
                                local.get 1
                                i32.or
                                local.get 2
                                local.get 3
                                i32.shr_u
                                local.tee 1
                                i32.const 2
                                i32.shr_u
                                i32.const 4
                                i32.and
                                local.tee 2
                                i32.or
                                local.get 1
                                local.get 2
                                i32.shr_u
                                local.tee 1
                                i32.const 1
                                i32.shr_u
                                i32.const 2
                                i32.and
                                local.tee 2
                                i32.or
                                local.get 1
                                local.get 2
                                i32.shr_u
                                local.tee 1
                                i32.const 1
                                i32.shr_u
                                i32.const 1
                                i32.and
                                local.tee 2
                                i32.or
                                local.get 1
                                local.get 2
                                i32.shr_u
                                i32.add
                                local.tee 3
                                i32.const 3
                                i32.shl
                                local.tee 0
                                i32.const 1220
                                i32.add
                                i32.load
                                local.tee 2
                                i32.load offset=8
                                local.tee 1
                                local.get 0
                                i32.const 1212
                                i32.add
                                local.tee 0
                                i32.eq
                                if  ;; label = @15
                                  i32.const 1172
                                  local.get 6
                                  i32.const -2
                                  local.get 3
                                  i32.rotl
                                  i32.and
                                  local.tee 6
                                  i32.store
                                  br 1 (;@14;)
                                end
                                local.get 0
                                local.get 1
                                i32.store offset=8
                                local.get 1
                                local.get 0
                                i32.store offset=12
                              end
                              local.get 2
                              i32.const 8
                              i32.add
                              local.set 1
                              local.get 2
                              local.get 5
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 2
                              local.get 3
                              i32.const 3
                              i32.shl
                              local.tee 3
                              i32.add
                              local.get 3
                              local.get 5
                              i32.sub
                              local.tee 3
                              i32.store
                              local.get 2
                              local.get 5
                              i32.add
                              local.tee 0
                              local.get 3
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              local.get 8
                              if  ;; label = @14
                                local.get 8
                                i32.const 3
                                i32.shr_u
                                local.tee 4
                                i32.const 3
                                i32.shl
                                i32.const 1212
                                i32.add
                                local.set 5
                                i32.const 1192
                                i32.load
                                local.set 2
                                block (result i32)  ;; label = @15
                                  local.get 6
                                  i32.const 1
                                  local.get 4
                                  i32.shl
                                  local.tee 4
                                  i32.and
                                  i32.eqz
                                  if  ;; label = @16
                                    i32.const 1172
                                    local.get 4
                                    local.get 6
                                    i32.or
                                    i32.store
                                    local.get 5
                                    br 1 (;@15;)
                                  end
                                  local.get 5
                                  i32.load offset=8
                                end
                                local.tee 4
                                local.get 2
                                i32.store offset=12
                                local.get 5
                                local.get 2
                                i32.store offset=8
                                local.get 2
                                local.get 5
                                i32.store offset=12
                                local.get 2
                                local.get 4
                                i32.store offset=8
                              end
                              i32.const 1192
                              local.get 0
                              i32.store
                              i32.const 1180
                              local.get 3
                              i32.store
                              br 12 (;@1;)
                            end
                            i32.const 1176
                            i32.load
                            local.tee 9
                            i32.eqz
                            br_if 1 (;@11;)
                            local.get 9
                            i32.const 0
                            local.get 9
                            i32.sub
                            i32.and
                            i32.const 1
                            i32.sub
                            local.tee 1
                            local.get 1
                            i32.const 12
                            i32.shr_u
                            i32.const 16
                            i32.and
                            local.tee 1
                            i32.shr_u
                            local.tee 2
                            i32.const 5
                            i32.shr_u
                            i32.const 8
                            i32.and
                            local.tee 3
                            local.get 1
                            i32.or
                            local.get 2
                            local.get 3
                            i32.shr_u
                            local.tee 1
                            i32.const 2
                            i32.shr_u
                            i32.const 4
                            i32.and
                            local.tee 2
                            i32.or
                            local.get 1
                            local.get 2
                            i32.shr_u
                            local.tee 1
                            i32.const 1
                            i32.shr_u
                            i32.const 2
                            i32.and
                            local.tee 2
                            i32.or
                            local.get 1
                            local.get 2
                            i32.shr_u
                            local.tee 1
                            i32.const 1
                            i32.shr_u
                            i32.const 1
                            i32.and
                            local.tee 2
                            i32.or
                            local.get 1
                            local.get 2
                            i32.shr_u
                            i32.add
                            i32.const 2
                            i32.shl
                            i32.const 1476
                            i32.add
                            i32.load
                            local.tee 0
                            i32.load offset=4
                            i32.const -8
                            i32.and
                            local.get 5
                            i32.sub
                            local.set 2
                            local.get 0
                            local.set 3
                            loop  ;; label = @13
                              block  ;; label = @14
                                local.get 3
                                i32.load offset=16
                                local.tee 1
                                i32.eqz
                                if  ;; label = @15
                                  local.get 3
                                  i32.const 20
                                  i32.add
                                  i32.load
                                  local.tee 1
                                  i32.eqz
                                  br_if 1 (;@14;)
                                end
                                local.get 1
                                i32.load offset=4
                                i32.const -8
                                i32.and
                                local.get 5
                                i32.sub
                                local.tee 3
                                local.get 2
                                local.get 2
                                local.get 3
                                i32.gt_u
                                local.tee 3
                                select
                                local.set 2
                                local.get 1
                                local.get 0
                                local.get 3
                                select
                                local.set 0
                                local.get 1
                                local.set 3
                                br 1 (;@13;)
                              end
                            end
                            local.get 0
                            i32.load offset=24
                            local.set 10
                            local.get 0
                            local.get 0
                            i32.load offset=12
                            local.tee 4
                            i32.ne
                            if  ;; label = @13
                              local.get 0
                              i32.load offset=8
                              local.tee 1
                              i32.const 1188
                              i32.load
                              i32.lt_u
                              drop
                              local.get 4
                              local.get 1
                              i32.store offset=8
                              local.get 1
                              local.get 4
                              i32.store offset=12
                              br 11 (;@2;)
                            end
                            local.get 0
                            i32.const 20
                            i32.add
                            local.tee 3
                            i32.load
                            local.tee 1
                            i32.eqz
                            if  ;; label = @13
                              local.get 0
                              i32.load offset=16
                              local.tee 1
                              i32.eqz
                              br_if 3 (;@10;)
                              local.get 0
                              i32.const 16
                              i32.add
                              local.set 3
                            end
                            loop  ;; label = @13
                              local.get 3
                              local.set 7
                              local.get 1
                              local.tee 4
                              i32.const 20
                              i32.add
                              local.tee 3
                              i32.load
                              local.tee 1
                              br_if 0 (;@13;)
                              local.get 4
                              i32.const 16
                              i32.add
                              local.set 3
                              local.get 4
                              i32.load offset=16
                              local.tee 1
                              br_if 0 (;@13;)
                            end
                            local.get 7
                            i32.const 0
                            i32.store
                            br 10 (;@2;)
                          end
                          i32.const -1
                          local.set 5
                          local.get 0
                          i32.const -65
                          i32.gt_u
                          br_if 0 (;@11;)
                          local.get 0
                          i32.const 19
                          i32.add
                          local.tee 1
                          i32.const -16
                          i32.and
                          local.set 5
                          i32.const 1176
                          i32.load
                          local.tee 8
                          i32.eqz
                          br_if 0 (;@11;)
                          block (result i32)  ;; label = @12
                            i32.const 0
                            local.get 5
                            i32.const 256
                            i32.lt_u
                            br_if 0 (;@12;)
                            drop
                            i32.const 31
                            local.get 5
                            i32.const 16777215
                            i32.gt_u
                            br_if 0 (;@12;)
                            drop
                            local.get 1
                            i32.const 8
                            i32.shr_u
                            local.tee 1
                            local.get 1
                            i32.const 1048320
                            i32.add
                            i32.const 16
                            i32.shr_u
                            i32.const 8
                            i32.and
                            local.tee 1
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
                            local.tee 3
                            local.get 3
                            i32.const 245760
                            i32.add
                            i32.const 16
                            i32.shr_u
                            i32.const 2
                            i32.and
                            local.tee 3
                            i32.shl
                            i32.const 15
                            i32.shr_u
                            local.get 1
                            local.get 2
                            i32.or
                            local.get 3
                            i32.or
                            i32.sub
                            local.tee 1
                            i32.const 1
                            i32.shl
                            local.get 5
                            local.get 1
                            i32.const 21
                            i32.add
                            i32.shr_u
                            i32.const 1
                            i32.and
                            i32.or
                            i32.const 28
                            i32.add
                          end
                          local.set 7
                          i32.const 0
                          local.get 5
                          i32.sub
                          local.set 2
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 7
                                i32.const 2
                                i32.shl
                                i32.const 1476
                                i32.add
                                i32.load
                                local.tee 3
                                i32.eqz
                                if  ;; label = @15
                                  i32.const 0
                                  local.set 1
                                  br 1 (;@14;)
                                end
                                i32.const 0
                                local.set 1
                                local.get 5
                                i32.const 0
                                i32.const 25
                                local.get 7
                                i32.const 1
                                i32.shr_u
                                i32.sub
                                local.get 7
                                i32.const 31
                                i32.eq
                                select
                                i32.shl
                                local.set 0
                                loop  ;; label = @15
                                  block  ;; label = @16
                                    local.get 3
                                    i32.load offset=4
                                    i32.const -8
                                    i32.and
                                    local.get 5
                                    i32.sub
                                    local.tee 6
                                    local.get 2
                                    i32.ge_u
                                    br_if 0 (;@16;)
                                    local.get 3
                                    local.set 4
                                    local.get 6
                                    local.tee 2
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.set 2
                                    local.get 3
                                    local.set 1
                                    br 3 (;@13;)
                                  end
                                  local.get 1
                                  local.get 3
                                  i32.const 20
                                  i32.add
                                  i32.load
                                  local.tee 6
                                  local.get 6
                                  local.get 3
                                  local.get 0
                                  i32.const 29
                                  i32.shr_u
                                  i32.const 4
                                  i32.and
                                  i32.add
                                  i32.const 16
                                  i32.add
                                  i32.load
                                  local.tee 3
                                  i32.eq
                                  select
                                  local.get 1
                                  local.get 6
                                  select
                                  local.set 1
                                  local.get 0
                                  i32.const 1
                                  i32.shl
                                  local.set 0
                                  local.get 3
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 1
                              local.get 4
                              i32.or
                              i32.eqz
                              if  ;; label = @14
                                i32.const 0
                                local.set 4
                                i32.const 2
                                local.get 7
                                i32.shl
                                local.tee 1
                                i32.const 0
                                local.get 1
                                i32.sub
                                i32.or
                                local.get 8
                                i32.and
                                local.tee 1
                                i32.eqz
                                br_if 3 (;@11;)
                                local.get 1
                                i32.const 0
                                local.get 1
                                i32.sub
                                i32.and
                                i32.const 1
                                i32.sub
                                local.tee 1
                                local.get 1
                                i32.const 12
                                i32.shr_u
                                i32.const 16
                                i32.and
                                local.tee 1
                                i32.shr_u
                                local.tee 3
                                i32.const 5
                                i32.shr_u
                                i32.const 8
                                i32.and
                                local.tee 0
                                local.get 1
                                i32.or
                                local.get 3
                                local.get 0
                                i32.shr_u
                                local.tee 1
                                i32.const 2
                                i32.shr_u
                                i32.const 4
                                i32.and
                                local.tee 3
                                i32.or
                                local.get 1
                                local.get 3
                                i32.shr_u
                                local.tee 1
                                i32.const 1
                                i32.shr_u
                                i32.const 2
                                i32.and
                                local.tee 3
                                i32.or
                                local.get 1
                                local.get 3
                                i32.shr_u
                                local.tee 1
                                i32.const 1
                                i32.shr_u
                                i32.const 1
                                i32.and
                                local.tee 3
                                i32.or
                                local.get 1
                                local.get 3
                                i32.shr_u
                                i32.add
                                i32.const 2
                                i32.shl
                                i32.const 1476
                                i32.add
                                i32.load
                                local.set 1
                              end
                              local.get 1
                              i32.eqz
                              br_if 1 (;@12;)
                            end
                            loop  ;; label = @13
                              local.get 1
                              i32.load offset=4
                              i32.const -8
                              i32.and
                              local.get 5
                              i32.sub
                              local.tee 6
                              local.get 2
                              i32.lt_u
                              local.set 0
                              local.get 6
                              local.get 2
                              local.get 0
                              select
                              local.set 2
                              local.get 1
                              local.get 4
                              local.get 0
                              select
                              local.set 4
                              local.get 1
                              i32.load offset=16
                              local.tee 3
                              if (result i32)  ;; label = @14
                                local.get 3
                              else
                                local.get 1
                                i32.const 20
                                i32.add
                                i32.load
                              end
                              local.tee 1
                              br_if 0 (;@13;)
                            end
                          end
                          local.get 4
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 2
                          i32.const 1180
                          i32.load
                          local.get 5
                          i32.sub
                          i32.ge_u
                          br_if 0 (;@11;)
                          local.get 4
                          i32.load offset=24
                          local.set 7
                          local.get 4
                          local.get 4
                          i32.load offset=12
                          local.tee 0
                          i32.ne
                          if  ;; label = @12
                            local.get 4
                            i32.load offset=8
                            local.tee 1
                            i32.const 1188
                            i32.load
                            i32.lt_u
                            drop
                            local.get 0
                            local.get 1
                            i32.store offset=8
                            local.get 1
                            local.get 0
                            i32.store offset=12
                            br 9 (;@3;)
                          end
                          local.get 4
                          i32.const 20
                          i32.add
                          local.tee 3
                          i32.load
                          local.tee 1
                          i32.eqz
                          if  ;; label = @12
                            local.get 4
                            i32.load offset=16
                            local.tee 1
                            i32.eqz
                            br_if 3 (;@9;)
                            local.get 4
                            i32.const 16
                            i32.add
                            local.set 3
                          end
                          loop  ;; label = @12
                            local.get 3
                            local.set 6
                            local.get 1
                            local.tee 0
                            i32.const 20
                            i32.add
                            local.tee 3
                            i32.load
                            local.tee 1
                            br_if 0 (;@12;)
                            local.get 0
                            i32.const 16
                            i32.add
                            local.set 3
                            local.get 0
                            i32.load offset=16
                            local.tee 1
                            br_if 0 (;@12;)
                          end
                          local.get 6
                          i32.const 0
                          i32.store
                          br 8 (;@3;)
                        end
                        local.get 5
                        i32.const 1180
                        i32.load
                        local.tee 1
                        i32.le_u
                        if  ;; label = @11
                          i32.const 1192
                          i32.load
                          local.set 2
                          block  ;; label = @12
                            local.get 1
                            local.get 5
                            i32.sub
                            local.tee 3
                            i32.const 16
                            i32.ge_u
                            if  ;; label = @13
                              local.get 2
                              local.get 5
                              i32.add
                              local.tee 0
                              local.get 3
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              i32.const 1180
                              local.get 3
                              i32.store
                              i32.const 1192
                              local.get 0
                              i32.store
                              local.get 1
                              local.get 2
                              i32.add
                              local.get 3
                              i32.store
                              local.get 2
                              local.get 5
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              br 1 (;@12;)
                            end
                            local.get 2
                            local.get 1
                            i32.const 3
                            i32.or
                            i32.store offset=4
                            local.get 1
                            local.get 2
                            i32.add
                            i32.const 4
                            i32.add
                            local.tee 1
                            local.get 1
                            i32.load
                            i32.const 1
                            i32.or
                            i32.store
                            i32.const 1192
                            i32.const 0
                            i32.store
                            i32.const 1180
                            i32.const 0
                            i32.store
                          end
                          local.get 2
                          i32.const 8
                          i32.add
                          local.set 1
                          br 10 (;@1;)
                        end
                        local.get 5
                        i32.const 1184
                        i32.load
                        local.tee 0
                        i32.lt_u
                        if  ;; label = @11
                          i32.const 1196
                          i32.load
                          local.tee 1
                          local.get 5
                          i32.add
                          local.tee 2
                          local.get 0
                          local.get 5
                          i32.sub
                          local.tee 3
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          i32.const 1184
                          local.get 3
                          i32.store
                          i32.const 1196
                          local.get 2
                          i32.store
                          local.get 1
                          local.get 5
                          i32.const 3
                          i32.or
                          i32.store offset=4
                          local.get 1
                          i32.const 8
                          i32.add
                          local.set 1
                          br 10 (;@1;)
                        end
                        i32.const 0
                        local.set 1
                        local.get 5
                        local.get 5
                        i32.const 71
                        i32.add
                        local.tee 8
                        block (result i32)  ;; label = @11
                          i32.const 1644
                          i32.load
                          if  ;; label = @12
                            i32.const 1652
                            i32.load
                            br 1 (;@11;)
                          end
                          i32.const 1656
                          i64.const -1
                          i64.store align=4
                          i32.const 1648
                          i64.const 281474976776192
                          i64.store align=4
                          i32.const 1644
                          local.get 11
                          i32.const 12
                          i32.add
                          i32.const -16
                          i32.and
                          i32.const 1431655768
                          i32.xor
                          i32.store
                          i32.const 1664
                          i32.const 0
                          i32.store
                          i32.const 1616
                          i32.const 0
                          i32.store
                          i32.const 65536
                        end
                        local.tee 2
                        i32.add
                        local.tee 6
                        i32.const 0
                        local.get 2
                        i32.sub
                        local.tee 7
                        i32.and
                        local.tee 4
                        i32.ge_u
                        if  ;; label = @11
                          i32.const 1168
                          i32.const 48
                          i32.store
                          br 10 (;@1;)
                        end
                        block  ;; label = @11
                          i32.const 1612
                          i32.load
                          local.tee 1
                          i32.eqz
                          br_if 0 (;@11;)
                          i32.const 1604
                          i32.load
                          local.tee 2
                          local.get 4
                          i32.add
                          local.tee 3
                          local.get 2
                          i32.gt_u
                          local.get 1
                          local.get 3
                          i32.ge_u
                          i32.and
                          br_if 0 (;@11;)
                          i32.const 0
                          local.set 1
                          i32.const 1168
                          i32.const 48
                          i32.store
                          br 10 (;@1;)
                        end
                        i32.const 1616
                        i32.load8_u
                        i32.const 4
                        i32.and
                        br_if 4 (;@6;)
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 1196
                            i32.load
                            local.tee 2
                            if  ;; label = @13
                              i32.const 1620
                              local.set 1
                              loop  ;; label = @14
                                local.get 2
                                local.get 1
                                i32.load
                                local.tee 3
                                i32.ge_u
                                if  ;; label = @15
                                  local.get 3
                                  local.get 1
                                  i32.load offset=4
                                  i32.add
                                  local.get 2
                                  i32.gt_u
                                  br_if 3 (;@12;)
                                end
                                local.get 1
                                i32.load offset=8
                                local.tee 1
                                br_if 0 (;@14;)
                              end
                            end
                            i32.const 0
                            call 13
                            local.tee 0
                            i32.const -1
                            i32.eq
                            br_if 5 (;@7;)
                            local.get 4
                            local.set 6
                            i32.const 1648
                            i32.load
                            local.tee 1
                            i32.const 1
                            i32.sub
                            local.tee 2
                            local.get 0
                            i32.and
                            if  ;; label = @13
                              local.get 4
                              local.get 0
                              i32.sub
                              local.get 0
                              local.get 2
                              i32.add
                              i32.const 0
                              local.get 1
                              i32.sub
                              i32.and
                              i32.add
                              local.set 6
                            end
                            local.get 5
                            local.get 6
                            i32.ge_u
                            br_if 5 (;@7;)
                            local.get 6
                            i32.const 2147483646
                            i32.gt_u
                            br_if 5 (;@7;)
                            i32.const 1612
                            i32.load
                            local.tee 1
                            if  ;; label = @13
                              i32.const 1604
                              i32.load
                              local.tee 2
                              local.get 6
                              i32.add
                              local.tee 3
                              local.get 2
                              i32.le_u
                              br_if 6 (;@7;)
                              local.get 1
                              local.get 3
                              i32.lt_u
                              br_if 6 (;@7;)
                            end
                            local.get 6
                            call 13
                            local.tee 1
                            local.get 0
                            i32.ne
                            br_if 1 (;@11;)
                            br 7 (;@5;)
                          end
                          local.get 6
                          local.get 0
                          i32.sub
                          local.get 7
                          i32.and
                          local.tee 6
                          i32.const 2147483646
                          i32.gt_u
                          br_if 4 (;@7;)
                          local.get 6
                          call 13
                          local.tee 0
                          local.get 1
                          i32.load
                          local.get 1
                          i32.load offset=4
                          i32.add
                          i32.eq
                          br_if 3 (;@8;)
                          local.get 0
                          local.set 1
                        end
                        block  ;; label = @11
                          local.get 1
                          i32.const -1
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 5
                          i32.const 72
                          i32.add
                          local.get 6
                          i32.le_u
                          br_if 0 (;@11;)
                          i32.const 1652
                          i32.load
                          local.tee 2
                          local.get 8
                          local.get 6
                          i32.sub
                          i32.add
                          i32.const 0
                          local.get 2
                          i32.sub
                          i32.and
                          local.tee 2
                          i32.const 2147483646
                          i32.gt_u
                          if  ;; label = @12
                            local.get 1
                            local.set 0
                            br 7 (;@5;)
                          end
                          local.get 2
                          call 13
                          i32.const -1
                          i32.ne
                          if  ;; label = @12
                            local.get 2
                            local.get 6
                            i32.add
                            local.set 6
                            local.get 1
                            local.set 0
                            br 7 (;@5;)
                          end
                          i32.const 0
                          local.get 6
                          i32.sub
                          call 13
                          drop
                          br 4 (;@7;)
                        end
                        local.get 1
                        local.set 0
                        local.get 1
                        i32.const -1
                        i32.ne
                        br_if 5 (;@5;)
                        br 3 (;@7;)
                      end
                      i32.const 0
                      local.set 4
                      br 7 (;@2;)
                    end
                    i32.const 0
                    local.set 0
                    br 5 (;@3;)
                  end
                  local.get 0
                  i32.const -1
                  i32.ne
                  br_if 2 (;@5;)
                end
                i32.const 1616
                i32.const 1616
                i32.load
                i32.const 4
                i32.or
                i32.store
              end
              local.get 4
              i32.const 2147483646
              i32.gt_u
              br_if 1 (;@4;)
              local.get 4
              call 13
              local.set 0
              i32.const 0
              call 13
              local.set 1
              local.get 0
              i32.const -1
              i32.eq
              br_if 1 (;@4;)
              local.get 1
              i32.const -1
              i32.eq
              br_if 1 (;@4;)
              local.get 0
              local.get 1
              i32.ge_u
              br_if 1 (;@4;)
              local.get 1
              local.get 0
              i32.sub
              local.tee 6
              local.get 5
              i32.const 56
              i32.add
              i32.le_u
              br_if 1 (;@4;)
            end
            i32.const 1604
            i32.const 1604
            i32.load
            local.get 6
            i32.add
            local.tee 1
            i32.store
            i32.const 1608
            i32.load
            local.get 1
            i32.lt_u
            if  ;; label = @5
              i32.const 1608
              local.get 1
              i32.store
            end
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  i32.const 1196
                  i32.load
                  local.tee 2
                  if  ;; label = @8
                    i32.const 1620
                    local.set 1
                    loop  ;; label = @9
                      local.get 0
                      local.get 1
                      i32.load
                      local.tee 3
                      local.get 1
                      i32.load offset=4
                      local.tee 4
                      i32.add
                      i32.eq
                      br_if 2 (;@7;)
                      local.get 1
                      i32.load offset=8
                      local.tee 1
                      br_if 0 (;@9;)
                    end
                    br 2 (;@6;)
                  end
                  i32.const 1188
                  i32.load
                  local.tee 1
                  i32.const 0
                  local.get 0
                  local.get 1
                  i32.ge_u
                  select
                  i32.eqz
                  if  ;; label = @8
                    i32.const 1188
                    local.get 0
                    i32.store
                  end
                  i32.const 0
                  local.set 1
                  i32.const 1624
                  local.get 6
                  i32.store
                  i32.const 1620
                  local.get 0
                  i32.store
                  i32.const 1204
                  i32.const -1
                  i32.store
                  i32.const 1208
                  i32.const 1644
                  i32.load
                  i32.store
                  i32.const 1632
                  i32.const 0
                  i32.store
                  loop  ;; label = @8
                    local.get 1
                    i32.const 1232
                    i32.add
                    local.get 1
                    i32.const 1220
                    i32.add
                    local.tee 2
                    i32.store
                    local.get 2
                    local.get 1
                    i32.const 1212
                    i32.add
                    local.tee 3
                    i32.store
                    local.get 1
                    i32.const 1224
                    i32.add
                    local.get 3
                    i32.store
                    local.get 1
                    i32.const 1240
                    i32.add
                    local.get 1
                    i32.const 1228
                    i32.add
                    local.tee 3
                    i32.store
                    local.get 3
                    local.get 2
                    i32.store
                    local.get 1
                    i32.const 1248
                    i32.add
                    local.get 1
                    i32.const 1236
                    i32.add
                    local.tee 2
                    i32.store
                    local.get 2
                    local.get 3
                    i32.store
                    local.get 1
                    i32.const 1244
                    i32.add
                    local.get 2
                    i32.store
                    local.get 1
                    i32.const 32
                    i32.add
                    local.tee 1
                    i32.const 256
                    i32.ne
                    br_if 0 (;@8;)
                  end
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
                  local.tee 1
                  i32.add
                  local.tee 2
                  local.get 6
                  local.get 1
                  i32.sub
                  i32.const 56
                  i32.sub
                  local.tee 1
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  i32.const 1200
                  i32.const 1660
                  i32.load
                  i32.store
                  i32.const 1196
                  local.get 2
                  i32.store
                  i32.const 1184
                  local.get 1
                  i32.store
                  local.get 0
                  local.get 6
                  i32.add
                  i32.const 52
                  i32.sub
                  i32.const 56
                  i32.store
                  br 2 (;@5;)
                end
                local.get 1
                i32.load8_u offset=12
                i32.const 8
                i32.and
                br_if 0 (;@6;)
                local.get 2
                local.get 3
                i32.lt_u
                br_if 0 (;@6;)
                local.get 0
                local.get 2
                i32.le_u
                br_if 0 (;@6;)
                local.get 2
                i32.const -8
                local.get 2
                i32.sub
                i32.const 15
                i32.and
                i32.const 0
                local.get 2
                i32.const 8
                i32.add
                i32.const 15
                i32.and
                select
                local.tee 3
                i32.add
                local.tee 0
                i32.const 1184
                i32.load
                local.get 6
                i32.add
                local.tee 7
                local.get 3
                i32.sub
                local.tee 3
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 1
                local.get 4
                local.get 6
                i32.add
                i32.store offset=4
                i32.const 1200
                i32.const 1660
                i32.load
                i32.store
                i32.const 1184
                local.get 3
                i32.store
                i32.const 1196
                local.get 0
                i32.store
                local.get 2
                local.get 7
                i32.add
                i32.const 4
                i32.add
                i32.const 56
                i32.store
                br 1 (;@5;)
              end
              i32.const 1188
              i32.load
              local.get 0
              i32.gt_u
              if  ;; label = @6
                i32.const 1188
                local.get 0
                i32.store
              end
              local.get 0
              local.get 6
              i32.add
              local.set 4
              i32.const 1620
              local.set 1
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          loop  ;; label = @12
                            local.get 4
                            local.get 1
                            i32.load
                            i32.ne
                            if  ;; label = @13
                              local.get 1
                              i32.load offset=8
                              local.tee 1
                              br_if 1 (;@12;)
                              br 2 (;@11;)
                            end
                          end
                          local.get 1
                          i32.load8_u offset=12
                          i32.const 8
                          i32.and
                          i32.eqz
                          br_if 1 (;@10;)
                        end
                        i32.const 1620
                        local.set 1
                        loop  ;; label = @11
                          local.get 2
                          local.get 1
                          i32.load
                          local.tee 3
                          i32.ge_u
                          if  ;; label = @12
                            local.get 3
                            local.get 1
                            i32.load offset=4
                            i32.add
                            local.tee 3
                            local.get 2
                            i32.gt_u
                            br_if 3 (;@9;)
                          end
                          local.get 1
                          i32.load offset=8
                          local.set 1
                          br 0 (;@11;)
                        end
                        unreachable
                      end
                      local.get 1
                      local.get 0
                      i32.store
                      local.get 1
                      local.get 1
                      i32.load offset=4
                      local.get 6
                      i32.add
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
                      local.get 5
                      i32.const 3
                      i32.or
                      i32.store offset=4
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
                      i32.add
                      local.tee 4
                      local.get 5
                      local.get 6
                      i32.add
                      local.tee 5
                      i32.sub
                      local.set 3
                      local.get 2
                      local.get 4
                      i32.eq
                      if  ;; label = @10
                        i32.const 1196
                        local.get 5
                        i32.store
                        i32.const 1184
                        i32.const 1184
                        i32.load
                        local.get 3
                        i32.add
                        local.tee 1
                        i32.store
                        local.get 5
                        local.get 1
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        br 3 (;@7;)
                      end
                      local.get 4
                      i32.const 1192
                      i32.load
                      i32.eq
                      if  ;; label = @10
                        i32.const 1192
                        local.get 5
                        i32.store
                        i32.const 1180
                        i32.const 1180
                        i32.load
                        local.get 3
                        i32.add
                        local.tee 1
                        i32.store
                        local.get 5
                        local.get 1
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 1
                        local.get 5
                        i32.add
                        local.get 1
                        i32.store
                        br 3 (;@7;)
                      end
                      local.get 4
                      i32.load offset=4
                      local.tee 1
                      i32.const 3
                      i32.and
                      i32.const 1
                      i32.eq
                      if  ;; label = @10
                        local.get 1
                        i32.const -8
                        i32.and
                        local.set 8
                        block  ;; label = @11
                          local.get 1
                          i32.const 255
                          i32.le_u
                          if  ;; label = @12
                            local.get 4
                            i32.load offset=8
                            local.tee 2
                            local.get 1
                            i32.const 3
                            i32.shr_u
                            local.tee 7
                            i32.const 3
                            i32.shl
                            i32.const 1212
                            i32.add
                            i32.eq
                            drop
                            local.get 2
                            local.get 4
                            i32.load offset=12
                            local.tee 1
                            i32.eq
                            if  ;; label = @13
                              i32.const 1172
                              i32.const 1172
                              i32.load
                              i32.const -2
                              local.get 7
                              i32.rotl
                              i32.and
                              i32.store
                              br 2 (;@11;)
                            end
                            local.get 1
                            local.get 2
                            i32.store offset=8
                            local.get 2
                            local.get 1
                            i32.store offset=12
                            br 1 (;@11;)
                          end
                          local.get 4
                          i32.load offset=24
                          local.set 9
                          block  ;; label = @12
                            local.get 4
                            local.get 4
                            i32.load offset=12
                            local.tee 0
                            i32.ne
                            if  ;; label = @13
                              local.get 0
                              local.get 4
                              i32.load offset=8
                              local.tee 1
                              i32.store offset=8
                              local.get 1
                              local.get 0
                              i32.store offset=12
                              br 1 (;@12;)
                            end
                            block  ;; label = @13
                              local.get 4
                              i32.const 20
                              i32.add
                              local.tee 1
                              i32.load
                              local.tee 2
                              br_if 0 (;@13;)
                              local.get 4
                              i32.const 16
                              i32.add
                              local.tee 1
                              i32.load
                              local.tee 2
                              br_if 0 (;@13;)
                              i32.const 0
                              local.set 0
                              br 1 (;@12;)
                            end
                            loop  ;; label = @13
                              local.get 1
                              local.set 7
                              local.get 2
                              local.tee 0
                              i32.const 20
                              i32.add
                              local.tee 1
                              i32.load
                              local.tee 2
                              br_if 0 (;@13;)
                              local.get 0
                              i32.const 16
                              i32.add
                              local.set 1
                              local.get 0
                              i32.load offset=16
                              local.tee 2
                              br_if 0 (;@13;)
                            end
                            local.get 7
                            i32.const 0
                            i32.store
                          end
                          local.get 9
                          i32.eqz
                          br_if 0 (;@11;)
                          block  ;; label = @12
                            local.get 4
                            local.get 4
                            i32.load offset=28
                            local.tee 2
                            i32.const 2
                            i32.shl
                            i32.const 1476
                            i32.add
                            local.tee 1
                            i32.load
                            i32.eq
                            if  ;; label = @13
                              local.get 1
                              local.get 0
                              i32.store
                              local.get 0
                              br_if 1 (;@12;)
                              i32.const 1176
                              i32.const 1176
                              i32.load
                              i32.const -2
                              local.get 2
                              i32.rotl
                              i32.and
                              i32.store
                              br 2 (;@11;)
                            end
                            local.get 9
                            i32.const 16
                            i32.const 20
                            local.get 9
                            i32.load offset=16
                            local.get 4
                            i32.eq
                            select
                            i32.add
                            local.get 0
                            i32.store
                            local.get 0
                            i32.eqz
                            br_if 1 (;@11;)
                          end
                          local.get 0
                          local.get 9
                          i32.store offset=24
                          local.get 4
                          i32.load offset=16
                          local.tee 1
                          if  ;; label = @12
                            local.get 0
                            local.get 1
                            i32.store offset=16
                            local.get 1
                            local.get 0
                            i32.store offset=24
                          end
                          local.get 4
                          i32.load offset=20
                          local.tee 1
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 0
                          i32.const 20
                          i32.add
                          local.get 1
                          i32.store
                          local.get 1
                          local.get 0
                          i32.store offset=24
                        end
                        local.get 4
                        local.get 8
                        i32.add
                        local.set 4
                        local.get 3
                        local.get 8
                        i32.add
                        local.set 3
                      end
                      local.get 4
                      local.get 4
                      i32.load offset=4
                      i32.const -2
                      i32.and
                      i32.store offset=4
                      local.get 3
                      local.get 5
                      i32.add
                      local.get 3
                      i32.store
                      local.get 5
                      local.get 3
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 3
                      i32.const 255
                      i32.le_u
                      if  ;; label = @10
                        local.get 3
                        i32.const 3
                        i32.shr_u
                        local.tee 2
                        i32.const 3
                        i32.shl
                        i32.const 1212
                        i32.add
                        local.set 1
                        block (result i32)  ;; label = @11
                          i32.const 1172
                          i32.load
                          local.tee 3
                          i32.const 1
                          local.get 2
                          i32.shl
                          local.tee 2
                          i32.and
                          i32.eqz
                          if  ;; label = @12
                            i32.const 1172
                            local.get 2
                            local.get 3
                            i32.or
                            i32.store
                            local.get 1
                            br 1 (;@11;)
                          end
                          local.get 1
                          i32.load offset=8
                        end
                        local.tee 2
                        local.get 5
                        i32.store offset=12
                        local.get 1
                        local.get 5
                        i32.store offset=8
                        local.get 5
                        local.get 1
                        i32.store offset=12
                        local.get 5
                        local.get 2
                        i32.store offset=8
                        br 3 (;@7;)
                      end
                      i32.const 31
                      local.set 1
                      local.get 3
                      i32.const 16777215
                      i32.le_u
                      if  ;; label = @10
                        local.get 3
                        i32.const 8
                        i32.shr_u
                        local.tee 1
                        local.get 1
                        i32.const 1048320
                        i32.add
                        i32.const 16
                        i32.shr_u
                        i32.const 8
                        i32.and
                        local.tee 1
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
                        local.get 1
                        local.get 2
                        i32.or
                        local.get 0
                        i32.or
                        i32.sub
                        local.tee 1
                        i32.const 1
                        i32.shl
                        local.get 3
                        local.get 1
                        i32.const 21
                        i32.add
                        i32.shr_u
                        i32.const 1
                        i32.and
                        i32.or
                        i32.const 28
                        i32.add
                        local.set 1
                      end
                      local.get 5
                      local.get 1
                      i32.store offset=28
                      local.get 5
                      i64.const 0
                      i64.store offset=16 align=4
                      local.get 1
                      i32.const 2
                      i32.shl
                      i32.const 1476
                      i32.add
                      local.set 2
                      i32.const 1176
                      i32.load
                      local.tee 0
                      i32.const 1
                      local.get 1
                      i32.shl
                      local.tee 4
                      i32.and
                      i32.eqz
                      if  ;; label = @10
                        local.get 2
                        local.get 5
                        i32.store
                        i32.const 1176
                        local.get 0
                        local.get 4
                        i32.or
                        i32.store
                        local.get 5
                        local.get 2
                        i32.store offset=24
                        local.get 5
                        local.get 5
                        i32.store offset=8
                        local.get 5
                        local.get 5
                        i32.store offset=12
                        br 3 (;@7;)
                      end
                      local.get 3
                      i32.const 0
                      i32.const 25
                      local.get 1
                      i32.const 1
                      i32.shr_u
                      i32.sub
                      local.get 1
                      i32.const 31
                      i32.eq
                      select
                      i32.shl
                      local.set 1
                      local.get 2
                      i32.load
                      local.set 0
                      loop  ;; label = @10
                        local.get 0
                        local.tee 2
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.get 3
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 1
                        i32.const 29
                        i32.shr_u
                        local.set 0
                        local.get 1
                        i32.const 1
                        i32.shl
                        local.set 1
                        local.get 2
                        local.get 0
                        i32.const 4
                        i32.and
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee 4
                        i32.load
                        local.tee 0
                        br_if 0 (;@10;)
                      end
                      local.get 4
                      local.get 5
                      i32.store
                      local.get 5
                      local.get 2
                      i32.store offset=24
                      local.get 5
                      local.get 5
                      i32.store offset=12
                      local.get 5
                      local.get 5
                      i32.store offset=8
                      br 2 (;@7;)
                    end
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
                    local.tee 1
                    i32.add
                    local.tee 7
                    local.get 6
                    local.get 1
                    i32.sub
                    i32.const 56
                    i32.sub
                    local.tee 1
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 4
                    i32.const 52
                    i32.sub
                    i32.const 56
                    i32.store
                    local.get 2
                    local.get 3
                    i32.const 55
                    local.get 3
                    i32.sub
                    i32.const 15
                    i32.and
                    i32.const 0
                    local.get 3
                    i32.const 55
                    i32.sub
                    i32.const 15
                    i32.and
                    select
                    i32.add
                    i32.const 63
                    i32.sub
                    local.tee 4
                    local.get 4
                    local.get 2
                    i32.const 16
                    i32.add
                    i32.lt_u
                    select
                    local.tee 4
                    i32.const 35
                    i32.store offset=4
                    i32.const 1200
                    i32.const 1660
                    i32.load
                    i32.store
                    i32.const 1196
                    local.get 7
                    i32.store
                    i32.const 1184
                    local.get 1
                    i32.store
                    local.get 4
                    i32.const 16
                    i32.add
                    i32.const 1628
                    i64.load align=4
                    i64.store align=4
                    local.get 4
                    i32.const 1620
                    i64.load align=4
                    i64.store offset=8 align=4
                    i32.const 1628
                    local.get 4
                    i32.const 8
                    i32.add
                    i32.store
                    i32.const 1624
                    local.get 6
                    i32.store
                    i32.const 1620
                    local.get 0
                    i32.store
                    i32.const 1632
                    i32.const 0
                    i32.store
                    local.get 4
                    i32.const 36
                    i32.add
                    local.set 1
                    loop  ;; label = @9
                      local.get 1
                      i32.const 7
                      i32.store
                      local.get 3
                      local.get 1
                      i32.const 4
                      i32.add
                      local.tee 1
                      i32.gt_u
                      br_if 0 (;@9;)
                    end
                    local.get 2
                    local.get 4
                    i32.eq
                    br_if 3 (;@5;)
                    local.get 4
                    local.get 4
                    i32.load offset=4
                    i32.const -2
                    i32.and
                    i32.store offset=4
                    local.get 4
                    local.get 4
                    local.get 2
                    i32.sub
                    local.tee 6
                    i32.store
                    local.get 2
                    local.get 6
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 6
                    i32.const 255
                    i32.le_u
                    if  ;; label = @9
                      local.get 6
                      i32.const 3
                      i32.shr_u
                      local.tee 3
                      i32.const 3
                      i32.shl
                      i32.const 1212
                      i32.add
                      local.set 1
                      block (result i32)  ;; label = @10
                        i32.const 1172
                        i32.load
                        local.tee 0
                        i32.const 1
                        local.get 3
                        i32.shl
                        local.tee 3
                        i32.and
                        i32.eqz
                        if  ;; label = @11
                          i32.const 1172
                          local.get 0
                          local.get 3
                          i32.or
                          i32.store
                          local.get 1
                          br 1 (;@10;)
                        end
                        local.get 1
                        i32.load offset=8
                      end
                      local.tee 3
                      local.get 2
                      i32.store offset=12
                      local.get 1
                      local.get 2
                      i32.store offset=8
                      local.get 2
                      local.get 1
                      i32.store offset=12
                      local.get 2
                      local.get 3
                      i32.store offset=8
                      br 4 (;@5;)
                    end
                    i32.const 31
                    local.set 1
                    local.get 2
                    i64.const 0
                    i64.store offset=16 align=4
                    local.get 6
                    i32.const 16777215
                    i32.le_u
                    if  ;; label = @9
                      local.get 6
                      i32.const 8
                      i32.shr_u
                      local.tee 1
                      local.get 1
                      i32.const 1048320
                      i32.add
                      i32.const 16
                      i32.shr_u
                      i32.const 8
                      i32.and
                      local.tee 1
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
                      local.get 1
                      local.get 3
                      i32.or
                      local.get 0
                      i32.or
                      i32.sub
                      local.tee 1
                      i32.const 1
                      i32.shl
                      local.get 6
                      local.get 1
                      i32.const 21
                      i32.add
                      i32.shr_u
                      i32.const 1
                      i32.and
                      i32.or
                      i32.const 28
                      i32.add
                      local.set 1
                    end
                    local.get 2
                    i32.const 28
                    i32.add
                    local.get 1
                    i32.store
                    local.get 1
                    i32.const 2
                    i32.shl
                    i32.const 1476
                    i32.add
                    local.set 3
                    i32.const 1176
                    i32.load
                    local.tee 0
                    i32.const 1
                    local.get 1
                    i32.shl
                    local.tee 4
                    i32.and
                    i32.eqz
                    if  ;; label = @9
                      local.get 3
                      local.get 2
                      i32.store
                      i32.const 1176
                      local.get 0
                      local.get 4
                      i32.or
                      i32.store
                      local.get 2
                      i32.const 24
                      i32.add
                      local.get 3
                      i32.store
                      local.get 2
                      local.get 2
                      i32.store offset=8
                      local.get 2
                      local.get 2
                      i32.store offset=12
                      br 4 (;@5;)
                    end
                    local.get 6
                    i32.const 0
                    i32.const 25
                    local.get 1
                    i32.const 1
                    i32.shr_u
                    i32.sub
                    local.get 1
                    i32.const 31
                    i32.eq
                    select
                    i32.shl
                    local.set 1
                    local.get 3
                    i32.load
                    local.set 0
                    loop  ;; label = @9
                      local.get 0
                      local.tee 3
                      i32.load offset=4
                      i32.const -8
                      i32.and
                      local.get 6
                      i32.eq
                      br_if 3 (;@6;)
                      local.get 1
                      i32.const 29
                      i32.shr_u
                      local.set 0
                      local.get 1
                      i32.const 1
                      i32.shl
                      local.set 1
                      local.get 3
                      local.get 0
                      i32.const 4
                      i32.and
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 4
                      i32.load
                      local.tee 0
                      br_if 0 (;@9;)
                    end
                    local.get 4
                    local.get 2
                    i32.store
                    local.get 2
                    i32.const 24
                    i32.add
                    local.get 3
                    i32.store
                    local.get 2
                    local.get 2
                    i32.store offset=12
                    local.get 2
                    local.get 2
                    i32.store offset=8
                    br 3 (;@5;)
                  end
                  local.get 2
                  i32.load offset=8
                  local.tee 1
                  local.get 5
                  i32.store offset=12
                  local.get 2
                  local.get 5
                  i32.store offset=8
                  local.get 5
                  i32.const 0
                  i32.store offset=24
                  local.get 5
                  local.get 2
                  i32.store offset=12
                  local.get 5
                  local.get 1
                  i32.store offset=8
                end
                local.get 6
                i32.const 8
                i32.add
                local.set 1
                br 5 (;@1;)
              end
              local.get 3
              i32.load offset=8
              local.tee 1
              local.get 2
              i32.store offset=12
              local.get 3
              local.get 2
              i32.store offset=8
              local.get 2
              i32.const 24
              i32.add
              i32.const 0
              i32.store
              local.get 2
              local.get 3
              i32.store offset=12
              local.get 2
              local.get 1
              i32.store offset=8
            end
            i32.const 1184
            i32.load
            local.tee 1
            local.get 5
            i32.le_u
            br_if 0 (;@4;)
            i32.const 1196
            i32.load
            local.tee 2
            local.get 5
            i32.add
            local.tee 3
            local.get 1
            local.get 5
            i32.sub
            local.tee 1
            i32.const 1
            i32.or
            i32.store offset=4
            i32.const 1184
            local.get 1
            i32.store
            i32.const 1196
            local.get 3
            i32.store
            local.get 2
            local.get 5
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 2
            i32.const 8
            i32.add
            local.set 1
            br 3 (;@1;)
          end
          i32.const 0
          local.set 1
          i32.const 1168
          i32.const 48
          i32.store
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 4
            i32.load offset=28
            local.tee 3
            i32.const 2
            i32.shl
            i32.const 1476
            i32.add
            local.tee 1
            i32.load
            local.get 4
            i32.eq
            if  ;; label = @5
              local.get 1
              local.get 0
              i32.store
              local.get 0
              br_if 1 (;@4;)
              i32.const 1176
              local.get 8
              i32.const -2
              local.get 3
              i32.rotl
              i32.and
              local.tee 8
              i32.store
              br 2 (;@3;)
            end
            local.get 7
            i32.const 16
            i32.const 20
            local.get 7
            i32.load offset=16
            local.get 4
            i32.eq
            select
            i32.add
            local.get 0
            i32.store
            local.get 0
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 0
          local.get 7
          i32.store offset=24
          local.get 4
          i32.load offset=16
          local.tee 1
          if  ;; label = @4
            local.get 0
            local.get 1
            i32.store offset=16
            local.get 1
            local.get 0
            i32.store offset=24
          end
          local.get 4
          i32.const 20
          i32.add
          i32.load
          local.tee 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.const 20
          i32.add
          local.get 1
          i32.store
          local.get 1
          local.get 0
          i32.store offset=24
        end
        block  ;; label = @3
          local.get 2
          i32.const 15
          i32.le_u
          if  ;; label = @4
            local.get 4
            local.get 2
            local.get 5
            i32.add
            local.tee 1
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 1
            local.get 4
            i32.add
            i32.const 4
            i32.add
            local.tee 1
            local.get 1
            i32.load
            i32.const 1
            i32.or
            i32.store
            br 1 (;@3;)
          end
          local.get 4
          local.get 5
          i32.add
          local.tee 0
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 4
          local.get 5
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          local.get 2
          i32.add
          local.get 2
          i32.store
          local.get 2
          i32.const 255
          i32.le_u
          if  ;; label = @4
            local.get 2
            i32.const 3
            i32.shr_u
            local.tee 2
            i32.const 3
            i32.shl
            i32.const 1212
            i32.add
            local.set 1
            block (result i32)  ;; label = @5
              i32.const 1172
              i32.load
              local.tee 3
              i32.const 1
              local.get 2
              i32.shl
              local.tee 2
              i32.and
              i32.eqz
              if  ;; label = @6
                i32.const 1172
                local.get 2
                local.get 3
                i32.or
                i32.store
                local.get 1
                br 1 (;@5;)
              end
              local.get 1
              i32.load offset=8
            end
            local.tee 2
            local.get 0
            i32.store offset=12
            local.get 1
            local.get 0
            i32.store offset=8
            local.get 0
            local.get 1
            i32.store offset=12
            local.get 0
            local.get 2
            i32.store offset=8
            br 1 (;@3;)
          end
          i32.const 31
          local.set 1
          local.get 2
          i32.const 16777215
          i32.le_u
          if  ;; label = @4
            local.get 2
            i32.const 8
            i32.shr_u
            local.tee 1
            local.get 1
            i32.const 1048320
            i32.add
            i32.const 16
            i32.shr_u
            i32.const 8
            i32.and
            local.tee 1
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
            local.get 1
            local.get 3
            i32.or
            local.get 5
            i32.or
            i32.sub
            local.tee 1
            i32.const 1
            i32.shl
            local.get 2
            local.get 1
            i32.const 21
            i32.add
            i32.shr_u
            i32.const 1
            i32.and
            i32.or
            i32.const 28
            i32.add
            local.set 1
          end
          local.get 0
          local.get 1
          i32.store offset=28
          local.get 0
          i64.const 0
          i64.store offset=16 align=4
          local.get 1
          i32.const 2
          i32.shl
          i32.const 1476
          i32.add
          local.set 3
          local.get 8
          i32.const 1
          local.get 1
          i32.shl
          local.tee 5
          i32.and
          i32.eqz
          if  ;; label = @4
            local.get 3
            local.get 0
            i32.store
            i32.const 1176
            local.get 5
            local.get 8
            i32.or
            i32.store
            local.get 0
            local.get 3
            i32.store offset=24
            local.get 0
            local.get 0
            i32.store offset=8
            local.get 0
            local.get 0
            i32.store offset=12
            br 1 (;@3;)
          end
          local.get 2
          i32.const 0
          i32.const 25
          local.get 1
          i32.const 1
          i32.shr_u
          i32.sub
          local.get 1
          i32.const 31
          i32.eq
          select
          i32.shl
          local.set 1
          local.get 3
          i32.load
          local.set 5
          block  ;; label = @4
            loop  ;; label = @5
              local.get 5
              local.tee 3
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 2
              i32.eq
              br_if 1 (;@4;)
              local.get 1
              i32.const 29
              i32.shr_u
              local.set 5
              local.get 1
              i32.const 1
              i32.shl
              local.set 1
              local.get 3
              local.get 5
              i32.const 4
              i32.and
              i32.add
              i32.const 16
              i32.add
              local.tee 6
              i32.load
              local.tee 5
              br_if 0 (;@5;)
            end
            local.get 6
            local.get 0
            i32.store
            local.get 0
            local.get 3
            i32.store offset=24
            local.get 0
            local.get 0
            i32.store offset=12
            local.get 0
            local.get 0
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 3
          i32.load offset=8
          local.tee 1
          local.get 0
          i32.store offset=12
          local.get 3
          local.get 0
          i32.store offset=8
          local.get 0
          i32.const 0
          i32.store offset=24
          local.get 0
          local.get 3
          i32.store offset=12
          local.get 0
          local.get 1
          i32.store offset=8
        end
        local.get 4
        i32.const 8
        i32.add
        local.set 1
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 10
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 0
          i32.load offset=28
          local.tee 3
          i32.const 2
          i32.shl
          i32.const 1476
          i32.add
          local.tee 1
          i32.load
          local.get 0
          i32.eq
          if  ;; label = @4
            local.get 1
            local.get 4
            i32.store
            local.get 4
            br_if 1 (;@3;)
            i32.const 1176
            local.get 9
            i32.const -2
            local.get 3
            i32.rotl
            i32.and
            i32.store
            br 2 (;@2;)
          end
          local.get 10
          i32.const 16
          i32.const 20
          local.get 10
          i32.load offset=16
          local.get 0
          i32.eq
          select
          i32.add
          local.get 4
          i32.store
          local.get 4
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 4
        local.get 10
        i32.store offset=24
        local.get 0
        i32.load offset=16
        local.tee 1
        if  ;; label = @3
          local.get 4
          local.get 1
          i32.store offset=16
          local.get 1
          local.get 4
          i32.store offset=24
        end
        local.get 0
        i32.const 20
        i32.add
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.const 20
        i32.add
        local.get 1
        i32.store
        local.get 1
        local.get 4
        i32.store offset=24
      end
      block  ;; label = @2
        local.get 2
        i32.const 15
        i32.le_u
        if  ;; label = @3
          local.get 0
          local.get 2
          local.get 5
          i32.add
          local.tee 1
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          local.get 1
          i32.add
          i32.const 4
          i32.add
          local.tee 1
          local.get 1
          i32.load
          i32.const 1
          i32.or
          i32.store
          br 1 (;@2;)
        end
        local.get 0
        local.get 5
        i32.add
        local.tee 3
        local.get 2
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        local.get 5
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 2
        local.get 3
        i32.add
        local.get 2
        i32.store
        local.get 8
        if  ;; label = @3
          local.get 8
          i32.const 3
          i32.shr_u
          local.tee 4
          i32.const 3
          i32.shl
          i32.const 1212
          i32.add
          local.set 5
          i32.const 1192
          i32.load
          local.set 1
          block (result i32)  ;; label = @4
            i32.const 1
            local.get 4
            i32.shl
            local.tee 4
            local.get 6
            i32.and
            i32.eqz
            if  ;; label = @5
              i32.const 1172
              local.get 4
              local.get 6
              i32.or
              i32.store
              local.get 5
              br 1 (;@4;)
            end
            local.get 5
            i32.load offset=8
          end
          local.tee 4
          local.get 1
          i32.store offset=12
          local.get 5
          local.get 1
          i32.store offset=8
          local.get 1
          local.get 5
          i32.store offset=12
          local.get 1
          local.get 4
          i32.store offset=8
        end
        i32.const 1192
        local.get 3
        i32.store
        i32.const 1180
        local.get 2
        i32.store
      end
      local.get 0
      i32.const 8
      i32.add
      local.set 1
    end
    local.get 11
    i32.const 16
    i32.add
    global.set 0
    local.get 1
  )
  (func (;10;) (type 4) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 0
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const 8
      i32.sub
      local.tee 2
      local.get 3
      i32.const 4
      i32.sub
      i32.load
      local.tee 1
      i32.const -8
      i32.and
      local.tee 3
      i32.add
      local.set 5
      block  ;; label = @2
        local.get 1
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 2
        local.get 2
        i32.load
        local.tee 1
        i32.sub
        local.tee 2
        i32.const 1188
        i32.load
        i32.lt_u
        br_if 1 (;@1;)
        local.get 1
        local.get 3
        i32.add
        local.set 3
        local.get 2
        i32.const 1192
        i32.load
        i32.ne
        if  ;; label = @3
          local.get 1
          i32.const 255
          i32.le_u
          if  ;; label = @4
            local.get 2
            i32.load offset=8
            local.tee 4
            local.get 1
            i32.const 3
            i32.shr_u
            local.tee 7
            i32.const 3
            i32.shl
            i32.const 1212
            i32.add
            i32.eq
            drop
            local.get 4
            local.get 2
            i32.load offset=12
            local.tee 1
            i32.eq
            if  ;; label = @5
              i32.const 1172
              i32.const 1172
              i32.load
              i32.const -2
              local.get 7
              i32.rotl
              i32.and
              i32.store
              br 3 (;@2;)
            end
            local.get 1
            local.get 4
            i32.store offset=8
            local.get 4
            local.get 1
            i32.store offset=12
            br 2 (;@2;)
          end
          local.get 2
          i32.load offset=24
          local.set 6
          block  ;; label = @4
            local.get 2
            local.get 2
            i32.load offset=12
            local.tee 0
            i32.ne
            if  ;; label = @5
              local.get 0
              local.get 2
              i32.load offset=8
              local.tee 1
              i32.store offset=8
              local.get 1
              local.get 0
              i32.store offset=12
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 2
              i32.const 20
              i32.add
              local.tee 1
              i32.load
              local.tee 4
              br_if 0 (;@5;)
              local.get 2
              i32.const 16
              i32.add
              local.tee 1
              i32.load
              local.tee 4
              br_if 0 (;@5;)
              i32.const 0
              local.set 0
              br 1 (;@4;)
            end
            loop  ;; label = @5
              local.get 1
              local.set 7
              local.get 4
              local.tee 0
              i32.const 20
              i32.add
              local.tee 1
              i32.load
              local.tee 4
              br_if 0 (;@5;)
              local.get 0
              i32.const 16
              i32.add
              local.set 1
              local.get 0
              i32.load offset=16
              local.tee 4
              br_if 0 (;@5;)
            end
            local.get 7
            i32.const 0
            i32.store
          end
          local.get 6
          i32.eqz
          br_if 1 (;@2;)
          block  ;; label = @4
            local.get 2
            local.get 2
            i32.load offset=28
            local.tee 4
            i32.const 2
            i32.shl
            i32.const 1476
            i32.add
            local.tee 1
            i32.load
            i32.eq
            if  ;; label = @5
              local.get 1
              local.get 0
              i32.store
              local.get 0
              br_if 1 (;@4;)
              i32.const 1176
              i32.const 1176
              i32.load
              i32.const -2
              local.get 4
              i32.rotl
              i32.and
              i32.store
              br 3 (;@2;)
            end
            local.get 6
            i32.const 16
            i32.const 20
            local.get 6
            i32.load offset=16
            local.get 2
            i32.eq
            select
            i32.add
            local.get 0
            i32.store
            local.get 0
            i32.eqz
            br_if 2 (;@2;)
          end
          local.get 0
          local.get 6
          i32.store offset=24
          local.get 2
          i32.load offset=16
          local.tee 1
          if  ;; label = @4
            local.get 0
            local.get 1
            i32.store offset=16
            local.get 1
            local.get 0
            i32.store offset=24
          end
          local.get 2
          i32.load offset=20
          local.tee 1
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          i32.const 20
          i32.add
          local.get 1
          i32.store
          local.get 1
          local.get 0
          i32.store offset=24
          br 1 (;@2;)
        end
        local.get 5
        i32.load offset=4
        local.tee 1
        i32.const 3
        i32.and
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 5
        local.get 1
        i32.const -2
        i32.and
        i32.store offset=4
        i32.const 1180
        local.get 3
        i32.store
        local.get 2
        local.get 3
        i32.add
        local.get 3
        i32.store
        local.get 2
        local.get 3
        i32.const 1
        i32.or
        i32.store offset=4
        br 1 (;@1;)
      end
      local.get 2
      local.get 5
      i32.ge_u
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=4
      local.tee 1
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 1
        i32.const 2
        i32.and
        i32.eqz
        if  ;; label = @3
          local.get 5
          i32.const 1196
          i32.load
          i32.eq
          if  ;; label = @4
            i32.const 1196
            local.get 2
            i32.store
            i32.const 1184
            i32.const 1184
            i32.load
            local.get 3
            i32.add
            local.tee 3
            i32.store
            local.get 2
            local.get 3
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 2
            i32.const 1192
            i32.load
            i32.ne
            br_if 3 (;@1;)
            i32.const 1180
            i32.const 0
            i32.store
            i32.const 1192
            i32.const 0
            i32.store
            br 3 (;@1;)
          end
          local.get 5
          i32.const 1192
          i32.load
          i32.eq
          if  ;; label = @4
            i32.const 1192
            local.get 2
            i32.store
            i32.const 1180
            i32.const 1180
            i32.load
            local.get 3
            i32.add
            local.tee 3
            i32.store
            local.get 2
            local.get 3
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 2
            local.get 3
            i32.add
            local.get 3
            i32.store
            br 3 (;@1;)
          end
          local.get 1
          i32.const -8
          i32.and
          local.get 3
          i32.add
          local.set 3
          block  ;; label = @4
            local.get 1
            i32.const 255
            i32.le_u
            if  ;; label = @5
              local.get 5
              i32.load offset=8
              local.tee 4
              local.get 1
              i32.const 3
              i32.shr_u
              local.tee 7
              i32.const 3
              i32.shl
              i32.const 1212
              i32.add
              i32.eq
              drop
              local.get 4
              local.get 5
              i32.load offset=12
              local.tee 1
              i32.eq
              if  ;; label = @6
                i32.const 1172
                i32.const 1172
                i32.load
                i32.const -2
                local.get 7
                i32.rotl
                i32.and
                i32.store
                br 2 (;@4;)
              end
              local.get 1
              local.get 4
              i32.store offset=8
              local.get 4
              local.get 1
              i32.store offset=12
              br 1 (;@4;)
            end
            local.get 5
            i32.load offset=24
            local.set 6
            block  ;; label = @5
              local.get 5
              local.get 5
              i32.load offset=12
              local.tee 0
              i32.ne
              if  ;; label = @6
                local.get 5
                i32.load offset=8
                local.tee 1
                i32.const 1188
                i32.load
                i32.lt_u
                drop
                local.get 0
                local.get 1
                i32.store offset=8
                local.get 1
                local.get 0
                i32.store offset=12
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 5
                i32.const 20
                i32.add
                local.tee 1
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                local.get 5
                i32.const 16
                i32.add
                local.tee 1
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                i32.const 0
                local.set 0
                br 1 (;@5;)
              end
              loop  ;; label = @6
                local.get 1
                local.set 7
                local.get 4
                local.tee 0
                i32.const 20
                i32.add
                local.tee 1
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                local.get 0
                i32.const 16
                i32.add
                local.set 1
                local.get 0
                i32.load offset=16
                local.tee 4
                br_if 0 (;@6;)
              end
              local.get 7
              i32.const 0
              i32.store
            end
            local.get 6
            i32.eqz
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 5
              local.get 5
              i32.load offset=28
              local.tee 4
              i32.const 2
              i32.shl
              i32.const 1476
              i32.add
              local.tee 1
              i32.load
              i32.eq
              if  ;; label = @6
                local.get 1
                local.get 0
                i32.store
                local.get 0
                br_if 1 (;@5;)
                i32.const 1176
                i32.const 1176
                i32.load
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store
                br 2 (;@4;)
              end
              local.get 6
              i32.const 16
              i32.const 20
              local.get 6
              i32.load offset=16
              local.get 5
              i32.eq
              select
              i32.add
              local.get 0
              i32.store
              local.get 0
              i32.eqz
              br_if 1 (;@4;)
            end
            local.get 0
            local.get 6
            i32.store offset=24
            local.get 5
            i32.load offset=16
            local.tee 1
            if  ;; label = @5
              local.get 0
              local.get 1
              i32.store offset=16
              local.get 1
              local.get 0
              i32.store offset=24
            end
            local.get 5
            i32.load offset=20
            local.tee 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.const 20
            i32.add
            local.get 1
            i32.store
            local.get 1
            local.get 0
            i32.store offset=24
          end
          local.get 2
          local.get 3
          i32.add
          local.get 3
          i32.store
          local.get 2
          local.get 3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 2
          i32.const 1192
          i32.load
          i32.ne
          br_if 1 (;@2;)
          i32.const 1180
          local.get 3
          i32.store
          br 2 (;@1;)
        end
        local.get 5
        local.get 1
        i32.const -2
        i32.and
        i32.store offset=4
        local.get 2
        local.get 3
        i32.add
        local.get 3
        i32.store
        local.get 2
        local.get 3
        i32.const 1
        i32.or
        i32.store offset=4
      end
      local.get 3
      i32.const 255
      i32.le_u
      if  ;; label = @2
        local.get 3
        i32.const 3
        i32.shr_u
        local.tee 1
        i32.const 3
        i32.shl
        i32.const 1212
        i32.add
        local.set 3
        block (result i32)  ;; label = @3
          i32.const 1172
          i32.load
          local.tee 4
          i32.const 1
          local.get 1
          i32.shl
          local.tee 1
          i32.and
          i32.eqz
          if  ;; label = @4
            i32.const 1172
            local.get 1
            local.get 4
            i32.or
            i32.store
            local.get 3
            br 1 (;@3;)
          end
          local.get 3
          i32.load offset=8
        end
        local.tee 1
        local.get 2
        i32.store offset=12
        local.get 3
        local.get 2
        i32.store offset=8
        local.get 2
        local.get 3
        i32.store offset=12
        local.get 2
        local.get 1
        i32.store offset=8
        br 1 (;@1;)
      end
      i32.const 31
      local.set 1
      local.get 2
      i64.const 0
      i64.store offset=16 align=4
      local.get 3
      i32.const 16777215
      i32.le_u
      if  ;; label = @2
        local.get 3
        i32.const 8
        i32.shr_u
        local.tee 1
        local.get 1
        i32.const 1048320
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 8
        i32.and
        local.tee 1
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
        local.get 1
        local.get 4
        i32.or
        local.get 0
        i32.or
        i32.sub
        local.tee 1
        i32.const 1
        i32.shl
        local.get 3
        local.get 1
        i32.const 21
        i32.add
        i32.shr_u
        i32.const 1
        i32.and
        i32.or
        i32.const 28
        i32.add
        local.set 1
      end
      local.get 2
      i32.const 28
      i32.add
      local.get 1
      i32.store
      local.get 1
      i32.const 2
      i32.shl
      i32.const 1476
      i32.add
      local.set 4
      block  ;; label = @2
        i32.const 1176
        i32.load
        local.tee 0
        i32.const 1
        local.get 1
        i32.shl
        local.tee 5
        i32.and
        i32.eqz
        if  ;; label = @3
          local.get 4
          local.get 2
          i32.store
          i32.const 1176
          local.get 0
          local.get 5
          i32.or
          i32.store
          local.get 2
          i32.const 24
          i32.add
          local.get 4
          i32.store
          local.get 2
          local.get 2
          i32.store offset=8
          local.get 2
          local.get 2
          i32.store offset=12
          br 1 (;@2;)
        end
        local.get 3
        i32.const 0
        i32.const 25
        local.get 1
        i32.const 1
        i32.shr_u
        i32.sub
        local.get 1
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set 1
        local.get 4
        i32.load
        local.set 0
        block  ;; label = @3
          loop  ;; label = @4
            local.get 0
            local.tee 4
            i32.load offset=4
            i32.const -8
            i32.and
            local.get 3
            i32.eq
            br_if 1 (;@3;)
            local.get 1
            i32.const 29
            i32.shr_u
            local.set 0
            local.get 1
            i32.const 1
            i32.shl
            local.set 1
            local.get 4
            local.get 0
            i32.const 4
            i32.and
            i32.add
            i32.const 16
            i32.add
            local.tee 5
            i32.load
            local.tee 0
            br_if 0 (;@4;)
          end
          local.get 5
          local.get 2
          i32.store
          local.get 2
          i32.const 24
          i32.add
          local.get 4
          i32.store
          local.get 2
          local.get 2
          i32.store offset=12
          local.get 2
          local.get 2
          i32.store offset=8
          br 1 (;@2;)
        end
        local.get 4
        i32.load offset=8
        local.tee 3
        local.get 2
        i32.store offset=12
        local.get 4
        local.get 2
        i32.store offset=8
        local.get 2
        i32.const 24
        i32.add
        i32.const 0
        i32.store
        local.get 2
        local.get 4
        i32.store offset=12
        local.get 2
        local.get 3
        i32.store offset=8
      end
      i32.const 1204
      i32.const 1204
      i32.load
      i32.const 1
      i32.sub
      local.tee 2
      i32.const -1
      local.get 2
      select
      i32.store
    end
  )
  (func (;11;) (type 2) (param i32) (result i32)
    (local i32 i32 i32 i64)
    block  ;; label = @1
      block (result i32)  ;; label = @2
        i32.const 0
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        drop
        local.get 0
        i64.extend_i32_u
        i64.const 2
        i64.shl
        local.tee 4
        i32.wrap_i64
        local.tee 2
        local.get 0
        i32.const 4
        i32.or
        i32.const 65536
        i32.lt_u
        br_if 0 (;@2;)
        drop
        i32.const -1
        local.get 2
        local.get 4
        i64.const 32
        i64.shr_u
        i32.wrap_i64
        select
      end
      local.tee 2
      call 9
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 4
      i32.sub
      i32.load8_u
      i32.const 3
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.store8
        local.get 0
        local.get 2
        i32.add
        local.tee 1
        i32.const 1
        i32.sub
        i32.const 0
        i32.store8
        local.get 2
        i32.const 3
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.store8 offset=2
        local.get 0
        i32.const 0
        i32.store8 offset=1
        local.get 1
        i32.const 3
        i32.sub
        i32.const 0
        i32.store8
        local.get 1
        i32.const 2
        i32.sub
        i32.const 0
        i32.store8
        local.get 2
        i32.const 7
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.store8 offset=3
        local.get 1
        i32.const 4
        i32.sub
        i32.const 0
        i32.store8
        local.get 2
        i32.const 9
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        local.get 0
        i32.sub
        i32.const 3
        i32.and
        local.tee 3
        i32.add
        local.tee 1
        i32.const 0
        i32.store
        local.get 1
        local.get 2
        local.get 3
        i32.sub
        i32.const -4
        i32.and
        local.tee 3
        i32.add
        local.tee 2
        i32.const 4
        i32.sub
        i32.const 0
        i32.store
        local.get 3
        i32.const 9
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 0
        i32.store offset=8
        local.get 1
        i32.const 0
        i32.store offset=4
        local.get 2
        i32.const 8
        i32.sub
        i32.const 0
        i32.store
        local.get 2
        i32.const 12
        i32.sub
        i32.const 0
        i32.store
        local.get 3
        i32.const 25
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 0
        i32.store offset=24
        local.get 1
        i32.const 0
        i32.store offset=20
        local.get 1
        i32.const 0
        i32.store offset=16
        local.get 1
        i32.const 0
        i32.store offset=12
        local.get 2
        i32.const 16
        i32.sub
        i32.const 0
        i32.store
        local.get 2
        i32.const 20
        i32.sub
        i32.const 0
        i32.store
        local.get 2
        i32.const 24
        i32.sub
        i32.const 0
        i32.store
        local.get 2
        i32.const 28
        i32.sub
        i32.const 0
        i32.store
        local.get 3
        local.get 1
        i32.const 4
        i32.and
        i32.const 24
        i32.or
        local.tee 3
        i32.sub
        local.tee 2
        i32.const 32
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        local.get 3
        i32.add
        local.set 1
        loop  ;; label = @3
          local.get 1
          i64.const 0
          i64.store
          local.get 1
          i32.const 24
          i32.add
          i64.const 0
          i64.store
          local.get 1
          i32.const 16
          i32.add
          i64.const 0
          i64.store
          local.get 1
          i32.const 8
          i32.add
          i64.const 0
          i64.store
          local.get 1
          i32.const 32
          i32.add
          local.set 1
          local.get 2
          i32.const 32
          i32.sub
          local.tee 2
          i32.const 31
          i32.gt_u
          br_if 0 (;@3;)
        end
      end
    end
    local.get 0
  )
  (func (;12;) (type 4) (param i32)
    local.get 0
    call 8
    unreachable
  )
  (func (;13;) (type 2) (param i32) (result i32)
    local.get 0
    i32.eqz
    if  ;; label = @1
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
      i32.const 0
      i32.lt_s
      br_if 0 (;@1;)
      local.get 0
      i32.const 16
      i32.shr_u
      memory.grow
      local.tee 0
      i32.const -1
      i32.eq
      if  ;; label = @2
        i32.const 1168
        i32.const 48
        i32.store
        i32.const -1
        return
      end
      local.get 0
      i32.const 16
      i32.shl
      return
    end
    unreachable
  )
  (func (;14;) (type 4) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        local.tee 6
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 6
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 6
        i32.const 1
        i32.add
        local.tee 0
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 6
        i32.const 2
        i32.add
        local.tee 0
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 6
        i32.const 3
        i32.add
        local.tee 0
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 6
        i32.const 4
        i32.add
        local.set 0
      end
      local.get 0
      i32.const 4
      i32.sub
      local.set 0
      loop  ;; label = @2
        local.get 0
        i32.const 4
        i32.add
        local.tee 0
        i32.load
        local.tee 3
        i32.const -1
        i32.xor
        local.get 3
        i32.const 16843009
        i32.sub
        i32.and
        i32.const -2139062144
        i32.and
        i32.eqz
        br_if 0 (;@2;)
      end
      local.get 3
      i32.const 255
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 0
        i32.const 1
        i32.add
        local.tee 0
        i32.load8_u
        br_if 0 (;@2;)
      end
    end
    i32.const -1
    i32.const 0
    block (result i32)  ;; label = @1
      local.get 0
      local.get 6
      local.tee 0
      i32.sub
      local.set 6
      block  ;; label = @2
        local.get 6
        i32.const 1056
        i32.load
        local.tee 7
        if (result i32)  ;; label = @3
          local.get 7
        else
          i32.const 0
          local.set 7
          call 21
          br_if 1 (;@2;)
          i32.const 1056
          i32.load
        end
        i32.const 1060
        i32.load
        local.tee 4
        i32.sub
        i32.gt_u
        if  ;; label = @3
          i32.const 1040
          local.get 0
          local.get 6
          i32.const 1072
          i32.load
          call_indirect (type 0)
          local.set 7
          br 1 (;@2;)
        end
        block (result i32)  ;; label = @3
          local.get 6
          i32.const 1104
          i32.load
          i32.const 0
          i32.lt_s
          br_if 0 (;@3;)
          drop
          local.get 0
          local.get 6
          i32.add
          local.set 5
          i32.const 0
          local.set 7
          loop  ;; label = @4
            local.get 6
            local.get 6
            local.get 7
            i32.add
            i32.eqz
            br_if 1 (;@3;)
            drop
            local.get 5
            local.get 7
            i32.add
            local.set 1
            local.get 7
            i32.const 1
            i32.sub
            local.tee 3
            local.set 7
            local.get 1
            i32.const 1
            i32.sub
            i32.load8_u
            i32.const 10
            i32.ne
            br_if 0 (;@4;)
          end
          i32.const 1040
          local.get 0
          local.get 3
          local.get 6
          i32.add
          i32.const 1
          i32.add
          local.tee 10
          i32.const 1072
          i32.load
          call_indirect (type 0)
          local.tee 7
          local.get 10
          i32.lt_u
          br_if 1 (;@2;)
          local.get 3
          local.get 5
          i32.add
          i32.const 1
          i32.add
          local.set 0
          i32.const 1060
          i32.load
          local.set 4
          local.get 3
          i32.const -1
          i32.xor
        end
        local.tee 7
        local.set 5
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.const 3
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            local.get 0
            i32.load8_u
            i32.store8
            local.get 5
            i32.const 1
            i32.sub
            local.set 3
            local.get 4
            i32.const 1
            i32.add
            local.set 1
            local.get 0
            i32.const 1
            i32.add
            local.tee 2
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 3
            i32.eqz
            br_if 1 (;@3;)
            local.get 4
            local.get 0
            i32.load8_u offset=1
            i32.store8 offset=1
            local.get 5
            i32.const 2
            i32.sub
            local.set 3
            local.get 4
            i32.const 2
            i32.add
            local.set 1
            local.get 0
            i32.const 2
            i32.add
            local.tee 2
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 3
            i32.eqz
            br_if 1 (;@3;)
            local.get 4
            local.get 0
            i32.load8_u offset=2
            i32.store8 offset=2
            local.get 5
            i32.const 3
            i32.sub
            local.set 3
            local.get 4
            i32.const 3
            i32.add
            local.set 1
            local.get 0
            i32.const 3
            i32.add
            local.tee 2
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 3
            i32.eqz
            br_if 1 (;@3;)
            local.get 4
            local.get 0
            i32.load8_u offset=3
            i32.store8 offset=3
            local.get 5
            i32.const 4
            i32.sub
            local.set 3
            local.get 4
            i32.const 4
            i32.add
            local.set 1
            local.get 0
            i32.const 4
            i32.add
            local.set 2
            br 1 (;@3;)
          end
          local.get 5
          local.set 3
          local.get 4
          local.set 1
          local.get 0
          local.set 2
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 3
            i32.and
            local.tee 0
            i32.eqz
            if  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.const 16
                i32.ge_u
                if  ;; label = @7
                  local.get 3
                  i32.const 16
                  i32.sub
                  local.tee 0
                  i32.const 16
                  i32.and
                  i32.eqz
                  if  ;; label = @8
                    local.get 1
                    local.get 2
                    i64.load align=4
                    i64.store align=4
                    local.get 1
                    local.get 2
                    i64.load offset=8 align=4
                    i64.store offset=8 align=4
                    local.get 1
                    i32.const 16
                    i32.add
                    local.set 1
                    local.get 2
                    i32.const 16
                    i32.add
                    local.set 2
                    local.get 0
                    local.set 3
                  end
                  local.get 0
                  i32.const 16
                  i32.lt_u
                  br_if 1 (;@6;)
                  loop  ;; label = @8
                    local.get 1
                    local.get 2
                    i64.load align=4
                    i64.store align=4
                    local.get 1
                    i32.const 8
                    i32.add
                    local.get 2
                    i32.const 8
                    i32.add
                    i64.load align=4
                    i64.store align=4
                    local.get 1
                    i32.const 16
                    i32.add
                    local.get 2
                    i32.const 16
                    i32.add
                    i64.load align=4
                    i64.store align=4
                    local.get 1
                    i32.const 24
                    i32.add
                    local.get 2
                    i32.const 24
                    i32.add
                    i64.load align=4
                    i64.store align=4
                    local.get 1
                    i32.const 32
                    i32.add
                    local.set 1
                    local.get 2
                    i32.const 32
                    i32.add
                    local.set 2
                    local.get 3
                    i32.const 32
                    i32.sub
                    local.tee 3
                    i32.const 15
                    i32.gt_u
                    br_if 0 (;@8;)
                  end
                end
                local.get 3
                local.set 0
              end
              local.get 0
              i32.const 8
              i32.and
              if  ;; label = @6
                local.get 1
                local.get 2
                i64.load align=4
                i64.store align=4
                local.get 2
                i32.const 8
                i32.add
                local.set 2
                local.get 1
                i32.const 8
                i32.add
                local.set 1
              end
              local.get 0
              i32.const 4
              i32.and
              if  ;; label = @6
                local.get 1
                local.get 2
                i32.load
                i32.store
                local.get 2
                i32.const 4
                i32.add
                local.set 2
                local.get 1
                i32.const 4
                i32.add
                local.set 1
              end
              local.get 0
              i32.const 2
              i32.and
              if  ;; label = @6
                local.get 1
                local.get 2
                i32.load16_u align=1
                i32.store16 align=1
                local.get 2
                i32.const 2
                i32.add
                local.set 2
                local.get 1
                i32.const 2
                i32.add
                local.set 1
              end
              local.get 0
              i32.const 1
              i32.and
              br_if 1 (;@4;)
              br 2 (;@3;)
            end
            block  ;; label = @5
              local.get 3
              i32.const 32
              i32.lt_u
              br_if 0 (;@5;)
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    i32.const 1
                    i32.sub
                    br_table 0 (;@8;) 1 (;@7;) 2 (;@6;) 3 (;@5;)
                  end
                  local.get 1
                  local.get 2
                  i32.load
                  local.tee 4
                  i32.store8
                  local.get 1
                  local.get 4
                  i32.const 16
                  i32.shr_u
                  i32.store8 offset=2
                  local.get 1
                  local.get 4
                  i32.const 8
                  i32.shr_u
                  i32.store8 offset=1
                  local.get 3
                  i32.const 3
                  i32.sub
                  local.set 3
                  local.get 1
                  i32.const 3
                  i32.add
                  local.set 9
                  i32.const 0
                  local.set 0
                  loop  ;; label = @8
                    local.get 0
                    local.get 9
                    i32.add
                    local.tee 1
                    local.get 0
                    local.get 2
                    i32.add
                    local.tee 5
                    i32.const 4
                    i32.add
                    i32.load
                    local.tee 8
                    i32.const 8
                    i32.shl
                    local.get 4
                    i32.const 24
                    i32.shr_u
                    i32.or
                    i32.store
                    local.get 1
                    i32.const 4
                    i32.add
                    local.get 5
                    i32.const 8
                    i32.add
                    i32.load
                    local.tee 4
                    i32.const 8
                    i32.shl
                    local.get 8
                    i32.const 24
                    i32.shr_u
                    i32.or
                    i32.store
                    local.get 1
                    i32.const 8
                    i32.add
                    local.get 5
                    i32.const 12
                    i32.add
                    i32.load
                    local.tee 8
                    i32.const 8
                    i32.shl
                    local.get 4
                    i32.const 24
                    i32.shr_u
                    i32.or
                    i32.store
                    local.get 1
                    i32.const 12
                    i32.add
                    local.get 5
                    i32.const 16
                    i32.add
                    i32.load
                    local.tee 4
                    i32.const 8
                    i32.shl
                    local.get 8
                    i32.const 24
                    i32.shr_u
                    i32.or
                    i32.store
                    local.get 0
                    i32.const 16
                    i32.add
                    local.set 0
                    local.get 3
                    i32.const 16
                    i32.sub
                    local.tee 3
                    i32.const 16
                    i32.gt_u
                    br_if 0 (;@8;)
                  end
                  local.get 0
                  local.get 9
                  i32.add
                  local.set 1
                  local.get 0
                  local.get 2
                  i32.add
                  i32.const 3
                  i32.add
                  local.set 2
                  br 2 (;@5;)
                end
                local.get 1
                local.get 2
                i32.load
                local.tee 4
                i32.store16 align=1
                local.get 3
                i32.const 2
                i32.sub
                local.set 3
                local.get 1
                i32.const 2
                i32.add
                local.set 9
                i32.const 0
                local.set 0
                loop  ;; label = @7
                  local.get 0
                  local.get 9
                  i32.add
                  local.tee 1
                  local.get 0
                  local.get 2
                  i32.add
                  local.tee 5
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 8
                  i32.const 16
                  i32.shl
                  local.get 4
                  i32.const 16
                  i32.shr_u
                  i32.or
                  i32.store
                  local.get 1
                  i32.const 4
                  i32.add
                  local.get 5
                  i32.const 8
                  i32.add
                  i32.load
                  local.tee 4
                  i32.const 16
                  i32.shl
                  local.get 8
                  i32.const 16
                  i32.shr_u
                  i32.or
                  i32.store
                  local.get 1
                  i32.const 8
                  i32.add
                  local.get 5
                  i32.const 12
                  i32.add
                  i32.load
                  local.tee 8
                  i32.const 16
                  i32.shl
                  local.get 4
                  i32.const 16
                  i32.shr_u
                  i32.or
                  i32.store
                  local.get 1
                  i32.const 12
                  i32.add
                  local.get 5
                  i32.const 16
                  i32.add
                  i32.load
                  local.tee 4
                  i32.const 16
                  i32.shl
                  local.get 8
                  i32.const 16
                  i32.shr_u
                  i32.or
                  i32.store
                  local.get 0
                  i32.const 16
                  i32.add
                  local.set 0
                  local.get 3
                  i32.const 16
                  i32.sub
                  local.tee 3
                  i32.const 17
                  i32.gt_u
                  br_if 0 (;@7;)
                end
                local.get 0
                local.get 9
                i32.add
                local.set 1
                local.get 0
                local.get 2
                i32.add
                i32.const 2
                i32.add
                local.set 2
                br 1 (;@5;)
              end
              local.get 1
              local.get 2
              i32.load
              local.tee 4
              i32.store8
              local.get 3
              i32.const 1
              i32.sub
              local.set 3
              local.get 1
              i32.const 1
              i32.add
              local.set 9
              i32.const 0
              local.set 0
              loop  ;; label = @6
                local.get 0
                local.get 9
                i32.add
                local.tee 1
                local.get 0
                local.get 2
                i32.add
                local.tee 5
                i32.const 4
                i32.add
                i32.load
                local.tee 8
                i32.const 24
                i32.shl
                local.get 4
                i32.const 8
                i32.shr_u
                i32.or
                i32.store
                local.get 1
                i32.const 4
                i32.add
                local.get 5
                i32.const 8
                i32.add
                i32.load
                local.tee 4
                i32.const 24
                i32.shl
                local.get 8
                i32.const 8
                i32.shr_u
                i32.or
                i32.store
                local.get 1
                i32.const 8
                i32.add
                local.get 5
                i32.const 12
                i32.add
                i32.load
                local.tee 8
                i32.const 24
                i32.shl
                local.get 4
                i32.const 8
                i32.shr_u
                i32.or
                i32.store
                local.get 1
                i32.const 12
                i32.add
                local.get 5
                i32.const 16
                i32.add
                i32.load
                local.tee 4
                i32.const 24
                i32.shl
                local.get 8
                i32.const 8
                i32.shr_u
                i32.or
                i32.store
                local.get 0
                i32.const 16
                i32.add
                local.set 0
                local.get 3
                i32.const 16
                i32.sub
                local.tee 3
                i32.const 18
                i32.gt_u
                br_if 0 (;@6;)
              end
              local.get 0
              local.get 9
              i32.add
              local.set 1
              local.get 0
              local.get 2
              i32.add
              i32.const 1
              i32.add
              local.set 2
            end
            local.get 3
            i32.const 16
            i32.and
            if  ;; label = @5
              local.get 1
              local.get 2
              i32.load8_u
              i32.store8
              local.get 1
              local.get 2
              i32.load offset=1 align=1
              i32.store offset=1 align=1
              local.get 1
              local.get 2
              i64.load offset=5 align=1
              i64.store offset=5 align=1
              local.get 1
              local.get 2
              i32.load16_u offset=13 align=1
              i32.store16 offset=13 align=1
              local.get 1
              local.get 2
              i32.load8_u offset=15
              i32.store8 offset=15
              local.get 2
              i32.const 16
              i32.add
              local.set 2
              local.get 1
              i32.const 16
              i32.add
              local.set 1
            end
            local.get 3
            i32.const 8
            i32.and
            if  ;; label = @5
              local.get 1
              local.get 2
              i64.load align=1
              i64.store align=1
              local.get 2
              i32.const 8
              i32.add
              local.set 2
              local.get 1
              i32.const 8
              i32.add
              local.set 1
            end
            local.get 3
            i32.const 4
            i32.and
            if  ;; label = @5
              local.get 1
              local.get 2
              i32.load align=1
              i32.store align=1
              local.get 2
              i32.const 4
              i32.add
              local.set 2
              local.get 1
              i32.const 4
              i32.add
              local.set 1
            end
            local.get 3
            i32.const 2
            i32.and
            if  ;; label = @5
              local.get 1
              local.get 2
              i32.load16_u align=1
              i32.store16 align=1
              local.get 2
              i32.const 2
              i32.add
              local.set 2
              local.get 1
              i32.const 2
              i32.add
              local.set 1
            end
            local.get 3
            i32.const 1
            i32.and
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 1
          local.get 2
          i32.load8_u
          i32.store8
        end
        i32.const 1060
        i32.const 1060
        i32.load
        local.get 7
        i32.add
        i32.store
        local.get 7
        local.get 10
        i32.add
        local.set 7
      end
      local.get 6
      local.get 6
      local.get 7
      i32.eq
      br_if 0 (;@1;)
      drop
      local.get 7
    end
    local.get 6
    i32.ne
    select
    i32.const 0
    i32.lt_s
    if  ;; label = @1
      return
    end
    block  ;; label = @1
      i32.const 1104
      i32.load
      i32.const 10
      i32.eq
      br_if 0 (;@1;)
      i32.const 1060
      i32.load
      local.tee 0
      i32.const 1056
      i32.load
      i32.eq
      br_if 0 (;@1;)
      i32.const 1060
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
        i32.const 1056
        i32.load
        local.tee 3
        if (result i32)  ;; label = @3
          local.get 3
        else
          call 21
          br_if 2 (;@1;)
          i32.const 1056
          i32.load
        end
        i32.const 1060
        i32.load
        local.tee 3
        i32.eq
        br_if 0 (;@2;)
        i32.const 1104
        i32.load
        i32.const 10
        i32.eq
        br_if 0 (;@2;)
        i32.const 1060
        local.get 3
        i32.const 1
        i32.add
        i32.store
        local.get 3
        i32.const 10
        i32.store8
        br 1 (;@1;)
      end
      i32.const 1040
      local.get 0
      i32.const 15
      i32.add
      i32.const 1
      i32.const 1072
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
  (func (;15;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=56
    call 4
    i32.const 65535
    i32.and
    local.tee 0
    if (result i32)  ;; label = @1
      i32.const 1168
      local.get 0
      i32.store
      i32.const -1
    else
      i32.const 0
    end
  )
  (func (;16;) (type 3) (param i32 i64 i32) (result i64)
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
      call 6
      i32.const 65535
      i32.and
      local.tee 0
      if  ;; label = @2
        i32.const 1168
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
  (func (;17;) (type 0) (param i32 i32 i32) (result i32)
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
        i32.const 1168
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
      i32.const 65535
      i32.and
      local.tee 2
      if  ;; label = @2
        i32.const 1168
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
  (func (;18;) (type 0) (param i32 i32 i32) (result i32)
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
      call 17
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
          call 17
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
  (func (;19;) (type 0) (param i32 i32 i32) (result i32)
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
          call 5
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
        i32.const 1168
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
    call 18
  )
  (func (;20;) (type 5)
    (local i32 i32 i32)
    i32.const 2712
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
          call_indirect (type 3)
          drop
        end
        local.get 0
        i32.load offset=52
        local.tee 0
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      i32.const 2716
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
      call_indirect (type 3)
      drop
    end
    block  ;; label = @1
      i32.const 1152
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
      call_indirect (type 3)
      drop
    end
    block  ;; label = @1
      i32.const 2716
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
      call_indirect (type 3)
      drop
    end
  )
  (func (;21;) (type 8) (result i32)
    (local i32)
    i32.const 1100
    i32.const 1100
    i32.load
    local.tee 0
    i32.const 1
    i32.sub
    local.get 0
    i32.or
    i32.store
    i32.const 1040
    i32.load
    local.tee 0
    i32.const 8
    i32.and
    if  ;; label = @1
      i32.const 1040
      local.get 0
      i32.const 32
      i32.or
      i32.store
      i32.const -1
      return
    end
    i32.const 1044
    i64.const 0
    i64.store align=4
    i32.const 1064
    i32.const 1080
    i32.load
    local.tee 0
    i32.store
    i32.const 1060
    local.get 0
    i32.store
    i32.const 1056
    local.get 0
    i32.const 1084
    i32.load
    i32.add
    i32.store
    i32.const 0
  )
  (func (;22;) (type 5)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 12
        i32.add
        local.get 1
        i32.const 8
        i32.add
        call 3
        i32.const 65535
        i32.and
        i32.eqz
        if  ;; label = @3
          local.get 1
          i32.load offset=12
          local.tee 0
          i32.eqz
          if  ;; label = @4
            i32.const 1668
            i32.const 1672
            i32.store
            br 3 (;@1;)
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              local.get 0
              i32.const 1
              i32.add
              local.tee 0
              i32.gt_u
              br_if 0 (;@5;)
              local.get 1
              i32.load offset=8
              call 9
              local.tee 2
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              call 11
              local.tee 0
              br_if 1 (;@4;)
              local.get 2
              call 10
            end
            i32.const 70
            call 12
            unreachable
          end
          local.get 0
          local.get 2
          call 2
          i32.const 65535
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          call 10
          local.get 0
          call 10
        end
        i32.const 71
        call 12
        unreachable
      end
      i32.const 1668
      local.get 0
      i32.store
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    block (result i32)  ;; label = @1
      global.get 0
      i32.const 16
      i32.sub
      local.tee 2
      global.set 0
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.const 8
              i32.add
              local.get 2
              i32.const 12
              i32.add
              call 1
              i32.const 65535
              i32.and
              i32.eqz
              if  ;; label = @6
                local.get 2
                i32.load offset=8
                local.tee 0
                i32.const 1
                i32.add
                local.tee 1
                local.get 0
                i32.lt_u
                br_if 1 (;@5;)
                local.get 2
                i32.load offset=12
                call 9
                local.tee 3
                i32.eqz
                br_if 2 (;@4;)
                local.get 1
                call 11
                local.tee 0
                i32.eqz
                br_if 3 (;@3;)
                local.get 0
                local.get 3
                call 0
                i32.const 65535
                i32.and
                br_if 4 (;@2;)
                local.get 2
                i32.load offset=8
                drop
                i32.const 1029
                call 14
                local.get 0
                i32.load
                local.tee 1
                if  ;; label = @7
                  local.get 0
                  i32.const 4
                  i32.add
                  local.set 0
                  loop  ;; label = @8
                    local.get 1
                    call 14
                    local.get 0
                    i32.load
                    local.set 1
                    local.get 0
                    i32.const 4
                    i32.add
                    local.set 0
                    local.get 1
                    br_if 0 (;@8;)
                  end
                end
                i32.const 1024
                call 14
                i32.const 1668
                i32.load
                local.tee 0
                i32.load
                local.tee 1
                if  ;; label = @7
                  local.get 0
                  i32.const 4
                  i32.add
                  local.set 0
                  loop  ;; label = @8
                    local.get 1
                    call 14
                    local.get 0
                    i32.load
                    local.set 1
                    local.get 0
                    i32.const 4
                    i32.add
                    local.set 0
                    local.get 1
                    br_if 0 (;@8;)
                  end
                end
                local.get 2
                i32.const 16
                i32.add
                global.set 0
                i32.const 0
                br 5 (;@1;)
              end
              i32.const 71
              call 12
              unreachable
            end
            i32.const 70
            call 12
            unreachable
          end
          i32.const 70
          call 12
          unreachable
        end
        local.get 3
        call 10
        i32.const 70
        call 12
        unreachable
      end
      local.get 3
      call 10
      local.get 0
      call 10
      i32.const 71
      call 12
      unreachable
    end
    local.tee 0
    if  ;; label = @1
      call 20
      local.get 0
      call 12
      unreachable
    end
    call 20
  )
  (table (;0;) 5 5 funcref)
  (memory (;0;) 2)
  (global (;0;) (mut i32) i32.const 68256)
  (export "memory" (memory 0))
  (export "_start" (func 22))
  (elem (;0;) (i32.const 1) func 15 19 16 18)
  (data (;0;) (i32.const 1024) "env:\00argv:")
  (data (;1;) (i32.const 1040) "\05")
  (data (;2;) (i32.const 1052) "\01")
  (data (;3;) (i32.const 1072) "\02\00\00\00\03\00\00\00\98\06\00\00\00\04")
  (data (;4;) (i32.const 1096) "\01\00\00\00\00\00\00\00\0a")
  (data (;5;) (i32.const 1152) "\10\04")
)
