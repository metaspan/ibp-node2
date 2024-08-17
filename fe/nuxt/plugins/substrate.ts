// import type { Plugin } from 'vue'
// import { defineNuxtPlugin } from '@nuxt/types'
// import { ApiPromise } from '@polkadot/api'
import { ApiPromise, WsProvider } from '@polkadot/api'

import { useStore } from '@/stores/index'
import { useSubstrateStore } from '@/stores/substrate'

const isServer = typeof window === 'undefined'

export default defineNuxtPlugin({
  name: 'substrate',
  // enforce: 'post',
  parallel: true,
  async setup(nuxtApp: any) {
    console.debug('plugins/substrate.ts: setup()')
    const substrate = new SubstrateAPI({ lite: false })
    await substrate.connect()
    nuxtApp.provide('substrate', substrate)
  },
  hooks: {
    'app:created'() {
      console.debug('plugins/substrate.ts: app:created, isServer:', isServer)
    }
  },
  env: {
    islands: false
  },
  // (app: any) => {
  // app.provide('$substrate', new SubstrateAPI({ lite: false }))
})

type TEndpoints = Record<string, Record<string, string>>
const endpoints: TEndpoints = {
  ibp: {
    local: 'ws://localhost:9944'
  },
  // polkadot: {
  //   ibp: 'wss://rpc.ibp.network/polkadot',
  //   dotters: 'wss://rpc.dotters.network/polkadot',
  //   parity: 'wss://rpc.polkadot.io',
  //   onFinality: 'wss://polkadot.api.onfinality.io/public-ws',
  //   dwellir: 'wss://polkadot-rpc.dwellir.com'
  // },
  // kusama: {
  //   // local: 'wss://192.168.1.85:30225',
  //   // local: 'http://192.168.1.85:40224',
  //   ibp: 'wss://rpc.ibp.network/kusama',
  //   dotters: 'wss://rpc.dotters.network/kusama',
  //   onFinality: 'wss://kusama.api.onfinality.io/public-ws',
  //   parity: 'wss://kusama-rpc.polkadot.io',
  //   dwellir: 'wss://kusama-rpc.dwellir.com'
  // },
  // parallel: {
  //   onFinality: 'wss://parallel.api.onfinality.io/public-ws'
  // }
}

export interface ISubstrateAPI {
  // chains: Record<string,ApiPromise | undefined>
  chainId: string
  connected: boolean
  api: ApiPromise | undefined
  // kusama: any
  // polkadot: number
}

class SubstrateAPI implements ISubstrateAPI {
  config: any = {
    lite: false,
    chain: 'ibp',
    endpoint: 'local'
  }

  connected: boolean = false
  chainId: string = '' // 'kusama'
  api: ApiPromise | undefined

  constructor (options: any) {
    this.config = { ...this.config, ...options }
    // useStore().$onAction((context: any) => {
    //   if (context.name === 'setChainId') {
    //     console.debug('plugins/substrate.ts: onAction', context.args)
    //     let newChainId = context.args[0]
    //     if(Object.keys(endpoints).includes(newChainId) && newChainId !== this.chainId) {
    //       //this.chainId = newChainId
    //       this.connect(newChainId)
    //     }
    //   }
    // })
  }

  async createWsProvider (chainId = 'ibp', endpoint = 'local') {
    console.debug('plugins/substrate.ts: createWsProvider()', chainId, endpoint)
    if (this.chainId === chainId && this.api) {
      console.debug('plugins/substrate.ts: we already have api for', chainId)
      await this.api.isReady
      return this.api
    }
    console.debug(`plugins/substrate.ts: not connected to ${chainId}, connecting...`)
    const wsUrl = endpoints[chainId][endpoint]
    console.debug(`plugins/substrate.ts: createWsProvider(${chainId}): wsUrl`, wsUrl)
    const provider = new WsProvider(wsUrl)

    provider.on('error', async (err: any) => {
      console.warn(`plugins/substrate.ts: on('error', ${chainId})`)
      // await store.dispatch('apiError', { chainId, error: err })
      useSubstrateStore().apiError({ chainId, error: err })
      console.error(err)
    })

    provider.on('connected', async () => {
      this.connected = true
      console.debug(`plugins/substrate.ts: on('connected', ${chainId})`)
      // await store.dispatch('substrate/apiConnected', chainId)
      useSubstrateStore().setApiConnected(chainId)
    })
    // console.debug(`${chain}: debug 3`)
    provider.on('disconnected', async (evt: any) => {
      this.connected = false
      console.debug(`plugins/substrate.ts: on('disconnected', ${chainId})`)
      console.debug(evt)
      // await store.dispatch('substrate/apiDisconnected', chainId)
      useSubstrateStore().setApiDisconnected(chainId)
    })

    console.debug('plugins/substrate.ts: about to connect api', chainId)
    // if (!provider.isConnected) await provider.connect()
    const api = await ApiPromise.create({ provider, noInitWarn: true, throwOnConnect: false })
    api.on('connected', () => { 
      console.debug('plugins/substate.ts: api connected')
      this.connected = true
      useSubstrateStore().setApiConnected(chainId)
    })

    api.on('disconnected', () => {
      console.debug('plugins/substate.ts: api disconnected')
      this.connected = false
      useSubstrateStore().setApiDisconnected(chainId)
    })

    await api.isReady
    console.debug(`plugins/substrate.ts: createWsProvider(${chainId}) api isReady`)
    // this.api = api // TODO duplication!
    // console.debug(`${chain}: debug 6`)
    return api
  }

  async connect (chainId = 'ibp'): Promise<void> {
    console.debug('plugins/substrate.ts: connect()', chainId)
    const api = await this.createWsProvider(chainId, this.config.endpoint)
    await api.isReady
    this.api = api
    console.debug(`plugins/substrate.ts: connect(${chainId}): we have an api...`)

    const ci = await this.chainInfo(chainId)
    const substrateStore = useSubstrateStore()
    substrateStore.setChainInfo(ci)
    return ci
  }

  // eslint-disable-next-line
  async chainInfo (chainId: string): Promise<any> {
    console.debug('plugins/substrate.ts: chainInfo()', chainId)
    // const chainInfo = await this[chain].api.registry.getChainProperties()
    const chainInfo = await this.api?.registry.getChainProperties()
    console.debug(`plugins/substrate.ts: chainInfo(${chainId})`, chainInfo?.toString())
    return JSON.parse(chainInfo?.toString()|| '{}')
  }
}

export { SubstrateAPI }
