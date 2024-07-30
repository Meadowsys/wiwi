For example, if you wanted to do this:

```rust,ignore
mod module1;
mod module2;
mod module3
// ...

pub use module1::*;
pub use module2::*;
pub use module3::*;
// ...
```

Instead of writing all that out manually, you could just do the following, which will expand to the above:

```rust,ignore
wiwi::export_all_submodules! {
   module1
   module2
   module3
   // ...
}
```
