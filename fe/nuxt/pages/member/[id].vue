<template>
  <v-container>
    <v-card :loading="loading">
      <v-card-title>
        <v-btn icon small flat to="/member">
          <v-icon>mdi-chevron-left</v-icon>
        </v-btn>
        {{ model?.id }}
      </v-card-title>
      <v-card-text>
        ID: {{ model.id }}<br>
        Status: {{ model.status }}<br>
        Level: {{ model.level }}
      </v-card-text>
    </v-card>
    <!-- {{ id }} -->
  </v-container>
</template>

<script lang="ts">
import { defineComponent, computed, onBeforeMount } from 'vue'

export default defineComponent({
  setup () {
    const store = useStore()
    const route = useRoute()
    const router = useRouter()
    const id = route.params.id.toString()
    const nuxtApp = useNuxtApp()
    const substrate = nuxtApp.$substrate as ISubstrateAPI
    const loading = ref(true)
    var api = null
    const model = ref({})

    onBeforeMount(async () => {
      console.debug('/member/[id].vue: onBeforeMount()')
      api = substrate.api
      await api.isReady
      console.debug('api is ready')
      const member = await api.query?.ibpMember.members(id);
      model.value = member.toJSON()
      // console.debug(member.toJSON())
      loading.value = false
    })
    
    return {
      loading,
      id,
      model,
    }
  }
})
</script>
