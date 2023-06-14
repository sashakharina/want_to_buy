<template>
  <div class="home-page">
    <app-button
        class="home-page__add-folder"
        scheme="default"
        modification="default"
        :icon-left="$icons.folderAdd"
    />
    <div class="home-page__content">
      <home-folder />
      <home-folder />
      <home-folder />
      <home-folder />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { HomeFolder } from '@/pages/home-page'
import { AppButton } from '@/common'
</script>

<style lang="scss" scoped>
.home-page {
  padding: toRem(24);
}

.home-page__content {
  display: grid;
  grid-template-columns: repeat(3, minmax(toRem(100), toRem(300)));
  gap: toRem(20);
  align-items: center;
  justify-content: center;
}

.home-page__add-folder {
  padding: 0;
  margin-left: auto;
  margin-bottom: toRem(24);

  &:deep(.app-button__icon-left) {
    width: toRem(60);
    height: toRem(60);
    color: var(--primary-light);
  }
}
</style>
