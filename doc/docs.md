# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [protos/contacts/person.proto](#protos_contacts_person-proto)
    - [Person](#Contacts-Person)
    - [Person.PhoneNumber](#Contacts-Person-PhoneNumber)
  
    - [Person.PhoneType](#Contacts-Person-PhoneType)
  
- [protos/contacts/address_book.proto](#protos_contacts_address_book-proto)
    - [AddressBook](#Contacts-AddressBook)
  
- [protos/example.proto](#protos_example-proto)
    - [Example](#-Example)
  
- [Scalar Value Types](#scalar-value-types)



<a name="protos_contacts_person-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## protos/contacts/person.proto



<a name="Contacts-Person"></a>

### Person



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| id | [int32](#int32) |  | Unique ID number for this person. |
| email | [string](#string) |  |  |
| phones | [Person.PhoneNumber](#Contacts-Person-PhoneNumber) | repeated |  |
| last_updated | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |






<a name="Contacts-Person-PhoneNumber"></a>

### Person.PhoneNumber



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| number | [string](#string) |  |  |
| type | [Person.PhoneType](#Contacts-Person-PhoneType) |  |  |





 


<a name="Contacts-Person-PhoneType"></a>

### Person.PhoneType


| Name | Number | Description |
| ---- | ------ | ----------- |
| MOBILE | 0 |  |
| HOME | 1 |  |
| WORK | 2 |  |


 

 

 



<a name="protos_contacts_address_book-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## protos/contacts/address_book.proto



<a name="Contacts-AddressBook"></a>

### AddressBook



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| people | [Person](#Contacts-Person) | repeated |  |





 

 

 

 



<a name="protos_example-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## protos/example.proto



<a name="-Example"></a>

### Example



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| id | [int32](#int32) |  | Unique ID number for this person. |
| email | [string](#string) |  |  |
| last_updated | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |





 

 

 

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

