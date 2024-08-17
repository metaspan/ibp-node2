import { defineStore } from 'pinia'
import axios from 'axios'
import moment from 'moment'

import { useStore } from './index'

import {
  type ICandidate,
  type ICandidateListFilter,
  type ICandidateListSort,
  Candidate,
  type ICandidateValidityItem,
  type IChainInfo
} from '../types/global'

interface IPagination {
  page: number
  itemsPerPage: number
}

interface ISort {
  sort: string
  sortAsc: boolean
}

export interface IOptions {
  page: number
  itemsPerPage: number
  sortBy: string[],
  sortDesc: string[]
}

interface IMinMax {
  min: number
  max: number
}

type TRanges = Record<string, IMinMax>

interface IChainState {
  // initial: boolean // has the chain been initialised?
  apiConnected: boolean
  updatedAt: string // null | moment.Moment
  chainInfo: any
  scoreDenoms: Record<string, string>
  list: ICandidate[]
  //candidate: ICandidate
  //filteredList: ICandidate[]
  loading: Record<string, boolean>
  loadingTimeout: any
  pagination: IPagination
  sort: ISort
  search: string
  filtering: boolean
  filter: ICandidateListFilter
  ranges: TRanges
  options: IOptions
  favourites: string[]
}

interface IState {
  initial: boolean // has the module been initialised
  chainId: string
  chains: Record<string, IChainState>
  democracy: any
  referenda: any[]
}

// https://github.com/w3f/1k-validators-be/blob/master/apps/1kv-backend/config/kusama-score.json
const kusamaScoreDenoms: Record<string, string> = {
  // as per message from Will
  "inclusion": "220",
  "spanInclusion": "220",
  "discovered": "30",
  "nominated": "100",
  "rank": "50",
  "bonded": "50",
  "faults": "5",
  "offline": "2",
  "location": "40",
  "region": "10",
  "country": "10",
  "provider": "100",
  "nominations": "100"
};

// https://github.com/w3f/1k-validators-be/blob/master/apps/1kv-backend/config/polkadot-score.json
const polkadotScoreDenoms: Record<string, string> = {
  "inclusion": "220",
  "spanInclusion": "220",
  "discovered": "30",
  "nominated": "100",
  "rank": "50",
  "bonded": "50",
  "faults": "5",
  "offline": "2",
  "location": "40",
  "region": "10",
  "country": "10",
  "provider": "100",
  "nominations": "100"
};

const initialState = {
  initial: true,
  chainId: 'kusama',
  // updatedAt: moment().add(-1, 'day'),
  chains: {
    polkadot: {
      apiConnected: false,
      updatedAt: '', //moment().add(-1, 'day').utc().format(),
      chainInfo: {},
      scoreDenoms: polkadotScoreDenoms,
      loading: { list: false, candidate: false },
      loadingTimeout: null,
      filtering: false,
      pagination: {
        page: 1,
        itemsPerPage: 15
      },
      sort: {
        // sort: 'score',
        sort: 'total',
        sortAsc: false
      },
      search: '',
      filter: { valid: false, active: false, nominated: false, sort: 'total', sortAsc: true },
      options: {} as IOptions,
      ranges: {
        total: { min: 0, max: 0 },
        rank: { min: 0, max: 0 },
        bonded: { min: 0, max: 0 }
      },
      favourites: ['16ce9zrmiuAtdi9qv1tuiQ1RC1xR6y6NgnBcRtMoQeAobqpZ'],
      list: [],
      //filteredList: [],
      //candidate: new Candidate({ stash: 'loading', valid: false, validity: [{ valid: false }] as ICandidateValidityItem[] } as ICandidate)
    } as IChainState,
    kusama: {
      apiConnected: false,
      updatedAt: '', // moment().add(-1, 'day').utc().format(),
      chainInfo: {},
      scoreDenoms: kusamaScoreDenoms,
      loading: { list: false, candidate: false },
      loadingTimeout: null,
      filtering: false,
      pagination: {
        page: 1,
        itemsPerPage: 15
      },
      sort: {
        sort: 'total',
        sortAsc: false
      },
      search: '',
      filter: { valid: false, active: false, nominated: false, sort: 'total', sortAsc: false },
      options: {} as IOptions,
      ranges: {
        total: { min: 0, max: 0 },
        rank: { min: 0, max: 0 },
        bonded: { min: 0, max: 0 }
      },
      favourites: [
        'HyLisujX7Cr6D7xzb6qadFdedLt8hmArB6ZVGJ6xsCUHqmx',
        'FAR296Aqh9i8W5bi7BS7a8Bkhbw5LX5xCXP22c1Jvc2tM5v'
      ],
      list: [],
      //filteredList: [],
      //candidate: new Candidate({ valid: false, validity: [{ valid: false }] as ICandidateValidityItem[] } as ICandidate)
    } as IChainState,
  },
  democracy: {},
  referenda: []
} as IState

interface IMutLoading {
  key: string
  chain: string
  loading: boolean
}

export const useCandidateStore = defineStore('candidate', {
  state: (): IState => initialState, // stateManager.getStateSync(STORAGE_KEY, initialState), // .then(s => s),
  getters: {
    apiConnected (): boolean { return this.chains[useStore().chainId]?.apiConnected || false },
    chainInfo (): IChainInfo { return this.chains[useStore().chainId].chainInfo || {} },
    list (): ICandidate[] { return this.chains[useStore().chainId]?.list || [] },
    // candidate (): ICandidate { return this.chains[useStore().chainId]?.candidate || {} },
    // filteredList (): ICandidate[] {
    //   console.debug('stores/candidate.ts: getters.filteredList()', useStore().chainId)
    //   return this.chains[useStore().chainId]?.filteredList || []
    // },
    favourites (): string[] { return this.chains[useStore().chainId]?.favourites || [] },
    search (): string { return this.chains[useStore().chainId]?.search || '' },
    filter (): ICandidateListFilter { return this.chains[useStore().chainId]?.filter || {} },
    filtering (): boolean { return this.chains[useStore().chainId]?.filtering || false },
    loading (): boolean {
      return this.chains[useStore().chainId]?.loading.list === true || this.chains[useStore().chainId]?.loading.candidate === true || false
    },
    options (): IOptions { return this.chains[useStore().chainId]?.options || {} },
    ranges (): TRanges { return this.chains[useStore().chainId]?.ranges || [] },
    denoms (): Record<string, string> { 
      console.debug('stores/candidate.ts: getters.denoms()', useStore().chainId)
      //console.debug(this.chains[useStore().chainId])
      return this.chains[useStore().chainId]?.scoreDenoms || {}
    },
    updatedAt (): string {
      return moment(this.chains[useStore().chainId]?.updatedAt).toString()
    },
  },
  actions: {
    // eslint-disable-next-line
    async init () {
      console.debug('store/modules/candidate.ts: init()', useStore().chainId)
      // state: getState() already does this
      // const sstate = await getState(state)
      // await commit('SET_CHAIN_ID', useStore().chainId)
      // useStore().chainId = useStore().chainId
      // const newState = await stateManager.getState('candidate', state)
      // await commit('INIT', newState)
      // // await dispatch('setChainId', sstate.chainId, { root: true })
      // await commit('SET_INITIAL', false)
      this.initial = false
    },
    // async setChainId (chainId: string) {
    //   console.debug('store/modules/candidate.ts: setChainId()', chainId)
    //   useStore().chainId = chainId
    // },
    async apiClose () {
      // await dispatch('apiStatus', { connected: false, chain: 'kusama' })
      this.setApiStatus({ connected: false, chainId: 'kusama' })
      // await dispatch('apiStatus', { connected: false, chain: 'polkadot' })
      this.setApiStatus({ connected: false, chainId: 'polkadot' })
    },
    async setApiStatus ({ connected, chainId }: any) {
      // await commit('API_STATUS', { connected, chain })
      this.chains[chainId].apiConnected = connected
    },
    // async getList () {
    //   console.debug('store/modules/candidate.ts: getList()', this.chainId, 'initial:', this.initial)
    //   let list = []
    //   // console.debug('store/modules/candidate.ts: getList(): updatedAt', this.chains[this.chainId].updatedAt.toString())
    //   if (
    //     !this.chains[this.chainId]?.updatedAt ||
    //     moment().diff(moment(this.chains[this.chainId]?.updatedAt), 'seconds') > 60
    //   ) {
    //     console.debug('store/modules/candidate.ts: getList(): reloading list from api', this.chainId)
    //     if (this.chains[this.chainId]?.loading.list) {
    //       console.warn('We are already loading the list...')
    //       return
    //     }
    //     try {
    //       // eslint-disable-next-line
    //       let res = null as any
    //       await commit('SET_LOADING', { key: 'list', loading: true })
    //       // var res = await axios.get(`${baseUrl}/candidates`)
    //       res = await axios.get(
    //         // 'https://619wrsnit5.execute-api.eu-west-2.amazonaws.com/default/kusama-1kv-candidates'
    //         // `//api.metaspan.io/api/${this.chainId}/candidate`
    //         `${this.baseUrl}/api/${this.chainId}/candidate`
    //       )
    //       // console.debug('HHHHHHHHH candidates.ts: got data:', res.data)
    //       list = res.data?.candidates ? res.data.candidates : res.data
    //       if (list) {
    //         await commit('SET_LIST', { list })
    //       } else {
    //         console.debug('there is no data')
    //         console.debug(res)
    //       }
    //       // console.debug("committed")
    //     } catch (err) {
    //       await commit('SET_LOADING', { key: 'list', loading: false })
    //       console.debug('OOPS, caught an error')
    //       console.error(err)
    //     } finally {
    //       await commit('SET_LOADING', { key: 'list', loading: false })
    //     }
    //     await dispatch('filterList')
    //   } else {
    //     dispatch('addAlert', { id: moment().valueOf(), type: 'warning', title: 'Cache age < 60 seconds', text: 'Serving from local cache' }, { root: true })
    //     console.log(`age of cache: ${moment().diff(moment(this.chains[this.chainId].updatedAt), 'seconds')} seconds`)
    //   }
    // },

    // async paginate ({ state, commit }: any, { pagination }: any) {
    //   console.debug('store/modules/candidate.ts: paginate()', pagination)
    //   await commit('SET_PAGINATION', { pagination })
    //   const options = Object.assign(this.chains[this.chainId].options, pagination)
    //   await commit('SET_OPTIONS', { options })
    // },

    async handleOptions (options: IOptions) {
      // await commit('SET_OPTIONS', { options })
      this.chains[useStore().chainId].options = options
      // await dispatch('filterList')
    },

    async handleFilter (filter: ICandidateListFilter) {
      console.debug('store/modules/candidate.ts: actions.handleFilter()', { filter })
      // await commit('SET_FILTER', { filter })
      this.chains[useStore().chainId].filter = filter
      // await dispatch('filterList')
    },

    async setSearch (search: string) {
      // await commit('SET_SEARCH', { search })
      this.chains[useStore().chainId].search = search
      // await dispatch('filterList')
    },

    setList(list: ICandidate[]) {
      this.chains[useStore().chainId].list = list
    },

    // async filterList ({ state, commit }: any) {
    //   console.debug('store/modules/candidate.ts: filterList()')
    //   commit('SET_FILTERING', { value: true })
    //   const filter: ICandidateListFilter = this.chains[this.chainId]?.filter || {}
    //   // const sort: ICandidateListSort = this.chains[this.chainId].sort
    //   const sort = { sort: filter.sort, sortAsc: filter.sortAsc }
    //   const search: string = this.chains[this.chainId]?.search || ''
    //   const favourites: string[] = this.chains[this.chainId]?.favourites || []
    //   console.debug('store/modules/candidate.ts: filter, sort, search', filter, sort, search, favourites)
    //   const flist = (this.chains[this.chainId]?.list || [])
    //     .filter((item: ICandidate, idx: number) => {
    //       // console.debug('item', idx, item)
    //       if ((filter.favourite && !favourites.includes(item.stash)) ||
    //         (filter.valid && !item.valid) ||
    //         (filter.active && !item.active) ||
    //         (filter.nominated && !item.nominated_1kv) ||
    //         (filter.rank && !(item.rank > filter.rank)) ||
    //         // (filter.score && !(item.score.total > filter.score))
    //         (filter.total && !(item.total > filter.total))
    //       ) {
    //         // console.debug('filter', filter.valid, 'item', item.valid)
    //         return false
    //       } else if (search !== '' && !(
    //         item.name.toLowerCase().includes(search.toLowerCase()) ||
    //         item.identity?.name?.toLowerCase().includes(search.toLowerCase()) ||
    //         // {{ candidate.identity ? candidate.identity.name : '' }}
    //         item.stash.includes(search)
    //       )) {
    //         // console.debug('test:', item.stash, search, item.stash.includes(search))
    //         return false
    //       } else {
    //         // console.debug('test:', item.stash, 'ok!')
    //         return true
    //       }
    //     })
    //     .sort((a: ICandidate, b:ICandidate): number => {
    //       const sortField = sort.sort as keyof ICandidate
    //       const sortAsc: boolean = sort.sortAsc
    //       if (sortField === 'name') {
    //         // console.debug('sort:', a.name, b.name)
    //         return sortAsc
    //           ? String(a.name).localeCompare(b.name)
    //           : String(b.name).localeCompare(a.name)
    //       } else {
    //         return sortAsc
    //           ? (a[sortField] as number) - (b[sortField] as number)
    //           : (b[sortField] as number) - (a[sortField] as number)
    //       }
    //     })
    //   commit('SET_FILTERED_LIST', { flist })
    //   commit('SET_FILTERING', { value: false })
    // },

    // async setCandidate ({ chainId, stash }: any) {
    //   console.debug('store/modules/candidate.ts: setCandidate()', chainId, stash)
    //   let v = this.chains[chainId].list.find((i: ICandidate) => i.stash === stash)
    //   if (!v) {
    //     console.debug('not in cache... axios direct')
    //     // await commit('SET_LOADING', { key: 'candidate', chainId, loading: true })
    //     this.setLoading({ key: 'candidate', value: true })
    //     // const res = await axios.get(`//api.metaspan.io/api/kusama/candidate/${stash}`)
    //     const res = await axios.get(`${useStore().baseUrl}/api/${chainId}/candidate/${stash}`)
    //     if (res?.data) {
    //       v = res.data
    //     } else {
    //       console.warn('API ERROR', res)
    //     }
    //     // await commit('SET_LOADING', { key: 'candidate', chainId, loading: false })
    //     this.setLoading({ key: 'candidate', value: false })
    //   }
    //   // await commit('SET_CANDIDATE', { chainId, model: v })
    //   if(v) this.chains[this.chainId].candidate = v
    //   // await dispatch('polkadot/get', stash, {root:true})
    // },

    async setLoading ({ key, value }: any) {
      console.debug('store/modules/candidate.ts: actions.setLoading()', value)
      // await commit('SET_LOADING', { key, chain, loading: value })
      if (this.chains[useStore().chainId]) this.chains[useStore().chainId].loading[key] = value
      else console.warn('this.chains[useStore().chainId] is', this.chains[useStore().chainId])
    },

    async toggleFav ({ chain, stash}: any) {
      // console.debug('toggleFav()', stash)
      // await commit('TOGGLE_FAV', { chain, stash })
      const idx = this.chains[useStore().chainId].favourites.findIndex((v:string) => {
        return v === stash
      })
      // console.debug('idx', idx);
      if (idx > -1) {
        this.chains[useStore().chainId].favourites = this.chains[useStore().chainId].favourites.filter((f: string) => {
          return f !== stash
        })
      } else {
        this.chains[useStore().chainId].favourites.push(stash)
      }
    },

    async resetCache (): Promise<void> {
      console.debug('store/modules/candidate.ts: actions.resetCache()')
    },
    async getDemocracy ({ stash }: any): Promise<void> {
      console.debug('store/modules/candidate.ts: actions.getDemocracy()')
      const res = await axios.get(`${useStore().baseUrl}/api/${useStore().chainId}/query/democracy/votingOf?accountId=${stash}`)
      if (res.data) {
        // await commit('SET_DEMOCRACY', res.data.votingOf || {})
        this.democracy = res.data.votingOf || {}
      }
    },

    async getReferenda ({ stash }: any): Promise<void> {
      console.debug('store/modules/candidate.ts: actions.getReferenda()')
      // commit('SET_REFERENDA', [])
      this.referenda = []
      let res: any = await axios.get(`${useStore().baseUrl}/api/${useStore().chainId}/query/referenda/referendumCount`)
      if (res.data && res.data.referendumCount) {
        const count = res.data.referendumCount
        console.debug('referendumCount', count)
        const referenda: any[] = []
        for (let i = count; i > 0; i--) {
          res = await axios.get(`${useStore().baseUrl}/api/${useStore().chainId}/query/convictionVoting/votingFor`, {
            params: {
              accountId: stash,
              id: i
            }
          })
          const info = await axios.get(`${useStore().baseUrl}/api/${useStore().chainId}/query/referenda/referendumInfoFor`, { params: { id: i } })
          referenda.push({ id: i, ...res?.data?.votingFor, info: info.data.referendumInfoFor })
          // await commit('ADD_REFERENDA', { id: i, ...res?.data?.votingFor, info: info.data.referendumInfoFor })
          this.referenda.push({ id: i, ...res?.data?.votingFor, info: info.data.referendumInfoFor })
        }
        // res = await axios.get(`${useStore().baseUrl}/api/${state.chainId}/query/democracy/votingOf?accountId=${stash}`)
        // if (res.data) {
        //   await commit('SET_DEMOCRACY', res.data.votingOf || {})
        // }
        // commit('SET_REFERENDA', referenda)
      }
    }
  }
})

