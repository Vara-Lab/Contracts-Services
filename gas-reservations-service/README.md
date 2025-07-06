# Gas Reservations service

This service is intended to be used in existing contracts with Sails.

This service helps you to create gas reservations and store it in the service state, so that you can use them later in delayed messages.

## Calculate gas to reserve

To calculate the total amount of gas to reserve, you must divide the total amount of gas by `1,000,000,000,000`, for example:

> 700_000_000_000 / 1_000_000_000_000 = 0.7 of gas reservation

## Service

You can find the service `GasReservationService` in the `src/services` directory. This service helps to store new gas reservations that you specified in the message that you
send to the smart contract.

It contains two commands and two queries methods:

- Commands:
    + **new_gas_reservation**: This command creates a new gas reservation by giving the amount of gas to reserve and the time in blocks in which the reservation will be valid.
    + **remove_expired_gas_reservation**: This command deletes expired gas reservations (internally, when obtaining a gas reservation for use, it deletes those that have already expired).

- Queries:
    + **total_active_gas_reservations**: Query that returns the number of active gas reservations.
    + **expired_gas_reservations**: Query that returns data of expired gas reservations

## Setting the service:

In your 'Cargo.toml' file, you need to add the keyring-service crate:

```toml
[dependencies]
# crates ...
gas-reservations-service = { git = "https://github.com/Vara-Lab/Contracts-Services" }
# crates ...
```

If you are working in a workspace, **you have to specify in the members the keyring service**:

```toml
[workspace.dependencies]
# crates ...
gas-reservations-service = { git = "https://github.com/Vara-Lab/Contracts-Services" }
# crates ...
```

Cargo.toml example in one member in your workspace:

```toml
[dependencies]
# crates ...
gas-reservations-service.workspace = true
# crates ...
```

## Ways to use the service:

### 1. Using as a normal service

1. First, you need to import the GasReservationService from the crate where you specified your contract's program (You have to make sure to specify in your Cargo.toml file the service as in the previous section).

    ```rust
    use gas_reservations_service::services::GasReservationService;
    ```

2. Then, you only need to add the service as a method of the program to expose it to the consumers:
    
    ```rust
    #[export(route = "GasReservationService")]
    pub fn gas_reservation_svc(&self) -> GasReservationService {
        GasReservationService::new()
    }
    ```

> A complete example of how to implement it:

```rust
#![no_std]
// necesary crates
use sails_rs::prelude::*;

// imports of more services, etc

use gas_reservations_service::services::GasReservationService;

// Program of your contract
#[derive(Default)]
pub struct Program;

#[program]
impl Program {
    // services ...

    // Gas reservation service
    #[export(route = "GasReservationService")]
    pub fn gas_reservation_svc(&self) -> GasReservationService {
        GasReservationService::new()
    }
}
```

### 2. Using it as an extended service

1. To extend the service, first you need to import the service in the service file:

    ```rust
    use gas_reservations_service::services::GasReservationService;
    ```

2. First, you need to add an extra attribute to your service struct, which will store the service keyring.

    ```rust
    pub struct Service {
        gas_reservation_service: GasReservationService
    }
    ```

3. In the "service" macro you have to specify that you will extend the service, and in the service constructor, you have to assign a new "service" instance to the service attribute

    ```rust
    #[service(extends = GasReservationService)]
    impl Service {
        // Service constructor
        pub fn new() -> Self {
            Self {
                gas_reservation_service: GasReservationService::new()
            }
        }

        // commands and queries ...
    }
    ```

4. Then, you have to implement the `AsRef` trait on `GasReservationService` for your service:

    ```rust
    impl AsRef<GasReservationService> for Service {
        fn as_ref(&self) -> &GasReservationService {
            // You have to return a reference to the attribute that 
            // you specified to store the gas reservation service
            &self.gas_reservation_service
        }
    }
    ```

> A complete example of how to implement it:

```rust
use sails_rs::prelude::*;

// Import the gas reservation service
use gas_reservations_service::services::GasReservationService;

pub struct Service {
    // Set the attribute for the gas reservation service
    gas_reservation_service: GasReservationService
}

#[service(extends = GasReservationService)]
impl Service {
    // Service constructor
    pub fn new() -> Self {
        Self {
            // Set a new instance of the gas reservation service
            gas_reservation_service: GasReservationService::new()
        }
    }

    // commands and queries ...
}

impl AsRef<GasReservationService> for Service {
    fn as_ref(&self) -> &GasReservationService {
        // You have to return a reference to the attribute that 
        // you specified to store the gas reservation service
        &self.gas_reservation_service
    }
}
```

## Service state initialization.

Once you have implemented the service in your contract (extending it or using it as a service), you have to initialize the state of the service using the related "seed" function of GasReservationService. This step is important to be able to use the service correctly and to use the gas reservations in the service state.

You have to put this related function in your program constructor:

```rust
// code ..
use gas_reservations_service::services::GasReservationService;

#[derive(Default)]
pub struct Program;

#[program]
impl Program {
    // program constructor
    pub fn new() -> Self {
        // init the gas reservation service state
        GasReservationService::seed();

        Self
    }
}
```

With this steps now you can use the gas reservation service and reserve gas in your contract!

## Notes:

Once you've reserved gas, you can use the reservations using their "ReservationId" and use them for other services. To do this, follow these steps:

1. Import the `GasReservationState` in your service file:

    ```rust
    use gas_reservations_service::state::GasReservationState;    
    ```

2. Then, in your service command, you need to call the related function `get_reservation_id` from `GasReservationState` to get a ReservationId:

    ```rust
    pub fn my_command(&mut self) {
        let reservation_id = GasReservationState::get_reservation_id(); // Returns Option<ReservationId>
    }
    ```

3. You can use it to send messages from reservations like [send_from_reservation](https://docs.rs/gstd/latest/gstd/msg/fn.send_from_reservation.html), [send_delayed_from_reservation](https://docs.rs/gstd/latest/gstd/msg/fn.send_delayed_from_reservation.html), etc. Using sails, you can use some macros that are in [Contract-Utils](https://github.com/Vara-Lab/Gear-Contract-Utils) repository using the `send_delayed_msg!` macro:

    ```rust
    pub fn command(&mut self, address: ActorId) {
        let reservation_id = GasReservationState::get_reservation_id(); // Returns Option<ReservationId>

        send_delayed_msg!(
	    address,    	 // Contract address to send the delayed message
	    "ContractService",   // Contract service name
	    "ServiceMethod",     // Service method name
	    0,			 // Explicit gas for message, is ignored because reservation id, set to 0
	    10,	 		 // amount of blocks to wait
	    (),			 // payload
	    0,			 // Tokens to send
	    Some(reservation_id) // Reservation id
        );
    }
    ```

