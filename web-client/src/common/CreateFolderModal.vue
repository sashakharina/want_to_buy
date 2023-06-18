<template>
  <modal
      :is-shown="isShown"
      is-close-by-click-outside
      @update:is-shown="emit('update:isShown', $event)"
  >
    <div class="create-folder-modal__inner">
      <h3 class="create-folder-modal__title">
        Створення списку
      </h3>
      <div
          class="create-folder-modal__field-wrapper"
      >
        <span class="create-folder-modal__input-desc">
          Назва:
        </span>
        <input-field
            class="create-folder-modal__input"
            v-model="form.name"
            :error-message="getFieldErrorMessage('name')"
            :disabled="isFormDisabled"
            @blur="touchField('name')"
        />
      </div>
        <div
            class="create-folder-modal__field-wrapper"
        >
        <span class="create-folder-modal__input-desc">
          Рівень доступу:
        </span>
          <select-field
              class="create-folder-modal__input"
              v-model="form.permissionLevel"
              :value-options="folderPermissions"
              :error-message="getFieldErrorMessage('permissionLevel')"
              :disabled="isFormDisabled"
              @blur="touchField('permissionLevel')"
          />
        </div>
      <div class="create-folder-modal__actions">
        <app-button
            class="login-form__btn"
            text="Cancel"
            scheme="flat"
            :disabled="isFormDisabled"
            @click="close"
        />
        <app-button
            class="login-form__btn"
            text="Submit"
            :disabled="isFormDisabled || !isFieldsValid"
            @click="submit"
        />
      </div>
    </div>
  </modal>
</template>

<script lang="ts" setup>
import { reactive } from 'vue'
import { InputField, SelectField } from '@/fields'
import { Modal, AppButton } from '@/common'
import {useForm, useFormValidation} from "@/composables";
import {email, required} from "@/validators";
import {requiredIf} from "@vuelidate/validators";

defineProps<{
  isShown: boolean
}>()

const emit = defineEmits<{
  (e: 'update:isShown', value: boolean): void
  (e: 'submit'): void
}>()

const folderPermissions = reactive(['Приватний', 'Публічний', 'За посиланням'])

const form = reactive({
  name: '',
  permissionLevel: folderPermissions[0],
})

const { isFormDisabled} = useForm()

const { getFieldErrorMessage, touchField, isFieldsValid } = useFormValidation(
    form,
    {
      name: { required },
      permissionLevel: { required }
    },
)

const close = () => emit('update:isShown', false)

const submit = () => {
  close()
  emit('submit')
}
</script>

<style lang="scss" scoped>
.create-folder-modal__inner {
  display: flex;
  flex-direction: column;
  gap: toRem(8);
  padding: toRem(24);
  border-radius: toRem(12);
  background: var(--background-quaternary-light);
}
.create-folder-modal__field-wrapper {
  display: grid;
  grid-template-columns: toRem(150) minmax(toRem(200), 1fr);
  gap: toRem(12);
  align-items: center;
  padding-bottom: toRem(20);
}

.create-folder-modal__input-desc {
  text-align: right;
}

.create-folder-modal__title {
  margin-bottom: toRem(16);
  text-align: center;
}

.create-folder-modal__actions {
  display: flex;
  gap: toRem(12);
}

.login-form__btn {
  flex: 1;
}
</style>
