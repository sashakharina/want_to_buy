<template>
  <router-link
      class="home-folder"
      :to="{ name: $routes.folderPage, params: { id: 1 } }"
  >
    <span class="home-folder__title">
      Музичне
    </span>
    <icon :name="$icons.folder" />
    <div class="home-folder__actions">
      <app-button :icon-left="$icons.lockClosed" />
      <app-button :icon-left="$icons.trash" @click.prevent.stop="emit('delete-folder')" />
    </div>
  </router-link>
</template>

<script lang="ts" setup>
import { Icon, AppButton } from '@/common'

const emit = defineEmits<{
  (e: 'delete-folder'): void
}>()
</script>

<style lang="scss" scoped>
.home-folder {
  width: 100%;
  display: flex;
  flex-direction: column;
  border-radius: toRem(12);
  background: var(--background-primary-main);
  padding: toRem(12);
  color: var(--text-primary-main);
  background: var(--background-quaternary-light);

  &:not([disabled]):hover,
  &:not([disabled]):focus,
  &:not([disabled]):active {
    background: var(--background-quaternary-light);
  }
}

.home-folder__actions {
  display: flex;
  justify-content: flex-end;
  gap: toRem(12);
  margin-left: auto;
}
</style>
