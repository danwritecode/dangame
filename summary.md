# **Dangame Architecture Summary**

## **Project Structure**

- **common/**  
  Shared types, animation logic, and serialization code used by both client and server.
- **game/**  
  The client/game executable. Handles rendering, input, local simulation, and networking.
- **server/**  
  The authoritative game server. Receives client updates, maintains world state, and broadcasts updates.

---

## **Key Modules**

### **common/**
- `animation.rs` — Animation state, types, and texture management.
- `types.rs` — Core data structures (`ServerClient`, `ClientEventType`, etc.) used for network messages.

### **game/src/**
- `main.rs` — Main game loop. Handles state transitions (menu/game), input, rendering, and networking.
- `characters/` —  
  - `character.rs` — Trait for all characters (player and remote).
  - `character_1.rs`, etc. — Implementations for each character type.
  - `server_character.rs` — Representation of remote (networked) characters for rendering.
- `server.rs` — Handles all client-side networking (connects to server, sends/receives updates).
- `maps/` — Map loading and rendering.
- `ui/` — Main menu and UI logic.
- `constants.rs` — Game constants (physics, window size, etc.).

### **server/src/**
- `main.rs` — Main server loop. Receives client updates, updates authoritative state, and broadcasts to all clients.

---

## **Networking Flow**

- **Client → Server:**  
  - Client sends its current state (`ServerClient`) at a fixed interval (e.g., every 10ms).
- **Server:**  
  - Receives updates from all clients.
  - Maintains a `HashMap<ClientId, ServerClient>` for all connected clients.
  - Broadcasts changed states to all clients at a fixed interval (e.g., every 16ms).
- **Server → Client:**  
  - Client receives updates for all remote players.
  - Updates its local `server_clients` map.

---

## **Remote Player Interpolation**

- Each remote player (`ServerCharacter`) stores the last and current positions received from the server.
- On each render frame, the client **interpolates** between these positions based on the time since the last update, for smooth movement.

---

## **Game Loop (Client)**

1. **Menu or Game State:**  
   - Menu: UI for character/map selection and multiplayer connect.
   - Game: Runs the main loop.
2. **Each Frame (Game):**
   - Handle input and update local player.
   - If multiplayer:
     - Send local player state to server at fixed interval.
     - Receive and process server updates.
     - Interpolate and render remote players.
   - Render map, characters, and UI.

---

## **Key Patterns**

- **Separation of Concerns:**  
  - Network state (`ServerClient`) is separate from render state (`ServerCharacter`).
- **Trait-based Character Logic:**  
  - All characters implement a common trait for polymorphism.
- **Resource Sharing:**  
  - Uses `Rc<RefCell<World>>` for shared mutable access to the physics world.

---

## **Where to Resume**

- **main.rs** in `game/` is the entry point for the client.
- **main.rs** in `server/` is the entry point for the server.
- **Networking logic** is in `game/src/server.rs` and `server/src/main.rs`.
- **Character logic** is in `game/src/characters/`.

---
