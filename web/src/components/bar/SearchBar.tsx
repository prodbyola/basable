// SearchBar.tsx
import * as React from 'react';
import { useState } from 'react';
import TextField from '@mui/material/TextField';
import InputAdornment from '@mui/material/InputAdornment';
import SearchIcon from '@mui/icons-material/Search';

const SearchBar: React.FC = () => {
  const [searchTerm, setSearchTerm] = useState<string>('');

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSearchTerm(event.target.value);
  };

  return (
    <TextField
      value={searchTerm}
      onChange={handleChange}
      variant="outlined"
      placeholder="Search"
      InputProps={{
        endAdornment: (
          <InputAdornment position="end">
            <SearchIcon />
          </InputAdornment>
        )
      }}
      sx={{
        '& .MuiOutlinedInput-root': {
          borderRadius: '10px', // Set the border radius here
          marginLeft: '80px',
          width: '200px'
        }
      }}
    />
  );
};

export default SearchBar;
