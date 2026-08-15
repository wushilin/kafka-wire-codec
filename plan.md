I plan to use published kafka wire protocol json to write protocol parsing and encoding SDK in rust.

Kafka source code is available at:

https://github.com/apache/kafka/

For different versions, we may use the specific tag.

Protocol json is part of the folder. For example: https://github.com/apache/kafka/tree/trunk/clients/src/main/resources/common/message

I want to write a code generator for Rust that encapsulate the messages.

