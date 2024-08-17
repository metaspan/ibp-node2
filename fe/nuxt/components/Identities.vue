<template>
  <client-only>
  <v-container style="max-width: 900px" >
    Identities
    <v-text-field v-model="search" label="Search" :loading="debouncing"></v-text-field>
    <v-list>
      <template v-for="(item, idx) in list" :key="`${item.accountId}${idx}`">

      <v-divider></v-divider>
      <v-list-item>

        <!-- <v-list-item-content> -->
          <v-list-item-title>{{item.identity.info.display}}</v-list-item-title>
          <v-list-item-subtitle>
            <v-row>
                <v-col class="col-6 col-sm-3"><ClickToCopy :text="item.accountId" :display="shortStash(item.accountId)"></ClickToCopy></v-col>
                <v-col>
                  <v-btn small elevation="2" icon>
                    <a :href="`https://${chainId}.subscan.io/account/${item.accountId}`" target="_blank" style="text-decoration: none; white-space: nowrap" nowrap>
                      <v-img width="16" height="16" src="/image/subscan-logo.png"></v-img>
                    </a>
                  </v-btn>
              </v-col>
            </v-row>
          </v-list-item-subtitle>
          <v-row>
            <v-col>
              <!-- <div v-for="key in Object.keys(item.identity.info)" v-bind:key="key">
                {{ key }}: {{item.identity.info[key] }}
              </div> -->
              <v-container fluid>
                <!-- <span v-if="item.identity.info.display"><v-icon small>mdi-file-document-outline</v-icon> {{item.identity.info.display}} <br></span> -->
                <span v-if="item.identity.info.email"><v-icon small>mdi-email</v-icon> <a :href="`mailto:${item.identity.info.email}`">{{item.identity.info.email}}</a> <br></span>
                <span v-if="item.identity.info.legal"><v-icon small>mdi-domain</v-icon> {{item.identity.info.legal}} <br></span>
                <span v-if="item.identity.info.riot"><v-icon small>mdi-account-box-outline</v-icon> {{item.identity.info.riot}} <br></span>
                <span v-if="item.identity.info.twitter"><v-icon small>mdi-twitter</v-icon> <a :href="`https://twitter.com/${item.identity?.info?.twitter}`" target="_blank" rel="noopener noreferrer">{{item.identity?.info?.twitter}}</a> <br></span>
                <span v-if="item.identity.info.web"><v-icon small>mdi-web</v-icon> <a :href="formatUrl(item.identity.info.web)" target="_blank">{{item.identity.info.web}}</a> </span>
              </v-container>
            </v-col>
            <v-col v-if="item.identity.children?.length > 0">Sub-identities <br>
              <v-list>
                <v-list-item v-for="child in item.identity.children" v-bind:key="child.accountId">
                  <!-- {{ child }} -->
                  <v-list-item-title>
                    <v-row>
                      <v-col>{{child.subId}} / </v-col>
                      <v-col><ClickToCopy :text="child.accountId" :display="shortStash(child.accountId)"></ClickToCopy></v-col>
                    </v-row>
                  </v-list-item-title>
                  <v-list-item-action>
                    <v-btn small elevation="2" icon>
                      <a :href="`https://${chainId}.subscan.io/account/${child.accountId}`" target="_blank" style="text-decoration: none; white-space: nowrap" nowrap>
                        <v-img width="16" height="16" src="/image/subscan-logo.png"></v-img>
                      </a>
                    </v-btn>
                  </v-list-item-action>
                </v-list-item>
              </v-list>
            </v-col>
          </v-row>
        <!-- </v-list-item-content> -->
        <hr>
      </v-list-item>

      </template>

    </v-list>
    <!-- {{ list }} -->
    <Loading :loading="loading"></Loading>
  </v-container>
  </client-only>
</template>

<script lang="ts">
import { defineComponent, computed, ref, watch, onBeforeMount } from 'vue'

import { GET_IDENTITIES } from '../graphql/identities'
import { useQuery } from '@vue/apollo-composable'

import { hexToString } from '@polkadot/util'
import Loading from './Loading.vue'
import ClickToCopy from './ClickToCopy.vue'

import { SubstrateAPI } from '@/plugins/substrate'

// import { debounce } from 'lodash'
function debounce(func: Function, wait: number) {
  let timeout: number | undefined;
  return function executedFunction(...args: any[]) {
    const later = () => {
      clearTimeout(timeout);
      func(...args);
    };
    clearTimeout(timeout);
    timeout = window.setTimeout(later, wait);
  };
}
interface IIdentityInfo {
  display?: string
  email?: string
  legal?: string
  riot?: string
  twitter?: string
  web?: string
}

interface IItemIdentity {
  deposit: number
  info: IIdentityInfo
  // judgements
  parent: any
  sub: string
  children: any
}
interface IItem {
  accountId: string
  chain: string
  identity: IItemIdentity
}

export default defineComponent({
  name: 'Identities',
  components: {
    ClickToCopy,
    Loading
  },
  setup() {
    const store = useStore()
    const router = useRouter()
    const chainId = computed(() => store.chainId)
    const search = ref('')
    const list = ref<IItem[]>()
    const debouncing = ref(false)
    // const loading = ref(false)
    const offset = ref(0)
    const limit = ref(20)
    const nuxtApp = useNuxtApp()
    const substrate = nuxtApp.$substrate as SubstrateAPI

    const variables: Record<string, any> | null = {
      chainId: chainId.value,
      // ids,
      search: search.value,
      // limit: limit.value,
      // offset: offset.value
    }
    const { query, result, loading, error, refetch, fetchMore, onResult } = useQuery(GET_IDENTITIES, variables, {
      fetchPolicy: 'cache-and-network'
    })
    
    const getSuperOf = async (stash: string) => {
      // console.debug(stash)
      const id = await substrate.api?.query.identity.superOf(stash)
      return Promise.resolve(id?.toJSON())
    }
    const superOf = async (child: string) => {
      let ret = ''
      // const id = await this.$substrate[this.chainId].query.identity.superOf(child)
      const id = await getSuperOf(child).then((id: any) => id)
      if (id) {
        console.debug('id', id)
        ret = hexToString(id[1]?.raw)
      }
      return ret
    }
    var debouncedSearch = (str: string) => { console.debug(str) }
    onBeforeMount(() => {
      // hack to get around this inside debounce?
      debouncedSearch = debounce((str: string) => {
        console.debug('debouncedSearch', str)
        getList()
        debouncing.value = false
      }, 1500)

    })

    const getList = async () => {
      // console.debug('getList', search.value)
      loading.value = true
      const variables = { chainId: chainId.value, search: search.value }
      const res = await refetch(variables)
      // console.debug('data', res)
      list.value = [] as IItem[]
      for (let i = 0; i < res?.data.Identities.length; i++) {
        var identity = res?.data.Identities[i]
        var children = identity.identity?.children || []
        var newKids = []
        for (let j = 0; j < children.length; j++) {
          let subId = ''
          if (typeof children[j] === 'string') {
            const sup = await substrate.api?.query.identity.superOf(children[j])
            console.debug('sup?.toJSON()', sup?.toJSON(), children[j])
            const supp: any = sup?.toJSON()
            if (supp) subId = hexToString(supp[1]?.raw)
            // children[j] = { accountId: children[j], subId }
          } else {
            console.debug('get super of', children[j])
          }
          newKids.push({ accountId: children[j], subId })
        }
        list.value.push({ ...identity, children: newKids })
      }
      loading.value = false
    }

    const formatUrl = (url: string) => {
      if (url.startsWith('https://') || url.startsWith('http://')) {
        return url
      } else {
        return 'https://' + url
      }
    }

    watch(() => chainId.value, newVal => {
      router.push(`/${newVal}/identity`)
      getList()
    })
    watch(() => search.value, newVal => {
      // console.debug('watch.search', newVal)
      debouncing.value = true
      debouncedSearch(newVal)
    })

    return {
      chainId,
      search,
      list,
      debouncing,
      loading,
      shortStash,
      formatUrl,
      superOf,
      getSuperOf,
      getList,
      debouncedSearch
    }
  },
  created () {
  }
})
</script>

<style scoped>
.subscan {
  width: 16px;
  height: 16px;
  object-fit: contain;
  object-position: 50% -30;
}
</style>
