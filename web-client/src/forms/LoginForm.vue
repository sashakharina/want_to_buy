<template>
  <form class="login-form" @submit.prevent="submit">
    <div
        v-if="currentStep === LOGIN_STEPS.register"
        class="login-form__field-wrapper"
    >
      <span class="login-form__input-desc">
        Логін:
      </span>
      <input-field
          class="login-form__input"
          v-model="form.login"
          placeholder="example@gmail.com"
          :error-message="getFieldErrorMessage('login')"
          :disabled="isFormDisabled"
          @blur="touchField('login')"
      />
    </div>
    <div class="login-form__field-wrapper">
      <span class="login-form__input-desc">
        Електронна пошта:
      </span>
      <input-field
          class="login-form__input"
          v-model="form.email"
          placeholder="example@gmail.com"
          :error-message="getFieldErrorMessage('email')"
          :disabled="isFormDisabled"
          @blur="touchField('email')"
      />
    </div>
    <div v-if="currentStep !== LOGIN_STEPS.email" class="login-form__field-wrapper">
      <span class="login-form__input-desc">
          Пароль:
      </span>
      <input-field
          class="login-form__input"
          v-model="form.password"
          type="password"
          placeholder="Password"
          :error-message="getFieldErrorMessage('password')"
          :disabled="isFormDisabled"
          @blur="touchField('password')"
      />
    </div>
    <template v-if="currentStep === LOGIN_STEPS.register">
      <div class="login-form__field-wrapper">
        <span class="login-form__input-desc">
            Номер Телефону:
        </span>
        <input-field
            v-model="form.phoneNumber"
            v-mask="'+## (###) ### ## ##'"
            class="login-form__input"
            placeholder="+38 (099) 111 11 11"
            :error-message="getFieldErrorMessage('phoneNumber')"
            :disabled="isFormDisabled"
            @blur="touchField('phoneNumber')"
        />
      </div>
      <div class="login-form__field-wrapper">
        <span class="login-form__input-desc">
            Дата Народження:
        </span>
        <input-field
            v-model="form.birthDate"
            v-mask="'##.##.####'"
            class="login-form__input"
            placeholder="01.12.1900"
            :error-message="getFieldErrorMessage('birthDate')"
            :disabled="isFormDisabled"
            @blur="touchField('birthDate')"
        />
      </div>
    </template>
    <div class="login-form__actions">
      <app-button
          v-if="currentStep !== LOGIN_STEPS.email"
          class="login-form__btn"
          text="Cancel"
          scheme="flat"
          :disabled="isFormDisabled"
          @click="cancel"
      />
      <app-button
        class="login-form__btn"
        text="Submit"
        type="submit"
        :disabled="isFormDisabled || !isFieldsValid"
      />
    </div>
  </form>
</template>

<script lang="ts" setup>
import { AppButton } from '@/common'
import { InputField } from '@/fields'

import {computed, reactive, ref} from 'vue'
import { Bus, ErrorHandler } from '@/helpers'
import { useI18n } from 'vue-i18n'
import { useForm, useFormValidation } from '@/composables'
import { email, required } from '@/validators'
import { HTTP_STATUS_CODES } from '@distributedlab/json-api-client'
import {requiredIf} from "@vuelidate/validators";
import {api} from "@/api";
import {useAuthStore} from "@/store";

enum LOGIN_STEPS {
  email = 'email',
  password = 'password',
  register = 'register'
}

const { createSession } = useAuthStore()

const { t } = useI18n({ useScope: 'global' })

const currentStep = ref(LOGIN_STEPS.email)

const form = reactive({
  email: '',
  password: '',
  login: '',
  phoneNumber: '',
  birthDate: ''
})

const { isFormDisabled, disableForm, enableForm } = useForm()

const { isFormValid, getFieldErrorMessage, touchField, cleanErrors, isFieldsValid } = useFormValidation(
  form,
  {
    email: { email, required },
    login: { required: requiredIf(() => currentStep.value === LOGIN_STEPS.register) },
    password: { required: requiredIf(() => currentStep.value !== LOGIN_STEPS.email) },
    phoneNumber: { required: requiredIf(() => currentStep.value === LOGIN_STEPS.register)  },
    birthDate: { required: requiredIf(() => currentStep.value === LOGIN_STEPS.register)  }
  },
)

const clearForm = () => {
  form.email = ''
  form.login= ''
  form.password= ''
  form.phoneNumber= ''
  form.birthDate = ''
  cleanErrors()
}

const cancel = () =>  {
  clearForm()
  currentStep.value = LOGIN_STEPS.email
}

const chooseStep = () => {
  try {
    currentStep.value = LOGIN_STEPS.register
  } catch (e) {
    if(e.code === HTTP_STATUS_CODES.NOT_FOUND) {
      currentStep.value = LOGIN_STEPS.register
    }
    ErrorHandler.processWithoutFeedback(e)
  }
  cleanErrors()
}

const submit = async () => {
  if (!isFormValid()) return

  if(currentStep.value === LOGIN_STEPS.email) {
    chooseStep()
    return
  }

  disableForm()
  try {
    await createSession(form.login, form.email, form.password)
    Bus.success(t('login-form.login-success-msg'))
  } catch (error) {
    ErrorHandler.process(error)
  }
  enableForm()
}
</script>

<style lang="scss" scoped>
.login-form {
  display: grid;
  grid-gap: toRem(6);
  padding: toRem(12);
  background: var(--background-secondary-light);
  border-radius: toRem(12);
}

.login-form__field-wrapper {
  display: grid;
  grid-template-columns: toRem(150) minmax(toRem(200), 1fr);
  gap: toRem(12);
  align-items: center;
  padding-bottom: toRem(20);
}

.login-form__input-desc {
  text-align: right;
}

.login-form__actions {
  display: flex;
  gap: toRem(12);
  align-items: center;
  height: toRem(42);
}

.login-form__btn {
  flex: 1;
  height: 100%;
}
</style>
