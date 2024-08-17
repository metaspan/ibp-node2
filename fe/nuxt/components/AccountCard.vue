<template>
   <v-card>
    <v-card-text>

      <v-row no-gutters>
        <v-col cols="1">
          <p class="text-center">
            <Identicon :value="account.address" :size="32" :theme="'polkadot'"></Identicon>
          </p>
        </v-col>
        <v-col>
          <v-card-title>
            {{ account.meta?.name }}<br>
          </v-card-title>
          <v-card-subtitle>
            {{ account.address.substring(0,8) }}...
            <!-- {{ account.address }}<br> -->
          </v-card-subtitle>
          <!-- {{ balance.data }} -->
  
          Free: {{ formatBalance(balance.data?.free) }}<br>
          Reserved: {{ balance.data?.reserved }}<br>
          Frozen: {{ balance.data?.frozen }}<br>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { Identicon } from '@polkadot/vue-identicon';
import { encodeAddress } from '@polkadot/util-crypto'

import type { ISubstrateAPI } from '~/plugins/substrate';
import { SubstrateAPI } from '~/plugins/substrate';
import type { IAccount } from '@/types/global'
import type { ApiPromise } from '@polkadot/api';

export default defineComponent({
  components: {
    Identicon
  },
  props: {
    account: {
      type: Object as () => IAccount,
      required: true
    }
  },
  setup(props) {
    const nuxtApp = useNuxtApp()
    const substrate = nuxtApp.$substrate as ISubstrateAPI
    var api = null
    const lAccount = computed<IAccount>(() => { return {
      ...props.account,
      address: encodeAddress(props.account.address, 42)
    } })
    const balance = ref<any>(0)
    var ss58Prefix = 42
    const decimals = ref(12)

    onBeforeMount(async () => {
      console.debug('AccountCard.vue: setup()')
      api = substrate.api as ApiPromise
      await api.isReady
      ss58Prefix = api.registry.chainSS58 || 42; // Default to 42 if not set
      const chainInfo: any = await api.registry.getChainProperties()
      decimals.value = chainInfo.tokenDecimals[0] || 12; // Default to 12 if not set
      console.debug('api is ready', ss58Prefix)
      balance.value = await api.query?.system.account(props.account.address);
    })

    const formatBalance = (balance: any) => {
      const denom = 10 ** decimals.value
      return balance ? Number(balance)/denom : '0'
    }

    return {
      decimals,
      account: {
        ...props.account,
        // convert the address format to local chain format
        // address: encodeAddress(props.account.address, ss58Prefix)
      },
      lAccount,
      balance,
      formatBalance
    }
  }

})
</script>
