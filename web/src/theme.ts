import { createTheme } from "@mui/material";

const theme = createTheme({
    typography: {
        button: {
            fontSize: "16px",
            fontWeight: "400",
            lineHeight: "20px",
            textAlign: "center",
        },
        body1: {
            fontSize: "16px",
            fontWeight: "400",
            lineHeight: "24px",
            textAlign: "left",
        },
    },
    palette: {
        primary: {
            light: "#8991DE",
            dark: "#303A8F",
            main: "#4451CA"
        },
        background: {
            default: "#C4C4C4"
        }
    },
    components: {
        MuiButton: {
            styleOverrides: {
                sizeSmall: {
                    padding: "8px 24px",
                },
                sizeMedium: {
                    width: "111px",
                    height: "48px",
                    padding: "14px 32px",
                },
                root: {
                    textTransform: "initial",
                },
            },
        },
    },
});

export default theme