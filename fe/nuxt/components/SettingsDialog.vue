<template>
  <v-dialog
    v-model="showSettingsDialog"
    scrollable
    max-width="800px"
  >
    <v-card>
      <v-app-bar>Settings</v-app-bar>
      <v-card-text>
        <!-- <v-card-title>Settings</v-card-title> -->
        <v-radio-group v-model="rad_endpoint" :loading="loading" label="Endpoint">
          <v-radio v-for="item in kusamaEndpoints" :key="item.id" :value="item.id" :label="item.url"></v-radio>
        </v-radio-group>
        <!-- <v-list>
          <v-list-item v-for="item in kusamaEndpoints" v-bind:key="item.id">
            <v-list-item-icon>{{item.id == endpoint? '+': '-'}}</v-list-item-icon>
          {{item}}
          </v-list-item>
        </v-list> -->

      </v-card-text>
      <!-- <v-card-actions>
        <v-btn>Save</v-btn>
      </v-card-actions> -->
    </v-card>

  </v-dialog>

</template>

<script lang="ts">
import { defineComponent, ref, computed, watch, onBeforeMount } from 'vue'
// import { usePolkadotStore } from '../stores/polkadot'
type IEndpoint = Record<string, string>

export default defineComponent({
  setup () {
    const store = useStore()
    const chainId = computed(() => store.chainId)
    // const polkadotStore = usePolkadotStore()
    // const kusamaEndpoints = computed(() => polkadotStore.endpoints['kusama'])
    const settingsDialog = computed(() => store.showSettingsDialog)
    const showSettingsDialog = ref(false)
    const loading = ref(false)
    const rad_endpoint = ref()

    watch(() => settingsDialog.value, newVal => {
      // console.debug('SettingsDialog.vue: watch.showSettingsDialog()', val)
      showSettingsDialog.value = newVal
    })
    watch(() => showSettingsDialog.value, newVal => {
      // console.debug('SettingsDialog.vue: watch.showSettingsDialog()', val)
      store.setShowSettingsDialog(newVal)
      // this.showSettingsDialog = val
    })
    watch(() => rad_endpoint.value, newVal => {
      console.warn('DEPRICATED: rad_endpoint()')
      // polkadotStore.setEndpoint(newVal)
    })

    /**
     * @deprecated
     */
    onBeforeMount(() => {
      console.warn('DEPRICATED: rad_endpoint()')
      // rad_endpoint.value = polkadotStore.endpoint
      showSettingsDialog.value = store.showSettingsDialog
    })

    return {
      settingsDialog,
      showSettingsDialog,
      // kusamaEndpoints,
      loading,
      rad_endpoint
    }
  }
})
</script>
