import { SvgIcon } from "@mui/material";
import * as React from "react";

export const HelpIcon = ({ color = "#363636" }) => {
  return (
    <SvgIcon>
      <svg
        width="20"
        height="20"
        viewBox="0 0 20 20"
        fill={color}
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          fillRule="evenodd"
          clipRule="evenodd"
          d="M10 20C15.5227 20 20 15.5227 20 10C20 4.47733 15.5227 0 10 0C4.47733 0 0 4.47733 0 10C0 15.5227 4.47733 20 10 20ZM10 6C9.32464 6 8.77778 6.54687 8.77778 7.22222V7.25434C8.77778 7.68403 8.42969 8.03212 8 8.03212C7.57031 8.03212 7.22222 7.68403 7.22222 7.25434V7.22222C7.22222 5.68838 8.46616 4.44444 10 4.44444C11.5338 4.44444 12.7778 5.68838 12.7778 7.22222V7.34722C12.7778 8.27951 12.3299 9.15538 11.5729 9.70144L11.4549 9.78651C11.0295 10.0929 10.7778 10.5851 10.7778 11.1094V11.6667C10.7778 12.0964 10.4297 12.4445 9.99999 12.4445C9.5703 12.4445 9.22221 12.0964 9.22221 11.6667V11.1094C9.22221 10.0851 9.71439 9.12331 10.5451 8.52431L10.6632 8.43924C11.0139 8.18664 11.2222 7.77951 11.2222 7.34722V7.22222C11.2222 6.54687 10.6754 6 10 6ZM11 14.5782C11 15.1303 10.5521 15.5782 10 15.5782C9.44791 15.5782 9 15.1303 9 14.5782C9 14.0253 9.44791 13.5782 10 13.5782C10.5521 13.5782 11 14.0253 11 14.5782Z"
          fill="#363636"
        />
      </svg>
    </SvgIcon>
  );
};
