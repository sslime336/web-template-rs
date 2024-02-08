import { create } from 'zustand'

interface UserState {
    username: string
}

const useUserStore = create<UserState>()((set) => ({
    username: "Unknown"
}))