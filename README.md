# Storekit: Distro-agnostic package backend

Storekit is like packagekit, but better. It's a library, so not other processes running and no leaking memory in the background and it's easily extensible.

You send a message to the backend and the backend talks to your package manager to do what you want it to do.

Example:
```rs
use storekit;

fn main() {
    set_backend(storekit::backends::arch);

    send_message(storekit::Message::Install("sl"))
}
```

## Adding your own backends
1. Add a rust file in `src/backends/modules/`, set its name to the id of your backend followed by `.rs`.

2. Copy the following code:
```rs
pub struct Handler {}

impl crate::Backend for Handler {
    fn send_message(message: crate::Message) -> Result<(), String> {
        // ...
        // example: Ok(())
    }
    fn get_status() -> crate::BackendStatus {
        // ...
        // example: return crate::BackendStatus::Waiting;
    }
}
```

3. Update the send_message() function to interact with your package manager:
    - If you need to use the library of your package manager, write your own bindings. Avoid adding packages. Your bindings can be in `src/backends/bindings/yourModule.rs`.
        - Do: Create your own binding file that uses the library of your package manager.
        - Don't: Add a seperate package like `libapt` and then use it.
    - Never run commands to interface with the package manager if you don't have to. Use its library instead.
    - Always use official libraries.
        - Do: Use an official library, such as for Arch, use `libalpm`
    - Do not use packagekit!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
```rs
fn send_message(message: crate::Message) -> Result<(), crate::FailureReason> {
    // replace the Ok(())'s with your code
    return match message {
        crate::Message::Install(_) => Ok(()),
        crate::Message::Remove(_) => Ok(()),
        crate::Message::Query(_) => Ok(()),
        crate::Message::Upgrade(_) => Ok(()),
        crate::Message::Sync() => Ok(()),
        crate::Message::Clean() => Ok(()),
    }
}
```

4. Add the get_status function:
```rs
static mut currentStatus: crate::BackendStatus = crate::BackendStatus::Waiting;

impl crate::Backend for Handler {
    fn send_message(message: crate::Message) -> Result<(), crate::FailureReason> {
        // ...
        // this will set status when doing an action
    }
    fn get_status() -> crate::BackendStatus {
        return currentStatus;
    }
}
```

5. Add your module to the `src/backends/modules/mod.rs` file:
```rs
// pub mod arch;
pub mod yourBackend;
```

6. Add your module to the `src/backends/mod.rs` file:
```rs
mod modules;

// pub use modules::arch::Handler as arch;
pub use modules::yourBackend::Handler as yourBackend;
```

7. Done, now your backend is importable. It should be imported automatically by storekit.

## Using Storekit
Never partially update. Therefore, syncing is not recommended - Some backends also have features that prevent partial updates - Such as the Arch Linux backend.