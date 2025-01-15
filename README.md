# Craw: A WebSocket-Based Chat Server

**Craw** is a WebSocket-based chat server built with [Actix-Web](https://actix.rs/), designed to be a simple, extensible, learning experiment with WebSockets and the Actor model design pattern. The project will support a Python CLI client [craw-py-client](github.com/brandonblock/craw-py-client) and a Rust WebAssembly client (future).

## Features and Roadmap

### **Planned Features**

- [*] **Broadcast Messaging (MVP)**

  - Support basic broadcast messaging where all connected clients receive every message.
  - No persistence; messages are ephemeral.

- [ ] **User Authentication/Accounts**

  - Introduce usernames and passwords for authentication.
  - Implement account creation workflow
  - Implement secure session management.

- [ ] **Clients**

  - Python CLI client
  - Rust Wasm client

- [ ] **Direct Messaging**

  - Enable users to send private, one-on-one messages.
  - Add commands or a protocol for initiating direct messages.

- [ ] **End-to-End Encryption (E2EE)**

  - Provide encryption for both broadcast and direct messages to ensure privacy.
  - Use public-key cryptography for secure message exchange.

- [ ] **Message Persistence**
  - Allow the server to hold messages until the intended recipient connects.
  - Implement message queues for offline users.

---

## Getting Started

nuthin to start yet
