use gclient::GearApi;
use gear_core::ids::{MessageId, ProgramId};
use sails_rs::{ActorId, Encode};

pub const USERS_STR: &[&str] = &["//John", "//Mike", "//Dan"];

pub trait ApiUtils {
    fn get_actor_id(&self) -> ActorId;
    fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId;
}

impl ApiUtils for GearApi {
    fn get_actor_id(&self) -> ActorId {
        ActorId::new(
            self.account_id()
                .encode()
                .try_into()
                .expect("Unexpected invalid account id length."),
        )
    }
    fn get_specific_actor_id(&self, value: impl AsRef<str>) -> ActorId {
        let api_temp = self
            .clone()
            .with(value)
            .expect("Unable to build `GearApi` instance with provided signer.");
        api_temp.get_actor_id()
    }
}

pub async fn get_new_client(api: &GearApi, name: &str) -> GearApi {
    let alice_balance = api
        .total_balance(api.account_id())
        .await
        .expect("Error total balance");
    let amount = alice_balance / 5;
    api.transfer_keep_alive(
        api.get_specific_actor_id(name)
            .encode()
            .as_slice()
            .try_into()
            .expect("Unexpected invalid `ProgramId`."),
        amount,
    )
    .await
    .expect("Error transfer");

    api.clone().with(name).expect("Unable to change signer.")
}

pub async fn init(api: &GearApi) -> (MessageId, ProgramId, Vec<ActorId>) {
    let request = ["New".encode()].concat();

    let path_to_checker = "../target/wasm32-unknown-unknown/release/mandelbrot_checker.opt.wasm";
    let path_to_manager = "../target/wasm32-unknown-unknown/release/manager.opt.wasm";

    // init 100 checkers
    let mut checkers: Vec<ActorId> = Vec::new();
    for i in 0..10 {
        let args: Vec<_> = (0..10)
            .map(|j| {
                let mut salt = gclient::now_micros().to_le_bytes();
                salt[0] ^= i as u8;
                salt[1] ^= j as u8;

                (
                    gclient::code_from_os(path_to_checker).expect("Unable to read code from OS"),
                    salt,
                    request.clone(),
                    5_000_000_000,
                    0,
                )
            })
            .collect();
        let (messages, _hash) = api
            .upload_program_bytes_batch(args)
            .await
            .expect("Unable to upload programs batch");

        let uploaded_checkers: Vec<ActorId> = messages
            .into_iter()
            .filter_map(|res| res.ok().map(|(_, program_id)| program_id))
            .collect();
        checkers.extend(uploaded_checkers);
        println!(
            "Batch {} uploaded, amount of checkers: {}",
            i + 1,
            checkers.len()
        );
    }

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(path_to_manager).unwrap(),
            gclient::now_micros().to_le_bytes(),
            request,
            700_000_000_000,
            0,
        )
        .await
        .expect("Error upload program bytes");

    (message_id, program_id, checkers)
}

#[macro_export]
macro_rules! send_request {
    (api: $api:expr, program_id: $program_id:expr, service_name: $name:literal, action: $action:literal, payload: ($($val:expr),*)) => {
        $crate::send_request!(api: $api, program_id: $program_id, service_name: $name, action: $action, payload: ($($val),*), value: 0)
    };

    (api: $api:expr, program_id: $program_id:expr, service_name: $name:literal, action: $action:literal, payload: ($($val:expr),*), value: $value:expr) => {
        {
            let request = [
                $name.encode(),
                $action.to_string().encode(),
                ($($val),*).encode(),
            ].concat();

            let gas_info = $api
                .calculate_handle_gas(None, $program_id, request.clone(), $value, true)
                .await?;

            let (message_id, _) = $api
                .send_message_bytes($program_id, request.clone(), gas_info.min_limit, $value)
                .await?;

            message_id
        }
    };
}

#[macro_export]
macro_rules! get_state {

    (api: $api:expr, listener: $listener:expr, program_id: $program_id:expr, service_name: $name:literal, action: $action:literal, return_type: $return_type:ty, payload: ($($val:expr),*)) => {
        {
            let request = [
                $name.encode(),
                $action.to_string().encode(),
                ($($val),*).encode(),
            ].concat();

            let gas_info = $api
                .calculate_handle_gas(None, $program_id, request.clone(), 0, true)
                .await
                .expect("Error send message bytes");

            let (message_id, _) = $api
                .send_message_bytes($program_id, request.clone(), gas_info.min_limit, 0)
                .await
                .expect("Error listen reply");

            let (_, raw_reply, _) = $listener
                .reply_bytes_on(message_id)
                .await
                .expect("Error listen reply");

            let decoded_reply = <(String, String, $return_type)>::decode(&mut raw_reply.unwrap().as_slice()).expect("Erroe decode reply");
            decoded_reply.2
        }
    };
}
