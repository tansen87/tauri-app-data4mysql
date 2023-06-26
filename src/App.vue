<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import data4mysql from "./components/data4mysql.vue";
</script>

<template>
  <div class="container">
    <div class="row">
      <a href="https://github.com/tansen87/tauri-app-data4mysql" target="_blank">
        <img src="./assets/github.svg" class="logo github" alt="Github logo" />
      </a>
    </div>

    <p>Click on the icon to view the source code</p>
    <hr/>
    <data4mysql />

  </div>
</template>
