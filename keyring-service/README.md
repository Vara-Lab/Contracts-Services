# Keyring service

This service is intended to be used in existing contracts with Sails.

This service manages the signless and walletless feature, storing the keyring locked account in the contract. It gives an struct that handles all parts of this feature, this struct can help you manage both features if you will implement the signless or walletless verification in your services.

## Services

You can find two services in the directory "services", inside "src":

- KeyringService: This service helps to store and bind the "keyring" accounts with the user data (user address or user coded name)
    + **bind_keyring_data_to_user_address**: This method links the given user address with the given "keyring" data, this method needs to be called by the "keyring" account (sub account that will sign the messages - signless feature).
    + **bind_keyring_data_to_user_coded_name**: This method links the given user coded name with the given "keyring" data, this method need to be called by the "keyring" account (sub account that will sign the messages - signless feature).
- Keyring query service: This service helps to give to the external consumers the necessary data about the keyring accounts. It contains three methods:
    + **keyring_address_from_user_address**: This method gives to the external consumers the keyring address from the given user address.
    + **keyring_address_from_user_coded_name**: This method gives to the external consumers the keyring address from the given user coded name.
    + **keyring_account_data**: This method gives to the external consumers the keyring data from the given keyring address.

## Steps to use the service:

### Step 1: Import the crate in your contract project.

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

### Step 2: Use the keyring service

To use the keyring service, you can extend it in one of your services or use it as another service in your contract.

- Use it as a service.

    1. First, you need to import the KeyringService from the crate where you specified your contract's program.

        ```rust
        use keyring_service::services::keyring_service::KeyringService;
        ```

    2. Then, you only need to add the service as a method of the program to expose it to the consumers:
        
        ```rust
        #[route("KeyringService")]
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
        #[route("KeyringService")]
        pub fn keyring_svc(&self) -> KeyringService {
            KeyringService::new()
        }
    }
    ```

- Use it as a extended service:

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


### Step 3: Use the keyring query service

To use the keyring query service, you can extend it in one of your services or use it as another service in your contract.

- Use it as a service.

    1. First, you need to import the KeyringQueryService from the crate where you specified your contract's program.

        ```rust
        use keyring_service::services::keyring_query_service::KeyringQueryService;
        ```

    2. Then, you only need to add the service as a method of the program to expose it to the consumers:
        
        ```rust
        #[route("KeyringQueryService")]
        pub fn keyring_query_svc(&self) -> KeyringQueryService {
            KeyringQueryService::new()
        }
        ```
    
    > A complete example of how to implement it:

    ```rust
    #![no_std]
    // necesary crates
    use sails_rs::prelude::*;

    // imports of more services, etc

    use keyring_service::services::keyring_query_service::KeyringQueryService;

    // Program of your contract
    #[derive(Default)]
    pub struct Program;

    #[program]
    impl Program {
        // services ...

        // Keyring service
        #[route("KeyringQueryService")]
        pub fn keyring_query_svc(&self) -> KeyringQueryService {
            KeyringQueryService::new()
        }
    }
    ```

- Use it as a extended service:

    1. To extend the service, first you need to import the service in the service file:

        ```rust
        use keyring_service::services::keyring_query_service::KeyringQueryService;
        ```

    2. First, you need to add an extra attribute to your service struct, which will store the service keyring.

        ```rust
        pub struct Service {
            keyring_query_service: KeyringQueryService
        }
        ```
    
    3. In the "service" macro you have to specify that you will extend the service, and in the service constructor, you have to assign a new "service" instance to the service attribute

        ```rust
        #[service(extends = KeyringQueryService)]
        impl Service {
            // Service constructor
            pub fn new() -> Self {
                Self {
                    keyring_query_service: KeyringQueryService::new()
                }
            }

            // commands and queries ...
        }
        ```
    
    4. Then, you have to implement the `AsRef` trait on `KeyringQueryService` for your service:

        ```rust
        impl AsRef<KeyringQueryService> for Service {
            fn as_ref(&self) -> &KeyringQueryService {
                // You have to return a reference to the attribute that 
                // you specified to store the keyring service
                &self.keyring_query_service
            }
        }
        ```

    > A complete example of how to implement it:

    ```rust
    use sails_rs::prelude::*;

    // Import the keyring service
    use keyring_service::services::keyring_query_service::KeyringQueryService;

    pub struct Service {
        // Extra attribute to store the keyring query service instance
        keyring_query_service: KeyringQueryService
    }

    #[service(extends = KeyringQueryService)]
    impl Service {
        // Service constructor
        pub fn new() -> Self {
            Self {
                keyring_query_service: KeyringQueryService::new()
            }
        }

        // commands and queries ...
    }

    impl AsRef<KeyringQueryService> for Service {
        fn as_ref(&self) -> &KeyringQueryService {
            // You have to return a reference to the attribute that 
            // you specified to store the keyring service
            &self.keyring_service
        }
    }
    ```


### Step 4: Initialize the state - Important

Once you have implemented both services in your contract (extending them or using them as a service), you have to initialize the state of these services using the related "seed" function of keyring.

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