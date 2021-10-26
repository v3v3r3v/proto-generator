///
//  Generated code. Do not modify.
//  source: protos/example/example.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use exampleMessageDescriptor instead')
const ExampleMessage$json = const {
  '1': 'ExampleMessage',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'id', '3': 2, '4': 1, '5': 5, '10': 'id'},
    const {'1': 'email', '3': 3, '4': 1, '5': 9, '10': 'email'},
    const {'1': 'last_updated', '3': 4, '4': 1, '5': 11, '6': '.google.protobuf.Timestamp', '10': 'lastUpdated'},
    const {'1': 'surname', '3': 5, '4': 1, '5': 9, '10': 'surname'},
    const {'1': 'date_of_birth', '3': 6, '4': 1, '5': 11, '6': '.google.protobuf.Timestamp', '10': 'dateOfBirth'},
    const {'1': 'gender', '3': 7, '4': 1, '5': 14, '6': '.Example.ExampleMessage.Gender', '10': 'gender'},
  ],
  '4': const [ExampleMessage_Gender$json],
};

@$core.Deprecated('Use exampleMessageDescriptor instead')
const ExampleMessage_Gender$json = const {
  '1': 'Gender',
  '2': const [
    const {'1': 'NONE', '2': 0},
    const {'1': 'MALE', '2': 1},
    const {'1': 'FEMALE', '2': 2},
  ],
};

/// Descriptor for `ExampleMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List exampleMessageDescriptor = $convert.base64Decode('Cg5FeGFtcGxlTWVzc2FnZRISCgRuYW1lGAEgASgJUgRuYW1lEg4KAmlkGAIgASgFUgJpZBIUCgVlbWFpbBgDIAEoCVIFZW1haWwSPQoMbGFzdF91cGRhdGVkGAQgASgLMhouZ29vZ2xlLnByb3RvYnVmLlRpbWVzdGFtcFILbGFzdFVwZGF0ZWQSGAoHc3VybmFtZRgFIAEoCVIHc3VybmFtZRI+Cg1kYXRlX29mX2JpcnRoGAYgASgLMhouZ29vZ2xlLnByb3RvYnVmLlRpbWVzdGFtcFILZGF0ZU9mQmlydGgSNgoGZ2VuZGVyGAcgASgOMh4uRXhhbXBsZS5FeGFtcGxlTWVzc2FnZS5HZW5kZXJSBmdlbmRlciIoCgZHZW5kZXISCAoETk9ORRAAEggKBE1BTEUQARIKCgZGRU1BTEUQAg==');
