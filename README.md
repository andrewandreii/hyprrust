# Hyprrust

A crate that provides a rust interface to communicate with the Hyprland sockets.

While there are other crates that are used to communicate with the Hyprland sockets, this one might stand out because of:
 - Ability to connect to more hyprland sockets/instances
 - Ability to implement your own Hyprland commands
 - Events can be received through an async channel
 - Batching commands
 - Filtering commands (both exclusion and inclusion filters)

### Get started

To start, add this crate to your project:
```sh
cargo add hyprrust
```

Look at the [examples](./examples).

And read the documentation.
