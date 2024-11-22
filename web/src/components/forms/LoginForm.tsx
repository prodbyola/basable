// web/src/components/forms/AuthForm.tsx
import React, { useState } from "react";
import {
  Box,
  Checkbox,
  Button,
  TextField,
  IconButton,
  InputAdornment,
  Typography,
} from "@mui/material";
import { Visibility, VisibilityOff } from "@mui/icons-material";
import { Link, useNavigate } from "react-router-dom";

const AuthForm: React.FC = () => {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const navigate = useNavigate();

  const handleClickShowPassword = () => setShowPassword(!showPassword);
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    console.log("Email:", email, "Password:", password);

    navigate("/welcome");
  };

  return (
    <form onSubmit={handleSubmit}>
      <TextField
        label="Email"
        type="email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
        fullWidth
        margin="normal"
        sx={{
          "& .MuiOutlinedInput-root": {
            borderRadius: "10px", // makes the border circular
          },
        }}
      />
      <TextField
        label="Password"
        type={showPassword ? "text" : "password"}
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        fullWidth
        margin="normal"
        InputProps={{
          endAdornment: (
            <InputAdornment position="end">
              <IconButton onClick={handleClickShowPassword} edge="end">
                {showPassword ? <VisibilityOff /> : <Visibility />}
              </IconButton>
            </InputAdornment>
          ),
        }}
        sx={{
          "& .MuiOutlinedInput-root": {
            borderRadius: "10px", // makes the border circular
          },
        }}
      />

      <Box
        display="flex"
        justifyContent="space-between"
        alignItems="center"
        width="100%"
        mt={1}
      >
        <Box display="flex" alignItems="center">
          <Checkbox
            color="primary"
            sx={{
              "& .MuiSvgIcon-root": {
                color: "gray",
              },
              "&.Mui-checked .MuiSvgIcon-root": {
                color: "primary.main", // Set checked color to primary
              },
            }}
          />
          <Typography variant="body2">Remember me</Typography>
        </Box>
        <Link
          to="/forgotpassword"
          // variant="body2"
          color="primary"
          // underline="none"
        >
          Forgot Password?
        </Link>
      </Box>

      <Button
        type="submit"
        variant="contained"
        fullWidth
        sx={{
          backgroundColor: "#5a5fcf",
          color: "#fff",
          textTransform: "none",
          my: 2,
        }}
      >
        Login
      </Button>
      <Typography variant="caption" color="textSecondary">
        By signing up, you agree to our{" "}
        <Link to="#">
          Terms and <br /> Service
        </Link>{" "}
        and <Link to="#">Privacy Policy</Link>.
      </Typography>

      <Typography variant="body2" color="black" sx={{ mt: 3 }}>
        Don't have an account yet?{" "}
        <Link to="/" color="primary">
          Sign up
        </Link>
        {" or "}
        <Link to="/connect">Login as Guest</Link>
      </Typography>
    </form>
  );
};

export default AuthForm;
