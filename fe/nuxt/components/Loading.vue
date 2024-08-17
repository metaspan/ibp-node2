<template>
    <v-overlay v-model="isLoading" :absolute="absolute" class="align-center justify-center">
      <v-progress-circular
        :size="size"
        color="primary"
        indeterminate
        :contained="absolute"
      ></v-progress-circular>
    </v-overlay>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'Loading',
  props: {
    loading: {
      type: Boolean,
      required: true
    },
    size: {
      type: Number,
      required: false,
      default: 150
    },
    absolute: {
      type: Boolean,
      required: false,
      default: false
    }
  },
  watch: {
    loading (newVal) {
      console.debug('Loading.vue: watch.loading', newVal)
      this.isLoading = newVal
    },
    isLoading (newVal) {
      this.$emit('isLoading', newVal)
    }
  },
  data () {
    return {
      isLoading: false
    }
  }
  // created () {
  //   console.debug('Loading.vue: created')
  // },
  // mounted () {
  //   console.debug('Loading.vue: mounted')
  // }
})
</script>

<style scoped>
.fade-transition-enter-active {
  background-color: #b2fab2;
  transition: background-color 2s, all 2s;
}
.fade-transition-enter-to {
  background-color: white;
}
.fade-transition-leave-active {
  background-color: white;
  transition: background-color 2s, all 2s;
}
.fade-transition-leave-to {
  background-color: #fccece;
}
</style>
