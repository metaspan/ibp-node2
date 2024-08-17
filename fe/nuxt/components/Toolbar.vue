<template>
  <v-app-bar
    color="white"
    scroll-behavior="elevate"
    app
    :dark="dark"
  >
    <template v-slot:prepend>   
      <v-btn icon flat to="/" :ripple="false">
        <v-icon>mdi-server-outline</v-icon>
      </v-btn>     
    </template>
    <v-toolbar-title class="d-none d-sm-inline text-none">
        ibp.network
    </v-toolbar-title>

    <v-toolbar-items>

      <v-btn to="/account" class="text-none">
        <v-icon size="large">mdi-wallet-outline</v-icon>
        Account
      </v-btn>
      <v-btn to="/member" class="text-none">
        <v-icon size="large">mdi-account-hard-hat-outline</v-icon>
        <span class="d-none d-sm-inline">Member</span>
      </v-btn>
      <v-btn to="/service" class="text-none">
        <v-icon size="large">mdi-tools</v-icon>
        <span class="d-none d-sm-inline">Service</span>
      </v-btn>
      <v-btn to="/alert" class="text-none">
        <v-icon size="large">mdi-alert-outline</v-icon>
        <span class="d-none d-sm-inline">Alert</span>
      </v-btn>

    </v-toolbar-items>
    <v-spacer></v-spacer>
    <v-tooltip>
      <template v-slot:activator="{ props }">
        <v-btn
          small icon
          v-bind="props"
        >
          <v-icon :color="apiConnected ? 'primary' : ''">mdi-api{{ apiConnected ? '' : '-off' }}</v-icon>
        </v-btn>
        </template>
      <span>API connected: {{ apiConnected ? 'yes' : 'no' }}</span>
    </v-tooltip>

    <v-app-bar-nav-icon @click="toggleNavDrawer()"></v-app-bar-nav-icon>

  </v-app-bar>

</template>

<script lang="ts">
import { defineComponent, computed, ref, watch } from 'vue'

import { useTheme } from 'vuetify'
import { type ISubstrateAPI } from '@/plugins/substrate'

export default defineComponent({
  components: {
    // Alerts,
    // ChainMenu,
    // PwaDialog
  },
  setup (props, ctx) {
    const router = useRouter()
    const store = useStore()
    const substrateStore = useSubstrateStore()
    const theme = useTheme()
    const chainId = computed(() => store.chainId)
    // const dark = computed(() => store.state.dark)
    const dark = ref(false)
    // const settingsDialog = computed(() => store.state.showSettingsDialog)
    const settingsDialog = ref(false)
    const apiConnected = computed(() => substrateStore.connected)
    // const apiConnected = ref(false)

    const showSettingsDialog = ref(false)
    const bgColor = ref('grey-lighten-3')
    const nuxtApp = useNuxtApp()
    const substrate = nuxtApp.$substrate as ISubstrateAPI

    const toggleNavDrawer = () => {
      // store.dispatch('toggleNavDrawer')
      store.toggleNavDrawer()
    }

    // // SSR: leave this until we're on the client
    // watch(() => substrate?.connected, (newVal) => {
    //   console.debug('Toolbar.vue: watch substrate connected', newVal)
    // })
  
    watch(() => theme.global.current.value, (newVal) => {
      console.debug('watch theme', newVal)
      bgColor.value = (newVal.dark) ? '' : 'grey-lighten-3'
    })
  
    watch(() => settingsDialog.value, (val) => {
      showSettingsDialog.value = val
    })
  
    watch(() => showSettingsDialog.value, (val) => {
      ctx.emit('onSettingsDialog', val)
      // store.dispatch('setShowSettingsDialog', val)
      store.setShowSettingsDialog(val)
    })

    const navTo = (path: string) => {
      const name = path ? `chainId-${path}` : ''
      console.debug('navTo', path, name)
      router.push({
        // path: `/${chainId.value}/${path}`,
        name,
        params: { chainId: chainId.value }
      })
    }

    return {
      chainId,
      dark,
      settingsDialog,
      showSettingsDialog,
      apiConnected,
      bgColor,
      toggleNavDrawer,
      navTo
    }
  }
})
</script>

<style scoped>
.toolbar-title {
  cursor: pointer;
  color: inherit;
  text-decoration: inherit;
}
.toolbar-title-dark {
  cursor: pointer;
  color: inherit;
  text-decoration: inherit;
}
</style>
