## Vara Lab Smart Contracts Examples

Smart contracts whose logic you can see and use to implement specific logic in your own smart contracts.

## Examples

- `Demo Sails Staking Broker`: This smart contract will help illustrate how to use the Staking Built-in Actor, it provides
  a basic implementation called `Sails Staking Broker`. This smart contrat acts as simple example to demostrate the concepts
  of bonding, unbonding, and nominating validators on behalf of users.

  > Note: The Sails Staking Broker is intended for demostration purposes only. It accepts user messages to perform actions such as bonding tokens, unbonding funds, and nominating validators. However, it does not handle certain critical aspects, such as managing the unbonding period or other complexities involved in a real-world scenario. A production-level implementation, like a liquid staking contract, would be significantly more intricate, you can refer to the [Staking Service](https://github.com/Vara-Lab/Contracts-Services/tree/main/staking-service) to implement this feature and handle all aspects of staking.