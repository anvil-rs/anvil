# swivel

A pluggable web framework

Matching CLI called revolve?


```rust
trait Controller {
  fn index() -> String {};
  fn show() -> String {};
  fn store() -> String {};
  fn create() -> String {};
  fn edit() -> String {};
  fn update() -> String {};
  fn delete() -> String {};
};
```

```rust
trait Middleware {
  fn handle() -> String {};
};
```

```rust
struct Router {
  routes: Vec<Route>,
};

impl Router {
  fn add_route() -> String {};
  fn get() -> String {};
  fn post() -> String {};
  fn put() -> String {};
  fn delete() -> String {};
};
```



```rust

trait Request {
  fn authorize() -> bool {};
  fn rules() -> Vec<Rule> {};
};

```

```rust
trait Model {

};
```
