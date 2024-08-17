import { defineStore } from 'pinia'

import { type IPool } from '../types/global'
import { useStore } from './index'

interface IPagination {
  page: number
  itemsPerPage: number
}

interface IOptions {
  page: number
  itemsPerPage: number
  sortBy: string
  sortDesc: boolean
}

interface IFilter {
  valid: boolean
  active: boolean
}

interface IChainState {
  apiConnected: boolean
  updatedAt: null|Date
  loading: boolean
  pagination: IPagination
  options: IOptions,
  search: string
  filter: IFilter
  favourites: string[]
  list: IPool[]
  filteredList: IPool[]
  pool: IPool
}
interface IState {
  initial: boolean
  chainId: string
  chains: Record<string, IChainState>
}

const initialState = {
  initial: true,
  chainId: 'kusama',
  chains: {
    kusama: {
      apiConnected: false,
      updatedAt: null,
      loading: false,
      pagination: {
        page: 1,
        itemsPerPage: 15
      } as IPagination,
      options: {
        page: 1,
        itemsPerPage: 10,
        sortBy: 'name',
        sortDesc: false
      } as IOptions,
      search: '',
      filter: {
        valid: false,
        active: false
      } as IFilter,
      favourites: [],
      list: [] as IPool[],
      filteredList: [],
      pool: { points: 0 } as IPool
    },
    polkadot: {
      apiConnected: false,
      updatedAt: null,
      loading: false,
      pagination: {
        page: 1,
        itemsPerPage: 15
      } as IPagination,
      options: {
        page: 1,
        itemsPerPage: 10,
        sortBy: 'name',
        sortDesc: false
      } as IOptions,
      search: '',
      filter: {
        valid: false,
        active: false
      } as IFilter,
      favourites: [],
      list: [] as IPool[],
      filteredList: [],
      pool: { points: 0 } as IPool
    }
  }
} as IState

export const usePoolStore = defineStore('pool', {
  state: (() => initialState),
  getters: {
    list (): IPool[] {
      console.debug('store/modules/pool.ts: getters.list()', useStore().chainId)
      return this.chains[useStore().chainId].list
    },
    pool (): IPool {
      console.debug('store/modules/pool.ts: getters.pool()', useStore().chainId)
      return this.chains[useStore().chainId].pool
    }
  },
  actions: {
    async init () {
      this.initial = false
    },
    async setChainId (chainId: string) {
      this.chainId = chainId
    },
    async loading (loading: boolean) {
      this.chains[this.chainId].loading = loading
    },
    async addPool (pool: IPool) {
      // console.debug('store/modules/pool.ts: addPool()', pool)
      // await commit('ADD_POOL', pool)
      this.list.push(pool)
    },
    async setList (list: IPool[]) {
      console.debug('pool.ts: action.setList()', list)
      // await commit('SET_LIST', list)
      this.chains[this.chainId].list = list
    },
    async setPool (poolId: string) {
      console.debug('store/modules/pool.ts: setPool()', poolId)
      const v = this.list.find((pool: IPool) => Number(pool.id) === Number(poolId))
      // await commit('SET_POOL', v)
      if(v) this.chains[useStore().chainId].pool = v
    },
    // async resetCache (): Promise<void> {
    //   console.debug('store/modules/pool.ts: actions.resetCache()')
    //   // clearState()
    //   // await stateManager.clearState(STORAGE_KEY)
    // }
  }
})
