<template>
  <div>

    <span class="click-to-copy" @click.stop="copyToClipboard(text)">{{display || text}}</span>

    <v-snackbar
      :absolute="false"
      bottom
      v-model="snackbar"
    >
      {{ snackText }}

      <template v-slot:actions>
        <v-btn
          color="pink"
          @click="snackbar = false"
        >
          Close
        </v-btn>
      </template>
    </v-snackbar>

  </div>

</template>

<script lang="ts">
import { defineComponent } from 'vue'
export default defineComponent({
  props: ['text', 'display'],
  data () {
    return {
      snackbar: false,
      snackText: ''
    }
  },
  methods: {
    async copyToClipboard (text: string) {
      // navigator clipboard api needs a secure context (https)
      if (navigator.clipboard && window.isSecureContext) {
        // navigator clipboard api method'
        navigator.clipboard.writeText(text)
      } else {
        // text area method
        const textArea = document.createElement('textarea')
        textArea.value = text
        // make the textarea out of viewport
        textArea.style.position = 'fixed'
        textArea.style.left = '-999999px'
        textArea.style.top = '-999999px'
        document.body.appendChild(textArea)
        textArea.focus()
        textArea.select()
        await new Promise((resolve, reject) => {
          // here the magic happens
          document.execCommand('copy') ? resolve(1) : reject(new Error('no reason'))
          textArea.remove()
        })
      }
      this.snackText = 'Copied to clipboard'
      this.snackbar = true
      setTimeout(() => { this.snackbar = false }, 2000)
    }

  }
})
</script>
<style scoped>
.click-to-copy:hover {
  cursor: pointer;
  text-decoration-line: underline;
  text-decoration-style: dotted;
}
</style>
