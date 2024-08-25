import React from 'react';
import './App.scss';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import CreateGuest from './pages/CreateGuest';
import ConnectionOverview from './pages/ConnectionOverview';
import DashboardLayout from './layouts/DashboardLayout';

function App() {
  return (
    <Router>
      <div className="App">
        <Routes>
          <Route path="/" element={<CreateGuest />} />
          <Route path="/overview" element={<ConnectionOverview />} />
          <Route path="/dashboard" element={<DashboardLayout />} />
        </Routes>
      </div>
    </Router>
  );
}

export default App;
