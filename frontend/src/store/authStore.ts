import { create } from "zustand";
import { persist } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";
import { clearTokens, setTokens } from "@/services/api";
import type { TokenPair, User } from "@/types";

interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
}

interface AuthActions {
  login: (user: User, tokens: TokenPair) => void;
  logout: () => void;
  setUser: (user: User) => void;
  setLoading: (loading: boolean) => void;
}

export const useAuthStore = create<AuthState & AuthActions>()(
  persist(
    immer((set) => ({
      user: null,
      isAuthenticated: false,
      isLoading: false,

      login: (user, tokens) => {
        setTokens(tokens.access_token, tokens.refresh_token);
        set((state) => {
          state.user = user;
          state.isAuthenticated = true;
          state.isLoading = false;
        });
      },

      logout: () => {
        clearTokens();
        set((state) => {
          state.user = null;
          state.isAuthenticated = false;
        });
      },

      setUser: (user) => {
        set((state) => {
          state.user = user;
          state.isAuthenticated = true;
        });
      },

      setLoading: (loading) => {
        set((state) => {
          state.isLoading = loading;
        });
      },
    })),
    {
      name: "ajo-auth",
      partialize: (state) => ({ user: state.user, isAuthenticated: state.isAuthenticated }),
    }
  )
);
