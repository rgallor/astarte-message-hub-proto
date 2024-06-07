from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class MessageHubError(_message.Message):
    __slots__ = ("code", "description")
    class ErrorCode(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = ()
        UNAUTHORIZED: _ClassVar[MessageHubError.ErrorCode]
        INVALID_DATA: _ClassVar[MessageHubError.ErrorCode]
        ASTARTE_SDK_ERROR: _ClassVar[MessageHubError.ErrorCode]
    UNAUTHORIZED: MessageHubError.ErrorCode
    INVALID_DATA: MessageHubError.ErrorCode
    ASTARTE_SDK_ERROR: MessageHubError.ErrorCode
    CODE_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    code: MessageHubError.ErrorCode
    description: str
    def __init__(self, code: _Optional[_Union[MessageHubError.ErrorCode, str]] = ..., description: _Optional[str] = ...) -> None: ...
