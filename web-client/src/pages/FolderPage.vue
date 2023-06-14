<template>
  <div class="folder-page">
    <div class="folder-page__title-wrapper">
      <div class="folder-page__title-content">
        <span class="folder-page__title">
          Музичне
        </span>
        <app-button
            class="folder-page__title-type-button"
            :icon-left="$icons.link"
        />
      </div>
      <app-button
          class="folder-page__add-button"
          :icon-left="$icons.xCircle"
      />
    </div>
    <div class="folder-page__content">
      <folder-item />
      <folder-item />
      <folder-item />
      <folder-item />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { AppButton } from '@/common'
import { FolderItem } from '@/pages/folder-page'
</script>

<style lang="scss" scoped>
.folder-page {
  padding: 0;
}

.folder-page__title-wrapper {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: toRem(12) toRem(24);
  background: var(--background-quaternary-light);
}

.folder-page__title-content {
  display: flex;
  gap: toRem(12);
  align-items: center;
}

.folder-page__title {
  font-size: toRem(24);
  color: var(--primary-main);
}

.folder-page__add-button {
  padding: 0;

  &:deep(.app-button__icon-left) {
    transform: rotate(45deg);
    width: toRem(30);
    height: toRem(30);
  }
}

.folder-page__title-type-button {
  &:deep(.app-button__icon-left) {
    width: toRem(20);
    height: toRem(20);
  }
}

.folder-page__content {
  display: grid;
  grid-template-columns: repeat(3, toRem(400));
  gap: toRem(12);
  align-items: center;
  justify-content: center;
  margin-top: toRem(24);
}
</style>
