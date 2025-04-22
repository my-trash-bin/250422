# yschgen — YAML Schema Code Generator

> WIP, not work yet

**A minimal Rust tool that transforms a YAML-based schema DSL into:**

- ✅ JSON Schema
- ✅ C/C++ structs, enums and tagged unions
- ✅ C/C++ parser code that converts JSON objects into C/C++ structs

---

## ✨ Features

- **Human-readable DSL** for defining data types
- Supports `struct`s, `enum`s, and `tagged unions`
- Emits:
  - Type-safe `C`/`C++` header files
  - `JSON Schema` for validation or documentation
  - `C`/`C++` parsing functions that validate and convert JSON to typed structs

---

## 📦 Installation

```sh
cargo install --path .
```

---

## 📝 Schema Definition (YAML DSL)

Define types in a YAML file like this:

```yaml
types:

  Shape:
    type: union
    variants:
      - Circle
      - Square

  Circle:
    type: struct
    fields:
      radius: float

  Square:
    type: struct
    fields:
      size: float
```

---

## 🚀 Usage

```sh
# Generate code and schema
cargo run -- schema.yaml out
```

This will generate:
- `out/types_schema.json` — JSON Schema file
- `out/include.h` — Common JSONC parser (fixed)
- `out/types.h` — C declarations for types
- `out/types_parser.c` — JSON parsing logic
- `out/types.hpp` — C++ declarations for types
- `out/types_parser.cpp` — JSON parsing logic

---

## 📤 Output Example

### 🔹 C Header (`types.h`)

```c
struct Circle;
struct Square;
struct Shape;

struct Circle {
    float radius;
};

struct Square {
    float size;
};

enum ShapeType {
    Square,
    Circle,
};

struct Shape {
    enum ShapeType type;
    union {
        struct Circle *circle;
        struct Square *square;
    };
};
```

### 🔸 JSON Schema (`types_schema.json`)

```json
{
  "Shape": {
    "oneOf": [
      { "required": ["type"], "properties": { "type": { "const": "Circle" } } },
      { "required": ["type"], "properties": { "type": { "const": "Square" } } }
    ]
  },
  "Circle": {
    "type": "object",
    "properties": {
      "radius": { "type": "number" }
    },
    "required": ["radius"]
  }
}
```

---

## 🛠 Roadmap

- [ ] Array and optional field support
- [ ] CLI flags for per-target control
- [ ] Built-in JSONC comment support
- [ ] Zig and Rust codegen backend

---

## 📜 License

MIT

---

## ❤️ Contributing

Pull requests and issues are welcome! Schema format is intentionally minimal and stable.
