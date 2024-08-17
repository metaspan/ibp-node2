<template>
  <v-container>
    <v-toolbar color="transparent">
      <v-toolbar-title>
        <v-icon>mdi-account-hard-hat-outline</v-icon>
        Members
      </v-toolbar-title>
      <v-toolbar-items>
        <v-btn icon @click="showAddMemberDialog=true">
          <v-icon>mdi-plus</v-icon>
        </v-btn>
        <v-btn icon @click="refresh()">
          <v-icon>mdi-refresh</v-icon>
        </v-btn>
      </v-toolbar-items>
    </v-toolbar>
    <!-- {{ list }} -->
    <!-- {{ accounts }} -->

    <v-row>
      <v-col v-for="item in list">
        <MemberCard :member="item" :account="getAccount(item.id)" @click="gotoMember(item.id)"></MemberCard>
      </v-col>
    </v-row>

    <v-card>
      <v-card-title>Monitors</v-card-title>
      <v-card-text>
        {{ monitors }}
      </v-card-text>
    </v-card>

    <v-card>
      <v-card-title>Curators</v-card-title>
      <v-card-text>
        {{ curators }}
      </v-card-text>
    </v-card>

    <v-dialog
      v-model="showAddMemberDialog"
      max-width="400"
      persistent
    >
      <!-- <template v-slot:activator="{ props: activatorProps }">
        <v-btn v-bind="activatorProps">
          Open Dialog
        </v-btn>
      </template> -->

      <v-card
        prepend-icon="mdi-account-hard-hat-outline"
        title="Add member"
      >
        <v-card-text>
          <v-text-field
            label="Name"
            outlined
          ></v-text-field>
          <v-text-field
            label="Address"
            outlined
          ></v-text-field>
        </v-card-text>
        <template v-slot:actions>
          <v-spacer></v-spacer>
          <v-btn @click="showAddMemberDialog = false">
            Disagree
          </v-btn>
          <v-btn @click="showAddMemberDialog = false">
            Agree
          </v-btn>
        </template>
      </v-card>
    </v-dialog>

</v-container>
</template>

<script lang="ts">
import { defineComponent, computed, onBeforeMount, ref } from 'vue'
import { web3Accounts, web3Enable, web3FromAddress, web3FromSource } from '@polkadot/extension-dapp'
import { hexToString, hexToU8a } from '@polkadot/util'
// import Candidates from '~/components/Candidates.vue';
import SubstrateAPI from '~/plugins/substrate'
import type { ISubstrateAPI } from '~/plugins/substrate'
import type { IPlugin, IAccount } from '~/types/global'
import MemberCard from '~/components/MemberCard.vue';

export default defineComponent({
  components: {
    // Candidates
    MemberCard
  },
  setup () {
    console.debug('member/index.vue: setup()')
    const route = useRoute()
    const router = useRouter()
    const store = useStore()
    const list = ref<any[]>()
    const accounts = ref<IAccount[]>([])
    const injected = ref<IPlugin[]>([])
    const monitors = ref<any[]>([])
    const curators = ref<any[]>([])
    const showAddMemberDialog = ref(false)

    const nuxtApp = useNuxtApp()
    const substrate = nuxtApp.$substrate as ISubstrateAPI
    var api = null

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
      console.debug('/member/index.vue: onBeforeMount()')
      console.debug('api', substrate.api)
      await refresh()
    })

    const gotoMember = (id: string) => {
      router.push(`/member/${id}`)
    }

    const getAccount = (id: string) => {
      return accounts.value.find(a => a.address === id)
    }

    const getAccountName = (address: string) => {
      const account = accounts.value.find(a => a.address === address)
      return account ? account.meta.name : address
    }

    const refresh = async () => {
      api = substrate.api
      await api?.isReady
      console.debug('api is ready')
      const members = await api?.query?.ibpMember.members.entries() || [];
      // console.debug('members', members)
      list.value = members.map(m => {
        let _m: any = m[1].toJSON()
        return { ..._m, name: hexToString(_m.name) }
      })
      console.debug('list', list.value)
      
      const accounts = ref<IAccount[]>([])
      await allAccounts()
      console.debug('accounts', accounts.value)
      
      const _curators = await api?.query?.ibpMember.curators.entries() || [];
      curators.value = _curators.map(m => {
        console.debug('curator', m[0].toHuman())
        let _m: any = m[0].toHuman()
        return _m[0]
      })

      const _monitors = await api?.query?.ibpMember.monitors.entries() || [];
      monitors.value = _monitors.map(m => {
        console.debug('monitor', m[0].toHuman())
        let _m: any = m[0].toHuman()
        return _m[0]
      })
    }

    return {
      list,
      accounts,
      monitors,
      curators,
      showAddMemberDialog,
      gotoMember,
      getAccount,
      getAccountName,
      refresh,
    }
  }
})
</script>
