import './App.css';
import { Provider } from 'urql';
import { client } from './index';
import SidebarDrawer from './components/SidebarDrawer';
import { SnackbarProvider } from 'notistack';
import { SnackbarUtilsConfigurator } from './components/SnackbarUtils';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import SignIn from './components/Login';

function App() {
  return (
    <SnackbarProvider maxSnack={3} anchorOrigin={{ horizontal: 'left', vertical: 'bottom' }}>
      <SnackbarUtilsConfigurator />
      <Provider value={client}>
        <Router>
          <Routes>
            <Route index element={<SidebarDrawer />} />
            <Route path='login' element={<SignIn />} />
            <Route path='signup' element={<SignIn signUp={true} />} />
          </Routes>
        </Router>
      </Provider>
    </ SnackbarProvider >
  );
}

export default App;
