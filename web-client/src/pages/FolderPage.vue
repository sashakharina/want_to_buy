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
      <div class="folder-page__title-actions">
        <div class="folder-page__sorting-wrapper">
          <span class="folder-page__sorting-text">
            Сортувати за:
          </span>
          <select-field
            v-model="sortingValue"
            class="folder-page__title-select"
            :value-options="sortingValues"
          />
        </div>
        <app-button
            class="folder-page__add-button"
            :icon-left="$icons.xCircle"
            @click="openModal"
        />
      </div>
    </div>
    <div class="folder-page__content">
      <div v-for="(item, idx) in folders" :key="idx" class="folder-page__folder-item-wrapper">
        <folder-item class="folder-page__folder-item"/>
        <div class="folder-page__folder-item-plug">
          <app-button v-if="item.isChecked" class="folder-page__folder-item-plug-icon" scheme="default" modification="default" size="default" :icon-left="$icons.checkCircle" @click="toggleCheck(idx)"/>
          <app-button v-else class="folder-page__folder-item-plug-icon folder-page__folder-item-plug-icon--not-checked" scheme="default" modification="default" size="default"  @click="toggleCheck(idx)" />
          <app-button class="folder-page__folder-item-plug-icon" scheme="default" modification="default" size="default" :icon-left="$icons.clipboardCopy" />
          <app-button class="folder-page__folder-item-plug-icon" scheme="default" modification="default" size="default" :icon-left="$icons.trash" @click="decreaseFoldersAmount(idx)"/>
        </div>
      </div>
    </div>
    <create-purchase-modal v-model:is-shown="isModalOpened" @submit="increaseFoldersCount" />
  </div>
</template>

<script lang="ts" setup>
import {computed, reactive, ref} from 'vue'
import { AppButton, CreatePurchaseModal, Icon } from '@/common'
import { FolderItem } from '@/pages/folder-page'
import { SelectField } from '@/fields'
import {toReactive} from "@vueuse/core";

const foldersCount = ref(4)

const sortingValues = reactive(['пріорітетністю', 'ціною', 'датою'])

const sortingValue = ref(sortingValues[0])

const isModalOpened = ref(false)

const folders = ref([{ isChecked: false }, { isChecked: false }, { isChecked: false }, { isChecked: false }])

const openModal = () => {
  isModalOpened.value = true
}

const decreaseFoldersAmount = (id: number) => {
  folders.value = folders.value.filter((_, idx) => idx !== id)
}

const increaseFoldersCount = () => {
  folders.value.push({ isChecked: false })
}

const toggleCheck = (idx: number) => {
  folders.value[idx].isChecked = !folders.value[idx].isChecked
}
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
  margin: toRem(24) 0;
}

.folder-page__folder-item-wrapper {
  position: relative;
  padding-right: toRem(40);
}

.folder-page__folder-item {
  position: relative;
  z-index: 1;
}

.folder-page__folder-item-plug {
  display: flex;
  flex-direction: column;
  gap: toRem(10);
  padding: toRem(10);
  padding-left: toRem(20);
  z-index: 0;
  background: var(--text-primary-light);
  position: absolute;
  top: 0;
  right: 0;
  border-radius: 0 toRem(10) toRem(10) 0;
}

.folder-page__folder-item-plug-icon {
  max-width: toRem(20);
  max-height: toRem(20);
  color: var(--text-primary-invert-main);

  &--not-checked {
    width: toRem(19.2);
    height: toRem(19.2);
    border-radius: toRem(5);
    border: toRem(1.5) solid var(--text-primary-invert-main);

    &:not([disabled]):hover,
    &:not([disabled]):focus,
    &:not([disabled]):active {
      border: toRem(1.5) solid var(--text-primary-invert-main);
    }
  }

  &:not([disabled]):hover,
  &:not([disabled]):focus,
  &:not([disabled]):active {
    color: var(--text-primary-invert-main);
  }
}

.folder-page__sorting-wrapper {
  display: flex;
  gap: toRem(6);
  align-items: center;
}

.folder-page__title-actions {
  display: flex;
  gap: toRem(24);
}

.folder-page__title-select {
  width: toRem(150);
}
</style>
