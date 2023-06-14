<template>
  <div class="folder-item">
    <div class="folder-item__image-wrapper">
      <img
          class="folder-item__img"
          alt="Folder item"
          src="https://shorturl.at/qIL14"
      >
    </div>
    <div class="folder-item__price">
      <span class="folder-item__item-name">
        Сінтезатор
      </span>
      <span class="folder-item__price-text">
        2000 UAH
      </span>
    </div>
    <div class="folder-item__hearts">
      <icon
          v-for="(item, idx) in hearts"
          :key="idx"
          class="folder-item__hearts-icon"
          :name="$icons.heart"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Icon } from '@/common'
const HEARTS_COUNT = 7
const hearts = Array(HEARTS_COUNT).fill(0)
</script>

<style lang="scss" scoped>
.folder-item {
  border-radius: toRem(12);
  background: var(--background-quaternary-light);
  padding: toRem(12);
}

.folder-item__image-wrapper {
  max-width: 100%;
  max-height: toRem(250);
}

.folder-item__img {
  width: toRem(376);
  height: toRem(250);
  object-fit: cover;
  margin: 0 auto;
  border-radius: toRem(12);
}

.folder-item__price {
  display: flex;
  justify-content: space-between;
  padding: toRem(12) 0;
}

.folder-item__item-name {
  font-size: toRem(20);
  color: var(--primary-main);
  font-weight: 900;
}

.folder-item__price-text {
  font-size: toRem(16);
  color: var(--success-dark);
  font-weight: 900;
}

.folder-item__hearts {
  display: flex;
  gap: toRem(6);
}

.folder-item__hearts-icon {
  width: toRem(20);
  height: toRem(20);
  color: var(--primary-dark);
}
</style>
