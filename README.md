# Cloneless Cow
Allows for the storage of a reference or owned instance of a generic type T.

## Features

- Convenient storage of a reference or owned instance, even (and especially) for types that do not implement Clone
- Able to access the instance reference via `.as_ref()`


## Limitations

- Impossible to clone T directly out of the instance or get back the owned T
  - This feature of Cow is also a limitation of Cow which this crate does not suffer from, as Cow requires T to implement the Clone trait when you may only want access to a reference of T