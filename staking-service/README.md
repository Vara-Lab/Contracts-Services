# Staking service


This service can be used with existing contracts that implement [Sails](https://github.com/gear-tech/sails/tree/master/rs).

This service manages all the functionalities of the [staking built-in actor](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024) on Vara Network, managing all actions, keeping a history of each user, managing current eras, collecting rewards, make bonds, etc.

You can even take only the service actions (`StakingActions`) to be able to implement them in your own services and thus modify how each of the processes will be managed, and if you want to have complete control, you can use the state (`StakingData`) to directly modify each value, controlling each time of unbond, bond, etc.

> **IMPORTANT**: As a contract admin, you must [bond 50 tokens](https://wiki.vara.network/docs/staking) or more for the service to work properly, and for the contract to be able to stake correctly.

## Service

You can find the `StakingService` service in the `src/services` directory. This service helps manage and store every action each user performs on the contract, and also provides sufficient information about each user, such as bonds they have made, unbonds, etc.

It contains eight commands and nine queries methods:

- Commands:
    + **bond**: This method will create a new bond for the user, storing the necessary information about the bond and calling the built-in actor to create it. At the end, it throws an event that a bond was created.
    + **unbond**: This method will create a new unbond for the user, it will receive the tokens to unbond (it will check the user total bond) from the user to be able to send the action to the built-in actor, it will store the data such as the era in which it was created, the block, when it can be withdrawn, etc. At the end it will launch an event that an unbond has been made
    + **nominate**: This method will assign the validators that the contract will nominate, this action can only be performed by an admin.
    + **chill**: This method causes you to temporarily stop participating in staking without unbonding the tokens, it can only be called by an admin.
    + **rebond**: This method will rebond to an unbond id, which the contract stores every time an unbond is made, thus putting back into stake the tokens that were in the process of being unbonded.
    + **withdraw_unbonded**: This method will retrieve tokens that have expired and return to the user the amount of tokens specified in the unbond. It only works on unbonds that have passed a total of 8 eras.
    + **set_payee**: This method assigns the address where the rewards are sent, for now, when collecting the rewards, they will be stored in the contract.
        > NOTE: The rewards are stored in the contract, you can create a method that returns the tokens, or create a custom method.
    + **collect_rewards**: This method will collect the validator rewards in each pending era, it can only be called by an admin.

- Queries:
    + **num_of_eras_to_get_rewards**: This query returns the number of eras available to request rewards (if any).
    + **nominations**: This query returns the addresses of the validators that were nominated.
    + **user_history**: This query returns the user's history.
    + **user_total_bond**: Returns the total number of tokens bonded to the specified user.
    + **user_total_unbond**: returns the total number of tokens that are in the process of being unbonded.
    + **user_bonds**: returns the information of each bond that the user has.
    + **user_unbonds**: returns the information of each unbond that the user has.
    + **user_pending_unbonds**: Returns the specified user's active unbonds (which can still be rebonded).
    + **user_unbonds_to_withdraw**: Returns the unbonds that can already be withdrawn from the specified user.

## Setting the service:

In your 'Cargo.toml' file, you need to add the staking-service crate:

```toml
[dependencies]
# crates ...
staking-service = { git = "https://github.com/Vara-Lab/Contracts-Services"}
# crates ...
```

If you are working in a workspace, **you have to specify in the members the staking service**:

```toml
[workspace.dependencies]
# crates ...
staking-service = { git = "https://github.com/Vara-Lab/Contracts-Services"}
# crates ...
```

Cargo.toml example in one member in your workspace:

```toml
[dependencies]
# crates ...
staking-service.workspace = true
# crates ...
```

## Ways to use the service:

### 1. Using as a normal service

1. First, you need to import the StakingService from the crate where you specified your contract's program (You have to make sure to specify in your Cargo.toml file the service as in the previous section).

    ```rust
    use staking_service::services::StakingService;
    ```

2. Then, you only need to add the service as a method of the program to expose it to the consumers:
    
    ```rust
    #[export(route = "StakingService")]
    pub fn staking_svc(&self) -> StakingService {
        StakingService::new()
    }
    ```

> A complete example of how to implement it:

```rust
#![no_std]
// necesary crates
use sails_rs::prelude::*;

// imports of more services, etc

use staking_service::services::StakingService;

// Program of your contract
#[derive(Default)]
pub struct Program;

#[program]
impl Program {
    // services ...

    // staking service
    #[export(route = "StakingService")]
    pub fn staking_svc(&self) -> StakingService {
        StakingService::new()
    }
}
```

### 2. Using it as an extended service

1. To extend the service, first you need to import the service in the service file:

    ```rust
    use staking_service::services::StakingService;
    ```

2. First, you need to add an extra attribute to your service struct, which will store the service staking.

    ```rust
    pub struct Service {
        staking_service: StakingService
    }
    ```

3. In the "service" macro you have to specify that you will extend the service, and in the service constructor, you have to assign a new "service" instance to the service attribute

    ```rust
    #[service(extends = StakingService)]
    impl Service {
        // Service constructor
        pub fn new() -> Self {
            Self {
                staking_service: StakingService::new()
            }
        }

        // commands and queries ...
    }
    ```

4. Then, you have to implement the `AsRef` trait on `StakingService` for your service:

    ```rust
    impl AsRef<StakingService> for Service {
        fn as_ref(&self) -> &StakingService {
            // You have to return a reference to the attribute that 
            // you specified to store the staking service
            &self.staking_service
        }
    }
    ```

> A complete example of how to implement it:

```rust
use sails_rs::prelude::*;

// Import the staking service
use staking_service::services::StakingService;

pub struct Service {
    // Set the attribute for the staking service
    staking_service: StakingService
}

#[service(extends = StakingService)]
impl Service {
    // Service constructor
    pub fn new() -> Self {
        Self {
            // Set a new instance of the staking service
            staking_service: StakingService::new()
        }
    }

    // commands and queries ...
}

impl AsRef<StakingService> for Service {
    fn as_ref(&self) -> &StakingService {
        // You have to return a reference to the attribute that 
        // you specified to store the staking service
        &self.staking_service
    }
}
```


## Service state initialization.

Once you have implemented the service in your contract (extending it or using it as a service), you have to initialize the state of the service using the related "seed" function of staking. This step is important to be able to use the service correctly.

> **IMPORTANT**: A boolean argument needs to be passed to the service's seed function. This bool tells the service whether it is on the testnet or the mainnet, since the current block, current era, etc. are different on the testnet and the mainnet.

You have to put this related function in your program constructor:

```rust
// code ..
use staking_service::services::StakingService;

#[derive(Default)]
pub struct Program;

#[program]
impl Program {
    // program constructor
    pub fn new(on_mainnet: bool) -> Self {
        // init the staking service state
        StakingService::seed(on_mainnet);

        Self
    }
}
```

With this steps now you can use the staking service!
