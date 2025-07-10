# Keyring service

This service is intended to be used in existing contracts with Sails.

This service manages the signless and walletless feature, storing the keyring locked account in the contract. It gives an struct that handles all parts of this feature, this struct can help you manage both features if you will implement the signless or walletless verification in your services.

## Service

You can find the service `KeyringService` in the `src/services` directory. This service helps to store and bind the "keyring" accounts with the user data (user address or user coded name), and helps to give to the external consumers the necessary data about the keyring accounts.

It contains two commands and three queries methods:

- Commands:
    + **bind_keyring_data_to_user_address**: This method links the given user address with the given "keyring" data, this method needs to be called by the "keyring" account (sub account that will sign the messages - signless feature).
    + **bind_keyring_data_to_user_coded_name**: This method links the given user coded name with the given "keyring" data, this method need to be called by the "keyring" account (sub account that will sign the messages - signless feature).

- Queries:
    + **keyring_address_from_user_address**: This method gives to the external consumers the keyring address from the given user address.
    + **keyring_address_from_user_coded_name**: This method gives to the external consumers the keyring address from the given user coded name.
    + **keyring_account_data**: This method gives to the external consumers the keyring data from the given keyring address.

## Setting the service:

In your 'Cargo.toml' file, you need to add the keyring-service crate:

```toml
[dependencies]
# crates ...
keyring-service = { git = "https://github.com/Vara-Lab/Contracts-Services"}
# crates ...
```

If you are working in a workspace, **you have to specify in the members the keyring service**:

```toml
[workspace.dependencies]
# crates ...
keyring-service = { git = "https://github.com/Vara-Lab/Contracts-Services"}
# crates ...
```

Cargo.toml example in one member in your workspace:

```toml
[dependencies]
# crates ...
keyring-service.workspace = true
# crates ...
```


## Ways to use the service:

### 1. Using as a normal service

1. First, you need to import the KeyringService from the crate where you specified your contract's program (You have to make sure to specify in your Cargo.toml file the service as in the previous section).

    ```rust
    use keyring_service::services::keyring_service::KeyringService;
    ```

2. Then, you only need to add the service as a method of the program to expose it to the consumers:
    
    ```rust
    #[export(route = "KeyringService")]
    pub fn keyring_svc(&self) -> KeyringService {
        KeyringService::new()
    }
    ```

> A complete example of how to implement it:

```rust
#![no_std]
// necesary crates
use sails_rs::prelude::*;

// imports of more services, etc

use keyring_service::services::keyring_service::KeyringService;

// Program of your contract
#[derive(Default)]
pub struct Program;

#[program]
impl Program {
    // services ...

    // Keyring service
    #[export(route = "KeyringService")]
    pub fn keyring_svc(&self) -> KeyringService {
        KeyringService::new()
    }
}
```

### 2. Using it as an extended service

1. To extend the service, first you need to import the service in the service file:

    ```rust
    use keyring_service::services::keyring_service::KeyringService;
    ```

2. First, you need to add an extra attribute to your service struct, which will store the service keyring.

    ```rust
    pub struct Service {
        keyring_service: KeyringService
    }
    ```

3. In the "service" macro you have to specify that you will extend the service, and in the service constructor, you have to assign a new "service" instance to the service attribute

    ```rust
    #[service(extends = KeyringService)]
    impl Service {
        // Service constructor
        pub fn new() -> Self {
            Self {
                keyring_service: KeyringService::new()
            }
        }

        // commands and queries ...
    }
    ```

4. Then, you have to implement the `AsRef` trait on `KeyringService` for your service:

    ```rust
    impl AsRef<KeyringService> for Service {
        fn as_ref(&self) -> &KeyringService {
            // You have to return a reference to the attribute that 
            // you specified to store the keyring service
            &self.keyring_service
        }
    }
    ```

> A complete example of how to implement it:

```rust
use sails_rs::prelude::*;

// Import the keyring service
use keyring_service::services::keyring_service::KeyringService;

pub struct Service {
    // Set the attribute for the keyring service
    keyring_service: KeyringService
}

#[service(extends = KeyringService)]
impl Service {
    // Service constructor
    pub fn new() -> Self {
        Self {
            // Set a new instance of the keyring service
            keyring_service: KeyringService::new()
        }
    }

    // commands and queries ...
}

impl AsRef<KeyringService> for Service {
    fn as_ref(&self) -> &KeyringService {
        // You have to return a reference to the attribute that 
        // you specified to store the keyring service
        &self.keyring_service
    }
}
```


## Service state initialization.

Once you have implemented the service in your contract (extending it or using it as a service), you have to initialize the state of the service using the related "seed" function of keyring. This step is important to be able to use the service correctly.

You have to put this related function in your program constructor:

```rust
// code ..
use keyring_service::services::keyring_service::KeyringService;

#[derive(Default)]
pub struct Program;

#[program]
impl Program {
    // program constructor
    pub fn new() -> Self {
        // init the keyring service state
        KeyringService::seed();

        Self
    }
}
```

With this steps now you can use the keyring service with signless and walletless feature in your contract!
