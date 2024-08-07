import React from 'react';
import './App.scss';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import CreateGuest from './pages/create-guest';
import ConnectionOverview from './pages/ConnectionOverview';

function App() {
  return (
    <Router>
      <div className="App">
        <Routes>
          <Route path="/" element={<CreateGuest />} />
          <Route path='/overview' element={<ConnectionOverview />} />
        </Routes>
      </div>
    </Router>
  );
}

export default App;
