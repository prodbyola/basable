import { Divider, Menu, MenuItem } from "@mui/material";
import { DownloadFormat, downloadFormats } from "../../utils";

export type DownloadMenuProps = {
  anchorEl: null | HTMLElement;
  onClose: (format: DownloadFormat | undefined) => void;
};

const DownloadMenu = ({ anchorEl, onClose }: DownloadMenuProps) => {
  const open = Boolean(anchorEl);

  return (
    <Menu
      id="basic-menu"
      anchorEl={anchorEl}
      open={open}
      onClose={() => onClose(undefined)}
      MenuListProps={{
        "aria-labelledby": "basic-button",
      }}
    >
      {downloadFormats.map((format) => (
        <div key={format}>
          <MenuItem onClick={() => onClose(format)} sx={{ width: '120px' }}>
            {format}
          </MenuItem>
          <Divider />
        </div>
      ))}
    </Menu>
  );
};

export default DownloadMenu;
