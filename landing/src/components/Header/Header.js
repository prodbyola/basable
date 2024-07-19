import React, { useEffect, useState, useRef } from 'react';
import './Header.scss';
import Logo from './assets/logo.svg';
import LinkedIn from './assets/LinkedIn.svg';
import X from './assets/X.svg';
import Github from './assets/Github.svg';
import HamburgerIcon from './assets/Hamburger.png';

const Header = () => {
  const [activeLink, setActiveLink] = useState('');
  const [menuOpen, setMenuOpen] = useState(false);
  const navRef = useRef(null);
  const buttonsRef = useRef(null);
  const mobileViewRef = useRef(null);

  useEffect(() => {
    const handleScroll = () => {
      const sections = [
        { id: 'about', link: 'about' },
        { id: 'features', link: 'features' },
        { id: 'contact', link: 'contact' }
      ];

      const scrollPosition = window.scrollY + window.innerHeight / 2;

      for (let section of sections) {
        const sectionElement = document.getElementById(section.id);
        if (sectionElement) {
          const sectionTop = sectionElement.offsetTop;
          const sectionHeight = sectionElement.offsetHeight;
          if (
            scrollPosition >= sectionTop &&
            scrollPosition < sectionTop + sectionHeight
          ) {
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

  useEffect(() => {
    const handleResize = () => {
      const mobileView = mobileViewRef.current;
      if (window.innerWidth <= 639) {
        if (mobileView && navRef.current && buttonsRef.current) {
          mobileView.appendChild(navRef.current);
          mobileView.appendChild(buttonsRef.current);
        }
      } else {
        if (mobileView && navRef.current && buttonsRef.current) {
          mobileView.parentElement.insertBefore(navRef.current, mobileView);
          mobileView.parentElement.insertBefore(buttonsRef.current, mobileView);
        }
      }
    };

    window.addEventListener('resize', handleResize);
    handleResize();

    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }, []);

  return (
    <header className="header">
      <div className="container">
        <img src={Logo} alt="Logo" className="logo" />
        <nav ref={navRef} className="nav">
          <a
            href="#about"
            className={`nav-link ${activeLink === 'about' ? 'active' : ''}`}
          >
            About us
          </a>
          <a
            href="#features"
            className={`nav-link ${activeLink === 'features' ? 'active' : ''}`}
          >
            Features
          </a>
          <a
            href="#contact"
            className={`nav-link ${activeLink === 'contact' ? 'active' : ''}`}
          >
            Contact us
          </a>
        </nav>
        <div ref={buttonsRef} className="header-buttons">
          <button className="btn login">Join now</button>
          <a
            href="https://www.linkedin.com"
            target="_blank"
            rel="noopener noreferrer"
          >
            <img src={LinkedIn} alt="LinkedIn" className="icon" />
          </a>
          <a
            href="https://www.twitter.com"
            target="_blank"
            rel="noopener noreferrer"
          >
            <img src={X} alt="X" className="icon" />
          </a>
          <a
            href="https://www.github.com"
            target="_blank"
            rel="noopener noreferrer"
          >
            <img src={Github} alt="GitHub" className="icon" />
          </a>
        </div>
        <img
          src={HamburgerIcon}
          alt="Menu"
          className="hamburger-icon"
          onClick={() => setMenuOpen(!menuOpen)}
        />
        <div
          ref={mobileViewRef}
          className={`mobile-view ${menuOpen ? 'open' : ''}`}
        ></div>
      </div>
    </header>
  );
};

export default Header;
