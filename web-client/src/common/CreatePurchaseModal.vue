<template>
  <modal
      :is-shown="isShown"
      is-close-by-click-outside
      @update:is-shown="emit('update:isShown', $event)"
  >
    <div class="create-purchase-modal__inner">
      <h3 class="create-purchase-modal__title">
        Нова покупка
      </h3>
      <div
          class="create-purchase-modal__field-wrapper"
      >
        <span class="create-purchase-modal__input-desc">
          Назва:
        </span>
        <input-field
            class="create-purchase-modal__input"
            v-model="form.name"
            :error-message="getFieldErrorMessage('name')"
            :disabled="isFormDisabled"
            @blur="touchField('name')"
        />
      </div>
      <div
          class="create-purchase-modal__field-wrapper"
      >
        <span class="create-purchase-modal__input-desc">
          Пріорітетність:
        </span>
        <select-field
            class="create-purchase-modal__input"
            v-model="form.priority"
            :value-options="purchasePriorities"
            :error-message="getFieldErrorMessage('priority')"
            :disabled="isFormDisabled"
            @blur="touchField('priority')"
        />
      </div>
      <div
          class="create-purchase-modal__field-wrapper"
      >
        <span class="create-purchase-modal__input-desc">
          Ціна:
        </span>
        <div class="create-purchase-modal__field-price">
          <input-field
              class="create-purchase-modal__input"
              v-model="form.price"
              :error-message="getFieldErrorMessage('price')"
              :disabled="isFormDisabled"
              @blur="touchField('price')"
          />
          <select-field
              class="create-purchase-modal__price-select"
              v-model="form.currency"
              :value-options="purchaseCurrencies"
              :error-message="getFieldErrorMessage('currency')"
              :disabled="isFormDisabled"
              @blur="touchField('currency')"
          />
        </div>
      </div>
      <div
          class="create-purchase-modal__field-wrapper"
      >
        <span class="create-purchase-modal__input-desc">
          Опис:
        </span>
        <textarea-field
            class="create-purchase-modal__input"
            v-model="form.description"
            :value-options="purchasePriorities"
            :error-message="getFieldErrorMessage('description')"
            :disabled="isFormDisabled"
            @blur="touchField('description')"
        />
      </div>
      <div
          class="create-purchase-modal__field-wrapper"
      >
        <span class="create-purchase-modal__input-desc">
          Посилання:
        </span>
        <input-field
            class="create-purchase-modal__input"
            v-model="form.link"
            :error-message="getFieldErrorMessage('link')"
            :disabled="isFormDisabled"
            @blur="touchField('link')"
        />
      </div>
      <div
          class="create-purchase-modal__field-wrapper"
      >
        <span class="create-purchase-modal__input-desc">
          Картинка:
        </span>
        <input-field
            class="create-purchase-modal__input"
            v-model="form.imageLink"
            :error-message="getFieldErrorMessage('imageLink')"
            :disabled="isFormDisabled"
            @blur="touchField('imageLink')"
        />
      </div>
      <img class="create-purchase-modal__image" v-if="form.imageLink" :src="form.imageLink" />
      <div class="create-purchase-modal__actions">
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
import { InputField, SelectField, TextareaField } from '@/fields'
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

const purchasePriorities = reactive([0, 1, 2, 3, 4, 5, 6, 7])
const purchaseCurrencies = reactive(['UAH', 'EUR', 'USD'])

const form = reactive({
  name: '',
  priority: purchasePriorities[0],
  price: '',
  currency: purchaseCurrencies[0],
  imageLink: '',
  description: '',
  link: '',
})

const { isFormDisabled} = useForm()

const { getFieldErrorMessage, touchField, isFieldsValid } = useFormValidation(
    form,
    {
      name: { required },
      priority: { required },
      price: {},
      currency: {},
      imageLink: {},
      description: {},
      link: {},
    },
)

const close = () => emit('update:isShown', false)

const submit = () => {
  close()
  emit('submit')
}
</script>

<style lang="scss" scoped>
.create-purchase-modal__inner {
  display: flex;
  flex-direction: column;
  gap: toRem(8);
  padding: toRem(24);
  border-radius: toRem(12);
  background: var(--background-quaternary-light);
}
.create-purchase-modal__field-wrapper {
  display: grid;
  grid-template-columns: toRem(150) minmax(toRem(200), 1fr);
  gap: toRem(12);
  align-items: center;
  padding-bottom: toRem(20);
}

.create-purchase-modal__input-desc {
  text-align: right;
}

.create-purchase-modal__title {
  margin-bottom: toRem(16);
  text-align: center;
}

.create-purchase-modal__actions {
  display: flex;
  gap: toRem(12);
}

.login-form__btn {
  flex: 1;
}

.create-purchase-modal__field-price {
  display: flex;
  gap: toRem(12);
}

.create-purchase-modal__price-select {
  width: toRem(50);
}

.create-purchase-modal__image {
  width: toRem(150);
  height: toRem(150);
  object-fit: contain;
  margin: 0 auto;
}
</style>
