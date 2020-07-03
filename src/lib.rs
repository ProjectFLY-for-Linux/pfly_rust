//! pfly_rust is a crate to interact with the projectFly X-Plane IPC service.
//!
//! This was originally made to create a Linux supported alternative to the native X-Plane projectFly plugin, which is from a project to port projectFly over to Linux.
//!
//! Creating a connection is super easy, calling [`init`] will give you a socket object that is bonded and connected to projectFly.
//! You can then use [`send_message`] to send a message to projectFly with the structure of [`PflyIpcData`].
//!
//! [`init`]: fn.init.html
//! [`send_message`]: fn.send_message.html
//! [`PflyIpcData`]: struct.PflyIpcData.html

use serde::Serialize;
use socket2::{Domain, SockAddr, Socket, Type};

/// Connects to the projectFly Unix socket at `/tmp/pf.sock`.
///
/// Returns said socket for future use.
///
/// # Example
///
/// ```
/// let pfly_socket = pfly_rust::init();
/// ```
pub fn init() -> Socket {
    let pfly_socket = Socket::new(Domain::unix(), Type::stream(), None).unwrap();
    let pfly_socket_addr = &SockAddr::unix("/tmp/pf.sock").unwrap();

    if pfly_socket.connect(pfly_socket_addr).is_err() {
        panic!("Could not connect to projectFly socket!")
    }

    return pfly_socket;
}

/// Sends a message to the projectFly socket with a [`PflyIpcData`] payload converted into u8.
///
/// Returns false if any errors ocurred when sending
///
/// # Arguments
/// * `pfly_socket` - The socket object from init()
/// * `data` - Information to be sent in the form of [`PflyIpcData`]
///
/// # Example
///
/// ```
/// let pfly_socket = pfly_rust::init();
///
/// pfly_rust::send_message(pfly_socket, pfly_rust::PflyIpcData{
///     altitude: 569,
///     agl: 0,
///     groundspeed: 0,
///     ias: 0,
///     headingTrue: 0,
///     headingMagnetic: 0,
///     latitude: 43.6772222,
///     longitude: -79.6305556,
///     verticalSpeed: 0,
///     landingVerticalSpeed: 0,
///     gForce: 1000, // Divided by 1000 by projectFly
///     fuel: 20000,
///     transponder: 1425,
///     bridgeType: 3, // From projectFly: bridgeTypes = ['simconnect', 'fsuipc', 'if', 'xplane']
///     isOnGround: 1,
///     isSlew: 0,
///     isPaused: 0,
///     pitch: 0,
///     roll: 0,
///     time: 0, // This is calculated by projectFly
///     fps: 120,
///     aircraftType: "B77W" // Unused by projectFly, still required just in case
/// });
/// ```
///
/// [`PflyIpcData`]: struct.PflyIpcData.html
pub fn send_message(pfly_socket: Socket, data: PflyIpcData) -> bool {
    let payload: Vec<u8> = bincode::serialize(&data).unwrap();

    return pfly_socket.send(payload.as_ref()).is_ok();
}

/// Structure of data that projectFly expects over it's X-Plane IPC connection.
///
/// As found in `/src/app/providers/flightsim.service.ts` of the projectFly source.
#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct PflyIpcData {
    pub altitude: i32,
    pub agl: i32,
    pub groundspeed: i32,
    pub ias: i32,
    pub headingTrue: i32,
    pub headingMagnetic: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub verticalSpeed: i32,
    pub landingVerticalSpeed: i32,
    pub gForce: i32,
    pub fuel: i32,
    pub transponder: i32,
    pub bridgeType: u8,
    pub isOnGround: bool,
    pub isSlew: bool,
    pub isPaused: bool,
    pub pitch: i32,
    pub roll: i32,
    pub time: i32, // This is calculated at projectFly
    pub fps: i32,
    pub aircraftType: &'static str, // This isn't applied on the projectFly side for some reason, still adding it just incase
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
