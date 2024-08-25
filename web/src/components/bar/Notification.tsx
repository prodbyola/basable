import { Box, IconButton, SvgIcon } from "@mui/material";
import * as React from "react";

export const AppNotification = () => {
  return (
    <Box sx={{ width: "28px", height: "28px", position: "relative" }}>
      <IconButton
        sx={{
          width: "28px",
          height: "28px",
          position: "absolute",
          top: "0",
          left: "0",
          background: '#F7F7F7'
        }}
        aria-label="notification"
      >
        <SvgIcon sx={{ width: "24px", height: "24px" }}>
          <svg
            width="20"
            height="20"
            viewBox="0 0 20 20"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M15.6243 8.09139V7.50422C15.6243 4.28027 13.1062 1.66675 10 1.66675C6.8938 1.66675 4.37573 4.28027 4.37573 7.50422V8.09139C4.37573 8.79605 4.17476 9.48494 3.79817 10.0712L2.8753 11.508C2.03235 12.8204 2.67587 14.6042 4.14197 15.0192C7.97728 16.1048 12.0227 16.1048 15.858 15.0192C17.3241 14.6042 17.9676 12.8204 17.1247 11.508L16.2018 10.0712C15.8252 9.48494 15.6243 8.79605 15.6243 8.09139Z"
              stroke="#0A0A0A"
              stroke-width="1.5"
            />
            <path
              d="M6.25 15.8333C6.79586 17.2897 8.26871 18.3333 10 18.3333C11.7313 18.3333 13.2041 17.2897 13.75 15.8333"
              stroke="#0A0A0A"
              stroke-width="1.5"
              stroke-linecap="round"
            />
          </svg>
        </SvgIcon>
      </IconButton>
      <SvgIcon
        sx={{
          height: "6px",
          width: "6px",
          position: "absolute",
          top: "2px",
          right: "6px",
        }}
      >
        <svg
          width="6"
          height="6"
          viewBox="0 0 6 6"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <rect width="6" height="6" rx="3" fill="#F90505" />
        </svg>
      </SvgIcon>
    </Box>
  );
};
