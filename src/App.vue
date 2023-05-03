<template>
  <div class="container">
    <router-view/>
  </div>
</template>

<script lang="ts">
import { move_window, Position } from 'tauri-plugin-positioner-api'

move_window(Position.TopRight)

export default {
  async created () {
    this.$router.isReady().then(() => {
      this.$router.replace({
        name: 'dayMemo'
      });
    });

    move_window(Position.TopRight)
  },
};
</script>
