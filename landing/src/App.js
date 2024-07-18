import React from 'react';
import Header from './components/Header';
import Hero from './components/Hero';
import './App.css';
import Footer from './components/footer/Footer';
import Contact from './components/contact/Contact';


const App = () => {
  return (
    <div className="App">
      <Header />
      <Hero />
      <Contact />
      <Footer  />
    </div>
  );
};

export default App;
