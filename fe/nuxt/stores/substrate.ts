import { defineStore } from 'pinia'
import type { IChainInfo } from '~/types/global'

export interface IChainState {
}

interface IState {
  api: any
  chainId: string
  decimals: Record<number, number>
  // id: string
  // name: string
  connected: boolean
  chainInfo: any
}

export const useSubstrateStore = defineStore('substrate', {
  state: (): IState => ({
    api: null,
    chainId: 'ibp',
    decimals: {
      0: 1,
      1: 10,
      2: 100,
      3: 1000,
      4: 10000,
      5: 100000,
      6: 1000000,
      7: 10000000,
      8: 100000000,
      9: 1000000000,
      10: 10000000000,
      11: 100000000000,
      12: 1000000000000
    },
    connected: false,
    chainInfo: {}
  }),
  // getters: {
  //   chainInfo (): IChainInfo {
  //     return this.chainInfo || {}
  //   },
  //   connected (): boolean {
  //     return this.connected
  //   },
  // },
  actions: {
    async init () {
      console.debug('store/modules/substrate.ts: init()')
    },
    async setChainId (chainId: string) {
      console.debug('store/modules/substrate.ts: setChainId()', chainId)
      // await commit('SET_CHAIN_ID', chainId)
      this.chainId = chainId
    },
    async apiClose () {
      console.debug('store/modules/substrate.ts: apiclose()')
      // await dispatch('apiDisconnected', { chainId: 'polkadot' })
      this.setApiDisconnected('polkadot')
      // await dispatch('apiDisconnected', { chainId: 'kusama' })
      this.setApiDisconnected('kusama')
    },
    async setChainInfo (chainInfo: any) {
      console.debug('store/modules/substrate.ts: setChainInfo()', chainInfo)
      // await commit('SET_CHAIN_INFO', { chainId, chainInfo: { ...chainInfo } })
      this.chainInfo = { ...chainInfo }
    },
    async setApiConnected (chainId: string) {
      console.debug('store/modules/substrate.ts: apiConnected()', chainId)
      // commit('SET_API_CONNECTED', { chainId, connected: true })
      this.connected = true
    },
    async setApiDisconnected (chainId: string) {
      console.debug('store/modules/substrate.ts: apiDisconnected()', chainId)
      // commit('SET_API_CONNECTED', { chainId, connected: false })
      this.connected = false
    },
    async apiError (error: any) {
      console.log('API ERROR chain', this.chainId)
      console.error(error)
    }
  }
})
