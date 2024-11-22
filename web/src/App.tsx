import './App.scss';
import './components/forms/index.scss'
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import CreateGuest from './pages/CreateGuest';
import ConnectionOverview from './pages/ConnectionOverview';
import DashboardLayout from './layouts/DashboardLayout';
import DashboardMain from './pages/dashboard/DashboardMain';
import DatabaseTable from './pages/dashboard/DatabaseTable';
import AuthPage from './pages/AuthPage';
import LoginPage from './pages/LoginPage';
import ForgotPassword from './pages/ForgotPassword';
import Welcome from './pages/Welcome';

const App = () => {
  return (
    <Router>
      <div className="App">
        <Routes>
          <Route path="/" element={<AuthPage />} />
          <Route path="/login" element={<LoginPage />} />
          <Route path="/forgotpassword" element={<ForgotPassword />} />
          <Route path="/welcome" element={<Welcome />} />

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
