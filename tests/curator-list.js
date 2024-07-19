import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { customTypes } from "./types.js";  // Import custom types

;(async () => {
  const provider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({
    provider,
    types: customTypes
   });
  await api.isReady;

  // list all curators
  const curators = await api.query.ibpMember.curators.entries();
  for (const [key, value] of curators) {
    console.log(key.args[0].toHuman(), value.toHuman());
  }

  process.exit(0);

})().catch(console.error);
