export interface IPlugin {
  accounts?: any
  metadata?: any
  name: string
  provider: any
  signer: any
  version: string
}

export interface IAccountMeta {
  name?: string
  genesisHash?: string | null | undefined
  source: string // name of plugin?
}
export interface IAccount {
  address: string
  meta: IAccountMeta
  type: string
}

export interface IMember {
  id: string
  name: string
  status: string
  level: string
}


export interface IChainInfo {
  ss58Format: number
  tokenDecimals: number
  tokenSymbol: string
}

export interface IWalletBalance {
  free: number
  reserved: number
  miscFrozen: number
  feeFrozen: number
}

export interface IdentityInfo {
  display: string
  email: string
  legal: string
  riot: string
  twitter: string
  web: string
}

export interface IIdentity {
  _id: string
  name: string
  info: IdentityInfo
  verified: boolean
  judgements: any[]
  parentIdentity: IIdentity
  sub: string
  subId: string
}

export interface ICandidateScore {
  address: string
  aggregate: number
  asn: number
  bonded: number
  councilStake: number
  country: number
  delegations: number
  democracy: number
  discovered: number
  faults: number
  inclusion: number
  location: number
  nominated: number
  nominatorStake: number
  offline: number
  provider: number
  randomness: number
  rank: number
  region: number
  spanInclusion: number
  total: number
  unclaimed: number
  updated: number
  __v?: number
  _id?: string
}

export interface ICandidateValidityItem {
  _id: string
  valid: boolean
  type: string
}

export interface ICandidate {
  chain?: string
  active: boolean
  commission: number
  councilStake: number
  councilVotes: number[]
  country: string
  democracyVoteCount: number
  democracyVotes: string[] | number[]
  discoveredAt: string | number //| Date
  faults: string[]
  stash: string
  total: number
  score: ICandidateScore
  rank: number
  identity: IIdentity
  location: string
  name: string
  nominatedAt: string | number //| Date
  onlineSince: number | string
  provider: string
  region: string
  rewardDestination?: string
  unclaimedEras: string[]
  valid: boolean
  validity: ICandidateValidityItem[]
  nominated_1kv: boolean
  nominations?: number | INomination[]
  version?: string
}

export interface INomination {}

export class Candidate implements ICandidate {
  active = false
  commission = 0
  councilStake = 0
  councilVotes = [] as number[]
  country = ''
  democracyVoteCount = 0
  democracyVotes = [] as number[] | string[]
  discoveredAt: any = 0 // as number //| Date
  faults = [] as string[]
  stash = ''
  total = 0
  score = { total: 0 } as ICandidateScore
  rank = 0
  identity = {} as IIdentity
  location = ''
  name = ''
  nominatedAt: any = 0 // as number //| Date
  onlineSince = 0 as number | string
  provider = ''
  region = ''
  unclaimedEras = [] as string[]
  valid = false
  validity = [] as ICandidateValidityItem[]
  // eslint-disable-next-line
  nominated_1kv = false

  constructor (data: ICandidate) {
    this.active = !!data.active
    this.commission = data.commission || 0
    this.councilStake = data.councilStake || 0
    this.councilVotes = data.councilVotes || [] as number[]
    this.democracyVoteCount = data.democracyVoteCount || 0
    this.democracyVotes = data.democracyVotes
    this.discoveredAt = data.discoveredAt || 0
    this.faults = data.faults || [] as string[]
    this.stash = data.stash || ''
    this.total = data.total || 0
    this.score = data.score || { total: 0 } as ICandidateScore
    this.rank = data.rank || 0
    // console.debug('identity', JSON.stringify(Object.keys(data.identity ? data.identity : {})))
    this.identity = data.identity || {} as IIdentity
    this.location = data.location || ''
    this.name = data.name || ''
    this.nominatedAt = data.nominatedAt || 0
    this.onlineSince = data.onlineSince || 0
    this.unclaimedEras = data.unclaimedEras || []
    this.valid = !!data.valid
    this.validity = data.validity || [] as ICandidateValidityItem[]
    this.nominated_1kv = data.nominated_1kv
    this.setValid()
  }

  setValid (validity?: ICandidateValidityItem[]): void {
    if (validity) {
      this.validity = validity
    }
    this.valid = this.validity.filter(f => f.valid === false).length === 0
  }
}

// export class Candidate <ICandidate> {
//   constructor (data: ICandidate) {
//     Object.keys(data).forEach(k => {
//       this[k] = data[k]
//     })
//     this.setValid()
//   }

//   setValid (validity?: ICandidateValidityItem[]): void {
//     if (validity) {
//       this.validity = validity
//     }
//     this.valid = validity.filter(f => f.valid).length === 0
//   }
// }

export interface ICache {
  // eslint-disable-next-line
  items: Record<string, any>
}

export interface ICandidateListFilter {
  favourite?: boolean
  valid: boolean
  active: boolean
  nominated: boolean
  rank?: number
  // score?: number
  total?: number
  // TODO do we need to split this out?
  sort: string
  sortAsc: boolean
}

export interface ICandidateListSort {
  sort: string
  sortAsc: boolean
}

// export interface CacheItem {
//   data: Record<string, Item>
// }

export interface IPoolNominee {
  stash: string
}

export interface IPoolMember {
  accountId: string
  stash?: string
  reward?: string
  points: number
}

export interface IPoolRoles {
  bouncer: undefined | string
  creator: undefined | string
  depositor: undefined | string
  root: undefined | string
  nominator: undefined | string
  stateToggler: undefined | string
}

export interface IPool {
  id: number | string
  name: undefined | string
  roles: IPoolRoles
  state?: undefined | string
  points?: number
  claimable?: number
  nominees?: IPoolNominee[]
  members?: IPoolMember[]
}

export class Pool implements IPool {
  id = 0
  name = ''
  roles = {
    bouncer: undefined,
    creator: undefined,
    root: undefined,
    depositor: undefined,
    nominator: undefined,
    stateToggler: undefined
  }

  state = undefined
  points = 0
  claimable = 0
  nominees = [] as IPoolNominee[]
  members = [] as IPoolMember[]

  constructor (id: number, name: string) {
    this.id = id
    this.name = name
  }
}

interface IValidatorPrefs {
  commission: number
  blocked: boolean
}
export interface IValidator {
  active: boolean
  commission: number
  councilStake: number
  councilVotes: number[]
  democracyVoteCount: number
  // eslint-disable-next-line
  democracyVotes: any[]
  discoveredAt: number | Date
  // eslint-disable-next-line
  faults: any
  stash: string
  shortStash: string
  prefs: IValidatorPrefs
  nominators: string[]
  score: ICandidateScore
  rank: number
  identity: IIdentity
  location: string
  name: string
  nominatedAt: number | Date
  // eslint-disable-next-line
  onlineSince: any
  // eslint-disable-next-line
  total: number
  unclaimedEras: any[]
  valid: boolean
  // validity: ICandidateValidityItem[]
  StorageKey: number[]
  // eslint-disable-next-line
  Type: any
}
