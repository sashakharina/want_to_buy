<template>
  <div class="loader">
    <spinner />
  </div>
</template>

<script lang="ts" setup>
/* create skeletons */
import { Spinner } from '@/common/loaders'
</script>

<style lang="scss" scoped>
.loader {
  overflow: hidden;
  display: grid;
  place-items: center;
}
</style>
