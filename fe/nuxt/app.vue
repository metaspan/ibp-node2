<template>
  <v-app :dark="dark">

    <Toolbar v-on:onSettingsDialog="onSettingsDialog"></Toolbar>
    <v-main>
      <nuxt-page v-slot="{ Component }">
        <!-- <transition name="fade" mode="out-in" :duration="500">
          <component :is="Component" />
        </transition> -->
      </nuxt-page>
    </v-main>
    <NavDrawer app></NavDrawer>
    <bottom-nav></bottom-nav>

    <SettingsDialog></SettingsDialog>
    <Loading :loading="appLoading || loading"></Loading>

  </v-app>
</template>

<script lang="ts">

import { defineComponent, ref, computed, inject, onBeforeMount, onMounted } from 'vue'
import { useTheme } from "vuetify"

import Toolbar from './components/Toolbar.vue'
import SettingsDialog from './components/SettingsDialog.vue'
import NavDrawer from './components/NavDrawer.vue'
// import BottomNav from './components/BottomNav.vue'
import Loading from './components/Loading.vue'

export default defineComponent({
  name: 'App',
  components: {
    Toolbar,
    SettingsDialog,
    NavDrawer,
    // BottomNav,
    Loading
  },
  setup () {
    console.debug('app.vue: setup() starting...')
    const store = useStore()
    const theme = useTheme()
    const dark = ref(false)
    // const loading = computed(() => store.state.loading)
    const loading = ref(false)
    const appLoading = ref(true)
    const showSettingsDialog = computed(() => store.showSettingsDialog)
    // const $substrate: SubstrateAPI = inject('$substrate') || new SubstrateAPI({ lite: false })
    const $substrate = useNuxtApp().$substrate

    // const { $pwa } = useNuxtApp()

    const settingsDialog = ref(false) 
    const matcher = ref()

    const onSettingsDialog = (v: boolean) => {
      settingsDialog.value = v
    }

    const onDark = (evt: any) => {
      // console.debug('onDark', evt)
      theme.global.name.value = evt.matches ? 'dark' : 'light'
    }

    onBeforeMount(async () => {
      const splash = document.getElementById('splash')
      if (splash) {
        console.debug('removing splash')
        const parent = splash.parentElement
        if (parent) parent.removeChild(splash)
      }
    })

    onMounted(async () => {
      console.debug('app.vue: mounted')
      matcher.value = window.matchMedia('(prefers-color-scheme: dark)')
      // set the initial state from the matcher  await this.onDark(this.matcher)
      matcher.value.addListener(onDark)
      onDark(matcher.value)
      store.init()
      appLoading.value = false
    })

    return {
      appLoading,
      loading,
      dark,
      onSettingsDialog,
      showSettingsDialog,
      settingsDialog,
      matcher,
      onMounted
    }
  },
  // NOT HERE, watch chain in ChainHome.vue
  // async chain (newval) {
  //   console.debug('woot, chain is', newval)
  //   await this.$substrate.connect(this.chain)
  //   // const chainInfo = await this.$substrate[this.chain].registry.getChainProperties()
  //   // console.log(chainInfo)
  // }
})
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 1s ease;
  transition: all 0.4s;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  /* filter: blur(1rem); */
}
.none {
  color: white;
  /* text-decoration: none; */
}
</style>



<!-- <template>
  <div>
    <NuxtWelcome />
  </div>
</template> -->
