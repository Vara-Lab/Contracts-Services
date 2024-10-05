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

[Todo]: needs to test in other contract ot check it