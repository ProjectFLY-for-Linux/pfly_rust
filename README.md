# pfly_rust

pfly_rust is a crate to interact with the projectFly X-Plane IPC service.

This was originally made to create a Linux supported alternative to the native X-Plane projectFly plugin, which is from a project to port projectFly over to Linux.

Creating a connection is super easy, calling `init` will give you a socket object that is bonded and connected to projectFly.
You can then use `send_message` to send a message to projectFly with the structure of `PflyIpcData`.