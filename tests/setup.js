import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { customTypes } from "./types.js";  // Import custom types
import { getNonce, handleTransactionResponse } from './utils.js';

;(async () => {
  const provider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({
    provider,
    types: customTypes
   });
  console.log(api.genesisHash.toHex());

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

  // send some funds 100 UNIT from alice to charlie
  tx = api.tx.balances.transferKeepAlive(charlie.address, 100_000_000_000_000);
  await tx.signAndSend(alice, { nonce: alice_nonce++, tip: 1000000000 }, (r) => handleTransactionResponse(api, r))

  // sleep 6 seconds to allow the funds to be transferred
  await new Promise(resolve => setTimeout(resolve, 6000));

  // Define custom type instance
  const level = api.createType('MembershipLevel', 'One');

  // Register Members, ALICE and CHARLIE
  tx = api.tx.ibpMember.registerMember();
  await tx.signAndSend(alice, { nonce: alice_nonce++, tip: 1000000000 }, (r) => handleTransactionResponse(api, r))
  await tx.signAndSend(charlie, { nonce: charlie_nonce++, tip: 1000000000 }, (r) => handleTransactionResponse(api, r))
  
  // Make BOB a Curator - TODO: This should be done by the council
  tx = api.tx.ibpMember.assignCurator(bob.address);
  await tx.signAndSend(alice, { nonce: alice_nonce++, tip: 1000000000 }, (r) => handleTransactionResponse(api, r))

  // register CHARLIE as a monitor - must be done by a curator
  tx = api.tx.ibpMember.assignMonitor(charlie.address);
  await tx.signAndSend(bob, { nonce: bob_nonce++ }, (r) => handleTransactionResponse(api, r))

  // unlock ALICE and CHARLIE - must be done by a curator
  tx = api.tx.ibpMember.unlockMember(alice.address);
  await tx.signAndSend(bob, { nonce: bob_nonce++ }, (r) => handleTransactionResponse(api, r))
  tx = api.tx.ibpMember.unlockMember(charlie.address);
  await tx.signAndSend(bob, { nonce: bob_nonce++ }, (r) => handleTransactionResponse(api, r))

  // set ALICE and CHARLIE as level - must be done by a monitor
  tx = api.tx.ibpMember.updateMemberLevel(alice.address, 'Five');
  await tx.signAndSend(bob, { nonce: bob_nonce++ }, (r) => handleTransactionResponse(api, r))
  tx = api.tx.ibpMember.updateMemberLevel(charlie.address, 'Three');
  await tx.signAndSend(bob, { nonce: bob_nonce++ }, (r) => handleTransactionResponse(api, r))

  // Register all services as active
  let services = [
    { id: 'westend-rpc', chainId: 'westend', serviceType: 'RPC', level: 'Three' },
    { id: 'kusama-rpc', chainId: 'kusama', serviceType: 'RPC', level: 'Three' },
    { id: 'polkadot-rpc', chainId: 'polkadot', serviceType: 'RPC', level: 'Three' },
    { id: 'asset-hub-westend-rpc', chainId: 'asset-hub-westend', serviceType: 'RPC', level: 'Five' },
    { id: 'asset-hub-kusama-rpc', chainId: 'asset-hub-kusama', serviceType: 'RPC', level: 'Five' },
    { id: 'asset-hub-polkadot-rpc', chainId: 'asset-hub-polkadot', serviceType: 'RPC', level: 'Five' },
  ]
  for(let i = 0; i < services.length; i++) {
    const service = services[i];
    tx = api.tx.ibpService.registerService(service.id, service.chainId, service.serviceType, service.level, 'Active');
    await tx.signAndSend(bob, { nonce: bob_nonce++, tip: 1000000000 }, (r) => handleTransactionResponse(api, r));
    // console.log(result.toJSON());
  }

  // set a service override for alice asset-hub-westend-rpc
  tx = api.tx.ibpService.setServiceOverride('asset-hub-westend-rpc');
  await tx.signAndSend(alice, { nonce: alice_nonce++, tip: 1000000000 }, (r) => handleTransactionResponse(api, r));

  // sleep 6 seconds to allow charlie to monitor the services
  await new Promise(resolve => setTimeout(resolve, 6000));

  // create some alerts
  // Alice is the member
  // Charlie is the monitor
  let alerts = [
    { is: 1, member_id: alice.address, domain_id: 'testnet.ibp.network', service_id: 'westend-rpc', alert_type: 'HTTPTest' },
    { id: 2, member_id: alice.address, domain_id: 'testnet.ibp.network', service_id: 'kusama-rpc', alert_type: 'HTTPTest' },
    { id: 3, member_id: alice.address, domain_id: 'testnet.ibp.network', service_id: 'polkadot-rpc', alert_type: 'HTTPTest' },
    { id: 4, member_id: alice.address, domain_id: 'testnet.ibp.network', service_id: 'asset-hub-westend-rpc', alert_type: 'HTTPTest' },
  ]
  for(let i = 0; i < alerts.length; i++) {
    const alert = alerts[i];
    tx = api.tx.ibpAlert.registerAlert(alert.id, alert.member_id, alert.service_id, alert.domain_id, alert.alert_type);
    await tx.signAndSend(charlie, { nonce: charlie_nonce++, tip: 1000000000 }, (r) => handleTransactionResponse(api, r));
  }

  // wait for 30 seconds before exiting
  setTimeout(() => {
    process.exit(0);
  }, 30_000);

})().catch(console.error);
