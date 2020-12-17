import reactLogo from './logo.svg';
import logo from './rectangle.png';
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Coming soon...Hopefully
        </p>
      </header>
      <footer className="App-footer">
      <img src={reactLogo} alt="logo" height="50"/>
      &copy; {new Date().getFullYear()} Copyright: Joel Anderson
      </footer>
    </div>
  );
}

export default App;
