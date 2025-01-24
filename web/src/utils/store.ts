import { create } from "zustand";
import { CurrentUser, TableConfig, TableSummaryType } from "./data_types";

const defaultSnackbar = {
  showAlert: false,
  message: "",
  alertColor: "success" as "success" | "error" | "info" | "warning",
  loading: false,
};

type SnackBarOption = typeof defaultSnackbar;

type StoreType = {
  tables: TableSummaryType[];
  tableConfigs: TableConfig[];
  currentTableConfig: Partial<TableConfig>
  currentUser: CurrentUser;
  snackBar: SnackBarOption;
  openTableConfigDialog: boolean;
  /** 
   * We use `stateTrigger` to communicate between components and trigger certain
   * events when needed.
   */
  stateTrigger: number;
  updateTables: (tables: TableSummaryType[]) => void;
  addTableConfig: (config: TableConfig) => void;
  updateTableConfig: (config: TableConfig) => void;
  setCurrentTableConfig: (config: Partial<TableConfig>) => void
  showAlert: (alterType: "success" | "error", msg: string) => void;
  hideAlert: () => void;
  setOpenTableConfigDialog: (value: boolean) => void
  updateStateTrigger: () => void;

  /**
   * Reset states on logout
   * @returns 
   */
  logout: () => void;
};

export const userDefaults: CurrentUser = {
  name: "Guest User",
  isLogged: false,
  role: "Demo Account",
};

export const useStore = create<StoreType>((set, get) => ({
  tables: [],
  tableConfigs: [],
  currentTableConfig: {},
  currentUser: userDefaults,
  snackBar: defaultSnackbar,
  openTableConfigDialog: false,
  stateTrigger: 0,
  updateTables: (tables: TableSummaryType[]) => set({ tables }),
  addTableConfig: (config: TableConfig) => {
    const tableConfigs = get().tableConfigs;
    tableConfigs.push(config);

    set({ tableConfigs });
  },
  updateTableConfig(config) {
    const tableConfigs = get().tableConfigs;
    const cfg = tableConfigs.find((c) => c.name === config.name);

    if (cfg) {
      const i = tableConfigs.indexOf(cfg);
      tableConfigs.splice(i, 1, config);
      set({ tableConfigs });
    }
  },
  setCurrentTableConfig(config) {
    set({currentTableConfig: config})
  },
  showAlert(alterType, message) {
    const snackBar = {
      ...get().snackBar,
      showAlert: true,
      loading: false,
      alertColor: alterType,
      message,
    };

    set({ snackBar });
  },
  hideAlert() {
    const snackBar = {
      ...get().snackBar,
      showAlert: false,
    };
    set({ snackBar });
  },
  setOpenTableConfigDialog(value){
    set({ openTableConfigDialog: value })
  },
  updateStateTrigger() {
    const stateTrigger = get().stateTrigger + 1
    set({ stateTrigger })
  },
  logout() {
    set({
      currentUser: userDefaults,
      tables: [],
      snackBar: defaultSnackbar,
      tableConfigs: [],
    });
  },
}));
