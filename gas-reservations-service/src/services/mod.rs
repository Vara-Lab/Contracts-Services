use sails_rs::prelude::*;
use crate::state	::{
    GasReservationState,
    GasReservationData
};
use gear_contract_utils::utils;

#[derive(Default, Clone)]
pub struct GasReservationService;

#[service]
impl GasReservationService {
    // Service "Constructor"
    pub fn new() -> Self {
        Self
    }

    // Function to initialize the service state, call only once
    // in the "program" constructor
    pub fn seed() {
        GasReservationState::init_state();
    }

    // Command to reserve gas
    pub fn new_gas_reservation(&mut self, gas_amount: u64, blocks: u32) -> ContractResponse {
        utils::panicking(|| {
            let result = GasReservationState::state_mut()
                .create_reservation(gas_amount, blocks);

            if let Err(e) = result {
                return Err(e);
            }

            Ok(())
        });

        ContractResponse::ReservationCreated
    }

    // Command to remove expired gas reservation
    pub fn remove_expired_gas_reservation(&mut self) -> ContractResponse {
        GasReservationState::state_mut().remove_expired_gas_reservations();
        ContractResponse::ExpiredGasReservationDeleted
    }

    // Get the number of total active gas reservations
    pub fn total_active_gas_reservations(&self) -> ContractResponse {
        let total = GasReservationState::state_ref()
            .active_gas_reservations();

        ContractResponse::ActiveGasReservations(total)
    }

    // Get expired data of gas reservation
    pub fn expired_gas_reservations(&self) -> ContractResponse {
        let reservations = GasReservationState::state_ref()
            .get_expired_gas_reservations();

        ContractResponse::ExpiredGasReservations(reservations)
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ContractResponse {
    ReservationCreated,
    ExpiredGasReservationDeleted,
    ActiveGasReservations(u32),
    ExpiredGasReservations(Vec<GasReservationData>)
}

