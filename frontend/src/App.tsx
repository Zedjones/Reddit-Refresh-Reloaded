import logo from './logo.svg';
import './App.css';
import { Provider } from 'urql';
import { client } from './index';
import SidebarDrawer from './components/SidebarDrawer';

function App() {
  return (
    <Provider value={client}>
      <div className="App">
        <header className="App-header">
          <SidebarDrawer />
        </header>
      </div>
    </Provider>
  );
}

export default App;
