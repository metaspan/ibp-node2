<template>
  <v-container fluid>

    <v-toolbar color="transparent">
      <v-toolbar-title>
        <v-icon size="small">mdi-alert-outline</v-icon>
        Alerts</v-toolbar-title>
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
          <th>MonitorId</th>
          <th>AlertId</th>
          <th>Type</th>
          <th>
            <v-combobox
              clearable
              chips
              multiple
              v-model="memberFilter"
              label="Member"
              :items="members"
              item-title="name"
              item-value="id"
            ></v-combobox>
            <!-- {{ members }} -->
            <!-- {{ memberFilter }} -->
          </th>
          <th>Domain</th>
          <th>Service</th>
          <th>!</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item in filteredList">
          <td>{{ shortStash(item.monitorId) }}</td>
          <td>{{ item.alertId }}</td>
          <td>{{ item.alertType }}</td>
          <td>
            <nuxt-link :to="`/member/${item.memberId}`">
              {{ shortStash(item.memberId) }}
            </nuxt-link>
          </td>
          <td>{{ item.domainId }}</td>
          <td>{{ item.serviceId }}</td>
          <td>
            <v-menu>
              <template v-slot:activator="{ props }">
                <v-btn icon="mdi-dots-vertical" variant="text" size="small" v-bind="props"></v-btn>
              </template>
              <v-list density="compact">
                <v-list-item>Clear</v-list-item>
              </v-list>
            </v-menu>
          </td>
        </tr>
      </tbody>
    </v-table>

</v-container>
</template>

<script lang="ts">
import { defineComponent, computed, onBeforeMount, ref } from 'vue'
import { hexToString } from '@polkadot/util';
// import Candidates from '~/components/Candidates.vue';
import type { ISubstrateAPI } from '~/plugins/substrate'
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
    const list = ref<any[]>()
    // const api: SubstrateAPI = ref()
    const nuxtApp = useNuxtApp()
    const substrate = nuxtApp.$substrate as ISubstrateAPI
    var api = null

    const members = ref<any[]>([])
    const memberFilter = ref<string[]>([])

    onBeforeMount(async () => {
      console.debug('/member/index.vue: onBeforeMount()')
      console.debug('api', substrate.api)
      await refresh()
    })

    const shortStash = (stash: string) => {
      return `${stash.slice(0, 6)}...${stash.slice(-6)}`
    }

    const gotoService = (id: string) => {
      router.push(`/service/${id}`)
    }

    const refresh = async () => {
      api = substrate.api
      await api?.isReady
      console.debug('api is ready')
      
      const _members = await api?.query?.ibpMember.members.entries() || [];
      members.value = _members.map(m => {
        const _m: any = m[1].toJSON()
        return { ..._m, name: hexToString(_m.name) }
      })
      const alerts = await api?.query?.ibpAlert.alerts.entries() || [];

      console.debug('alerts', alerts)
      list.value = alerts.map(m => {
        const [key, value] = m
        console.debug(value.toHuman())
        return {
          id: key.toJSON(),
          ...value.toHuman()
        }
      })
    }

    const filteredList = computed(() => {
      return list.value?.filter(item => {
        if (memberFilter.value.length === 0) return true
        return memberFilter.value.map((m: any) => m.id).includes(item.memberId)
      })
    })

    return {
      list,
      filteredList,
      members,
      memberFilter,
      gotoService,
      shortStash,
      refresh,
    }
  }
})
</script>
