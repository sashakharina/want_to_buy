import {defineStore} from 'pinia'
import {api} from '@/api'
import {AuthorizeUserResponse, Jwt} from '@/types'
import {router} from '@/router'
import {REQUEST_METHODS, ROUTE_NAMES} from '@/enums'
import {memorizedJwtRefresh, parseJwt} from '@/helpers'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    accessToken: null as Jwt | null,
    isLoggedIn: false,
  }),
  persist: true,
  actions: {
    async createSession(nickname: string, email: string, password: string): Promise<void> {
      const { data } = await api.post('/users',{
        Nickname: nickname,
        Email: email,
        Password: password
      },)
      this.accessToken = parseJwt(data.access)
      this.isLoggedIn = true
    },

    async logout(): Promise<void> {
      if (this.isLoggedIn) {
        await api.post('/integrations/auth/logout', {})
        this.accessToken = null
        this.isLoggedIn = false
        await router.push({ name: ROUTE_NAMES.loginPage })
      }
    },

    async restoreSession(): Promise<void> {
      const { access } = await memorizedJwtRefresh()
      this.accessToken = parseJwt(access)
    },
  },
  getters: {
    currentUserId: state => state.accessToken?.userId || null,
  },
})
