<template>
  <v-container>
    <v-toolbar color="transparent">
      <v-toolbar-title>
        <v-icon size="small">mdi-tools</v-icon>
        Services
      </v-toolbar-title>
      <v-toolbar-items>
        <v-btn icon @click="refresh()">
          <v-icon>mdi-refresh</v-icon>
        </v-btn>
      </v-toolbar-items>
    </v-toolbar>
    <!-- {{ list }} -->

    <v-table>
      <thead>
        <tr>
          <th>ChainId</th>
          <th>ServiceId</th>
          <th>Type</th>
          <th>Status</th>
          <th>Level</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item in list" @click="gotoService(item.id)">
          <td>{{ item.chainId }}</td>
          <td>{{ item.id }}</td>
          <td>{{ item.serviceType }}</td>
          <td>{{ item.status }}</td>
          <td>{{ item.level }}</td>
        </tr>
      </tbody>
    </v-table>
  </v-container>
</template>

<script lang="ts">
import { defineComponent, computed, onBeforeMount, ref } from 'vue'
// import Candidates from '~/components/Candidates.vue';
import SubstrateAPI from '~/plugins/substrate'

export default defineComponent({
  components: {
    // Candidates
  },
  setup () {
    console.debug('member/index.vue: setup()')
    const route = useRoute()
    const router = useRouter()
    const store = useStore()
    const list: any[] = ref()
    // const api: SubstrateAPI = ref()
    const nuxtApp = useNuxtApp()
    const substrate = nuxtApp.$substrate as ISubstrateAPI
    var api = null

    onBeforeMount(async () => {
      console.debug('/member/index.vue: onBeforeMount()')
      console.debug('api', substrate.api)
      await refresh()
    })

    const gotoService = (id: string) => {
      router.push(`/service/${id}`)
    }

    const refresh = async () => {
      api = substrate.api
      await api?.isReady
      console.debug('api is ready')
      const services = await api?.query?.ibpService.services.entries() || [];

      console.debug('services', services)
      list.value = services.map(m => {
        const [key, value] = m
        console.debug(value.toHuman())
        return {
          id: key.toJSON(),
          ...value.toHuman()
        }
      })
    }

    return {
      list,
      gotoService,
      refresh,
    }
  }
})
</script>
