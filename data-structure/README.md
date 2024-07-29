
# 1. Linkedlist
利用到以下知识点 
1. NonNull
2. Box::into_raw 把Box<T>转变为T的指针 *mut T
```rust
pub fn into_raw(b: Box<T, A>) -> *mut T
```
3. Box::from_raw 把指针 *mut T 转变未Box<T>
```rust
pub unsafe fn from_raw(raw: *mut T) -> Box<T>
```
# 2. Vector
利用到以下知识点 
1. NonNull