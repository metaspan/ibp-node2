//import type { NuxtConfig } from '@nuxt/types'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  // TODO: this needs to be true for production,
  // but there is a problem with the @polkadot/dapp import as it does not support missing window object
  ssr: false,
  plugins: [
    '@/plugins/vuetify',
  ],
  css: [
    'vuetify/lib/styles/main.sass',
    '@mdi/font/css/materialdesignicons.min.css',
  ],
  modules: [
    '@pinia/nuxt',
    '@nuxtjs/plausible',
    // // '@nuxt/typescript-build',
    // // '@nuxtjs/pwa',
    '@vite-pwa/nuxt',
  ],
  plausible: {
    domain: 'ibp-node.metaspan.io',
    hashMode: true,
    trackLocalhost: true,
    apiHost: 'https://click.metaspan.io',
  },
  build: {
    transpile: [
      'vuetify',
      // 'vuex-module-decorators',
    ],
  },
  nitro: {
    esbuild: {
      options: {
        // https://github.com/nuxt/nuxt/issues/14348 # support bigint
        target: 'esnext',    
      }
    },
    prerender: {
      crawlLinks: true,
      routes: ['/sitemap.xml', '/robots.txt']
    }
  },
  devServer: {
    port: 8080,
    host: '0.0.0.0',
  },
  devtools: { enabled: true },
  // buildModules: ['@nuxt/typescript-build'],
  // app: {
  //   pageTransition: { name: 'page', mode: 'out-in' }
  // },
})
