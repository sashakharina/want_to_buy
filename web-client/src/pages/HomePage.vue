<template>
  <div class="home-page">
    <app-button
        class="home-page__add-folder"
        scheme="default"
        modification="default"
        :icon-left="$icons.folderAdd"
        @click="openCreateFolderModal"
    />
    <div class="home-page__content">
      <home-folder v-for="(folder, idx) in folders" :key="idx" @delete-folder="decreaseFoldersAmount"/>
    </div>
    <create-folder-modal v-model:is-shown="isCreateFolderModalOpened" @submit="increaseFoldersCount"/>
  </div>
</template>

<script lang="ts" setup>
import {computed, ref} from 'vue'
import { HomeFolder } from '@/pages/home-page'
import { AppButton, CreateFolderModal } from '@/common'

const foldersCount = ref(4)

const isCreateFolderModalOpened = ref(false)

const folders = computed(() => Array(foldersCount.value).fill(0))

const openCreateFolderModal = () => {
  isCreateFolderModalOpened.value = true
}

const decreaseFoldersAmount = () => {
  foldersCount.value -= 1
}

const increaseFoldersCount = () => {
  foldersCount.value += 1
}
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
