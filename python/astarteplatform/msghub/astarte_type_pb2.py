# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: astarteplatform/msghub/astarte_type.proto
# Protobuf Python Version: 5.29.0
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import runtime_version as _runtime_version
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
_runtime_version.ValidateProtobufRuntimeVersion(
    _runtime_version.Domain.PUBLIC,
    5,
    29,
    0,
    '',
    'astarteplatform/msghub/astarte_type.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n)astarteplatform/msghub/astarte_type.proto\x12\x16\x61starteplatform.msghub\x1a\x1fgoogle/protobuf/timestamp.proto\"$\n\x12\x41starteDoubleArray\x12\x0e\n\x06values\x18\x01 \x03(\x01\"%\n\x13\x41starteIntegerArray\x12\x0e\n\x06values\x18\x01 \x03(\x05\"%\n\x13\x41starteBooleanArray\x12\x0e\n\x06values\x18\x01 \x03(\x08\")\n\x17\x41starteLongIntegerArray\x12\x0e\n\x06values\x18\x01 \x03(\x03\"$\n\x12\x41starteStringArray\x12\x0e\n\x06values\x18\x01 \x03(\t\"(\n\x16\x41starteBinaryBlobArray\x12\x0e\n\x06values\x18\x01 \x03(\x0c\"B\n\x14\x41starteDateTimeArray\x12*\n\x06values\x18\x01 \x03(\x0b\x32\x1a.google.protobuf.Timestamp\"\xd1\x01\n\x15\x41starteDataTypeObject\x12R\n\x0bobject_data\x18\x01 \x03(\x0b\x32=.astarteplatform.msghub.AstarteDataTypeObject.ObjectDataEntry\x1a\x64\n\x0fObjectDataEntry\x12\x0b\n\x03key\x18\x01 \x01(\t\x12@\n\x05value\x18\x02 \x01(\x0b\x32\x31.astarteplatform.msghub.AstarteDataTypeIndividual:\x02\x38\x01\"\xc1\x06\n\x19\x41starteDataTypeIndividual\x12\x18\n\x0e\x61starte_double\x18\x01 \x01(\x01H\x00\x12\x19\n\x0f\x61starte_integer\x18\x02 \x01(\x05H\x00\x12\x19\n\x0f\x61starte_boolean\x18\x03 \x01(\x08H\x00\x12\x1e\n\x14\x61starte_long_integer\x18\x04 \x01(\x03H\x00\x12\x18\n\x0e\x61starte_string\x18\x05 \x01(\tH\x00\x12\x1d\n\x13\x61starte_binary_blob\x18\x06 \x01(\x0cH\x00\x12\x37\n\x11\x61starte_date_time\x18\x07 \x01(\x0b\x32\x1a.google.protobuf.TimestampH\x00\x12J\n\x14\x61starte_double_array\x18\x08 \x01(\x0b\x32*.astarteplatform.msghub.AstarteDoubleArrayH\x00\x12L\n\x15\x61starte_integer_array\x18\t \x01(\x0b\x32+.astarteplatform.msghub.AstarteIntegerArrayH\x00\x12L\n\x15\x61starte_boolean_array\x18\n \x01(\x0b\x32+.astarteplatform.msghub.AstarteBooleanArrayH\x00\x12U\n\x1a\x61starte_long_integer_array\x18\x0b \x01(\x0b\x32/.astarteplatform.msghub.AstarteLongIntegerArrayH\x00\x12J\n\x14\x61starte_string_array\x18\x0c \x01(\x0b\x32*.astarteplatform.msghub.AstarteStringArrayH\x00\x12S\n\x19\x61starte_binary_blob_array\x18\r \x01(\x0b\x32..astarteplatform.msghub.AstarteBinaryBlobArrayH\x00\x12O\n\x17\x61starte_date_time_array\x18\x0e \x01(\x0b\x32,.astarteplatform.msghub.AstarteDateTimeArrayH\x00\x42\x11\n\x0findividual_data\"\xb3\x01\n\x0f\x41starteDataType\x12O\n\x12\x61starte_individual\x18\x01 \x01(\x0b\x32\x31.astarteplatform.msghub.AstarteDataTypeIndividualH\x00\x12G\n\x0e\x61starte_object\x18\x02 \x01(\x0b\x32-.astarteplatform.msghub.AstarteDataTypeObjectH\x00\x42\x06\n\x04\x64\x61tab\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'astarteplatform.msghub.astarte_type_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  DESCRIPTOR._loaded_options = None
  _globals['_ASTARTEDATATYPEOBJECT_OBJECTDATAENTRY']._loaded_options = None
  _globals['_ASTARTEDATATYPEOBJECT_OBJECTDATAENTRY']._serialized_options = b'8\001'
  _globals['_ASTARTEDOUBLEARRAY']._serialized_start=102
  _globals['_ASTARTEDOUBLEARRAY']._serialized_end=138
  _globals['_ASTARTEINTEGERARRAY']._serialized_start=140
  _globals['_ASTARTEINTEGERARRAY']._serialized_end=177
  _globals['_ASTARTEBOOLEANARRAY']._serialized_start=179
  _globals['_ASTARTEBOOLEANARRAY']._serialized_end=216
  _globals['_ASTARTELONGINTEGERARRAY']._serialized_start=218
  _globals['_ASTARTELONGINTEGERARRAY']._serialized_end=259
  _globals['_ASTARTESTRINGARRAY']._serialized_start=261
  _globals['_ASTARTESTRINGARRAY']._serialized_end=297
  _globals['_ASTARTEBINARYBLOBARRAY']._serialized_start=299
  _globals['_ASTARTEBINARYBLOBARRAY']._serialized_end=339
  _globals['_ASTARTEDATETIMEARRAY']._serialized_start=341
  _globals['_ASTARTEDATETIMEARRAY']._serialized_end=407
  _globals['_ASTARTEDATATYPEOBJECT']._serialized_start=410
  _globals['_ASTARTEDATATYPEOBJECT']._serialized_end=619
  _globals['_ASTARTEDATATYPEOBJECT_OBJECTDATAENTRY']._serialized_start=519
  _globals['_ASTARTEDATATYPEOBJECT_OBJECTDATAENTRY']._serialized_end=619
  _globals['_ASTARTEDATATYPEINDIVIDUAL']._serialized_start=622
  _globals['_ASTARTEDATATYPEINDIVIDUAL']._serialized_end=1455
  _globals['_ASTARTEDATATYPE']._serialized_start=1458
  _globals['_ASTARTEDATATYPE']._serialized_end=1637
# @@protoc_insertion_point(module_scope)
