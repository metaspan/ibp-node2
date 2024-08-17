<template>
  <v-container>
    <v-toolbar color="transparent">
      <v-toolbar-title>
        <v-icon size="small">mdi-account-group</v-icon>&nbsp;
        Accounts
      </v-toolbar-title>
      <v-toolbar-items>
        <!-- <v-btn icon @click="showAddMemberDialog=true">
          <v-icon>mdi-plus</v-icon>
        </v-btn> -->
        <v-btn icon @click="refresh()">
          <v-icon>mdi-refresh</v-icon>
        </v-btn>
      </v-toolbar-items>
    </v-toolbar>

    <!-- {{ accounts }} -->

    <v-row>
      <v-col v-for="item in accounts">
        <AccountCard :account="item"></AccountCard>
      </v-col>
    </v-row>

  </v-container>
</template>

<script lang="ts">
import { defineComponent, computed, onBeforeMount, ref } from 'vue'
import { ApiPromise } from '@polkadot/api';
import { web3Accounts, web3Enable, web3FromAddress, web3FromSource } from '@polkadot/extension-dapp'
import { stringToHex } from '@polkadot/util'
import { Identicon } from '@polkadot/vue-identicon';

import type { ISubstrateAPI } from '~/plugins/substrate';
import AccountCard from '~/components/AccountCard.vue';
import type { IPlugin, IAccount } from '~/types/global';

export default defineComponent({
  components: {
    // Candidates
    AccountCard,
    Identicon
  },
  setup () {
    console.debug('member/index.vue: setup()')
    const route = useRoute()
    const router = useRouter()
    const store = useStore()
    const list = ref<any[]>()
    // const api: SubstrateAPI = ref()
    const nuxtApp = useNuxtApp()
    const substrate = nuxtApp.$substrate as ISubstrateAPI
    var api = null
    const injected = ref<IPlugin[]>([])
    const selPlugin = ref<IPlugin>()
    const peers = ref<any[]>()
    const selPeer: any = ref()
    const accounts = ref<IAccount[]>([])
    const selAccount = ref<IAccount>()

    const filteredAccounts = computed((): any[] => {
      return accounts.value.filter((f: IAccount) => f.meta.source === selPlugin.value?.name)
    })
    const allInjected = async () => {
      injected.value = (await web3Enable('Dotsama IBP Monitor')) as []
    }
    const allAccounts = async () => {
      if (!injected.value) await allInjected()
      const web3accs = await web3Accounts()
      console.debug('web3accs', web3accs)
      accounts.value = web3accs as []
    }

    onBeforeMount(async () => {
      console.debug('/account/index.vue: onBeforeMount()')
      console.debug('api', substrate.api)
      await refresh()
    })

    const refresh = async () => {
      api = substrate.api as ApiPromise
      await api.isReady
      console.debug('api is ready')

      const resp = await allInjected()
      console.debug(resp, injected.value)
      if (injected.value.length > 0) {
        selPlugin.value = injected.value[0]
      }
      allAccounts()
      console.debug('this.filteredAccounts.length', accounts.value, filteredAccounts.value.length)
      if (filteredAccounts.value.length > 0) selAccount.value = filteredAccounts.value[0]
    }

    return {
      list,
      accounts,
      refresh,
    }
  }
})
</script>
