<template>
  <div class="login-page">
    <div class="login-page__content">
      <h2 class="login-page__title">
        Авторизація
      </h2>
      <login-form class="login-page__form" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { LoginForm } from '@/forms'
</script>

<style lang="scss" scoped>
.login-page {
  width: 100vw;
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
}

.login-page__form {
  min-width: toRem(400);
}

.login-page__content {
  border-radius: toRem(12);
  background: #9a82db;
}

.login-page__title {
  padding: toRem(12);
  color: var(--text-primary-invert-main);
}
</style>
