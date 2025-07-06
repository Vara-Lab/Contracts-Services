use sails_rs::{
    prelude::*,
    collections::VecDeque,
    gstd::exec
};
use gstd::{ReservationId, ReservationIdExt};
pub type Blocks = u32;

static mut GAS_RESERVATION_STATE: Option<GasReservationState> = None;

#[derive(Default)]
pub struct GasReservationState {
    pub gas_reservations_data: VecDeque<GasReservationData>,
    pub current_reservation_id: u64
}

impl GasReservationState {
    pub fn init_state() {
        unsafe {
            GAS_RESERVATION_STATE = Some(GasReservationState::default());
        };
    }

    pub fn state_mut() -> &'static mut Self {
        unsafe { 
            GAS_RESERVATION_STATE
                .as_mut()
                .expect("State is not initialized")
        }
    }

    pub fn state_ref() -> &'static Self {
        unsafe { 
            GAS_RESERVATION_STATE
                .as_ref()
                .expect("State is not initialized")
        }
    }
}

impl GasReservationState {
    pub fn get_reservation_id() -> Option<ReservationId> {
	let state = Self::state_mut();
        let block_height = exec::block_height();

        loop {
            let data = state.gas_reservations_data.pop_front();

            let Some(GasReservationData { reservation_id, created_at, duration, .. }) = data else {
                return None;
            };

            let expiration = created_at + duration;

            if block_height < expiration {
                return Some(reservation_id);
            }
        };
    }

    pub fn create_reservation(&mut self, amount: u64, blocks: Blocks) -> Result<ReservationId, String> {
        if amount == 0 {
            return Err(String::from("Gas to store can not be 0"));
        }

        if blocks == 0 {
            return Err(String::from("Blocks can not be 0"));
        }
            
        let gas_reservation_data_id = self.current_reservation_id
            .checked_add(1)
            .ok_or(String::from("reservation ids overflow"))?;

        let reservation_id: ReservationId = ReservationId::reserve(amount, blocks)
            .map_err(|e| e.to_string())?;

        let block_height = exec::block_height();
        let reservation_data = GasReservationData::new(
            self.current_reservation_id,
            reservation_id, 
            block_height, 
            blocks
        );

        self.gas_reservations_data.push_back(reservation_data);
        self.current_reservation_id = gas_reservation_data_id;

        Ok(reservation_id)
    }

    pub fn remove_expired_gas_reservations(&mut self) {
        let block_height = exec::block_height();
        
        self.gas_reservations_data.retain(|data| (data.created_at + data.duration) < block_height);
    }

    pub fn active_gas_reservations(&self) -> u32 {
        let mut total = 0;
        let block_height = exec::block_height();

        self.gas_reservations_data
            .iter()
            .for_each(|data| {
                if (data.created_at + data.duration) > block_height {
                    total += 1;
                }
            });

        total
    }

    pub fn get_expired_gas_reservations(&self) -> Vec<GasReservationData> {
        let block_height = exec::block_height();
        self.gas_reservations_data
            .iter()
            .filter(|&data| (data.created_at + data.duration) < block_height)
            .map(|data| data.clone())
            .collect()
    }
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct GasReservationData {
    id: u64,
    reservation_id: ReservationId,
    created_at: Blocks,
    duration: Blocks
}

impl GasReservationData { 
    pub fn new(
        id: u64,
        reservation_id: ReservationId,
        created_at: Blocks,
        duration: Blocks
    ) -> Self {
        Self {
            id,
            reservation_id,
            created_at,
            duration
        }
    }
}
