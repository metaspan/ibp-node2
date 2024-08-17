import * as pwa  from '@vite-pwa/nuxt'
import type { UnwrapNestedRefs } from 'vue'

export interface PwaInjection {
  /**
   * @deprecated use `isPWAInstalled` instead
   */
  isInstalled: boolean
  /**
   * From version v0.3.5+. 
   */  
  isPWAInstalled: Ref<boolean>
  showInstallPrompt: Ref<boolean>
  cancelInstall: () => void
  install: () => Promise<void>
  swActivated: Ref<boolean>
  registrationError: Ref<boolean>
  offlineReady: Ref<boolean>
  needRefresh: Ref<boolean>
  updateServiceWorker: (reloadPage?: boolean | undefined) => Promise<void>
  cancelPrompt: () => Promise<void>
  getSWRegistration: () => ServiceWorkerRegistration | undefined
}

// declare module '#app' {
//   interface NuxtApp {
//     $pwa: UnwrapNestedRefs<PwaInjection>
//   }
// }

export default defineNuxtPlugin((nuxtApp) => {
  const isInstalled = ref(false)
  const isPWAInstalled = ref(false)
  const showInstallPrompt = ref(false)
  const swActivated = ref(false)
  const registrationError = ref(false)
  const offlineReady = ref(false)
  const needRefresh = ref(false)

  let pwa: UnwrapNestedRefs<PwaInjection>
  nuxtApp.provide('pwa2', {
    isInstalled,
    isPWAInstalled,
    showInstallPrompt,
    cancelInstall: () => {
      showInstallPrompt.value = false
    },
    install: async () => {
      if (pwa) {
        await pwa.install()
      }
    },
    swActivated,
    registrationError,
    offlineReady,
    needRefresh,
    updateServiceWorker: async (reloadPage?: boolean) => {
      if (pwa) {
        await pwa.updateServiceWorker(reloadPage)
      }
    },
    cancelPrompt: async () => {
      if (pwa) {
        await pwa.cancelPrompt()
      }
    },
    getSWRegistration: () => {
      if (pwa) {
        return pwa.getSWRegistration()
      }
    },

  })

})
