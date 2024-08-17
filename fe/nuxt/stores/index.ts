import { defineStore } from 'pinia'
import { useSubstrateStore } from './substrate'
import { useCandidateStore } from './candidate'
import { usePoolStore } from './pool'

const chains = {
  ibp: {
    id: 'ibp',
    name: 'IBP',
    icon: ''
  },
  // kusama: {
  //   id: 'kusama',
  //   name: 'Kusama',
  //   icon: 'assets/kusama-logo.png'
  // },
  // polkadot: {
  //   id: 'polkadot',
  //   name: 'Polkadot',
  //   icon: 'assets/polkadot-logo.png'
  // }
}

interface IAlert {
  id: string
  type: 'info' | 'warning' | 'error'
  title?: string
  text: string
}
const isServer = typeof window === 'undefined'
const baseUrl = 'https://api.metaspan.io'

export const useStore = defineStore('store', {
  state: () => ({
    appVersion: process.env.PACKAGE_VERSION || '0',
    initial: true,
    loading: false,
    // baseUrl: process.env.NODE_ENV === 'production'
    //   ? 'https://api.metaspan.io'
    //   : `//${window.location.hostname}:${window.location.port}`,
    baseUrl,
    dark: false,
    showSettingsDialog: false,
    showNavDrawer: false,
    alerts: [] as IAlert[],
    chains,
    chainId: 'ibp'
  }),
  actions: {
    async init () {
      // await commit('SET_LOADING', true)
      this.loading = true
      // await dispatch('setChainId', state.chainId) // TODO: is this really necessary?
      this.setChainId(this.chainId)
      // await dispatch('substrate/init', {}, { root: true })
      useSubstrateStore().init()
      // await dispatch('candidate/init', {}, { root: true })
      useCandidateStore().init()
      // await dispatch('pool/init', {}, { root: true })
      usePoolStore().init()
      // await dispatch('selector/init', {}, { root: true })
      // await commit('SET_INITIAL', false)
      this.initial = false
      // await commit('SET_LOADING', false)
      this.loading = false
    },
    setLoading (loading: boolean) {
      this.loading = loading
    },
    setDark (dark: boolean) {
      this.dark = dark
    },
    setShowSettingsDialog (value: boolean) {
      this.showSettingsDialog = value
    },
    addAlert (alert: IAlert) {
      this.alerts.push(alert)
    },
    clearAlert (alert: IAlert) {
      this.alerts = this.alerts.filter(f => f.id !== alert.id)
    },
    setShowNavDrawer (value: boolean) {
      this.showNavDrawer = value
    },
    toggleNavDrawer () {
      this.showNavDrawer = !this.showNavDrawer
    },
    resetCache () {
      console.debug('stores/index.ts: actions.resetCache()')
    },
    async setChainId (chainId: string) {
      console.debug('stores/index.js: actions.setChain()', chainId)
      this.chainId = chainId
      useSubstrateStore().setChainId(chainId)
      usePoolStore().setChainId(chainId)
    },
  }
})

// export default store
