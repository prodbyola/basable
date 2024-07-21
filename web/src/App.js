import React from 'react';
import './App.css';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import CreateGuest from './pages/create-guest';

function App() {
  return (
    <Router>
      <div className="App">
        <Routes>
          <Route path="/" element={<CreateGuest />} />
        </Routes>
      </div>
    </Router>
  );
}

export default App;
