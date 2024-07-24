import '../styles/create-guest.scss';
import * as React from 'react';
import Button from '@mui/material/Button';
import { createTheme, ThemeProvider } from '@mui/material/styles';
import Typography from '@mui/material/Typography';
import logo from '../assets/images/basable-logo.svg';
import illustration from '../assets/images/connection-illustration.svg';
import gradient1 from '../assets/images/gradient-1.svg';
import gradient2 from '../assets/images/gradient-2.svg';
import gradient3 from '../assets/images/gradient-3.svg';
import IconButton from '@mui/material/IconButton';
import OutlinedInput from '@mui/material/OutlinedInput';
import InputAdornment from '@mui/material/InputAdornment';
import Visibility from '@mui/icons-material/Visibility';
import VisibilityOff from '@mui/icons-material/VisibilityOff';
import Select from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';

const theme = createTheme({
  typography: {
    button_medium: {
      fontSize: '16px',
      fontWeight: '400',
      lineHeight: '20px',
      textAlign: 'center'
    },

    button_normal: {
      fontSize: '18px',
      fontWeight: '400',
      lineHeight: '24px',
      textAlign: 'center'
    },
    field_text1: {
      fontFamily: 'Inter',
      fontSize: '16px',
      fontWeight: '400',
      lineHeight: '24px',
      textAlign: 'left'
    }
  },
  palette: {
    primary: {
      primary100: '#eceefa',
      primary200: '#d2d5f2',
      primary300: '#afb4e8',
      primary400: '#8991de',
      primary500: '#6670d4',
      main: '#4451ca',
      primary700: '#3a45ac',
      primary800: '#303a8f'
    }
  },
  components: {
    MuiButton: {
      styleOverrides: {
        sizeSmall: {
          padding: '8px 24px'
        },
        sizeMedium: {
          width: '111px',
          height: '48px',
          padding: '14px 32px'
        },
        sizeNormal: {
          width: '111px',
          height: '56px',
          padding: '16px 32px'
        },
        root: {
          textTransform: 'initial'
        }
      }
    }
  }
});

function CreateGuest() {
  const [showPassword, setShowPassword] = React.useState(false);
  const [dbType, setDbType] = React.useState('0');
  const [srcType, setSrcType] = React.useState('1');
  const handleClickShowPassword = () => setShowPassword((show) => !show);

  const handleMouseDownPassword = (event) => {
    event.preventDefault();
  };

  const handleDbChange = (event) => {
    setDbType(event.target.value);
  };
  const handleSrcChange = (event) => {
    setSrcType(event.target.value);
  };

  return (
    <div className="create-guest">
      <div className="content-container">
        <ThemeProvider theme={theme}>
          <div className="left-frame">
            <div className="left-header">
              <img src={logo} alt="basable-logo" />
              <div className="basable-text">
                <span className="title">Connect your database</span>
                <span className="description">
                  Connect your database today and start your data exploration
                  <br />
                  journey with Basable.
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
                <div className="user-pass-input">
                  <div className="user-input">
                    <label>
                      Username <span>*</span>
                    </label>
                    <OutlinedInput
                      sx={{
                        width: 1,
                        typography: 'field_text1'
                      }}
                      placeholder="Enter username"
                      type="text"
                    />
                  </div>
                  <div className="pass-input">
                    <label>
                      Password <span>*</span>
                    </label>
                    <OutlinedInput
                      sx={{
                        width: 1,
                        typography: 'field_text1'
                      }}
                      placeholder="Enter password"
                      id="outlined-adornment-password"
                      type={showPassword ? 'text' : 'password'}
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
                    typography: 'field_text1'
                  }}
                  placeholder="Enter database name"
                  type="text"
                />
              </div>
              <div className="databasetype-host-div">
                <div className="database-type-div">
                  <label>
                    Database Type <span>*</span>
                  </label>
                  <Select
                    sx={{
                      width: 1,
                      typography: 'field_text1'
                    }}
                    value={dbType}
                    onChange={handleDbChange}
                  >
                    <MenuItem disabled value="0">
                      <em>Select database type</em>
                    </MenuItem>
                    <MenuItem value={10}>MongoDB</MenuItem>
                    <MenuItem value={20}>MySQL</MenuItem>
                    <MenuItem value={30}>PostgreSQL</MenuItem>
                  </Select>
                </div>
                <div className="host-div">
                  <label>
                    Host <span>*</span>
                  </label>
                  <OutlinedInput
                    sx={{
                      width: 1,
                      typography: 'field_text1'
                    }}
                    placeholder="Enter Host name"
                    type="text"
                  />
                </div>
              </div>
              <div className="port-source-div">
                <div className="port-div">
                  <label>
                    Port <span>*</span>
                  </label>
                  <OutlinedInput
                    sx={{
                      width: 1,
                      typography: 'field_text1'
                    }}
                    placeholder="Enter port type"
                    type="text"
                  />
                </div>
                <div className="source-div">
                  <label>
                    Source Type <span>*</span>
                  </label>

                  <Select
                    sx={{
                      width: 1,
                      typography: 'field_text1'
                    }}
                    value={srcType}
                    onChange={handleSrcChange}
                  >
                    <MenuItem disabled value="1">
                      <em>Select source type</em>
                    </MenuItem>
                    <MenuItem value={2}>Type1</MenuItem>
                    <MenuItem value={3}>Type2</MenuItem>
                    <MenuItem value={4}>Type3</MenuItem>
                  </Select>
                </div>
              </div>
              <div className="submit-div">
                <Button
                  color="primary"
                  size="normal"
                  sx={{ width: 1 }}
                  variant="contained"
                >
                  <Typography variant="button_normal">
                    Create guest connection
                  </Typography>
                </Button>
                <div className="submit-text">
                  Are you willing to access more features?
                  <a> Login/Sign up</a>
                </div>
              </div>
            </div>
          </div>
          <div className="right-frame">
            <div className="right-container">
              <div className="btn-container">
                <Button color="primary" size="medium" variant="outlined">
                  <Typography variant="button_medium">Signup</Typography>
                </Button>
                <Button
                  color="primary"
                  size="medium"
                  variant="contained"
                  sx={{ ml: 3 }}
                >
                  <Typography variant="button_medium">Login</Typography>
                </Button>
              </div>
              <img
                src={illustration}
                alt="illustration"
                className="right-img"
              />
              <div className="right-text">
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
              <img src={gradient1} alt="" className="back-gradient-1 back" />
              <img src={gradient2} alt="" className="back-gradient-2 back" />
              <img src={gradient3} alt="" className="back-gradient-3 back" />
            </div>
          </div>
        </ThemeProvider>
      </div>
    </div>
  );
}

export default CreateGuest;
