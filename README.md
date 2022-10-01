# mesh-security

Implementation of Sunny's [Mesh Security](https://youtu.be/Z2ZBKo9-iRs?t=4937) talk from Cosmoverse 20220 (Hackathon / Prototype status)

## Overview

TODO: how the pieces flow together

## Contracts

* `meta-staking` - a bridge between the rest of the contracts and the x/staking module to
                    provide a consistent, friendly interface for our use case
* `ilp` - an "IlLiquidity Pool" contract that locks tokens and issues multiple claims
            to other consumers, who can all slash that stake and eventually release their claim
* `mesh-provider` - IBC-enabled contract that issues claims on an ILP and speaks IBC to a consumer
* `mesh-consumer` - an IBC-enabled contract that receives messages from `ibc-provider` and
                    communicates with `meta-staking` to update the local delegations / validator power
