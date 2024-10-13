import { SvgIcon } from "@mui/material";
import * as React from "react";

const FilterIcon = ({ color = '#434343', size = '24' }) => {
  return (
    <SvgIcon>
      <svg
        width={size}
        height={size}
        viewBox="0 0 16 16"
        fill={color}
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M0.212758 1.61C2.24378 4.2 5.99412 9 5.99412 9V15C5.99412 15.55 6.44657 16 6.99957 16H9.01048C9.56348 16 10.0159 15.55 10.0159 15V9C10.0159 9 13.7562 4.2 15.7872 1.61C16.3 0.95 15.8275 0 14.9929 0H1.00707C0.17254 0 -0.300024 0.95 0.212758 1.61Z"
          fill="#656565"
        />
      </svg>
    </SvgIcon>
  );
};

export default FilterIcon;
