import { create } from "zustand";
import { immer } from "zustand/middleware/immer";
import type { Group } from "@/types";

interface GroupState {
  selectedGroup: Group | null;
  filters: {
    status: string | null;
    search: string;
  };
}

interface GroupActions {
  selectGroup: (group: Group | null) => void;
  setStatusFilter: (status: string | null) => void;
  setSearchFilter: (search: string) => void;
  resetFilters: () => void;
}

export const useGroupStore = create<GroupState & GroupActions>()(
  immer((set) => ({
    selectedGroup: null,
    filters: {
      status: null,
      search: "",
    },

    selectGroup: (group) => {
      set((state) => {
        state.selectedGroup = group;
      });
    },

    setStatusFilter: (status) => {
      set((state) => {
        state.filters.status = status;
      });
    },

    setSearchFilter: (search) => {
      set((state) => {
        state.filters.search = search;
      });
    },

    resetFilters: () => {
      set((state) => {
        state.filters.status = null;
        state.filters.search = "";
      });
    },
  }))
);
