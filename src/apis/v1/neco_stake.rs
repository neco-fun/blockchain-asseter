use axum::{extract::Path, http::StatusCode, Json};
use ethers::types::U256;
use log::info;

use crate::apis::request::request_model::GetNECOStakedInfoRequest;
use crate::{
    apis::response::response_model::NecoResponse, common::defines::NetworkType,
    models::NECOStakedInfo, services::neco_stake::NecoStakeService,
};

// get neco staked info by public address
#[utoipa::path(
    get,
    path = "/v1/neco-staked-info/{network}/{public_address}",
    tag = "NECO",
    params(
        GetNECOStakedInfoRequest
    ),
    responses(
        (status = 200, description = "Get NECO staked info successfully", body = NECOStakedInfoReponse),
        (status = 400, description = "Bad request", body = NecoResponse<()>),
    )
)]
pub async fn get_neco_staked_info(
    param: Path<GetNECOStakedInfoRequest>,
) -> Json<NecoResponse<NECOStakedInfo>> {
    info!(
        "get_neco_staked_info - public_address: {:?}, network: {:?}",
        param.public_address, param.network
    );
    let network = match param.network {
        0 => NetworkType::BSCMainNetwork,
        1 => NetworkType::BSCTestNetwork,
        _ => return NecoResponse::err(StatusCode::BAD_REQUEST, "network type error"),
    };

    let staked_amount = NecoStakeService::new(network)
        .get_neco_staked_amount(&param.public_address)
        .await
        .unwrap_or_else(|_| U256::from(0));
    let staked_time = NecoStakeService::new(network)
        .get_neco_staked_time(&param.public_address)
        .await
        .unwrap_or_else(|_| U256::from(0));

    NecoResponse::ok(NECOStakedInfo {
        public_address: param.public_address.clone(),
        staked_amount: staked_amount.to_string(),
        staked_time: staked_time.to_string(),
    })
}
