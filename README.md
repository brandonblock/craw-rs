# Craw: A WebSocket-Based Chat Server

**Craw** is a WebSocket-based chat server built with [Actix-Web](https://actix.rs/), designed to be a simple, extensible, learning experiment with `actix-web`, the Actor model design pattern, and WASM. The project will support a Python CLI client [craw-py-client](github.com/brandonblock/craw-py-client) and a Rust WebAssembly client (future).

## Features and Roadmap

### **Planned Features**

- [x] **Broadcast Messaging (MVP)**

  - Users given arbitrary sequential ID on connect
  - Support basic broadcast messaging where all connected clients receive every message.
  - No persistence; messages are ephemeral.

- [ ] **User Authentication/Accounts**

  - Implement account creation workflow
  - Implement secure session management/auth middleware.

- [ ] **Clients**

  - Add metadata to message objects and support formatting, etc.
  - Python CLI client
  - Rust Wasm client

- [ ] **Direct Messaging**

  - Enable users to send private messages.
  - Add commands or a protocol for initiating direct messages.

- [ ] **End-to-End Encryption (E2EE)**

  - Provide encryption for both broadcast and direct messages.
  - Use public-key cryptography for secure message exchange.

- [ ] **Message Persistence**
  - Implement message queues for offline users.

---

## Getting Started

nuthin to start yet
