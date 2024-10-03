import { ThemeProvider } from "@emotion/react";
import { ArrowForwardIos } from "@mui/icons-material";
import * as React from "react";
import theme from "../theme";
import { Card } from "@mui/material";

export type CardDetails = {
  label: string;
  value: number | string;
  action?: string;
}

export const DashboardCard = (props: CardDetails) => {
  return (
    <ThemeProvider theme={theme}>
      <Card className="dashCard" sx={{
        width: {
            sm: '48%',
            xs: '100%',
            md: '24%',
        }
      }}>
        <div className="dashCardUpperPart">
          <p className="dashCardLabel">{props.label}</p>
          {props.action && (
            <div className="dashCardAction">
              <p className="dashCardActionLabel">{props.action}</p>
              <ArrowForwardIos
                sx={{
                  height: "16px",
                  width: "16px",
                  fill: (theme) => theme.palette.primary.main,
                }}
              />
            </div>
          )}
        </div>
        <h3 className="dashCardValue">{props.value}</h3>
      </Card>
    </ThemeProvider>
  );
};
