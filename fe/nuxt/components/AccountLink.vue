<template>
  <span>
    <a :href="href" target="_blank">
      <!-- {{textStr}} -->
      <v-avatar size="x-small">
        <v-img src="/image/subscan-logo2.png"></v-img>
      </v-avatar>
    </a>
     <!-- <sup><v-icon x-small>mdi-open-in-new</v-icon></sup> -->
  </span>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    chainId: {
      type: String,
      required: true
    },
    accountId: {
      type: String,
      required: true
    },
    role: {
      type: String,
      required: false,
      default: 'account'
    },
    provider: {
      type: String,
      required: false,
      default: 'https://§chainId§.subscan.io/§role§/§accountId§'
    },
    text: {
      type: String,
      required: false
    }
  },
  computed: {
    href () { return (this.provider)
      .replace('§chainId§', this.chainId)
      .replace('§role§', this.role)
      .replace('§accountId§', this.accountId) },
    textStr () { return this.text ? this.text : shortStash(this.accountId) }
  },
  data: () => {
    return {}
  }

})
</script>
