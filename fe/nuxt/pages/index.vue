<template>
  <client-only>
  <v-container>
    <v-card class="my-card">
      <v-card-title>metaspan.io - tools</v-card-title>
      <v-card-text>
        <p>Here you will find various tools and utilities for blockchain-related applications.</p>
        <p>The tools are open source, and the code can be found on github, <a href="https://github.com/metaspan" target="_blank">here</a></p>
      </v-card-text>
    </v-card>

    <!-- <v-card class="my-card" elevation=1>
      <v-card-title>Featured dApp: metaspan.io - kusama 1kv</v-card-title>
      <v-card-text>
        <p>Kusama 1KV (1000 validators)</p>
        <p>
          <router-link to="/kusama/candidate">Candidates</router-link>
        </p>
      </v-card-text>
    </v-card> -->

    <!-- <v-card class="my-card" elevation=1>
      <v-card-title>Featured dApp: metaspan.io - met🌐pay</v-card-title>
      <v-card-text>
        <p>Pay with DOT or KSM</p>
        <p>
          <a href="https://metaspan.com/#metapay" target="_blank">met🌐pay</a> <v-icon small class="mdi">mdi-open-in-new</v-icon>
        </p>
      </v-card-text>
    </v-card> -->

    <!-- <v-card class="my-card" elevation=1>
      <v-card-title>Featured: metaspan.io - graphql endpoint for Kusama and Polkadot</v-card-title>
      <v-card-text>
        <p>Blending data from API and on-chain (realtime)</p>
        <p>
          <a href="https://gql.metaspan.io/graphql" target="_blank">try it here</a> <v-icon small class="mdi">mdi-open-in-new</v-icon>
        </p>
      </v-card-text>
    </v-card> -->

    <!-- <v-card class="my-card" elevation=1>
      <v-card-title>Featured: Prometheus monitoring for Kusama and Polkadot 1kV Candidates</v-card-title>
      <v-card-text>
        <p>Source code here: <br>
          <a href="https://github.com/metaspan/kusama-1kv-prometheus-exporter" target="_blank">https://github.com/metaspan/kusama-1kv-prometheus-exporter</a> <v-icon small class="mdi">mdi-open-in-new</v-icon> <br>
          <a href="https://github.com/metaspan/polkadot-1kv-prometheus-exporter" target="_blank"> https://github.com/metaspan/polkadot-1kv-prometheus-exporter</a> <v-icon small class="mdi">mdi-open-in-new</v-icon>
        </p>
        <p>
          Point your prometheus.yml file as follows:
          <pre>
scrape_configs:
  scrape_interval: 60s
  - job_name: "dot-1kv"
    metrics_path: "/metrics/16ce9zrmiuAtdi9qv1tuiQ1RC1xR6y6NgnBcRtMoQeAobqpZ"
    static_configs:
    - targets: ["dot-1kv-prometheus.metaspan.io"]
          </pre>
        </p>
      </v-card-text>
    </v-card> -->

    <!-- <v-card class="my-card" elevation="2">
      <v-card-title>API Endpoints</v-card-title>
      <v-card-text>
        <p>We provide the following endpoints (data is a cache of upstream, refreshed each 30 mins):</p>
        <table width="70%">
          <thead>
            <th>API</th>
            <th>Upstream</th>
          </thead>
          <tbody>
            <tr>
              <td><a href="https://gql.metaspan.io/graphql" target="_blank">https://gql.metaspan.io/graphql</a> <v-icon small class="mdi">mdi-open-in-new</v-icon></td>
              <td>combination of metaspan api and on-chain</td>
            </tr>
            <tr>
              <td>https://api.metaspan.io/api/{chain}/candidate</td>
              <td>https://{chain}.w3f.community/candidates</td>
            </tr>
            <tr>
              <td>https://api.metaspan.io/api/{chain}/candidate/{stash}</td>
              <td>https://{chain}.w3f.community/candidate/{stash}</td>
            </tr>

            <tr>
              <td>https://api.metaspan.io/api/{chain}/score</td>
              <td>https://{chain}.w3f.community/score</td>
            </tr>
            <tr>
              <td>https://api.metaspan.io/api/{chain}/score/{stash}</td>
              <td>https://{chain}.w3f.community/score/{stash}</td>
            </tr>

            <tr>
              <td>https://api.metaspan.io/api/{chain}/validator</td>
              <td>https://{chain}.w3f.community/validator</td>
            </tr>
            <tr>
              <td>https://api.metaspan.io/api/{chain}/validator/{stash}</td>
              <td>https://{chain}.w3f.community/validator/{stash}</td>
            </tr>

            <tr>
              <td>https://api.metaspan.io/api/{chain}/nominator</td>
              <td>https://{chain}.w3f.community/nominator</td>
            </tr>
            <tr>
              <td>https://api.metaspan.io/api/{chain}/nominator/{accountId}</td>
              <td>https://{chain}.w3f.community/nominator/{accountId}</td>
            </tr>

            <tr>
              <td>https://api.metaspan.io/api/{chain}/locationStats[?name=Your Location]</td>
              <td>https://{chain}.w3f.community/locationStats</td>
            </tr>
            <tr>
              <td>https://api.metaspan.io/api/{chain}/locationStats/{locationId}<br>
                <i><small>Warning: the locationId could be misleading. Please note there are multiple records for each location "name", recording changes over time</small></i></td>
              <td>https://{chain}.w3f.community/locationStats/{locationId}</td>
            </tr>
          </tbody>

        </table>
      </v-card-text>
    </v-card> -->

  </v-container>
  </client-only>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
// import HelloWorld from '@/components/HelloWorld.vue' // @ is an alias to /src

export default defineComponent({})
</script>

<style scoped>
.my-card {
  margin: 5px;
}
</style>
