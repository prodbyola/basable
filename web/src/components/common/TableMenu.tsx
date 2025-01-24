import {
  Divider,
  ListItemIcon,
  ListItemText,
  Menu,
  MenuItem,
} from "@mui/material";
import { NavSubmenu, useNetworkRequest, useStore } from "../../utils";
import { Delete, DeleteForever, Settings } from "@mui/icons-material";
import { useState } from "react";
import AlertDialog from "../AlertDialog";
import { useNavigate } from "react-router-dom";

type TableMenuProps = {
  open: boolean;
  item?: NavSubmenu;
  anchorEl: HTMLDivElement | null;
  onClose: () => void;
};

const TableMenu = ({ open, anchorEl, item, onClose }: TableMenuProps) => {
  const request = useNetworkRequest();
  const navigate = useNavigate();

  const tableConfigs = useStore((state) => state.tableConfigs);
  const setOpenTableConfig = useStore(
    (state) => state.setOpenTableConfigDialog
  );
  const setTableConfig = useStore((state) => state.setCurrentTableConfig);
  const updateTableConfigs = useStore((state) => state.updateTableConfigs);

  const showAlert = useStore((state) => state.showAlert);
  const updateStateTrigger = useStore((state) => state.updateStateTrigger);

  const [showClearDialog, setShowClearDialog] = useState(false);
  const [showDropDialog, setShowDropDialog] = useState(false);

  const clearTable = async () => {
    try {
      await request({
        method: "delete",
        path: "tables/data/clear/" + item?.value,
      });

      showAlert("success", "Table cleared successfully");
      updateStateTrigger();
    } catch (err: any) {
      showAlert("error", err.message);
    }
  };

  const dropTable = async () => {
    try {
      await request({
        method: "delete",
        path: "tables/drop/" + item?.value,
      });

      showAlert("success", "Table dropped successfully");

      // remove table
      const table = tableConfigs.find((c) => c.name === item?.value);
      if (table) {
        const i = tableConfigs.indexOf(table);
        tableConfigs.splice(i, 1);
        updateTableConfigs([...tableConfigs]);
      }

      // navigate to first available table
      if (tableConfigs.length) {
        const f = tableConfigs[0];
        const p = "/dashboard/tables/" + f.name;
        navigate(p);
      } else {
        navigate("/dashboard");
      }
    } catch (err: any) {
      showAlert("error", err.message);
    }
  };

  return (
    <>
      <Menu
        id="basable-table-menu"
        open={open}
        anchorEl={anchorEl}
        onClose={onClose}
      >
        <MenuItem
          onClick={() => {
            onClose();
            setShowClearDialog(true);
          }}
        >
          <ListItemIcon>
            <Delete fontSize="small" />
          </ListItemIcon>
          <ListItemText>Clear</ListItemText>
        </MenuItem>
        <Divider />
        <MenuItem
          onClick={() => {
            onClose();
            setShowDropDialog(true);
          }}
        >
          <ListItemIcon>
            <DeleteForever fontSize="small" />
          </ListItemIcon>
          <ListItemText>Drop</ListItemText>
        </MenuItem>
        <Divider />
        <MenuItem
          onClick={() => {
            onClose();
            const tc = tableConfigs.find((tc) => tc.name === item?.value);
            if (tc) setTableConfig(tc);

            setOpenTableConfig(true);
          }}
        >
          <ListItemIcon>
            <Settings fontSize="small" />
          </ListItemIcon>
          <ListItemText>Settings</ListItemText>
        </MenuItem>
      </Menu>
      <AlertDialog
        open={showClearDialog}
        title="Clear Table"
        content={`WARNING! This action will clear and delete all data from '${item?.label}' table. The action cannot be undone.`}
        actionText="Proceed"
        onHideDialog={() => setShowClearDialog(false)}
        onProceed={clearTable}
      />
      <AlertDialog
        open={showDropDialog}
        title="Drop Table"
        content={`WARNING! This action will delete '${item?.label}' table. The action cannot be undone.`}
        actionText="Proceed"
        onHideDialog={() => setShowDropDialog(false)}
        onProceed={dropTable}
      />
    </>
  );
};

export default TableMenu;
