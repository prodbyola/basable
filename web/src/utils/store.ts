import { create } from 'zustand'
import { CurrentUser, TableConfig, TableSummaryType } from './data_types'

type StoreType = {
    tables: TableSummaryType[]; 
    tableConfigs: TableConfig[];
    currentUser: CurrentUser,
    updateTables: (tables: TableSummaryType[]) => void;
    addTableConfig: (config: TableConfig) => void;
    logout: () => void
};

export const userDefaults: CurrentUser = {
  name: 'Guest User',
  isLogged: false,
  role: 'Demo Account'
}

export const useStore = create<StoreType>((set, get) => ({
    tables: [],
    tableConfigs: [],
    currentUser: userDefaults,
    updateTables: (tables: TableSummaryType[]) => set({ tables }),
    addTableConfig: (config: TableConfig) => {
      const tableConfigs = get().tableConfigs
      tableConfigs.push(config)
      
      set({ tableConfigs })
    },
    logout() {
      set({currentUser: userDefaults})
    },
}))