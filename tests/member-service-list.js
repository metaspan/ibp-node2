import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { hexToString } from '@polkadot/util';
import { customTypes, enumMapping } from "./types.js";  // Import custom types

;(async () => {
  const provider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({
    provider,
    types: customTypes
   });
  await api.isReady;

  // list all members and the services they should provide
  const members = await api.query.ibpMember.members.entries();
  const services = await api.query.ibpService.services.entries();

  for(let i = 0; i < members.length; i++) {
    const [key, value] = members[i];
    // value is status, membershipLevel
    const { status, level } = value.toJSON();
    const membershipLevel = enumMapping[level];
    console.log(key.args[0].toHuman(), value.toHuman(), membershipLevel);
    
    let servicesRequired = [];
    // for each service, check if member provides it
    for(let j = 0; j < services.length; j++) {
      const [serviceKey, serviceValue] = services[j];
      // servicedata is id, chainId, serviceType, serviceLevel, status
      const service = serviceValue.toJSON();
      // level =  _enum: ['Zero', 'One', 'Two', 'Three', 'Four', 'Five', 'Six']
      // if service level is greater than or equal to membership level, member should provide service
      if (membershipLevel >= enumMapping[service.level] && service.status === 'Active') {
        servicesRequired.push(service);
      }
    }
    // for each servicesRequired, print the details
    console.log("Services required:", servicesRequired.length);
    for(let k = 0; k < servicesRequired.length; k++) {
      const service = servicesRequired[k];
      console.log('-', hexToString(service.id), hexToString(service.chainId), service.serviceType, service.level, service.status);
    }
  }

  process.exit(0);

})().catch(console.error);
