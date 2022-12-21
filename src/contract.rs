use provenance_sdk_proto::provenance::marker::v1::AccessGrant;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // test just use POC
    AccessGrant {
        address: "tp1dlszg2sst9r69my4f84l3mj66zxcf3umcgujys30t84srg95dgvst74vwc".to_string(),
        permissions: vec![1],
    };

    Ok(Response::default())
}
