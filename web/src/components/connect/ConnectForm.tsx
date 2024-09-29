import * as React from "react";
import { useNavigate } from "react-router-dom";

import { ReactComponent as Logo } from "../../assets/images/basable-logo.svg";
import IconButton from "@mui/material/IconButton";
import OutlinedInput from "@mui/material/OutlinedInput";
import InputAdornment from "@mui/material/InputAdornment";
import Visibility from "@mui/icons-material/Visibility";
import VisibilityOff from "@mui/icons-material/VisibilityOff";
import Select from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";

import { BASABLE_COOKIE_NAME, BASE_URL } from "../../env";
import axios from "axios";
import { AuthTokenType } from "../../utils/data_types";
import { setCookie } from "../../utils";
import Snackbar, { SnackbarCloseReason } from "@mui/material/Snackbar";
import Alert from "@mui/material/Alert/Alert";

type ConnectInput = {
  username: string;
  password: string;
  host: string;
  port: number;
  db_name: string;
  source_type: string;
  source: string;
};

export const ConnectForm = () => {
  const navigate = useNavigate();

  const [connInfo, updateConnectInfo] = React.useState<Partial<ConnectInput>>({
    source: "0",
    source_type: "0",
    username: "",
    password: "",
    db_name: "",
    host: "localhost",
    port: 3306,
  });

  const [alertState, updateAlertState] = React.useState({
    opened: false,
    message: "",
    color: "success" as "success" | "error" | "info" | "warning",
  });

  const closeAlert = (
    event?: React.SyntheticEvent | Event,
    reason?: SnackbarCloseReason
  ) => {
    if (reason === "clickaway") {
      return;
    }

    updateAlertState((prevData) => ({
      ...prevData,
      opened: false,
    }));
  };

  const connect = async () => {
    updateAlertState((prevData) => ({
      ...prevData,
      opened: false,
    }));

    try {
      const access: AuthTokenType = await axios
        .post(BASE_URL + "auth/guest")
        .then((resp) => resp.data);

      const exp = access.exp / 86_400;

      setCookie(BASABLE_COOKIE_NAME, access.token, exp);
      console.log(connInfo)

      const conn = await axios
        .post(BASE_URL + "connect", connInfo, {
          headers: {
            "b-session-id": "Bearer " + access.token,
          },
        })
        .then((resp) => resp.data);

      updateAlertState((prevData) => ({
        ...prevData,
        opened: true,
        color: "success",
        message: "Connection successful! Redirecting to dashboard...",
      }));

      console.log(conn);
    } catch (err: any) {
      const message = err.response.data;
      updateAlertState((prevData) => ({
        ...prevData,
        opened: true,
        color: "error",
        message,
      }));
    }
  };

  const onChangeInput = (evt: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = evt.target;
    if (value) {
      updateConnectInfo((prevData) => ({
        ...prevData,
        [name]: value,
      }));
    }
  };

  const [showPassword, setShowPassword] = React.useState(false);
  const handleClickShowPassword = () => setShowPassword((show) => !show);

  const handleMouseDownPassword = (event) => {
    event.preventDefault();
  };

  return (
    <div className="left-frame">
      <div className="left-header">
        <Logo />
        <div className="basable-text">
          <span className="title">Connect your database</span>
          <span className="description">
            Connect your database today and start your data exploration journey
            with Basable.
          </span>
        </div>
      </div>
      <div className="form-container">
        <div className="user-pass-div">
          <div className="note-div">
            <span className="note-name">Note: </span>
            <span className="note-text">
              Enter username and password used when creating your database
            </span>
          </div>
          <div className="databasetype-host-div">
            <div className="source-div">
              <label>
                Source Type<span>*</span>
              </label>

              <Select
                sx={{
                  width: 1,
                  typography: "field_text1",
                }}
                value={connInfo.source_type}
                onChange={onChangeInput}
                name="source_type"
              >
                <MenuItem disabled value="0">
                  {" "}
                  Select Source Type
                </MenuItem>
                <MenuItem value="database">Database</MenuItem>
                <MenuItem value="file">File</MenuItem>
                <MenuItem value="cloud">Cloud</MenuItem>
              </Select>
            </div>
            <div className="database-type-div">
              <label>
                Database Type <span>*</span>
              </label>
              <Select
                sx={{
                  width: 1,
                  typography: "field_text1",
                }}
                value={connInfo.source}
                onChange={onChangeInput}
                name="source"
              >
                <MenuItem disabled value="0">
                  Select Database Type
                </MenuItem>
                <MenuItem value="mongo">MongoDB</MenuItem>
                <MenuItem value="mysql">MySQL</MenuItem>
                <MenuItem value="postgress">PostgreSQL</MenuItem>
              </Select>
            </div>
          </div>
          <div className="user-pass-input">
            <div className="user-input">
              <label>
                Username <span>*</span>
              </label>
              <OutlinedInput
                sx={{
                  width: 1,
                  typography: "field_text1",
                }}
                placeholder="Enter username"
                name="username"
                value={connInfo.username}
                onChange={onChangeInput}
              />
            </div>
            <div className="pass-input">
              <label>
                Password <span>*</span>
              </label>
              <OutlinedInput
                sx={{
                  width: 1,
                  typography: "field_text1",
                }}
                placeholder="Enter password"
                id="outlined-adornment-password"
                name="password"
                value={connInfo.password}
                type={showPassword ? "text" : "password"}
                onChange={onChangeInput}
                endAdornment={
                  <InputAdornment position="end">
                    <IconButton
                      aria-label="toggle password visibility"
                      onClick={handleClickShowPassword}
                      onMouseDown={handleMouseDownPassword}
                      edge="end"
                    >
                      {showPassword ? <VisibilityOff /> : <Visibility />}
                    </IconButton>
                  </InputAdornment>
                }
              />
            </div>
          </div>
        </div>
        <div className="database-name-div">
          <label>
            Database name <span>*</span>
          </label>
          <OutlinedInput
            sx={{
              width: 1,
              typography: "field_text1",
            }}
            placeholder="Enter database name"
            name="db_name"
            value={connInfo.db_name}
            onChange={onChangeInput}
          />
        </div>
        <div className="port-source-div">
          <div className="port-div">
            <label>
              Port <span>*</span>
            </label>
            <OutlinedInput
              sx={{
                width: 1,
                typography: "field_text1",
              }}
              placeholder="Enter port type"
              name="port"
              value={connInfo.port}
              type="number"
              onChange={onChangeInput}
            />
          </div>
          <div className="host-div">
            <label>
              Host <span>*</span>
            </label>
            <OutlinedInput
              sx={{
                width: 1,
                typography: "field_text1",
              }}
              placeholder="Enter Host name"
              name="host"
              value={connInfo.host}
              onChange={onChangeInput}
            />
          </div>
        </div>
        <div className="submit-div">
          <Button
            color="primary"
            sx={{ width: 1 }}
            variant="contained"
            onClick={() => connect()}
          >
            <Typography>Create Guest Connection</Typography>
          </Button>
          <div className="submit-text">
            Are you willing to access more features?
            <a href="/#"> Login/Sign up</a>
          </div>
        </div>
        <Snackbar
          anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
          open={alertState.opened}
          autoHideDuration={5000}
          onClose={closeAlert}
        >
          <Alert
            onClose={closeAlert}
            severity={alertState.color}
            variant="filled"
            sx={{ width: "100%" }}
          >
            {alertState.message}
          </Alert>
        </Snackbar>
      </div>
    </div>
  );
};
