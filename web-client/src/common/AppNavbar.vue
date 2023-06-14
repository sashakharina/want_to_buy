<template>
  <div class="app-navbar">
    <div class="app-navbar__user-photo-wrp">
      <span class="app-navbar__desc">
      {{ $config.APP_NAME }}
    </span>
      <icon
          class="app-navbar__user-img"
          :name="$icons.camera"
      />
    </div>
    <div class="app-navbar__user-links">
      <router-link
          class="app-navbar__user-link"
          :to="{ name: $routes.homePage }"
      >
        Домашня
      </router-link>
      <router-link
          class="app-navbar__user-link"
          :to="{ name: $routes.loginPage }"
      >
        Мій профіль
      </router-link>
      <router-link
          class="app-navbar__user-link"
          :to="{ name: $routes.loginPage }"
      >
        Контакти
      </router-link>
      <app-button
          class="app-navbar__logout-button"
          text="Вийти"
          scheme="default"
          modification="default"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { AppLogo, Icon, AppButton } from '@/common'

</script>

<style lang="scss" scoped>
.app-navbar {
  display: flex;
  align-items: center;
  padding: 0 var(--app-padding-left);
  background: var(--background-tetriatry-main);
  border-bottom: var(--border-primary-main);
  justify-content: space-between;

  @include respond-to(tablet) {
    flex-wrap: wrap;
  }
}

.app-navbar__logo {
  @include respond-to(xsmall) {
    width: 100%;
    margin-bottom: toRem(24);
  }
}

.app-navbar__desc {
  color: var(--text-primary-invert-light);
}

.app-navbar__user-photo-wrp {
  display: flex;
  gap: toRem(12);
  max-height: 100%;
  align-items: center;
}

.app-navbar__user-img {
  max-width: toRem(40);
  max-height: toRem(40);
}

.app-navbar__user-links {
  display: flex;
  gap: toRem(10);
  align-items: center;
}

.app-navbar__user-link {
  display: flex;
  height: toRem(65);
  padding: toRem(12);
  align-items: center;
  justify-content: center;
  color: var(--text-primary-invert-main);
  transition: background-color 0.2s ease-in-out;

  &:hover {
    background: var(--app-bg);
  }
}

.app-navbar__logout-button {
  padding: toRem(12);
  height: toRem(65);
  font-size: toRem(16);
  color: var(--text-primary-invert-main);
  transition-property: color, background-color;
  transition: 0.2s ease-in-out;

  &:not([disabled]):hover,
  &:not([disabled]):focus,
  &:not([disabled]):active {
    color: var(--text-primary-invert-main);
    background-color: var(--app-bg);
  }
}
</style>
