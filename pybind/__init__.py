from .exports import alarm_connector_def
from .imports import RootImports, http_client
from .intrinsics import _clamp, _decode_utf8, _encode_utf8, _list_canon_lift, _list_canon_lower, _load, _store
from .types import Err, Ok, Result
import ctypes
import os
from typing import List, Tuple, cast
import wasmtime

class Root:
    
    def __init__(self, store: wasmtime.Store, import_object: RootImports) -> None:
        path = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'root.core1.wasm')
        module = wasmtime.Module.from_file(store.engine, path)
        instance0 = wasmtime.Instance(store, module, []).exports(store)
        def lowering0_callee(caller: wasmtime.Caller) -> int:
            ret = import_object.time_client.get_sys_time_unix_millis()
            return _clamp(ret, 0, 18446744073709551615)
        lowering0_ty = wasmtime.FuncType([], [wasmtime.ValType.i64(), ])
        lowering0 = wasmtime.Func(store, lowering0_ty, lowering0_callee, access_caller = True)
        path = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'root.core0.wasm')
        module = wasmtime.Module.from_file(store.engine, path)
        instance1 = wasmtime.Instance(store, module, [
            instance0["0"],
            instance0["1"],
            instance0["2"],
            lowering0,
        ]).exports(store)
        core_memory0 = instance1["memory"]
        assert(isinstance(core_memory0, wasmtime.Memory))
        self._core_memory0 = core_memory0
        realloc0 = instance1["cabi_realloc"]
        assert(isinstance(realloc0, wasmtime.Func))
        self._realloc0 = realloc0
        def lowering1_callee(caller: wasmtime.Caller, arg0: int, arg1: int, arg2: int, arg3: int, arg4: int, arg5: int, arg6: int, arg7: int) -> None:
            ptr = arg1
            len0 = arg2
            list = _decode_utf8(self._core_memory0, caller, ptr, len0)
            ptr11 = arg3
            len12 = arg4
            result: List[Tuple[str, str]] = []
            for i13 in range(0, len12):
                base1 = ptr11 + i13 * 16
                load = _load(ctypes.c_int32, self._core_memory0, caller, base1, 0)
                load2 = _load(ctypes.c_int32, self._core_memory0, caller, base1, 4)
                ptr3 = load
                len4 = load2
                list5 = _decode_utf8(self._core_memory0, caller, ptr3, len4)
                load6 = _load(ctypes.c_int32, self._core_memory0, caller, base1, 8)
                load7 = _load(ctypes.c_int32, self._core_memory0, caller, base1, 12)
                ptr8 = load6
                len9 = load7
                list10 = _decode_utf8(self._core_memory0, caller, ptr8, len9)
                result.append((list5, list10,))
            ptr14 = arg5
            len15 = arg6
            list16 = cast(bytes, _list_canon_lift(ptr14, len15, 1, ctypes.c_uint8, self._core_memory0, caller))
            ret = import_object.http_client.make_http_request(http_client.HttpCallOptions(http_client.Methods(arg0), list, result, list16))
            record = ret
            field = record.status
            field17 = record.body
            _store(ctypes.c_uint16, self._core_memory0, caller, arg7, 0, _clamp(field, 0, 65535))
            ptr18, len19 = _list_canon_lower(field17, ctypes.c_uint8, 1, 1, self._realloc0, self._core_memory0, caller)
            _store(ctypes.c_uint32, self._core_memory0, caller, arg7, 8, len19)
            _store(ctypes.c_uint32, self._core_memory0, caller, arg7, 4, ptr18)
        lowering1_ty = wasmtime.FuncType([wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32(), ], [])
        lowering1 = wasmtime.Func(store, lowering1_ty, lowering1_callee, access_caller = True)
        def lowering2_callee(caller: wasmtime.Caller, arg0: int, arg1: int) -> None:
            ptr = arg0
            len0 = arg1
            list = _decode_utf8(self._core_memory0, caller, ptr, len0)
            import_object.print_client.print_host(list)
        lowering2_ty = wasmtime.FuncType([wasmtime.ValType.i32(), wasmtime.ValType.i32(), ], [])
        lowering2 = wasmtime.Func(store, lowering2_ty, lowering2_callee, access_caller = True)
        def lowering3_callee(caller: wasmtime.Caller, arg0: int) -> None:
            ret = import_object.creds_client.get_creds()
            record = ret
            field = record.access_key_id
            field0 = record.secret_access_key
            field1 = record.session_token
            ptr, len2 = _encode_utf8(field, self._realloc0, self._core_memory0, caller)
            _store(ctypes.c_uint32, self._core_memory0, caller, arg0, 4, len2)
            _store(ctypes.c_uint32, self._core_memory0, caller, arg0, 0, ptr)
            ptr3, len4 = _encode_utf8(field0, self._realloc0, self._core_memory0, caller)
            _store(ctypes.c_uint32, self._core_memory0, caller, arg0, 12, len4)
            _store(ctypes.c_uint32, self._core_memory0, caller, arg0, 8, ptr3)
            ptr5, len6 = _encode_utf8(field1, self._realloc0, self._core_memory0, caller)
            _store(ctypes.c_uint32, self._core_memory0, caller, arg0, 20, len6)
            _store(ctypes.c_uint32, self._core_memory0, caller, arg0, 16, ptr5)
        lowering3_ty = wasmtime.FuncType([wasmtime.ValType.i32(), ], [])
        lowering3 = wasmtime.Func(store, lowering3_ty, lowering3_callee, access_caller = True)
        path = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'root.core2.wasm')
        module = wasmtime.Module.from_file(store.engine, path)
        instance2 = wasmtime.Instance(store, module, [
            lowering1,
            lowering2,
            lowering3,
            instance0["$imports"],
        ]).exports(store)
        post_return0 = instance1["cabi_post_act:utils/alarm-connector-def#parse"]
        assert(isinstance(post_return0, wasmtime.Func))
        self._post_return0 = post_return0
        post_return1 = instance1["cabi_post_list-tables"]
        assert(isinstance(post_return1, wasmtime.Func))
        self._post_return1 = post_return1
        lift_callee0 = instance1["act:utils/alarm-connector-def#parse"]
        assert(isinstance(lift_callee0, wasmtime.Func))
        self.lift_callee0 = lift_callee0
        lift_callee1 = instance1["list-tables"]
        assert(isinstance(lift_callee1, wasmtime.Func))
        self.lift_callee1 = lift_callee1
    def list_tables(self, caller: wasmtime.Store) -> Result[str, str]:
        ret = self.lift_callee1(caller)
        assert(isinstance(ret, int))
        load = _load(ctypes.c_uint8, self._core_memory0, caller, ret, 0)
        expected: Result[str, str]
        if load == 0:
            load0 = _load(ctypes.c_int32, self._core_memory0, caller, ret, 4)
            load1 = _load(ctypes.c_int32, self._core_memory0, caller, ret, 8)
            ptr = load0
            len2 = load1
            list = _decode_utf8(self._core_memory0, caller, ptr, len2)
            expected = Ok(list)
        elif load == 1:
            load3 = _load(ctypes.c_int32, self._core_memory0, caller, ret, 4)
            load4 = _load(ctypes.c_int32, self._core_memory0, caller, ret, 8)
            ptr5 = load3
            len6 = load4
            list7 = _decode_utf8(self._core_memory0, caller, ptr5, len6)
            expected = Err(list7)
        else:
            raise TypeError("invalid variant discriminant for expected")
        self._post_return1(caller, ret)
        return expected
    def alarm_connector_def(self) -> alarm_connector_def.AlarmConnectorDef:
        return alarm_connector_def.AlarmConnectorDef(self)
