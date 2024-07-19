import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { customTypes } from "./types.js";  // Import custom types

;(async () => {
  const provider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({
    provider,
    types: customTypes
   });
  await api.isReady;

  // list all members
  const members = await api.query.ibpMember.members.entries();
  for (const [key, value] of members) {
    console.log(key.args[0].toHuman(), value.toHuman());
  }

  process.exit(0);

})().catch(console.error);
