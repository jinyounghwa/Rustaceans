
# Rust에서 `&mut` 없이 내부 가변성 제공하는 도구들

Rust에서는 `&mut`를 사용하지 않고도 값을 변경할 수 있도록 **내부 가변성 (Interior Mutability)** 개념이 존재합니다. 이를 위한 주요 도구들은 다음과 같습니다.

---

## 🧪 1. `Cell<T>`

- **단일 스레드 전용 내부 가변성**
- `Copy` 가능한 타입에 적합
- `get`, `set`, `replace` 등을 통해 값 읽기/쓰기 가능
- 내부 값을 참조할 수는 없고 복사본만 얻음

```rust
use std::cell::Cell;

let x = Cell::new(5);
x.set(10);
println!("{}", x.get()); // 10
```

---

## 🧪 2. `RefCell<T>`

- **단일 스레드 전용**, 런타임에 가변성 검사
- `borrow()`와 `borrow_mut()`로 참조를 얻음
- `&mut` 없이 가변 참조 가능하지만 **런타임에서 panic 가능**

```rust
use std::cell::RefCell;

let x = RefCell::new(vec![1, 2, 3]);
x.borrow_mut().push(4);
println!("{:?}", x.borrow()); // [1, 2, 3, 4]
```

---

## 🔐 3. `Mutex<T>`

- **멀티스레드 환경에서의 내부 가변성**
- 락을 통해 안전하게 값을 수정 가능
- `lock()`은 `Result<MutexGuard<T>>` 반환 → 에러 처리 필요

```rust
use std::sync::Mutex;

let x = Mutex::new(5);
{
    let mut guard = x.lock().unwrap();
    *guard = 10;
}
println!("{:?}", x.lock().unwrap()); // 10
```

---

## 🕸 4. `RwLock<T>`

- **멀티스레드 환경**에서 **읽기/쓰기 분리 가능**
- 여러 읽기는 가능하지만 쓰기는 하나만 가능
- `read()`, `write()` 사용

```rust
use std::sync::RwLock;

let x = RwLock::new(5);
{
    let mut w = x.write().unwrap();
    *w += 1;
}
println!("{}", *x.read().unwrap()); // 6
```

---

## ☢️ 5. `UnsafeCell<T>`

- Rust에서 **불변 참조를 가변으로 바꿀 수 있는 유일한 원시 타입**
- 다른 내부 가변성 구조물(`Cell`, `RefCell` 등)의 기반
- 일반적으로 직접 사용하지 않고, 상위 추상화를 사용

```rust
use std::cell::UnsafeCell;

let x = UnsafeCell::new(5);
// x.get()은 *mut T 반환, unsafe로 직접 조작 가능
```

---

## 📊 비교 요약

| 타입         | 스레드 안전성 | 런타임 체크 | 참조 가능 | 사용 예시                |
|--------------|----------------|--------------|--------------|---------------------------|
| `Cell<T>`    | ❌ (싱글)        | ❌           | ❌ (복사만) | 간단한 값, `Copy` 타입     |
| `RefCell<T>` | ❌ (싱글)        | ✅ (panic)    | ✅           | 복잡한 구조체 수정        |
| `Mutex<T>`   | ✅ (멀티)        | ✅ (lock)     | ✅           | 스레드 간 공유            |
| `RwLock<T>`  | ✅ (멀티)        | ✅ (lock)     | ✅           | 읽기 많고 쓰기 적은 경우 |
| `UnsafeCell` | ❌ (unsafe 필요) | ❌           | ✅ (unsafe) | 저수준 제어               |

---

## 📝 참고

- [Cell - Rust Docs](https://doc.rust-lang.org/std/cell/struct.Cell.html)
- [RefCell - Rust Docs](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
- [Mutex - Rust Docs](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- [RwLock - Rust Docs](https://doc.rust-lang.org/std/sync/struct.RwLock.html)
- [UnsafeCell - Rust Docs](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html)
