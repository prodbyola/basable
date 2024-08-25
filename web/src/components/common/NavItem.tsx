import { ExpandLess, ExpandMore } from "@mui/icons-material";
import {
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Collapse,
  List,
} from "@mui/material";
import FiberManualRecordIcon from "@mui/icons-material/FiberManualRecord";
import * as React from "react";

const navItemStyle = {
  mt: "10px",
  color: "#363636",
  fontFamily: '"Exo", sans-serif',
  "&.Mui-selected": {
    border: `1px solid`,
    borderColor: "#4451CA",
    borderRadius: "10px",
    color: "#4451CA",
  },
};

interface InnerItem {
  id: number;
  title: string;
}

interface NavItemProps {
  label: string;
  icon: React.ReactNode;
  selected?: boolean;
  expandable?: boolean;
  onClick?: React.MouseEventHandler;
  expanded?: boolean;
  innerItems?: InnerItem[];
}

export const NavItem = ({
  label,
  icon,
  selected,
  expanded,
  expandable = false,
  onClick,
  innerItems = [],
}: NavItemProps) => {
  return (
    <>
      <ListItemButton sx={navItemStyle} onClick={onClick} selected={selected}>
        <ListItemIcon
          sx={{
            minWidth: "32px",
          }}
        >
          {icon}
        </ListItemIcon>
        <ListItemText
          primary={label}
          sx={{
            "& .MuiTypography-body1": {
              fontFamily: '"Exo", sans-serif',
              color: "#363636",
            },
          }}
        />
        {expandable && (expanded ? <ExpandLess /> : <ExpandMore />)}
      </ListItemButton>
      {expandable && (
        <Collapse in={expanded} timeout="auto" unmountOnExit>
          <List component="div">
            {innerItems.map((item) => (
              <ListItemButton
                key={item.id}
                sx={{
                  fontFamily: '"Exo", sans-serif',
                  pl: 4,
                  pt: 0,
                  pb: 0,
                }}
              >
                <ListItemIcon>
                  <FiberManualRecordIcon
                    sx={{
                      "&.MuiSvgIcon-root": {
                        width: "4px",
                        height: "4px",
                        fill: "#363636",
                      },
                      "MuiListItemIcon-root": {
                        minWidth: "18px",
                      },
                    }}
                  />
                </ListItemIcon>
                <ListItemText primary={item.title} />
              </ListItemButton>
            ))}
          </List>
        </Collapse>
      )}
    </>
  );
};
