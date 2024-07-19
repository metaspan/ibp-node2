import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { customTypes } from "./types.js";  // Import custom types

;(async () => {
  const provider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({
    provider,
    types: customTypes
   });
  await api.isReady;

  // MemberChilled(T::AccountId),
  // MemberUnChilled(T::AccountId),

  api.query.system.events((events) => {
    console.log(`\nReceived ${events.length} events:`);

    // Loop through the Vec<EventRecord>
    events.forEach((record) => {
      // Extract the phase, event and the event types
      const { event, phase } = record;
      const types = event.typeDef;

      // response = { memberId, status }
      if (event.section === 'ibpMember' && event.method === 'MemberStatusUpdated') {
        console.log(event.data.toJSON());
        const [ memberId, prev_status, new_status ] = event.data.toJSON();
        console.log(`MemberStatusUpdated: ${memberId}, was: ${prev_status}, is now a ${new_status}`);
      }

    });
  });

  // process.exit(0);

})().catch(console.error);
