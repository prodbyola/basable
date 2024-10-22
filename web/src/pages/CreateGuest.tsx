import "../styles/create-guest.scss";
import * as React from "react";
import Button from "@mui/material/Button";
import { ThemeProvider } from "@mui/material/styles";
import Typography from "@mui/material/Typography";
import Illustration from "../assets/images/connection-illustration.svg?react";
import Gradient1 from "../assets/images/gradient-1.svg?react";
import Gradient2 from "../assets/images/gradient-2.svg?react";
import Gradient3 from "../assets/images/gradient-3.svg?react";
import { ConnectForm } from '../components/forms/ConnectForm'
import theme from "../theme";
import { getCookie } from "../utils";
import { BASABLE_COOKIE_NAME } from "../env";
import { useNavigate } from "react-router-dom";

const CreateGuest = () => {
  const cookie = getCookie(BASABLE_COOKIE_NAME)
  const navigate = useNavigate()

  React.useEffect(() => {
    if(cookie) navigate('/dashboard')
  }, [navigate, cookie])

  return (
    <div className="create-guest">
      <div className="content-container">
        <ThemeProvider theme={theme}>
          <ConnectForm />
          <div className="right-frame">
            <div className="right-container">
              <div className="btn-container">
                <Button color="primary" size="medium" variant="outlined">
                  <Typography>Signup</Typography>
                </Button>
                <Button
                  color="primary"
                  size="medium"
                  variant="contained"
                  sx={{ ml: 3 }}
                >
                  <Typography>Login</Typography>
                </Button>
              </div>
              <Illustration className="right-img" />
              <div className="right-text basable-text">
                <div className="right-title">
                  <span>Connect your database</span>
                </div>
                <div className="right-description">
                  <span>
                    Effortlessly visualize, edit, and manage your database with
                    our
                    <br /> seamless solution.
                  </span>
                </div>
              </div>
            </div>
            <div>
              <Gradient1 className="back-gradient-1 back" />
              <Gradient2 className="back-gradient-2 back" />
              <Gradient3 className="back-gradient-3 back" />
            </div>
          </div>
        </ThemeProvider>
      </div>
    </div>
  );
}

export default CreateGuest;
