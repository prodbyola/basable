import { SvgIcon } from "@mui/material";
import * as React from "react";

export const TablesIcon = ({ color = "#363636" }) => {
  return (
    <SvgIcon>
      <svg
        width="20"
        height="16"
        viewBox="0 0 20 16"
        fill={color}
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M4 0H3C1.34315 0 0 1.34315 0 3V13C0 14.6569 1.34315 16 3 16H4V0Z"
          fill={color}
        />
        <path
          d="M6 16H17C18.6569 16 20 14.6569 20 13V12H6V16Z"
          fill={color}
        />
        <path d="M20 10V6H6V10H20Z" fill={color} />
        <path d="M20 4V3C20 1.34315 18.6569 0 17 0H6V4H20Z" fill={color} />
      </svg>
    </SvgIcon>
  );
};
