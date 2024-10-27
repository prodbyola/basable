import React from "react";
import "./App.scss";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import CreateGuest from "./pages/CreateGuest";
import ConnectionOverview from "./pages/ConnectionOverview";
import DashboardLayout from "./layouts/DashboardLayout";
import DashboardMain from "./pages/dashboard/DashboardMain";
import DatabaseTable from "./pages/dashboard/DatabaseTable";

const App = () => {
  return (
    <Router>
      <div className="App">
        <Routes>
          <Route path="/connect" element={<CreateGuest />} />
          <Route path="/overview" element={<ConnectionOverview />} />
          <Route path="/dashboard" element={<DashboardLayout />}>
            <Route path="/dashboard" element={<DashboardMain />} />
            <Route
              path="/dashboard/tables/:tableID"
              element={<DatabaseTable />}
            />
          </Route>
        </Routes>
      </div>
    </Router>
  );
};

export default App;
