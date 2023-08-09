from ..intrinsics import _decode_utf8, _encode_utf8, _load
from ..types import Err, Ok, Result
import ctypes
from dataclasses import dataclass
from typing import List
import wasmtime

from typing import TYPE_CHECKING
if TYPE_CHECKING:
  from .. import Root

@dataclass
class EventDescription:
    language: str
    latest_description: str

EventDescList = List[EventDescription]
@dataclass
class Tag:
    key: str
    value: str

@dataclass
class Entity:
    entity_value: str
    tags: List[Tag]

EntityList = List[Entity]
@dataclass
class AlarmEvent:
    event_arn: str
    service: str
    event_type_code: str
    event_type_category: str
    start_time: str
    end_time: str
    event_description: EventDescList
    affected_entities: EntityList

class AlarmConnectorDef:
    component: 'Root'
    
    def __init__(self, component: 'Root') -> None:
        self.component = component
    def parse(self, caller: wasmtime.Store, input: str) -> Result[AlarmEvent, str]:
        ptr, len0 = _encode_utf8(input, self.component._realloc0, self.component._core_memory0, caller)
        ret = self.component.lift_callee0(caller, ptr, len0)
        assert(isinstance(ret, int))
        load = _load(ctypes.c_uint8, self.component._core_memory0, caller, ret, 0)
        expected: Result[AlarmEvent, str]
        if load == 0:
            load1 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 4)
            load2 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 8)
            ptr3 = load1
            len4 = load2
            list = _decode_utf8(self.component._core_memory0, caller, ptr3, len4)
            load5 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 12)
            load6 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 16)
            ptr7 = load5
            len8 = load6
            list9 = _decode_utf8(self.component._core_memory0, caller, ptr7, len8)
            load10 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 20)
            load11 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 24)
            ptr12 = load10
            len13 = load11
            list14 = _decode_utf8(self.component._core_memory0, caller, ptr12, len13)
            load15 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 28)
            load16 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 32)
            ptr17 = load15
            len18 = load16
            list19 = _decode_utf8(self.component._core_memory0, caller, ptr17, len18)
            load20 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 36)
            load21 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 40)
            ptr22 = load20
            len23 = load21
            list24 = _decode_utf8(self.component._core_memory0, caller, ptr22, len23)
            load25 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 44)
            load26 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 48)
            ptr27 = load25
            len28 = load26
            list29 = _decode_utf8(self.component._core_memory0, caller, ptr27, len28)
            load30 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 52)
            load31 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 56)
            ptr43 = load30
            len44 = load31
            result: List[EventDescription] = []
            for i45 in range(0, len44):
                base32 = ptr43 + i45 * 16
                load33 = _load(ctypes.c_int32, self.component._core_memory0, caller, base32, 0)
                load34 = _load(ctypes.c_int32, self.component._core_memory0, caller, base32, 4)
                ptr35 = load33
                len36 = load34
                list37 = _decode_utf8(self.component._core_memory0, caller, ptr35, len36)
                load38 = _load(ctypes.c_int32, self.component._core_memory0, caller, base32, 8)
                load39 = _load(ctypes.c_int32, self.component._core_memory0, caller, base32, 12)
                ptr40 = load38
                len41 = load39
                list42 = _decode_utf8(self.component._core_memory0, caller, ptr40, len41)
                result.append(EventDescription(list37, list42))
            load46 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 60)
            load47 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 64)
            ptr71 = load46
            len72 = load47
            result73: List[Entity] = []
            for i74 in range(0, len72):
                base48 = ptr71 + i74 * 16
                load49 = _load(ctypes.c_int32, self.component._core_memory0, caller, base48, 0)
                load50 = _load(ctypes.c_int32, self.component._core_memory0, caller, base48, 4)
                ptr51 = load49
                len52 = load50
                list53 = _decode_utf8(self.component._core_memory0, caller, ptr51, len52)
                load54 = _load(ctypes.c_int32, self.component._core_memory0, caller, base48, 8)
                load55 = _load(ctypes.c_int32, self.component._core_memory0, caller, base48, 12)
                ptr67 = load54
                len68 = load55
                result69: List[Tag] = []
                for i70 in range(0, len68):
                    base56 = ptr67 + i70 * 16
                    load57 = _load(ctypes.c_int32, self.component._core_memory0, caller, base56, 0)
                    load58 = _load(ctypes.c_int32, self.component._core_memory0, caller, base56, 4)
                    ptr59 = load57
                    len60 = load58
                    list61 = _decode_utf8(self.component._core_memory0, caller, ptr59, len60)
                    load62 = _load(ctypes.c_int32, self.component._core_memory0, caller, base56, 8)
                    load63 = _load(ctypes.c_int32, self.component._core_memory0, caller, base56, 12)
                    ptr64 = load62
                    len65 = load63
                    list66 = _decode_utf8(self.component._core_memory0, caller, ptr64, len65)
                    result69.append(Tag(list61, list66))
                result73.append(Entity(list53, result69))
            expected = Ok(AlarmEvent(list, list9, list14, list19, list24, list29, result, result73))
        elif load == 1:
            load75 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 4)
            load76 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 8)
            ptr77 = load75
            len78 = load76
            list79 = _decode_utf8(self.component._core_memory0, caller, ptr77, len78)
            expected = Err(list79)
        else:
            raise TypeError("invalid variant discriminant for expected")
        self.component._post_return0(caller, ret)
        return expected
