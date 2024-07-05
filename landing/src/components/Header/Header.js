import React, { useEffect, useState } from 'react';
import './Header.scss';
import Logo from './assets/logo.svg';
import LinkedIn from './assets/LinkedIn.svg';
import X from './assets/X.svg';
import Github from './assets/Github.svg';

const Header = () => {
  const [activeLink, setActiveLink] = useState('');

  useEffect(() => {
    const handleScroll = () => {
      const sections = [
        { id: 'hero', link: 'about' },
        { id: 'features', link: 'features' },
        { id: 'contact', link: 'contact' },
      ];

      const scrollPosition = window.scrollY + window.innerHeight / 2;

      for (let section of sections) {
        const sectionElement = document.getElementById(section.id);
        if (sectionElement) {
          const sectionTop = sectionElement.offsetTop;
          const sectionHeight = sectionElement.offsetHeight;
          if (scrollPosition >= sectionTop && scrollPosition < sectionTop + sectionHeight) {
            setActiveLink(section.link);
            return;
          }
        }
      }

      setActiveLink('');
    };

    window.addEventListener('scroll', handleScroll);
    return () => {
      window.removeEventListener('scroll', handleScroll);
    };
  }, []);

  return (
    <header className="header">
      <div className="container">
        <img src={Logo} alt="Logo" className="logo" />
        <nav className="nav">
          <a href="#about" className={`nav-link ${activeLink === 'about' ? 'active' : ''}`}>About us</a>
          <a href="#features" className={`nav-link ${activeLink === 'features' ? 'active' : ''}`}>Features</a>
          <a href="#contact" className={`nav-link ${activeLink === 'contact' ? 'active' : ''}`}>Contact us</a>
        </nav>
        <div className="header-buttons">
          <button className="btn login">Join now</button>
          <a href="https://www.linkedin.com" target="_blank" rel="noopener noreferrer">
            <img src={LinkedIn} alt="LinkedIn" className="icon" />
          </a>
          <a href="https://www.twitter.com" target="_blank" rel="noopener noreferrer">
            <img src={X} alt="X" className="icon" />
          </a>
          <a href="https://www.github.com" target="_blank" rel="noopener noreferrer">
            <img src={Github} alt="GitHub" className="icon" />
          </a>
        </div>
      </div>
    </header>
  );
};

export default Header;
