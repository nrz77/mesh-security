/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.19.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

// eslint-disable-next-line @typescript-eslint/no-empty-interface
export interface InstantiateMsg {}
export type ExecuteMsg =
  | {
      delegate: {
        amount: Uint128;
        validator: string;
      };
    }
  | {
      undelegate: {
        amount: Uint128;
        validator: string;
      };
    }
  | {
      withdraw_delegator_reward: {
        validator: string;
      };
    }
  | {
      withdraw_all_to_costumer: {
        consumer: string;
      };
    }
  | {
      withdraw_to_costumer: {
        consumer: string;
        validator: string;
      };
    }
  | {
      sudo: SudoMsg;
    };
export type Uint128 = string;
export type SudoMsg =
  | {
      add_consumer: {
        consumer_address: string;
        funds_available_for_staking: Coin;
      };
    }
  | {
      remove_consumer: {
        consumer_address: string;
      };
    };
export interface Coin {
  amount: Uint128;
  denom: string;
  [k: string]: unknown;
}
export type QueryMsg =
  | {
      all_delegations: {
        consumer: string;
      };
    }
  | {
      consumer: {
        address: string;
      };
    }
  | {
      consumers: {
        limit?: number | null;
        start?: string | null;
      };
    }
  | {
      delegation: {
        consumer: string;
        validator: string;
      };
    }
  | {
      all_validators: {
        consumer: string;
        limit?: number | null;
        start?: string | null;
      };
    };
export interface AllDelegationsResponse {
  delegations: Delegation[];
}
export interface Delegation {
  delegation: Uint128;
  validator: string;
}
export type Decimal = string;
export interface AllValidatorsResponse {
  validators: Validator[];
  [k: string]: unknown;
}
export interface Validator {
  address: string;
  commission: Decimal;
  max_change_rate: Decimal;
  max_commission: Decimal;
  [k: string]: unknown;
}
export interface ConsumerInfo {
  available_funds: Uint128;
  rewards: Uint128;
  rewards_denom: string;
  total_staked: Uint128;
}
export interface ConsumersResponse {
  consumers: ConsumerInfo[];
}
export type Addr = string;
export interface DelegationResponse {
  delegation?: FullDelegation | null;
  [k: string]: unknown;
}
export interface FullDelegation {
  accumulated_rewards: Coin[];
  amount: Coin;
  can_redelegate: Coin;
  delegator: Addr;
  validator: string;
  [k: string]: unknown;
}
