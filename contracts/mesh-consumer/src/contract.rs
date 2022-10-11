use core::time;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Response, StdResult,
    Uint128,
};
use cw2::set_contract_version;

use mesh_ibc::ConsumerMsg;
use meta_staking::msg::MeshConsumerRecieveRewardsMsg;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CHANNEL, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:mesh-consumer";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let meta_staking_contract_address =
        deps.api.addr_validate(&msg.meta_staking_contract_address)?;

    let config = Config {
        meta_staking_contract_address,
        provider: msg.provider,
        remote_to_local_exchange_rate: msg.remote_to_local_exchange_rate,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MeshConsumerRecieveRewardsMsg {
            rewards_by_validator,
        } => execute_receive_rewards(deps, env, info, rewards_by_validator),
    }
}

// We receive the rewards as funds from meta-stacking, and send it over IBC to mesh-provider
pub fn execute_receive_rewards(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    rewards_by_validator: Vec<(String, Uint128)>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let channel_id = CHANNEL.load(deps.storage)?;
    let timeout: IbcTimeout = env.block.time.plus_seconds(300).into();
    // NOTE We try to split the addr from the port_id, maybe better to set the addr in init?
    let provider_addr = config.provider.port_id.split(".").last();

    let provider_addr = match provider_addr {
        Some(addr) => addr,
        None => return Err(ContractError::ProviderAddrParsing {}),
    };

    let mut transfer_msgs = vec![];

    info.funds.iter().for_each(|coin| {
        if coin.amount.u128() > 0_u128 {
            let msg = IbcMsg::Transfer {
                channel_id: channel_id.clone(),
                to_address: provider_addr.to_string(),
                amount: coin.clone(),
                timeout: timeout.clone(),
            };
            transfer_msgs.push(msg)
        }
    });

    transfer_msgs.push(IbcMsg::SendPacket {
        channel_id: channel_id.clone(),
        data: to_binary(&ConsumerMsg::Rewards {
            rewards_by_validator,
            denom: info.funds[0].clone().denom,
        })?,
        timeout: timeout.clone(),
    });

    Ok(Response::default().add_messages(transfer_msgs))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::ProviderInfo;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Decimal};

    fn provider_info() -> ProviderInfo {
        ProviderInfo {
            port_id: "port-1".to_string(),
            connection_id: "conn-2".to_string(),
        }
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            meta_staking_contract_address: "meta_staking".to_string(),
            provider: provider_info(),
            remote_to_local_exchange_rate: Decimal::percent(10),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}
