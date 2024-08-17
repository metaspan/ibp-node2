import { hexToString } from '@polkadot/util'

export function shortStash (stash: string): string {
  if (typeof stash !== 'string' || stash === '') return stash
  return stash.slice(0, 6) + '...' + stash.slice(-6)
}

export function parseIdentity (id: any) {
  const idj = id.toJSON()
  // console.debug('idj', idj)
  if (idj) {
    const res = {} as any
    res.deposit = idj.deposit
    res.info = {
      // additional...
      additional: idj.info.additional.map((item: any) => {
        const [key, val] = item
        return {
          key: hexToString(key.raw),
          value: hexToString(val.raw)
        }
      }),
      display: idj.info.display.raw ? hexToString(idj.info.display.raw) : '',
      email: idj.info.email.raw ? hexToString(idj.info.email.raw) : '',
      // image...
      legal: idj.info.legal.raw ? hexToString(idj.info.legal.raw) : '',
      riot: idj.info.riot.raw ? hexToString(idj.info.riot.raw) : '',
      twitter: idj.info.twitter.raw ? hexToString(idj.info.twitter.raw) : '',
      web: idj.info.web.raw ? hexToString(idj.info.web.raw) : ''
    }
    res.judgements = idj.judgements
    return res
  }
  return null
}
