export const OsmosisChainInfo = {
  // Chain-id of the Osmosis chain.
  chainId: "constantine-3",
  chainName: "Constantine",
  rpc: "https://rpc.constantine.archway.io",
  rest: "https://api.constantine.archway.io",
  stakeCurrency: {
    // Coin denomination to be displayed to the user.
    coinDenom: "CONST",
    // Actual denom (i.e. uatom, uscrt) used by the blockchain.
    coinMinimalDenom: "uconst",
    // # of decimal points to convert minimal denomination to user-facing denomination.
    coinDecimals: 18,
    coinGeckoId: "constantine-network",
  },
  bip44: {
    coinType: 118,
  },
  // Bech32 configuration to show the address to user.
  // This field is the interface of
  // {
  //   bech32PrefixAccAddr: string;
  //   bech32PrefixAccPub: string;
  //   bech32PrefixValAddr: string;
  //   bech32PrefixValPub: string;
  //   bech32PrefixConsAddr: string;
  //   bech32PrefixConsPub: string;
  // }
  bech32Config: {
    bech32PrefixAccAddr: "archway",
    bech32PrefixAccPub: "archwaypub",
    bech32PrefixValAddr: "archwayvaloper",
    bech32PrefixValPub: "archwayvaloperpub",
    bech32PrefixConsAddr: "archwayvalcons",
    bech32PrefixConsPub: "archwayvalconspub",
  },
  // List of all coin/tokens used in this chain.
  currencies: [
    {
      // Coin denomination to be displayed to the user.
      coinDenom: "CONST",
      // Actual denom (i.e. uatom, uscrt) used by the blockchain.
      coinMinimalDenom: "uconst",
      // # of decimal points to convert minimal denomination to user-facing denomination.
      coinDecimals: 18,
      coinGeckoId: "constantine-network",
    },
  ],
  // List of coin/tokens used as a fee token in this chain.
  feeCurrencies: [
    {
      // Coin denomination to be displayed to the user.
      coinDenom: "CONST",
      // Actual denom (i.e. uosmo, uscrt) used by the blockchain.
      coinMinimalDenom: "uconst",
      // # of decimal points to convert minimal denomination to user-facing denomination.
      coinDecimals: 18,
      // (Optional) Keplr can show the fiat value of the coin if a coingecko id is provided.
      // You can get id from https://api.coingecko.com/api/v3/coins/list if it is listed.
      // coinGeckoId: ""
      // (Optional) This is used to set the fee of the transaction.
      // If this field is not provided and suggesting chain is not natively integrated, Keplr extension will set the Keplr default gas price (low: 0.01, average: 0.025, high: 0.04).
      // Currently, Keplr doesn't support dynamic calculation of the gas prices based on on-chain data.
      // Make sure that the gas prices are higher than the minimum gas prices accepted by chain validators and RPC/REST endpoint.
      gasPriceStep: {
        low: 0.0025,
        average: 0.025,
        high: 0.04,
      },
    },
  ],
};
