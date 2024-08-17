<template>
   <v-card>
      <v-row no-gutters>
        <v-col cols="1">
          <p class="text-center">
            <Identicon :value="member.id" :size="32" :theme="'polkadot'"></Identicon>
          </p>
        </v-col>
        <v-col>
          <v-card-title>
            {{ member.name }}
            ({{ getAccountName() }})
            {{ member.level }}
            {{ member.status }}
          </v-card-title>
          <v-card-subtitle>
            {{ member.id }}<br>
          </v-card-subtitle>
  
        </v-col>
      </v-row>
  </v-card>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { Identicon } from '@polkadot/vue-identicon';
import type { ISubstrateAPI } from '~/plugins/substrate';
import type { IMember, IAccount } from '@/types/global'

export default defineComponent({
  components: {
    Identicon
  },
  props: {
    member: {
      type: Object as () => IMember,
      required: true
    },
    account: {
      type: Object as () => IAccount,
      required: false
    }
  },
  setup(props) {
    const nuxtApp = useNuxtApp()
    const substrate = nuxtApp.$substrate as ISubstrateAPI
    var api = null
    const account = ref()
    const balance = ref<any>(0)
    var ss58Prefix = 42
    const decimals = ref(12)

    onBeforeMount(async () => {
      console.debug('AccountCard.vue: setup()')
      api = substrate.api
      await api.isReady
      ss58Prefix = api.registry.chainSS58 || 42; // Default to 42 if not set
      const chainInfo = await api.registry.getChainProperties()
      // decimals.value = chainInfo.tokenDecimals[0] || 12; // Default to 12 if not set
      // console.debug('api is ready', ss58Prefix)
      // balance.value = await api.query?.system.account(props.account.address);
    })

    const formatBalance = (balance: any) => {
      const denom = 10 ** decimals.value
      return balance ? Number(balance)/denom : '0'
    }

    const getAccountName = () => {
      return props.account ? props.account.meta.name : props.member.id.substring(0,8)
    }

    return {
      decimals,
      member: props.member,
      account: props.account,
      // balance,
      formatBalance,
      getAccountName,
    }
  }

})
</script>
