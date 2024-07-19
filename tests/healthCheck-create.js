import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { customTypes } from "./types.js";  // Import custom types
import { getNonce, handleTransactionResponse } from './utils.js';

;(async () => {
  const provider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({
    provider,
    types: customTypes
  });
  await api.isReady;

  // Create a keyring instance
  const keyring = new Keyring({ type: 'sr25519' });
  var tx, result;
  // Define accounts for ALICE and BOB
  const alice = keyring.addFromUri('//Alice');
  const bob = keyring.addFromUri('//Bob');
  const charlie = keyring.addFromUri('//Charlie');

  let alice_nonce = await getNonce(api, alice.address);
  let bob_nonce = await getNonce(api, bob.address);
  let charlie_nonce = await getNonce(api, charlie.address);

  // send a healthCheck from CHARLIE, about ALICE's westend service
  setInterval(async () => {
    // send some funds 100 UNIT from alice to charlie
    tx = api.tx.ibpCheck.registerHealthCheck(
      alice.address,
      'polkadot-rpc',
      'dotters.network',
      'RPC',
      'Error'
    );
    await tx.signAndSend(charlie, { nonce: charlie_nonce++, tip: 1000000000 }, (r) => handleTransactionResponse(api, r))
  }, 5000);

  // process.exit(0);

})().catch(console.error);
