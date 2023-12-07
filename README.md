### Todo List

- [ ] TCP Server
  - [x] Thread Per Connection
  - [x] Protocol Specification
  - [ ] Protocol Parser
  - [ ] Error handler
    - [ ] Response Message
- [ ] Message Handler
  - [ ] Single Thread Channel
  - [ ] Storage
    - [x] Lifecycle
      - [x] Page Replacement Algorithm
        - [x] NONE
        - [x] LRU
      - [ ] TTL
    - [x] Double Linked List + HaspMap
- [ ] Settings
  - [ ] settings.toml
  - [ ] Setting parser
- [ ] Client

### Protocol

**Request Message**

| NAME         | SIZE  | -                               |
| ------------ | ----- | ------------------------------- |
| Method       | 1byte | 0x00 GET, 0x01 SET, 0x02 DELETE |
| Key Length   | 1byte |                                 |
| Key          | -     |                                 |
| Value Length | 4byte | SET Only                        |
| Value        | -     | SET Only                        |

**Response Message**

| NAME         | SIZE  | -                       |
| ------------ | ----- | ----------------------- |
| Code         | 1byte | 0x00 Success, 0x01 Fail |
| Value Length | 4byte |                         |
| Value        | -     | GET Only                |
