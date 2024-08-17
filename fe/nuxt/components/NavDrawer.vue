<template>
  <client-only>
  <v-navigation-drawer
    v-model="drawer"
    location="right"
    temporary
    >
    <v-toolbar>
      <v-toolbar-title>Menu</v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn small icon @click="drawer=false"><v-icon>mdi-close</v-icon></v-btn>
    </v-toolbar>

    <v-list>
      <v-list-item>
        <v-list-item-title>
          <v-icon small>mdi-git</v-icon>
          App version: {{appVersion}}</v-list-item-title>
      </v-list-item>

      <v-list-item :to="`/${chainId}/delegate`" >
        <v-list-item-title>
          <v-icon size="large">mdi-vote-outline</v-icon>
          Delegate
        </v-list-item-title>
      </v-list-item>

      <!-- <v-list-item :to="`/${chainId}/nomination1kv`" >
        <v-list-item-title>
          <v-icon size="large">mdi-water-plus-outline</v-icon>
          1kv Nominations 
          </v-list-item-title>
      </v-list-item> -->

      <v-list-item @click="resetCache()">
        <v-list-item-title>
          <v-icon small>mdi-cached</v-icon>
            Reset cache</v-list-item-title>
      </v-list-item>

      <v-list-item @click="toggleDark()">
        <v-list-item-title>
          <v-icon small>mdi-theme-light-dark</v-icon>
          {{ dark ? 'Light' : 'Dark' }} mode
        </v-list-item-title>
      </v-list-item>

    </v-list>
  </v-navigation-drawer>
  </client-only>
</template>

<script lang="ts">
import { defineComponent, computed, ref, watch, onBeforeMount } from 'vue'
import { useApolloClient } from '@vue/apollo-composable'
import { useTheme } from 'vuetify'

export default defineComponent({
  name: 'NavDrawer',
  setup () {
    const theme = useTheme()
    const store = useStore()
    const chainId = computed(() => store.chainId)
    const apolloClient = useApolloClient()
    const drawer = ref(false)
    const appVersion = computed(() => store.appVersion)
    const showNavDrawer = computed<boolean>(() => store.showNavDrawer)
    const dark = computed(() => store.dark)

    const resetCache = async () => {
      console.debug('clearCache')
      store.resetCache()
      const result = await apolloClient.client.clearStore()
      console.debug('result', result)
    }

    const toggleDark = () => {
      // this.$vuetify.theme.dark = !this.dark
      theme.global.name.value = theme.global.name.value !== 'dark' ? 'dark' : 'light'
      // store.dispatch('setDark', !dark.value)
      store.setDark(!dark.value)
    }

    watch(() => drawer.value, newVal => {
      // store.dispatch('showNavDrawer', newVal)
      store.setShowNavDrawer(newVal)
    })

    watch(() => showNavDrawer.value, newVal => {
      drawer.value = newVal
    })

    return {
      store,
      theme,
      chainId,
      showNavDrawer,
      appVersion,
      dark,
      resetCache,
      toggleDark,
      drawer
    }
  },
  data () {
    return {
      drawer: false
    }
  }
})
</script>
